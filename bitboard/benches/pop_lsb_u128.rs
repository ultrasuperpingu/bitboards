use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[derive(Clone)]
struct BB {
    bits: u128,
}

impl BB {
    #[inline]
    fn pop_lsb_bmi2(&mut self) -> u32 {
        #[cfg(target_feature = "bmi2")]
        unsafe {
            let low = self.bits as u64;

            if low != 0 {
                let idx = std::arch::x86_64::_tzcnt_u64(low);
                let new_low = std::arch::x86_64::_blsr_u64(low);

                let high = self.bits & (!0u128 << 64);
                self.bits = high | new_low as u128;

                return idx as u32;
            }

            let high = (self.bits >> 64) as u64;
            let idx = std::arch::x86_64::_tzcnt_u64(high);
            let new_high = std::arch::x86_64::_blsr_u64(high);

            self.bits = (new_high as u128) << 64;

            (idx + 64) as u32
        }

        #[cfg(not(target_feature = "bmi2"))]
        {
            unreachable!("BMI2 version compiled without BMI2")
        }
    }

    #[inline]
    fn pop_lsb_fallback(&mut self) -> u32 {
        let low = self.bits as u64;

        if low != 0 {
            let idx = low.trailing_zeros();
            self.bits &= self.bits - 1;
            return idx;
        }

        let high = (self.bits >> 64) as u64;
        let idx = high.trailing_zeros();
        self.bits &= self.bits - 1;
        idx + 64
    }
}

fn bench_pop_lsb_u128(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_lsb_u128");

    let value = BB { bits: 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128 };

    group.bench_function("bmi2", |b| {
        b.iter(|| {
            let mut v = black_box(value.clone());
            for _ in 0..128 {
                black_box(v.pop_lsb_bmi2());
            }
        })
    });

    group.bench_function("fallback", |b| {
        b.iter(|| {
            let mut v = black_box(value.clone());
            for _ in 0..128 {
                black_box(v.pop_lsb_fallback());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_pop_lsb_u128);
criterion_main!(benches);
