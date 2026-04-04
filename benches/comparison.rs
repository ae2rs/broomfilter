use bloom::{ASMS as _, BloomFilter as LegacyBloom, optimal_num_hashes as legacy_optimal_num_hashes};
use bloomfilter::Bloom;
use bloomfilter::reexports::siphasher::sip::SipHasher13;
use broomfilter::Filter as BroomFilter;
use criterion::{BenchmarkId, Criterion, Throughput};
use fastbloom::BloomFilter as FastBloom;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;
use std::hash::BuildHasher;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

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
    precision_queries: usize,
}

struct ScenarioData {
    members: Vec<Key>,
    member_queries: Vec<Key>,
    absent_queries: Vec<Key>,
    mixed_queries: Vec<Key>,
    precision_queries: Vec<Key>,
}

#[derive(Clone)]
struct AccuracyReport {
    library: &'static str,
    false_negatives: usize,
    false_positives: usize,
    false_positive_rate: f64,
    config: String,
}

struct ScenarioResult {
    scenario: Scenario,
    accuracy_reports: Vec<AccuracyReport>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Operation {
    Build,
    ContainsMember,
    ContainsAbsent,
    ContainsMixed,
}

impl Operation {
    const ALL: [Operation; 4] = [
        Operation::Build,
        Operation::ContainsMember,
        Operation::ContainsAbsent,
        Operation::ContainsMixed,
    ];

    fn title(self) -> &'static str {
        match self {
            Operation::Build => "Build",
            Operation::ContainsMember => "Contains present keys",
            Operation::ContainsAbsent => "Contains absent keys",
            Operation::ContainsMixed => "Contains mixed workload",
        }
    }

    fn group_prefix(self) -> &'static str {
        match self {
            Operation::Build => "build",
            Operation::ContainsMember => "contains/member",
            Operation::ContainsAbsent => "contains/absent",
            Operation::ContainsMixed => "contains/mixed",
        }
    }

    fn group_id(self, scenario: Scenario) -> String {
        format!("{}/{}", self.group_prefix(), scenario.name)
    }

    fn group_directory(self, scenario: Scenario) -> PathBuf {
        PathBuf::from(self.group_id(scenario).replace('/', "_"))
    }

    fn throughput(self, scenario: Scenario) -> u64 {
        match self {
            Operation::Build => scenario.inserted_items as u64,
            Operation::ContainsMember
            | Operation::ContainsAbsent
            | Operation::ContainsMixed => scenario.query_batch_size as u64,
        }
    }
}

#[derive(Clone)]
struct PerformanceReport {
    library: &'static str,
    benchmark_label: String,
    mean_ns: f64,
    lower_ns: f64,
    upper_ns: f64,
    ns_per_element: f64,
    benchmark_link: Option<String>,
    group_link: Option<String>,
}

#[derive(Deserialize)]
struct EstimatesFile {
    mean: StatisticEstimate,
}

#[derive(Deserialize)]
struct StatisticEstimate {
    confidence_interval: ConfidenceInterval,
    point_estimate: f64,
}

#[derive(Deserialize)]
struct ConfidenceInterval {
    lower_bound: f64,
    upper_bound: f64,
}

trait FilterAdapter {
    type Filter;

    fn name() -> &'static str;
    fn config(scenario: Scenario) -> String;
    fn build(members: &[Key], scenario: Scenario) -> Self::Filter;
    fn contains(filter: &Self::Filter, key: &Key) -> bool;
}

struct BroomAdapter;
struct FastBloomAdapter;
struct BloomFilterCrateAdapter;
struct BloomCrateAdapter;

const SCENARIOS: [Scenario; 2] = [
    Scenario {
        name: "compact-128",
        inserted_items: 128,
        shared_filter_bits: 2_048,
        query_batch_size: 4_096,
        precision_queries: 100_000,
    },
    Scenario {
        name: "scale-4096",
        inserted_items: 4_096,
        shared_filter_bits: 65_536,
        query_batch_size: 16_384,
        precision_queries: 100_000,
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

    let precision_queries: Vec<Key> = (0..scenario.precision_queries)
        .map(|index| make_key(30_000_000 + index as u64))
        .collect();

    ScenarioData {
        members,
        member_queries,
        absent_queries,
        mixed_queries,
        precision_queries,
    }
}

const FASTBLOOM_SEED: u128 = 0x1357_9BDF_2468_ACE0_FEDC_BA98_7654_3210;
const BLOOMFILTER_SEED: [u8; 32] = [
    0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE,
    0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
    0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
];

#[derive(Clone, Copy, Default)]
struct FixedSipHasherBuilder<const K0: u64, const K1: u64>;

impl<const K0: u64, const K1: u64> BuildHasher for FixedSipHasherBuilder<K0, K1> {
    type Hasher = SipHasher13;

    fn build_hasher(&self) -> Self::Hasher {
        SipHasher13::new_with_keys(K0, K1)
    }
}

fn broom_size_exponent(shared_filter_bits: usize) -> usize {
    assert!(shared_filter_bits.is_power_of_two());
    shared_filter_bits.trailing_zeros() as usize
}

impl FilterAdapter for BroomAdapter {
    type Filter = BroomFilter;

    fn name() -> &'static str {
        "broomfilter"
    }

    fn config(scenario: Scenario) -> String {
        format!("{} bits", scenario.shared_filter_bits)
    }

    fn build(members: &[Key], scenario: Scenario) -> Self::Filter {
        let exponent = broom_size_exponent(scenario.shared_filter_bits);
        let mut filter = BroomFilter::new(exponent, members.len()).expect("unable to create filter");

        for key in members {
            filter.insert(&key.bytes);
        }

        filter
    }

    fn contains(filter: &Self::Filter, key: &Key) -> bool {
        filter.contains(&key.bytes)
    }
}

impl FilterAdapter for FastBloomAdapter {
    type Filter = FastBloom;

    fn name() -> &'static str {
        "fastbloom"
    }

    fn config(scenario: Scenario) -> String {
        format!("{} bits", scenario.shared_filter_bits)
    }

    fn build(members: &[Key], scenario: Scenario) -> Self::Filter {
        let mut filter =
            FastBloom::with_num_bits(scenario.shared_filter_bits)
                .seed(&FASTBLOOM_SEED)
                .expected_items(members.len());

        for key in members {
            filter.insert(&key.bytes);
        }

        filter
    }

    fn contains(filter: &Self::Filter, key: &Key) -> bool {
        filter.contains(&key.bytes)
    }
}

impl FilterAdapter for BloomFilterCrateAdapter {
    type Filter = Bloom<[u8; 16]>;

    fn name() -> &'static str {
        "bloomfilter"
    }

    fn config(scenario: Scenario) -> String {
        format!("{} bits", scenario.shared_filter_bits)
    }

    fn build(members: &[Key], scenario: Scenario) -> Self::Filter {
        let mut filter = Bloom::new_with_seed(
            scenario.shared_filter_bits / 8,
            members.len(),
            &BLOOMFILTER_SEED,
        )
        .expect("unable to create bloomfilter crate filter");

        for key in members {
            filter.set(&key.bytes);
        }

        filter
    }

    fn contains(filter: &Self::Filter, key: &Key) -> bool {
        filter.check(&key.bytes)
    }
}

impl FilterAdapter for BloomCrateAdapter {
    type Filter = LegacyBloom<
        FixedSipHasherBuilder<0x0123_4567_89AB_CDEF, 0xFEDC_BA98_7654_3210>,
        FixedSipHasherBuilder<0x0F1E_2D3C_4B5A_6978, 0x8877_6655_4433_2211>,
    >;

    fn name() -> &'static str {
        "bloom"
    }

    fn config(scenario: Scenario) -> String {
        format!("{} bits", scenario.shared_filter_bits)
    }

    fn build(members: &[Key], scenario: Scenario) -> Self::Filter {
        let mut filter = LegacyBloom::with_size_and_hashers(
            scenario.shared_filter_bits,
            legacy_optimal_num_hashes(scenario.shared_filter_bits, members.len() as u32),
            FixedSipHasherBuilder,
            FixedSipHasherBuilder,
        );

        for key in members {
            filter.insert(&key.bytes);
        }

        filter
    }

    fn contains(filter: &Self::Filter, key: &Key) -> bool {
        filter.contains(&key.bytes)
    }
}
fn measure_accuracy<T: FilterAdapter>(scenario: Scenario, data: &ScenarioData) -> AccuracyReport {
    let filter = T::build(&data.members, scenario);
    let false_negatives = data
        .members
        .iter()
        .filter(|key| !T::contains(&filter, key))
        .count();
    let false_positives = data
        .precision_queries
        .iter()
        .filter(|key| T::contains(&filter, key))
        .count();

    AccuracyReport {
        library: T::name(),
        false_negatives,
        false_positives,
        false_positive_rate: false_positives as f64 / data.precision_queries.len() as f64,
        config: T::config(scenario),
    }
}

fn print_accuracy_report(scenario: Scenario, reports: &[AccuracyReport]) {
    println!(
        "\n=== Precision summary: {} (inserted={}, shared bits={}, precision queries={}) ===",
        scenario.name,
        scenario.inserted_items,
        scenario.shared_filter_bits,
        scenario.precision_queries,
    );
    println!(
        "{:<14} {:>8} {:>12} {:>12}  config",
        "library",
        "false-",
        "false+",
        "fp-rate",
    );

    for report in reports {
        println!(
            "{:<14} {:>8} {:>12} {:>12.6}  {}",
            report.library,
            report.false_negatives,
            report.false_positives,
            report.false_positive_rate,
            report.config,
        );
    }
}

fn benchmark_label(report: &AccuracyReport) -> String {
    format!(
        "{} [fp={:.6}, fn={}]",
        report.library, report.false_positive_rate, report.false_negatives,
    )
}

fn bench_adapter<T: FilterAdapter>(
    c: &mut Criterion,
    scenario: Scenario,
    data: &ScenarioData,
    accuracy: &AccuracyReport,
) {
    let benchmark_label = benchmark_label(accuracy);

    {
        let mut group = c.benchmark_group(Operation::Build.group_id(scenario));
        group.throughput(Throughput::Elements(scenario.inserted_items as u64));
        group.bench_function(BenchmarkId::from_parameter(&benchmark_label), |b| {
            b.iter(|| black_box(T::build(&data.members, scenario)));
        });
        group.finish();
    }

    let filter = T::build(&data.members, scenario);

    {
        let mut group = c.benchmark_group(Operation::ContainsMember.group_id(scenario));
        group.throughput(Throughput::Elements(data.member_queries.len() as u64));
        group.bench_function(BenchmarkId::from_parameter(&benchmark_label), |b| {
            b.iter(|| {
                let hits = data
                    .member_queries
                    .iter()
                    .filter(|key| T::contains(&filter, key))
                    .count();
                black_box(hits)
            });
        });
        group.finish();
    }

    {
        let mut group = c.benchmark_group(Operation::ContainsAbsent.group_id(scenario));
        group.throughput(Throughput::Elements(data.absent_queries.len() as u64));
        group.bench_function(BenchmarkId::from_parameter(&benchmark_label), |b| {
            b.iter(|| {
                let hits = data
                    .absent_queries
                    .iter()
                    .filter(|key| T::contains(&filter, key))
                    .count();
                black_box(hits)
            });
        });
        group.finish();
    }

    {
        let mut group = c.benchmark_group(Operation::ContainsMixed.group_id(scenario));
        group.throughput(Throughput::Elements(data.mixed_queries.len() as u64));
        group.bench_function(BenchmarkId::from_parameter(&benchmark_label), |b| {
            b.iter(|| {
                let hits = data
                    .mixed_queries
                    .iter()
                    .filter(|key| T::contains(&filter, key))
                    .count();
                black_box(hits)
            });
        });
        group.finish();
    }
}

fn collect_accuracy_reports(scenario: Scenario, data: &ScenarioData) -> Vec<AccuracyReport> {
    vec![
        measure_accuracy::<BroomAdapter>(scenario, data),
        measure_accuracy::<FastBloomAdapter>(scenario, data),
        measure_accuracy::<BloomFilterCrateAdapter>(scenario, data),
        measure_accuracy::<BloomCrateAdapter>(scenario, data),
    ]
}

fn criterion_benchmark(c: &mut Criterion) -> Vec<ScenarioResult> {
    let mut results = Vec::new();

    for scenario in SCENARIOS {
        let data = prepare_scenario_data(scenario);
        let accuracy_reports = collect_accuracy_reports(scenario, &data);

        print_accuracy_report(scenario, &accuracy_reports);

        for accuracy in &accuracy_reports {
            match accuracy.library {
                "broomfilter" => bench_adapter::<BroomAdapter>(c, scenario, &data, accuracy),
                "fastbloom" => bench_adapter::<FastBloomAdapter>(c, scenario, &data, accuracy),
                "bloomfilter" => {
                    bench_adapter::<BloomFilterCrateAdapter>(c, scenario, &data, accuracy)
                }
                "bloom" => bench_adapter::<BloomCrateAdapter>(c, scenario, &data, accuracy),
                _ => unreachable!("unknown library benchmark requested"),
            }
        }

        results.push(ScenarioResult {
            scenario,
            accuracy_reports,
        });
    }

    results
}

fn configured_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2))
        .sample_size(20)
        .configure_from_args()
}

fn read_estimates(path: &Path) -> io::Result<EstimatesFile> {
    let content = fs::read_to_string(path)?;
    serde_json::from_str(&content).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse {}: {error}", path.display()),
        )
    })
}

fn encode_path(path: &Path) -> String {
    let mut encoded = String::new();

    for (index, segment) in path.iter().enumerate() {
        if index > 0 {
            encoded.push('/');
        }

        for byte in segment.to_string_lossy().bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                encoded.push(byte as char);
            } else {
                let _ = write!(encoded, "%{byte:02X}");
            }
        }
    }

    encoded
}

fn report_link(path: PathBuf, label: &str) -> Option<String> {
    path.exists()
        .then(|| format!("[{label}]({})", encode_path(&path)))
}

fn load_performance_report(
    output_directory: &Path,
    scenario: Scenario,
    accuracy: &AccuracyReport,
    operation: Operation,
) -> io::Result<PerformanceReport> {
    let benchmark_directory = operation
        .group_directory(scenario)
        .join(benchmark_label(accuracy));
    let estimates_path = output_directory
        .join(&benchmark_directory)
        .join("new")
        .join("estimates.json");
    let estimates = read_estimates(&estimates_path)?;

    let mean_ns = estimates.mean.point_estimate;
    let throughput = operation.throughput(scenario) as f64;
    let benchmark_report = output_directory
        .join(&benchmark_directory)
        .join("report")
        .join("index.html");
    let group_report = output_directory
        .join(operation.group_directory(scenario))
        .join("report")
        .join("index.html");

    Ok(PerformanceReport {
        library: accuracy.library,
        benchmark_label: benchmark_label(accuracy),
        mean_ns,
        lower_ns: estimates.mean.confidence_interval.lower_bound,
        upper_ns: estimates.mean.confidence_interval.upper_bound,
        ns_per_element: mean_ns / throughput,
        benchmark_link: report_link(benchmark_report, "plot"),
        group_link: report_link(group_report, "group plot"),
    })
}

fn format_duration(ns: f64) -> String {
    if ns >= 1_000_000_000.0 {
        format!("{:.2} s", ns / 1_000_000_000.0)
    } else if ns >= 1_000_000.0 {
        format!("{:.2} ms", ns / 1_000_000.0)
    } else if ns >= 1_000.0 {
        format!("{:.2} µs", ns / 1_000.0)
    } else {
        format!("{ns:.2} ns")
    }
}

fn format_ratio_against_broom(broom_mean_ns: f64, candidate_mean_ns: f64) -> String {
    format!("{:.2}x", broom_mean_ns / candidate_mean_ns)
}

fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn best_accuracy_report<'a>(reports: &'a [AccuracyReport]) -> Option<&'a AccuracyReport> {
    reports.iter().min_by(|left, right| {
        left.false_negatives
            .cmp(&right.false_negatives)
            .then_with(|| {
                left.false_positive_rate
                    .partial_cmp(&right.false_positive_rate)
                    .unwrap_or(Ordering::Equal)
            })
    })
}

fn render_precision_table(markdown: &mut String, scenario_result: &ScenarioResult) {
    let best_library = best_accuracy_report(&scenario_result.accuracy_reports).map(|report| report.library);

    markdown.push_str("| Library | False negatives | False positives | FP rate | Config |\n");
    markdown.push_str("| --- | ---: | ---: | ---: | --- |\n");

    let mut reports = scenario_result.accuracy_reports.clone();
    reports.sort_by(|left, right| {
        left.false_negatives
            .cmp(&right.false_negatives)
            .then_with(|| {
                left.false_positive_rate
                    .partial_cmp(&right.false_positive_rate)
                    .unwrap_or(Ordering::Equal)
            })
    });

    for report in reports {
        let library = if Some(report.library) == best_library {
            format!("**{}**", report.library)
        } else {
            report.library.to_string()
        };
        let _ = writeln!(
            markdown,
            "| {} | {} | {} | {:.6} | {} |",
            library,
            report.false_negatives,
            report.false_positives,
            report.false_positive_rate,
            escape_table_cell(&report.config),
        );
    }

    markdown.push('\n');
}

fn render_performance_table(
    markdown: &mut String,
    scenario: Scenario,
    reports: &[PerformanceReport],
) {
    if reports.is_empty() {
        markdown.push_str("No Criterion results were found for this benchmark group.\n\n");
        return;
    }

    let broom_mean_ns = reports
        .iter()
        .find(|report| report.library == "broomfilter")
        .map(|report| report.mean_ns)
        .unwrap_or(reports[0].mean_ns);

    if let Some(link) = reports.iter().find_map(|report| report.group_link.as_deref()) {
        let _ = writeln!(markdown, "Group report: {}  ", link);
    }
    let _ = writeln!(
        markdown,
        "Lower is better. All filters use the same {}-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over {} operations.\n",
        scenario.shared_filter_bits,
        scenario.query_batch_size
    );

    markdown.push_str(
        "| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |\n",
    );
    markdown.push_str(
        "| ---: | --- | --- | ---: | ---: | --- | --- |\n",
    );

    for (index, report) in reports.iter().enumerate() {
        let library = if report.library == "broomfilter" {
            format!("**{}**", report.library)
        } else {
            report.library.to_string()
        };
        let plot_link = report.benchmark_link.as_deref().unwrap_or("—");
        let _ = writeln!(
            markdown,
            "| {} | {} | {} ({}, {}) | {:.2} ns/op | {} | `{}` | {} |",
            index + 1,
            library,
            format_duration(report.mean_ns),
            format_duration(report.lower_ns),
            format_duration(report.upper_ns),
            report.ns_per_element,
            format_ratio_against_broom(broom_mean_ns, report.mean_ns),
            report.benchmark_label,
            plot_link,
        );
    }

    markdown.push('\n');
}

fn render_perf_report(output_directory: &Path, scenario_results: &[ScenarioResult]) -> io::Result<String> {
    let mut markdown = String::new();
    markdown.push_str("# Performance Report\n\n");
    markdown.push_str(
        "Generated from the Criterion benchmark suite in `benches/comparison.rs`. This report compares `broomfilter` against other mutable Bloom-style filters under equal per-scenario memory budgets, deterministic setup, and identical key datasets.\n\n",
    );

    let overall_report = output_directory.join("report").join("index.html");
    if let Some(link) = report_link(overall_report, "Criterion dashboard") {
        let _ = writeln!(markdown, "- Overall report: {}", link);
    }
    markdown.push_str("- Benchmarks covered: build, present lookups, absent lookups, and mixed lookups\n");
    markdown.push_str("- Comparison set: mutable Bloom-style filters that can be configured to the same exact bit budget through their public APIs\n");
    markdown.push_str("- Precision covered: false negatives on inserted keys and false positives over 100,000 deterministic absent-key probes per scenario\n\n");

    markdown.push_str("## Scenario Summary\n\n");
    markdown.push_str("| Scenario | Most precise | Fastest build | Fastest present lookup | Fastest absent lookup | Fastest mixed lookup |\n");
    markdown.push_str("| --- | --- | --- | --- | --- | --- |\n");

    let mut performance_by_scenario = Vec::new();

    for scenario_result in scenario_results {
        let mut operation_reports = Vec::new();
        for operation in Operation::ALL {
            let mut reports = Vec::new();
            for accuracy in &scenario_result.accuracy_reports {
                reports.push(load_performance_report(
                    output_directory,
                    scenario_result.scenario,
                    accuracy,
                    operation,
                )?);
            }
            reports.sort_by(|left, right| {
                left.mean_ns
                    .partial_cmp(&right.mean_ns)
                    .unwrap_or(Ordering::Equal)
            });
            operation_reports.push((operation, reports));
        }

        let most_precise = best_accuracy_report(&scenario_result.accuracy_reports)
            .map(|report| report.library)
            .unwrap_or("n/a");
        let fastest_build = operation_reports
            .iter()
            .find(|(operation, _)| *operation == Operation::Build)
            .and_then(|(_, reports)| reports.first())
            .map(|report| report.library)
            .unwrap_or("n/a");
        let fastest_member = operation_reports
            .iter()
            .find(|(operation, _)| *operation == Operation::ContainsMember)
            .and_then(|(_, reports)| reports.first())
            .map(|report| report.library)
            .unwrap_or("n/a");
        let fastest_absent = operation_reports
            .iter()
            .find(|(operation, _)| *operation == Operation::ContainsAbsent)
            .and_then(|(_, reports)| reports.first())
            .map(|report| report.library)
            .unwrap_or("n/a");
        let fastest_mixed = operation_reports
            .iter()
            .find(|(operation, _)| *operation == Operation::ContainsMixed)
            .and_then(|(_, reports)| reports.first())
            .map(|report| report.library)
            .unwrap_or("n/a");

        let _ = writeln!(
            markdown,
            "| `{}` | {} | {} | {} | {} | {} |",
            scenario_result.scenario.name,
            most_precise,
            fastest_build,
            fastest_member,
            fastest_absent,
            fastest_mixed,
        );

        performance_by_scenario.push((scenario_result, operation_reports));
    }

    markdown.push('\n');

    for (scenario_result, operation_reports) in performance_by_scenario {
        let scenario = scenario_result.scenario;
        let _ = writeln!(markdown, "## Scenario `{}`\n", scenario.name);
        let _ = writeln!(
            markdown,
            "- Inserted items: {}",
            scenario.inserted_items,
        );
        let _ = writeln!(
            markdown,
            "- Shared filter size: {} bits ({} bytes)",
            scenario.shared_filter_bits,
            scenario.shared_filter_bits / 8,
        );
        let _ = writeln!(markdown, "- Query batch size: {}", scenario.query_batch_size);
        let _ = writeln!(
            markdown,
            "- Precision probes: {}",
            scenario.precision_queries,
        );
        markdown.push('\n');

        markdown.push_str("### Precision\n\n");
        render_precision_table(&mut markdown, scenario_result);

        for (operation, reports) in operation_reports {
            let _ = writeln!(markdown, "### {}\n", operation.title());
            render_performance_table(&mut markdown, scenario, &reports);
        }
    }

    Ok(markdown)
}

fn write_perf_report(output_directory: &Path, scenario_results: &[ScenarioResult]) -> io::Result<()> {
    let report = render_perf_report(output_directory, scenario_results)?;
    fs::write("PERF_REPORT.md", report)
}

fn main() {
    let mut criterion = configured_criterion();
    let scenario_results = criterion_benchmark(&mut criterion);
    criterion.final_summary();

    if let Err(error) = write_perf_report(Path::new("target/criterion"), &scenario_results) {
        panic!("failed to write PERF_REPORT.md: {error}");
    }
}
