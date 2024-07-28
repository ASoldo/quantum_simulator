use crate::qubit::Qubit;
use num_complex::Complex;

pub struct Gate {
    pub matrix: [[Complex<f64>; 2]; 2],
}

impl Gate {
    pub fn new(matrix: [[Complex<f64>; 2]; 2]) -> Self {
        Gate { matrix }
    }

    pub fn apply(&self, qubit: &mut Qubit) {
        let new_state = [
            self.matrix[0][0] * qubit.state[0] + self.matrix[0][1] * qubit.state[1],
            self.matrix[1][0] * qubit.state[0] + self.matrix[1][1] * qubit.state[1],
        ];
        qubit.state = new_state;
    }
}

pub fn hadamard() -> Gate {
    let h = 1.0 / (2.0_f64).sqrt();
    Gate::new([
        [Complex::new(h, 0.0), Complex::new(h, 0.0)],
        [Complex::new(h, 0.0), Complex::new(-h, 0.0)],
    ])
}

pub fn pauli_x() -> Gate {
    Gate::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}
