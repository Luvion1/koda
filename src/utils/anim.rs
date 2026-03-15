pub fn lerp(start: f64, end: f64, amount: f64) -> f64 {
    start + (end - start) * amount
}

pub fn ease_out_expo(x: f64) -> f64 {
    if x == 1.0 {
        1.0
    } else {
        1.0 - (-10.0 * x).exp2()
    }
}

pub struct AnimatedValue {
    pub current: f64,
    pub target: f64,
}

impl AnimatedValue {
    pub fn new(initial: f64) -> Self {
        Self {
            current: initial,
            target: initial,
        }
    }

    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    pub fn update(&mut self, speed: f64) -> bool {
        let diff = (self.target - self.current).abs();
        if diff < 0.01 {
            self.current = self.target;
            return false;
        }

        self.current = lerp(self.current, self.target, speed);
        true
    }
}
