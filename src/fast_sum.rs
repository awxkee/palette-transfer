#[cfg(all(target_feature = "neon", target_arch = "aarch64"))]
use std::arch::aarch64::*;

pub trait FastSum {
    fn sums(&self) -> f64;
}

pub fn sum_slice(slice: &[f32]) -> f64 {
    if slice.len() == 0 {
        return 0f64;
    }
    let mut i = 0usize;
    let count = slice.len();
    let mut accumulator = 0f64;

    #[cfg(all(target_feature = "neon", target_arch = "aarch64"))]
    unsafe {
        let mut v_accumulator = vdupq_n_f64(accumulator);

        while i + 16 < count {
            let ptr = slice.as_ptr().add(i);
            let items_l = vld1q_f32_x4(ptr);
            let mut items = vaddq_f32(items_l.0, items_l.1);
            items = vaddq_f32(items, items_l.2);
            items = vaddq_f32(items, items_l.3);

            let hi = vcvt_high_f64_f32(items);
            let lo = vcvt_f64_f32(vget_low_f32(items));

            let p_sum = vpaddq_f64(hi, lo);
            v_accumulator = vaddq_f64(v_accumulator, p_sum);

            i += 16;
        }

        while i + 4 < count {
            let ptr = slice.as_ptr().add(i);
            let items = vld1q_f32(ptr);
            let hi = vcvt_high_f64_f32(items);
            let lo = vcvt_f64_f32(vget_low_f32(items));

            let p_sum = vpaddq_f64(hi, lo);
            v_accumulator = vaddq_f64(v_accumulator, p_sum);

            i += 4;
        }

        accumulator += vpaddd_f64(v_accumulator);
    }

    while i < count {
        accumulator += unsafe { *slice.get_unchecked(i) } as f64;
        i += 1;
    }

    accumulator
}

impl FastSum for Vec<f32> {
    fn sums(&self) -> f64 {
        sum_slice(self.as_slice())
    }
}

impl FastSum for Vec<f64> {
    fn sums(&self) -> f64 {
        if self.len() == 0 {
            return 0f64;
        }
        let mut i = 0usize;
        let count = self.len();
        let mut accumulator = 0f64;

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        #[cfg(target_feature = "neon")]
        unsafe {
            let mut v_accumulator = vdupq_n_f64(accumulator);

            while i + 8 < count {
                let slice = self.as_slice();
                let ptr0 = slice.as_ptr().add(i);
                let items0 = vld1q_f64(ptr0);

                let ptr1 = slice.as_ptr().add(i + 2);
                let items1 = vld1q_f64(ptr1);

                let ptr2 = slice.as_ptr().add(i + 4);
                let items2 = vld1q_f64(ptr2);

                let ptr3 = slice.as_ptr().add(i + 6);
                let items3 = vld1q_f64(ptr3);

                v_accumulator = vaddq_f64(v_accumulator, items0);
                v_accumulator = vaddq_f64(v_accumulator, items1);
                v_accumulator = vaddq_f64(v_accumulator, items2);
                v_accumulator = vaddq_f64(v_accumulator, items3);

                i += 8;
            }

            while i + 2 < count {
                let slice = self.as_slice();
                let ptr = slice.as_ptr().add(i);
                let items = vld1q_f64(ptr);

                v_accumulator = vaddq_f64(v_accumulator, items);

                i += 2;
            }

            accumulator += vpaddd_f64(v_accumulator);
        }

        while i < count {
            accumulator += unsafe { *self.get_unchecked(i) };
            i += 1;
        }

        accumulator
    }
}
