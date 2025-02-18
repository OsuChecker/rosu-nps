use criterion::{criterion_group, criterion_main, Criterion};
use rosu_map::Beatmap;
use rosu_nps::calc::calculate_by_frequency;
fn benchmark_distribution(c: &mut Criterion) {
    let file = "./resources/test.osu";
    let map = Beatmap::from_path(&file)
        .expect("File does not exist or is not a valid .osu file");

    let mut group = c.benchmark_group("distribution");
    for frequency in [
        0.001,  // 0.1%
        0.005,  // 0.5%
        0.01,   // 1%
        0.02,   // 2%
        0.05,   // 5%
        0.1,    // 10%
        0.25,   // 25%
        0.5     // 50%
    ].iter() {
        group.bench_with_input(
            format!("fréquence_{:.3}%", frequency * 100.0),
            frequency,
            |b, &freq| {
                b.iter(|| calculate_by_frequency(&map, freq))
            }
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_distribution);
criterion_main!(benches);