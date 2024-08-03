#[cfg(all(target_feature = "neon", target_arch = "aarch64"))]
use std::arch::aarch64::*;

pub trait StdDev {
    fn std_dev(&self, mean: f64) -> Option<f64>;
}

pub fn std_dev(slice: &[f32], mean: f64) -> Option<f64> {
    let mut accumulator = 0f64;

    let mut i = 0usize;
    let count = slice.len();

    #[cfg(all(target_feature = "neon", target_arch = "aarch64"))]
    unsafe {
        let v_mean = vdupq_n_f64(mean);
        let mut v_accumulator = vdupq_n_f64(accumulator);
        while i + 4 < count {
            let ptr = slice.as_ptr().add(i);
            let x = vld1q_f32(ptr);
            let mut hi = vcvt_high_f64_f32(x);
            let mut lo = vcvt_f64_f32(vget_low_f32(x));
            hi = vsubq_f64(hi, v_mean);
            lo = vsubq_f64(lo, v_mean);
            v_accumulator = vfmaq_f64(v_accumulator, hi, hi);
            v_accumulator = vfmaq_f64(v_accumulator, lo, lo);
            i += 4;
        }
        accumulator += vpaddd_f64(v_accumulator);
    }

    while i < count {
        let dx = unsafe { *slice.get_unchecked(i) } as f64 - mean;
        accumulator += dx * dx;
        i += 1;
    }
    return Some((accumulator / count as f64).sqrt());
}

impl StdDev for Vec<f32> {
    fn std_dev(&self, mean: f64) -> Option<f64> {
        std_dev(self.as_slice(), mean)
    }
}
