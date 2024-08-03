use crate::fast_sum::FastSum;

pub trait Average<T> {
    fn average(&self) -> T;
}

impl Average<f64> for Vec<f32> {
    fn average(&self) -> f64 {
        if self.len() == 0 {
            return 0.;
        }
        let count = self.len();
        let accumulator = self.sums();

        return accumulator / count as f64;
    }
}
