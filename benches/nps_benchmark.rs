use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rosu_nps::{calc_nps, calc_nps_range_by_time, calc_nps_range_by_hitobjects, calc_distribution, calc_distribution_2, calc_distribution_smart};
use rosu_map::Beatmap;

fn load_test_map() -> Beatmap {
    let b = include_bytes!("../assets/8thera.osu");
    Beatmap::from_bytes(b).unwrap()
}

fn bench_calc_nps(c: &mut Criterion) {
    let map = load_test_map();
    
    c.bench_function("calc_nps", |b| {
        b.iter(|| calc_nps(&map))
    });
}

fn bench_calc_nps_range(c: &mut Criterion) {
    let map = load_test_map();
    let first_time = map.hit_objects.first().unwrap().start_time;
    let last_time = map.hit_objects.last().unwrap().start_time;
    
    c.bench_function("calc_nps_range_full_range", |b| {
        b.iter(|| calc_nps_range_by_time(&map, first_time, last_time))
    });
}

fn bench_calc_nps_range_by_hitobjects(c: &mut Criterion) {
    let map = load_test_map();
    let first_obj = map.hit_objects.first().unwrap();
    let last_obj = map.hit_objects.last().unwrap();
    
    c.bench_function("calc_nps_range_by_hitobjects_full_range", |b| {
        b.iter(|| calc_nps_range_by_hitobjects(&map, first_obj, last_obj))
    });
}

fn bench_calc_nps_various_ranges(c: &mut Criterion) {
    let map = load_test_map();
    let total_objects = map.hit_objects.len();
    
    let mut group = c.benchmark_group("nps_range_comparison");
    
    // Test different range sizes
    let range_percentages = [10, 25, 50, 75, 100];
    
    for &percentage in &range_percentages {
        let range_size = (total_objects * percentage) / 100;
        let end_idx = range_size.min(total_objects - 1);
        
        let start_time = map.hit_objects[0].start_time;
        let end_time = map.hit_objects[end_idx].start_time;
        
        group.bench_with_input(
            BenchmarkId::new("range", format!("{}%", percentage)),
            &(start_time, end_time),
            |b, &(start, end)| {
                b.iter(|| calc_nps_range_by_time(&map, start, end))
            },
        );
    }
    
    group.finish();
}

fn bench_comparison_same_result(c: &mut Criterion) {
    let map = load_test_map();
    let first_time = map.hit_objects.first().unwrap().start_time;
    let last_time = map.hit_objects.last().unwrap().start_time;
    let first_obj = map.hit_objects.first().unwrap();
    let last_obj = map.hit_objects.last().unwrap();
    
    let mut group = c.benchmark_group("method_comparison");
    
    group.bench_function("calc_nps", |b| {
        b.iter(|| calc_nps(&map))
    });
    
    group.bench_function("calc_nps_range_by_time", |b| {
        b.iter(|| calc_nps_range_by_time(&map, first_time, last_time))
    });
    
    group.bench_function("calc_nps_range_by_hitobjects", |b| {
        b.iter(|| calc_nps_range_by_hitobjects(&map, first_obj, last_obj))
    });
    
    group.finish();
}

fn bench_distribution_comparison(c: &mut Criterion) {
    let map = load_test_map();
    
    let mut group = c.benchmark_group("distribution_comparison");
    
    // Test different numbers of parts
    let part_counts = [5, 10, 20, 50, 100, 200, 500];
    
    for &parts in &part_counts {
        group.bench_with_input(
            BenchmarkId::new("calc_distribution", parts),
            &parts,
            |b, &parts| {
                b.iter(|| calc_distribution(&map, parts))
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("calc_distribution_2", parts),
            &parts,
            |b, &parts| {
                b.iter(|| calc_distribution_2(&map, parts))
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("calc_distribution_smart", parts),
            &parts,
            |b, &parts| {
                b.iter(|| calc_distribution_smart(&map, parts))
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_calc_nps, 
    bench_calc_nps_range, 
    bench_calc_nps_range_by_hitobjects,
    bench_calc_nps_various_ranges,
    bench_comparison_same_result,
    bench_distribution_comparison
);
criterion_main!(benches); 