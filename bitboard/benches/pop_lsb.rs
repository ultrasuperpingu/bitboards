use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[derive(Clone)]
struct BB {
	bits: u64,
}

impl BB {
	#[inline]
	fn pop_lsb_bmi1(&mut self) -> u32 {
		#[cfg(target_feature = "bmi1")]
		unsafe {
			let idx = std::arch::x86_64::_tzcnt_u64(self.bits);
			self.bits = std::arch::x86_64::_blsr_u64(self.bits);
			idx as u32
		}

		#[cfg(not(target_feature = "bmi1"))]
		{
			unreachable!("BMI1 version compiled without BMI1")
		}
	}

	#[inline]
	fn pop_lsb_fallback(&mut self) -> u32 {
		let idx = self.bits.trailing_zeros();
		self.bits &= self.bits - 1;
		idx
	}
}

fn bench_pop_lsb(c: &mut Criterion) {
	let mut group = c.benchmark_group("pop_lsb");

	group.bench_function("bmi1", |b| {
		b.iter(|| {
			let mut bb = BB { bits: black_box(0xFFFF_FFFF_FFFF_FFFF) };
			for _ in 0..64 {
				black_box(bb.pop_lsb_bmi1());
			}
		})
	});

	group.bench_function("fallback", |b| {
		b.iter(|| {
			let mut bb = BB { bits: black_box(0xFFFF_FFFF_FFFF_FFFF) };
			for _ in 0..64 {
				black_box(bb.pop_lsb_fallback());
			}
		})
	});

	group.finish();
}

criterion_group!(benches, bench_pop_lsb);
criterion_main!(benches);
