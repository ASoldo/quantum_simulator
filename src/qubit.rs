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
}

impl Default for Qubit {
    fn default() -> Self {
        Self::new()
    }
}
