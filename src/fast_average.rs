use crate::fast_sum::FastSum;

pub trait Average<T> {
    fn average(&self) -> Option<T>;
}

impl Average<f64> for Vec<f32> {
    fn average(&self) -> Option<f64> {
        if self.len() == 0 {
            return None;
        }
        let count = self.len();
        let accumulator = self.sums();

        return Some(accumulator / count as f64);
    }
}
