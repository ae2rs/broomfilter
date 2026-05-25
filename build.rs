fn main() {
    if std::env::var("CARGO_FEATURE_BENCHMARKS").is_ok() {
        cc::Build::new()
            .cpp(true)
            .file("benches/cpp/blocked_filter.cpp")
            .compile("cpp_blocked_filter");
    }
}
