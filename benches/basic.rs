use broomfilter::Filter;
use std::env;
use std::hint::black_box;
use std::time::{Duration, Instant};

// Run the focused in-house optimization bench:
//   cargo bench --bench basic
//
// Profile a specific workload with cargo-flamegraph:
//   cargo flamegraph --bench basic -- --scenario scale-4096 --workload contains-mixed
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
enum Workload {
    Build,
    ContainsMember,
    ContainsAbsent,
    ContainsMixed,
}

impl Workload {
    const ALL: [Workload; 4] = [
        Workload::Build,
        Workload::ContainsMember,
        Workload::ContainsAbsent,
        Workload::ContainsMixed,
    ];

    fn parse(value: &str) -> Option<Self> {
        match value {
            "build" => Some(Self::Build),
            "contains-member" => Some(Self::ContainsMember),
            "contains-absent" => Some(Self::ContainsAbsent),
            "contains-mixed" => Some(Self::ContainsMixed),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::ContainsMember => "contains-member",
            Self::ContainsAbsent => "contains-absent",
            Self::ContainsMixed => "contains-mixed",
        }
    }
}

const SCENARIOS: [Scenario; 2] = [
    Scenario {
        name: "compact-128",
        inserted_items: 128,
        shared_filter_bits: 2_048,
        query_batch_size: 4_096,
        build_iterations: 200_000,
        contains_iterations: 10_000,
    },
    Scenario {
        name: "scale-4096",
        inserted_items: 4_096,
        shared_filter_bits: 65_536,
        query_batch_size: 16_384,
        build_iterations: 10_000,
        contains_iterations: 5_000,
    },
];

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
    workload: Workload,
    iterations: usize,
) -> (Duration, usize, usize) {
    let filter = build_filter(&data.members, scenario);
    let queries = match workload {
        Workload::ContainsMember => &data.member_queries,
        Workload::ContainsAbsent => &data.absent_queries,
        Workload::ContainsMixed => &data.mixed_queries,
        Workload::Build => unreachable!("build is not a contains workload"),
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
    workload: Workload,
    iterations: usize,
    total_operations: usize,
    duration: Duration,
) {
    let total_ns = duration.as_secs_f64() * 1_000_000_000.0;
    let ns_per_op = total_ns / total_operations as f64;

    println!(
        "scenario={} workload={} iterations={} total_ops={} total_time={:.3?} ns/op={:.2}",
        scenario.name,
        workload.name(),
        iterations,
        total_operations,
        duration,
        ns_per_op,
    );
}

fn print_usage_and_exit() -> ! {
    eprintln!(
        "usage: cargo bench --bench basic -- [--scenario <name>] [--workload <name>] [--iterations <n>]"
    );
    eprintln!("scenarios: compact-128, scale-4096");
    eprintln!("workloads: build, contains-member, contains-absent, contains-mixed");
    std::process::exit(2);
}

fn parse_args() -> (Option<String>, Option<Workload>, Option<usize>) {
    let mut scenario = None;
    let mut workload = None;
    let mut iterations = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bench" => {}
            "--scenario" => {
                scenario = Some(args.next().unwrap_or_else(|| print_usage_and_exit()));
            }
            "--workload" => {
                let value = args.next().unwrap_or_else(|| print_usage_and_exit());
                workload = Workload::parse(&value).or_else(|| {
                    eprintln!("unknown workload: {value}");
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

    (scenario, workload, iterations)
}

fn scenario_by_name(name: &str) -> Option<Scenario> {
    SCENARIOS
        .iter()
        .copied()
        .find(|scenario| scenario.name == name)
}

fn main() {
    let (scenario_filter, workload_filter, iteration_override) = parse_args();

    let scenarios: Vec<Scenario> = if let Some(name) = scenario_filter.as_deref() {
        vec![scenario_by_name(name).unwrap_or_else(|| {
            eprintln!("unknown scenario: {name}");
            print_usage_and_exit();
        })]
    } else {
        SCENARIOS.to_vec()
    };

    for scenario in scenarios {
        let data = prepare_scenario_data(scenario);
        let workloads: Vec<Workload> = if let Some(workload) = workload_filter {
            vec![workload]
        } else {
            Workload::ALL.to_vec()
        };

        for workload in workloads {
            match workload {
                Workload::Build => {
                    let iterations = iteration_override.unwrap_or(scenario.build_iterations);
                    let (duration, total_operations) = time_build(scenario, &data, iterations);
                    print_result(scenario, workload, iterations, total_operations, duration);
                }
                Workload::ContainsMember | Workload::ContainsAbsent | Workload::ContainsMixed => {
                    let iterations = iteration_override.unwrap_or(scenario.contains_iterations);
                    let (duration, total_operations, _) =
                        time_contains(scenario, &data, workload, iterations);
                    print_result(scenario, workload, iterations, total_operations, duration);
                }
            }
        }
    }
}
