pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval{min,max}
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min <  x && x <  self.max;
    }

    const EMPTY: Interval = Interval{min:f64::INFINITY, max:f64::NEG_INFINITY};
    const UNIVERSE: Interval = Interval{min:f64::NEG_INFINITY, max:f64::INFINITY};
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}


