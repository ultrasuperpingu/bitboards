use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[derive(Clone)]
struct BB {
    bits: u64,
}

impl BB {
    #[inline]
    fn pext_bmi2(&self, mask: &Self) -> u64 {
        #[cfg(target_feature = "bmi2")]
        unsafe {
            std::arch::x86_64::_pext_u64(self.bits, mask.bits)
        }

        #[cfg(not(target_feature = "bmi2"))]
        {
            unreachable!("BMI2 version compiled without BMI2")
        }
    }

    #[inline]
    fn pext_fallback(&self, mask: &Self) -> u64 {
        let mut res: u64 = 0;
        let mut bit: u64 = 1;
        let mut m = mask.bits;
        let mut v = self.bits;

        while m != 0 {
            if m & 1 != 0 {
                if v & 1 != 0 {
                    res |= bit;
                }
                bit <<= 1;
            }
            m >>= 1;
            v >>= 1;
        }
        res
    }
}

fn bench_pext(c: &mut Criterion) {
    let mut group = c.benchmark_group("pext");

    let value = BB { bits: 0xF0F0_F0F0_F0F0_F0F0 };
    let mask  = BB { bits: 0xCCCC_CCCC_CCCC_CCCC };

    group.bench_function("bmi2", |b| {
        b.iter(|| {
            let v = black_box(&value);
            let m = black_box(&mask);
            black_box(v.pext_bmi2(m));
        })
    });

    group.bench_function("fallback", |b| {
        b.iter(|| {
            let v = black_box(&value);
            let m = black_box(&mask);
            black_box(v.pext_fallback(m));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_pext);
criterion_main!(benches);
