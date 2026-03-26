use broomfilter::Filter;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{RngExt, SeedableRng, distr::Alphanumeric};
use std::hint::black_box;

fn random_string(rng: &mut rand::rngs::SmallRng) -> String {
    (0..30).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut filter = Filter::new(14, 10000).expect("unable to create filter");
    let mut rng_generator = rand::rngs::SmallRng::seed_from_u64(2);

    let random_strings: Vec<String> = (0..4)
        .into_iter()
        .map(|_| random_string(&mut rng_generator))
        .collect();

    println!("Random strings:\n{random_strings:?}");

    {
        let mut index_group = c.benchmark_group("Index");
        for s in &random_strings {
            index_group.bench_function(format!("index-{s}"), |b| {
                b.iter(|| filter.insert(black_box(&s)))
            });
        }
    }

    {
        let mut contains_group = c.benchmark_group("Contains");
        for s in &random_strings {
            contains_group.bench_function(format!("contains-{s}"), |b| {
                b.iter(|| filter.contains(black_box(&s)))
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
