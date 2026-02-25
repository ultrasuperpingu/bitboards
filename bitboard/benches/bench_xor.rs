use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

#[inline(never)]
fn xor_scalar_indexed(a: &mut [u64], b: &[u64]) {
	for i in 0..a.len() {
		a[i] ^= b[i];
	}
}

#[inline(never)]
fn xor_scalar_zip(a: &mut [u64], b: &[u64]) {
	for (x, y) in a.iter_mut().zip(b.iter()) {
		*x ^= *y;
	}
}

// Tes fonctions SIMD
#[target_feature(enable = "avx2")]
#[inline(never)]
unsafe fn xor_avx2(a: &mut [u64], b: &[u64]) { unsafe {
	use std::arch::x86_64::*;

	let len = a.len() / 4;
	let pa = a.as_mut_ptr() as *mut __m256i;
	let pb = b.as_ptr() as *const __m256i;

	for i in 0..len {
		let va = _mm256_loadu_si256(pa.add(i));
		let vb = _mm256_loadu_si256(pb.add(i));
		let vc = _mm256_xor_si256(va, vb);
		_mm256_storeu_si256(pa.add(i), vc);
	}
}}

#[target_feature(enable = "sse2")]
#[inline(never)]
unsafe fn xor_sse2(a: &mut [u64], b: &[u64]) { unsafe {
	use std::arch::x86_64::*;

	let len = a.len() / 2;
	let pa = a.as_mut_ptr() as *mut __m128i;
	let pb = b.as_ptr() as *const __m128i;

	for i in 0..len {
		let va = _mm_loadu_si128(pa.add(i));
		let vb = _mm_loadu_si128(pb.add(i));
		let vc = _mm_xor_si128(va, vb);
		_mm_storeu_si128(pa.add(i), vc);
	}
}}

fn bench_xor(c: &mut Criterion) {
	let mut rng = rand::rng();

	// Taille réaliste pour un bitset
	const N: usize = 25*31;

	let a: Vec<u64> = (0..N).map(|_| rng.next_u64()).collect();
	let b: Vec<u64> = (0..N).map(|_| rng.next_u64()).collect();

	c.bench_function("xor_scalar_indexed", |bench| {
		bench.iter(|| {
			let mut a2 = a.clone();
			xor_scalar_indexed(&mut a2, &b);
			std::hint::black_box(a2);
		})
	});

	c.bench_function("xor_scalar_zip", |bench| {
		bench.iter(|| {
			let mut a2 = a.clone();
			xor_scalar_zip(&mut a2, &b);
			std::hint::black_box(a2);
		})
	});

	#[cfg(target_feature = "sse2")]
	c.bench_function("xor_sse2", |bench| {
		bench.iter(|| {
			let mut a2 = a.clone();
			unsafe { xor_sse2(&mut a2, &b) };
			std::hint::black_box(a2);
		})
	});

	#[cfg(target_feature = "avx2")]
	c.bench_function("xor_avx2", |bench| {
		bench.iter(|| {
			let mut a2 = a.clone();
			unsafe { xor_avx2(&mut a2, &b) };
			std::hint::black_box(a2);
		})
	});
}

criterion_group!(benches, bench_xor);
criterion_main!(benches);
