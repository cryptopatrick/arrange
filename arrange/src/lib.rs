#[allow(dead_code)]
///
#[derive(Debug)]
pub struct IntRange {
    start: i32,
    stop: i32,
    step: usize,
}

impl IntRange {
    pub fn new(start: i32, stop: i32, step: usize) -> IntRange {
        // The step size has to be positive
        if step < 1 {
            panic!("Range: step has to be a positive whole number.");
        }

        IntRange { start, stop, step }
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.stop - self.start).abs() as usize
    }

    pub fn range(&self) -> Vec<i32> {
        if self.step == 0 {
            panic!("Range: step cannot be zero");
        }
        if self.start == self.stop {
            panic!("Range: start and stop cannot be the same");
        }

        let mut vector: Vec<i32> = Vec::with_capacity(self.len());

        if self.start > self.stop {
            // Handles decreasing ranges, for example: start=1 to stop=-10, by step=1
            for v in (self.stop..=self.start).rev().step_by(self.step) {
                vector.push(v);
            }
        } else {
            // Handles increasing ranges, for example: start=1 to stop=10, by step=1
            for v in (self.start..self.stop).step_by(self.step) {
                vector.push(v);
            }
        }

        vector
    }
}

#[derive(Debug)]
pub struct FloatRange {
    start: f64,
    stop: f64,
    step: f64,
}

impl FloatRange {
    pub fn new(start: f64, stop: f64, step: f64) -> FloatRange {
        // The step size has to be positive
        if step <= 0f64 {
            panic!("Range: step has to be a positive number.");
        }

        FloatRange { start, stop, step }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // (2.0-1.0)/0.2=5
        ((self.stop - self.start) / self.step).abs() as usize
    }

    pub fn range(&self) -> Vec<f64> {
        if self.step <= 0f64 {
            panic!("Range: step has to be a positive number");
        }
        if self.start == self.stop {
            panic!("Range: start and stop cannot be the same");
        }

        let mut vector: Vec<f64> = Vec::with_capacity(self.len());

        if self.start < self.stop {
            // Decreasing ranges, e.g.: start=1 to stop=-10, by step=1
            let mut v: f64 = self.start;
            while v < self.stop {
                vector.push(v);
                v = v + self.step;
            }
        } else if self.start > self.stop {
            // Increasing ranges, e.g.: start=1 to stop=10, by step=1
            let mut v: f64 = self.start;
            while v > self.stop {
                vector.push(v);
                v = v - self.step;
            }
        }

        vector
    }
}
