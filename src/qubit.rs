use num_complex::Complex;

pub struct Qubit {
    pub state: [Complex<f64>; 2],
}

impl Qubit {
    pub fn new() -> Self {
        Qubit {
            state: [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        }
    }

    pub fn from_state(state: [Complex<f64>; 2]) -> Self {
        Qubit { state }
    }

    pub fn measure(&self) -> usize {
        let prob_0 = self.state[0].norm_sqr();
        let random_number = rand::random::<f64>();

        if random_number < prob_0 {
            0
        } else {
            1
        }
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Self::new()
    }
}
