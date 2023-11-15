#[derive(Clone, Debug)]
pub struct Similarities {
    nums: Vec<u8>,
}

impl Similarities {
    pub fn new() -> Self {
        Similarities { nums: Vec::new() }
    }

    pub fn add(&mut self, num: u8) {
        self.nums.push(num);
    }

    pub fn empty(&self) -> bool {
        self.nums.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nums.len()
    }

    pub fn len_non_zeroes(&self) -> usize {
        self.nums.iter().filter(|x| **x > 0u8).count()
    }

    pub fn num_zeroes(&self) -> usize {
        self.nums.iter().filter(|x| **x == 0u8).count()
    }

    pub fn avg(&self) -> f32 {
        let sum: u32 = self.nums.iter().map(|x| *x as u32).sum();
        sum as f32 / self.nums.len() as f32
    }

    pub fn avg_non_zeroes(&self) -> f32 {
        let sum: u32 = self
            .nums
            .iter()
            .filter(|x| **x > 0u8)
            .map(|x| *x as u32)
            .sum();
        sum as f32 / self.len_non_zeroes() as f32
    }

    pub fn std_dev(&self) -> f32 {
        let avg = self.avg();
        let variance: f32 = self
            .nums
            .iter()
            .map(|x| {
                let s = *x as f32 - avg;
                s.powf(2.0)
            })
            .sum();
        let variance: f32 = variance / self.nums.len() as f32;
        variance.powf(0.5)
    }

    pub fn std_dev_non_zeroes(&self) -> f32 {
        let avg = self.avg_non_zeroes();
        let variance: f32 = self
            .nums
            .iter()
            .filter(|x| **x > 0u8)
            .map(|x| {
                let s = *x as f32 - avg;
                s.powf(2.0)
            })
            .sum();
        let variance: f32 = variance / self.len_non_zeroes() as f32;
        variance.powf(0.5)
    }
}

#[cfg(test)]
mod tests {
    use crate::stats::Similarities;

    #[test]
    fn test_avg() {
        let mut similarities = Similarities::new();
        similarities.add(10);
        similarities.add(20);
        similarities.add(30);
        assert_eq!(similarities.avg(), 20.0);
        assert_eq!(similarities.num_zeroes(), 0);
        similarities.add(0);
        assert_eq!(similarities.avg_non_zeroes(), 20.0);
        assert_eq!(similarities.avg(), 15.0);
        assert_eq!(similarities.num_zeroes(), 1);
    }
}
