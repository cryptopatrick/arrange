/// There are three fields that affect the range:
/// start : the starting value of the range
/// stop :  the stoping value of the range
/// step :  the distance from one element in the range to the next
///
/// Please note that the step is unidirectional.
/// we can only move inside the range implicitly
/// set by the start and stop range:
/// If start < stop, then the range is [start,stop).
/// If stop < start, then the range is (stop,start].
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

    /// Returns the length of the arrange.
    ///
    /// TODO: handle steps that are diff than 1
    #[inline]
    pub fn len(&self) -> usize {
        (self.stop - self.start).abs() as usize
    }

    /// Creates a range of Vec<i32> type.
    ///
    ///
    pub fn range(&self) -> Vec<i32> {
        // Checking range inputs to verify that they can be used to generate a range
        if self.step == 0 {
            panic!("Range: step cannot be zero");
        }
        // The start and stop value need to differ
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

    /// Returns the length of the arrange.
    ///
    /// TODO: handle steps that are diff than 1
    #[inline]
    pub fn len(&self) -> usize {
        // (2.0-1.0)/0.2=5
        ((self.stop - self.start) / self.step).abs() as usize
    }

    /// Creates a range of Vec<i32> type.
    ///
    ///
    pub fn range(&self) -> Vec<f64> {
        // Checking range inputs to verify that they can be used to generate a range
        if self.step <= 0f64 {
            panic!("Range: step has to be a positive number");
        }
        if self.start == self.stop {
            panic!("Range: start and stop cannot be the same");
        }

        // vector is used to hold the range of values we generate based on the
        // start, stop, step values passed.
        let mut vector: Vec<f64> = Vec::with_capacity(self.len());

        // Two different loops are used to handle two different key situations
        // 1. The start value is lower than the stop value.
        // 2. The start value is higher than the stop value.
        // (1) is supposed to be the more common pattern.
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
