use bitboard_proc_macro::bitboard;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[bitboard(width = 19, height = 19)]
struct Goban;

//#[bitboard(width = 10, height = 10)]
//struct CheckersBoard;

#[bitboard(width = 8, height = 8)]
struct ChessBoard;

fn bench_alignment(c: &mut Criterion) {
	let mut group = c.benchmark_group("alignment");
	let bb = black_box(ChessBoard::compute_diag_dec_mask(25));

	group.bench_function("dyn_alignment", |b| {
		b.iter(|| {
			black_box(bb.has_n_aligned(black_box(5)));
		})
	});

	group.bench_function("static_alignment", |b| {
		b.iter(|| {
			black_box(bb.has_aligned::<5>());
		})
	});
	group.bench_function("pattern_alignment", |b| {
		b.iter(|| {
			black_box(bb.detect_pattern_v(black_box(0b11111)));
		})
	});

	group.finish();
}

fn bench_shift_integer(c: &mut Criterion) {
	let mut group = c.benchmark_group("shift_int");
	let bb = black_box(ChessBoard::compute_diag_dec_mask(25));
	let nb_shift = 5;

	group.bench_function("shifted", |b| {
		//let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let x = bb;
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
			black_box(x.shifted(black_box(nb_shift), black_box(nb_shift)));
		})
	});
	group.bench_function("shifted_ne", |b| {
		//let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let mut i = 0;
			while i < 10 {
				let mut x = bb;
				let mut j = 0;
				while j < black_box(nb_shift) {
					x = black_box(x.shifted_ne());
					j += 1;
				}
				i += 1;
			}
		})
	});
	group.bench_function("shift_ne_by", |b| {
		//let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
			let mut x = black_box(bb.clone());
			x.shift_ne_by(black_box(nb_shift) as u8);
		})
	});
	group.bench_function("shift", |b| {
		//let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
			let mut x = black_box(bb.clone());
			x.shift(black_box(nb_shift), black_box(nb_shift));
		})
	});

	group.finish();
}
fn bench_shift_array(c: &mut Criterion) {
	let bb = black_box(Goban::compute_diag_dec_mask(25));
	let nb_shift = 5;

	let mut group = c.benchmark_group("array_shift");
	group.bench_function("shift_e", |b| {
		b.iter(|| {
			let mut i = 0;
			while i < 10 {
				let mut x = bb.clone_const();
				let mut j = 0;
				while j < black_box(nb_shift) {
					x = black_box(x.shifted_ne());
					j += 1;
				}
				i += 1;
			}
		})
	});
	group.bench_function("shift_e_by", |b| {
		b.iter(|| {
			//black_box(bb.shift_e().shift_e().shift_e().shift_e().shift_e());
			//black_box(bb.shift_e());
			let mut x = bb.clone();
			x.shift_ne_by(black_box(nb_shift));
		})
	});
	group.bench_function("shift", |b| {
		b.iter(|| {
			//black_box(bb.shift_e().shift_e().shift_e().shift_e().shift_e());
			//black_box(bb.shift_e());
			let mut x = bb.clone();
			x.shift(black_box(nb_shift) as i16, black_box(nb_shift) as i16);
		})
	});
	group.bench_function("shifted", |b| {
		//let bb = ChessBoard::compute_ray_n_mask(25).clone();
		b.iter(|| {
			let x = bb.clone_const();
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
			black_box(x.shifted(black_box(nb_shift) as i16, black_box(nb_shift) as i16));
		})
	});
	group.finish();
}
criterion_group!(
	benches,
	bench_alignment,
	bench_shift_integer,
	bench_shift_array
);
//criterion_group!(benches, bench_shift_integer);
criterion_main!(benches);
