use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[derive(Clone)]
struct BB {
	bits: u128,
}

impl BB {
	#[inline]
	fn pdep_bmi2(&self, mask: &Self) -> u128 {
		#[cfg(target_feature = "bmi2")]
		unsafe {
			let src = self.bits;

			let lo_mask = mask.bits as u64;
			let hi_mask = (mask.bits >> 64) as u64;

			let lo_count = lo_mask.count_ones() as u32;

			let src_lo = src as u64;
			let src_hi = (src >> lo_count) as u64;

			let lo_res = std::arch::x86_64::_pdep_u64(src_lo, lo_mask);
			let hi_res = std::arch::x86_64::_pdep_u64(src_hi, hi_mask);

			(lo_res as u128) | ((hi_res as u128) << 64)
		}

		#[cfg(not(target_feature = "bmi2"))]
		{
			unreachable!("BMI2 version compiled without BMI2")
		}
	}

	#[inline]
	fn pdep_fallback(&self, mask: &Self) -> u128 {
		let mut res: u128 = 0;
		let mut bit: u128 = 1;
		let mut m = mask.bits;
		let mut v = self.bits;

		while m != 0 {
			if m & 1 != 0 {
				if v & 1 != 0 {
					res |= bit;
				}
				v >>= 1;
			}
			bit <<= 1;
			m >>= 1;
		}
		res
	}
}

fn bench_pdep_u128(c: &mut Criterion) {
	let mut group = c.benchmark_group("pdep_u128");

	let value = BB { bits: 0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0u128 };
	let mask  = BB { bits: 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCCu128 };

	group.bench_function("bmi2", |b| {
		b.iter(|| {
			let v = black_box(&value);
			let m = black_box(&mask);
			black_box(v.pdep_bmi2(m));
		})
	});

	group.bench_function("fallback", |b| {
		b.iter(|| {
			let v = black_box(&value);
			let m = black_box(&mask);
			black_box(v.pdep_fallback(m));
		})
	});

	group.finish();
}

criterion_group!(benches, bench_pdep_u128);
criterion_main!(benches);
