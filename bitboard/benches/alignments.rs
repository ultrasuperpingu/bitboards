use bitboard_proc_macro::bitboard;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[bitboard(width=19, height=19)]
struct Goban;

#[bitboard(width=10, height=10)]
struct CheckersBoard;

#[bitboard(width=8, height=8)]
struct ChessBoard;


fn bench_alignment(c: &mut Criterion) {
	let mut group = c.benchmark_group("alignment");

	group.bench_function("dyn_alignment", |b| {
		let bb = Goban::compute_ray_n_mask(25).clone();
		b.iter(|| {
			black_box(bb.has_n_aligned(5));
		})
	});

	group.bench_function("static_alignment", |b| {
		let bb = Goban::compute_ray_n_mask(25).clone();
		b.iter(|| {
			black_box(bb.has_aligned::<5>());
		})
	});
	group.bench_function("pattern_alignment", |b| {
		let bb = Goban::compute_ray_n_mask(25).clone();
		b.iter(|| {
			black_box(bb.detect_pattern_v(0b11111));
		})
	});

	group.finish();
}


fn bench_shift_integer(c: &mut Criterion) {
	let mut group = c.benchmark_group("shift");

	group.bench_function("shift", |b| {
		let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let x = bb.clone();
			black_box(x.shift(5,5));
		})
	});

	group.bench_function("shift_scanline", |b| {
		let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let x = bb.clone();
			black_box(x.shift_scanline(5,5));
		})
	});
	group.bench_function("shifted_e", |b| {
		let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let x = bb.clone();
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			black_box(x.shifted_e());
		})
	});

	group.finish();
}
fn bench_shift_array(c: &mut Criterion) {
	let mut group = c.benchmark_group("array_shift");
	group.bench_function("shift_e", |b| {
		let bb = Goban::compute_ray_n_mask(25).clone();
		b.iter(|| {
			black_box(bb.shifted_e().shifted_e().shifted_e().shifted_e().shifted_e());
			//black_box(bb.shift_e());
			//let mut x = bb.clone();
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			//x = black_box(x.shifted_e());
			//x.shift_e();
		})
	});
	group.bench_function("shift_e_by", |b| {
		let bb = Goban::compute_ray_n_mask(25).clone();
		b.iter(|| {
			//black_box(bb.shift_e().shift_e().shift_e().shift_e().shift_e());
			//black_box(bb.shift_e());
			let mut x = bb.clone();
			x.shift_e_by(5);
		})
	});

	group.finish();
}
//criterion_group!(benches, bench_alignment, bench_shift_integer, bench_shift_array);
criterion_group!(benches, bench_shift_array);
criterion_main!(benches);
