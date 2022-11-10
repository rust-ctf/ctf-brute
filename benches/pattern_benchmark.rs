use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ctf_brute::ops::Pattern;

fn bench_fn(c: &mut Criterion) {
    c.bench_function("simple query", |b| {
        b.iter(|| {
            _ = Pattern::from_pattern("a{1,10}")
                .unwrap()
                .iter()
                .for_each(|x| {});
        });
    });

    c.bench_function("letter 3 chars", |b| {
        b.iter(|| {
            _ = Pattern::from_pattern(r"\l{3}")
                .unwrap()
                .iter()
                .for_each(|x| {});
        });
    });

    c.bench_function("numbers 0 to 5 chars", |b| {
        b.iter(|| {
            Pattern::from_pattern(r"\d{0,5}")
                .unwrap()
                .iter()
                .for_each(|x| {});
        });
    });
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);
