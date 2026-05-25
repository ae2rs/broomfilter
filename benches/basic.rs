use broomfilter::Filter;
use std::env;
use std::hint::black_box;
use std::time::{Duration, Instant};

// Run the focused in-house optimization bench:
//   cargo bench --features benchmarks --bench basic
//
// Profile insert-only:
//   cargo flamegraph --features benchmarks --bench basic -- --mode insert
//
// Profile present-only:
//   cargo flamegraph --features benchmarks --bench basic -- --mode present
//
// Profile absent-only:
//   cargo flamegraph --features benchmarks --bench basic -- --mode absent
//
// Profile mixed reads on a larger scenario:
//   cargo flamegraph --features benchmarks --bench basic -- --mode mixed --scenario stress-65536-2bpi --iterations 100000
// The flamegraph will be written to flamegraph.svg in the project root by default.

#[derive(Clone, Copy)]
struct Key {
    bytes: [u8; 16],
}

#[derive(Clone, Copy)]
struct Scenario {
    name: &'static str,
    inserted_items: usize,
    shared_filter_bits: usize,
    query_batch_size: usize,
    build_iterations: usize,
    contains_iterations: usize,
}

struct ScenarioData {
    members: Vec<Key>,
    member_queries: Vec<Key>,
    absent_queries: Vec<Key>,
    mixed_queries: Vec<Key>,
}

#[derive(Clone, Copy)]
enum Mode {
    Insert,
    Present,
    Absent,
    Mixed,
}

impl Mode {
    const ALL: [Mode; 4] = [Mode::Insert, Mode::Present, Mode::Absent, Mode::Mixed];

    fn parse(value: &str) -> Option<Self> {
        match value {
            "insert" => Some(Self::Insert),
            "present" => Some(Self::Present),
            "absent" => Some(Self::Absent),
            "mixed" | "read" => Some(Self::Mixed),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Present => "present",
            Self::Absent => "absent",
            Self::Mixed => "mixed",
        }
    }
}

const SCENARIOS: [Scenario; 4] = [
    Scenario {
        name: "compact-128",
        inserted_items: 128,
        shared_filter_bits: 2_048,
        query_batch_size: 4_096,
        build_iterations: 3_000_000,
        contains_iterations: 100_000,
    },
    Scenario {
        name: "scale-4096",
        inserted_items: 4_096,
        shared_filter_bits: 65_536,
        query_batch_size: 16_384,
        build_iterations: 150_000,
        contains_iterations: 40_000,
    },
    Scenario {
        name: "large-65536-16bpi",
        inserted_items: 65_536,
        shared_filter_bits: 1_048_576,
        query_batch_size: 65_536,
        build_iterations: 12_000,
        contains_iterations: 6_000,
    },
    Scenario {
        name: "stress-65536-2bpi",
        inserted_items: 65_536,
        shared_filter_bits: 131_072,
        query_batch_size: 65_536,
        build_iterations: 30_000,
        contains_iterations: 10_000,
    },
];

const DEFAULT_SCENARIO: &str = "scale-4096";

fn make_key(id: u64) -> Key {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&id.to_le_bytes());
    bytes[8..].copy_from_slice(&id.wrapping_mul(0x9E37_79B9_7F4A_7C15).to_le_bytes());
    Key { bytes }
}

fn prepare_scenario_data(scenario: Scenario) -> ScenarioData {
    let members: Vec<Key> = (0..scenario.inserted_items as u64).map(make_key).collect();

    let member_queries: Vec<Key> = (0..scenario.query_batch_size)
        .map(|index| members[index % members.len()])
        .collect();

    let absent_queries: Vec<Key> = (0..scenario.query_batch_size)
        .map(|index| make_key(10_000_000 + index as u64))
        .collect();

    let mixed_queries: Vec<Key> = (0..scenario.query_batch_size)
        .map(|index| {
            if index % 2 == 0 {
                members[index % members.len()]
            } else {
                make_key(20_000_000 + index as u64)
            }
        })
        .collect();

    ScenarioData {
        members,
        member_queries,
        absent_queries,
        mixed_queries,
    }
}

fn build_filter(members: &[Key], scenario: Scenario) -> Filter {
    let exponent = scenario.shared_filter_bits.trailing_zeros() as usize;
    let mut filter = Filter::new(exponent, members.len()).expect("unable to create filter");

    for key in members {
        filter.insert(&key.bytes);
    }

    filter
}

fn time_build(scenario: Scenario, data: &ScenarioData, iterations: usize) -> (Duration, usize) {
    let start = Instant::now();

    for _ in 0..iterations {
        black_box(build_filter(&data.members, scenario));
    }

    (start.elapsed(), iterations * scenario.inserted_items)
}

fn time_contains(
    scenario: Scenario,
    data: &ScenarioData,
    mode: Mode,
    iterations: usize,
) -> (Duration, usize, usize) {
    let filter = build_filter(&data.members, scenario);
    let queries = match mode {
        Mode::Present => &data.member_queries,
        Mode::Absent => &data.absent_queries,
        Mode::Mixed => &data.mixed_queries,
        Mode::Insert => unreachable!("insert mode does not have a contains workload"),
    };

    let start = Instant::now();
    let mut total_hits = 0usize;

    for _ in 0..iterations {
        let hits = queries
            .iter()
            .filter(|key| filter.contains(&key.bytes))
            .count();
        total_hits = total_hits.wrapping_add(hits);
    }

    black_box(total_hits);
    (start.elapsed(), iterations * queries.len(), total_hits)
}

fn print_result(
    scenario: Scenario,
    mode: Mode,
    iterations: usize,
    total_operations: usize,
    duration: Duration,
) {
    let total_ns = duration.as_secs_f64() * 1_000_000_000.0;
    let ns_per_op = total_ns / total_operations as f64;

    println!(
        "scenario={} mode={} iterations={} total_ops={} total_time={:.3?} ns/op={:.2}",
        scenario.name,
        mode.name(),
        iterations,
        total_operations,
        duration,
        ns_per_op,
    );
}

fn print_usage_and_exit() -> ! {
    eprintln!(
        "usage: cargo bench --features benchmarks --bench basic -- [--mode <name>] [--scenario <name>] [--iterations <n>]"
    );
    eprintln!("modes: insert, present, absent, mixed");
    eprintln!("scenarios: compact-128, scale-4096, large-65536-16bpi, stress-65536-2bpi");
    eprintln!("defaults: mode=all scenario=scale-4096 (legacy alias: read=mixed)");
    std::process::exit(2);
}

fn parse_args() -> (Option<String>, Option<Mode>, Option<usize>) {
    let mut scenario = None;
    let mut mode = None;
    let mut iterations = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bench" => {}
            "--mode" => {
                let value = args.next().unwrap_or_else(|| print_usage_and_exit());
                mode = Mode::parse(&value).or_else(|| {
                    eprintln!("unknown mode: {value}");
                    print_usage_and_exit();
                });
            }
            "--scenario" => {
                scenario = Some(args.next().unwrap_or_else(|| print_usage_and_exit()));
            }
            "--workload" => {
                let value = args.next().unwrap_or_else(|| print_usage_and_exit());
                mode = match value.as_str() {
                    "build" => Some(Mode::Insert),
                    "contains-member" => Some(Mode::Present),
                    "contains-absent" => Some(Mode::Absent),
                    "contains-mixed" => Some(Mode::Mixed),
                    _ => None,
                }
                .or_else(|| {
                    eprintln!(
                        "unsupported legacy workload: {value} (supported aliases: build, contains-member, contains-absent, contains-mixed)"
                    );
                    print_usage_and_exit();
                });
            }
            "--iterations" => {
                let value = args.next().unwrap_or_else(|| print_usage_and_exit());
                iterations = Some(value.parse().unwrap_or_else(|_| {
                    eprintln!("invalid iteration count: {value}");
                    print_usage_and_exit();
                }));
            }
            "--help" | "-h" => print_usage_and_exit(),
            _ => {
                eprintln!("unknown argument: {arg}");
                print_usage_and_exit();
            }
        }
    }

    (scenario, mode, iterations)
}

fn scenario_by_name(name: &str) -> Option<Scenario> {
    SCENARIOS
        .iter()
        .copied()
        .find(|scenario| scenario.name == name)
}

fn main() {
    let (scenario_filter, mode_filter, iteration_override) = parse_args();

    let scenarios: Vec<Scenario> = if let Some(name) = scenario_filter.as_deref() {
        vec![scenario_by_name(name).unwrap_or_else(|| {
            eprintln!("unknown scenario: {name}");
            print_usage_and_exit();
        })]
    } else {
        vec![scenario_by_name(DEFAULT_SCENARIO).expect("default scenario must exist")]
    };

    for scenario in scenarios {
        let data = prepare_scenario_data(scenario);
        let modes: Vec<Mode> = if let Some(mode) = mode_filter {
            vec![mode]
        } else {
            Mode::ALL.to_vec()
        };

        for mode in modes {
            match mode {
                Mode::Insert => {
                    let iterations = iteration_override.unwrap_or(scenario.build_iterations);
                    let (duration, total_operations) = time_build(scenario, &data, iterations);
                    print_result(scenario, mode, iterations, total_operations, duration);
                }
                Mode::Present | Mode::Absent | Mode::Mixed => {
                    let iterations = iteration_override.unwrap_or(scenario.contains_iterations);
                    let (duration, total_operations, _) =
                        time_contains(scenario, &data, mode, iterations);
                    print_result(scenario, mode, iterations, total_operations, duration);
                }
            }
        }
    }
}
