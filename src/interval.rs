#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval{min,max}
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Interval {
        Interval{min:a.min.min(b.min), max:a.max.max(b.max)}
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min <  x && x <  self.max;
    }

    pub fn clamp(&self, x: f64) -> f64{
        if x < self.min {return self.min;}
        if x > self.max {return self.max;}
        return x;
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval{min:self.min - padding, max: self.max + padding}
    }

    pub fn shrink(&mut self, a:f64, b:f64) {
        self.min = a.max(self.min);
        self.max = b.min(self.max);
    }

    const EMPTY: Interval = Interval{min:f64::INFINITY, max:f64::NEG_INFINITY};
    const UNIVERSE: Interval = Interval{min:f64::NEG_INFINITY, max:f64::INFINITY};
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}


