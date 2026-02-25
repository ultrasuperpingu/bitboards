use criterion::{criterion_group, criterion_main, Criterion};
use bitboard::runtime::small_bitboard::SmallBitboard;

fn bench_sliding_naive(c: &mut Criterion) {
    let offsets = vec![(1,0), (-1,0), (0,1), (0,-1)]; // directions tour
    c.bench_function("sliding_naive", |b| {
        b.iter(|| {
            SmallBitboard::generate_sliding_moves(&offsets, 8, 8, false)
        })
    });
}

fn bench_sliding_optimized(c: &mut Criterion) {
    let offsets = vec![(1,0), (-1,0), (0,1), (0,-1)];
    c.bench_function("sliding_optimized", |b| {
        b.iter(|| {
            SmallBitboard::generate_sliding_moves2(&offsets, 8, 8, false)
        })
    });
}

criterion_group!(benches, bench_sliding_naive, bench_sliding_optimized);
criterion_main!(benches);
