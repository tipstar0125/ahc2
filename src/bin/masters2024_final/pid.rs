pub struct Pid {
    kp: f64,
    ki: f64,
    kd: f64,
    sum_error: f64,
    prev_error: f64,
}

impl Pid {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            sum_error: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn update(&mut self, error: f64) -> f64 {
        self.sum_error += error;
        let p = self.kp * error;
        let i = self.ki * self.sum_error;
        let d = self.kd * (error - self.prev_error);
        self.prev_error = error;
        p + i + d
    }

    pub fn reset(&mut self) {
        self.sum_error = 0.0;
        self.prev_error = 0.0;
    }
}

