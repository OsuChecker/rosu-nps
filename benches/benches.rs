use criterion::{criterion_group, criterion_main, Criterion};
use rosu_map::Beatmap;
use rosu_nps::calc::calculate_by_frequency;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TEST_MAP: Beatmap = Beatmap::from_path("./resources/test.osu")
        .expect("File does not exist or is not a valid .osu file");
}


fn benchmark_distribution(c: &mut Criterion) {
    let mut group = c.benchmark_group("distribution");
    group.sample_size(1000);

    for frequency in [
        0.001,
        0.005,
        0.01,
        0.02,
        0.05,
        0.1,
        0.25,
        0.5
    ].iter() {
        group.bench_with_input(
            format!("fréquence_{:.3}%", frequency * 100.0),
            frequency,
            |b, &freq| {
                b.iter(|| calculate_by_frequency(&TEST_MAP, freq))
            }
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_distribution);
criterion_main!(benches);