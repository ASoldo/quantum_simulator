use crate::qubit::Qubit;
use num_complex::Complex;

pub struct Gate {
    pub matrix: Vec<Vec<Complex<f64>>>, // Matrix to handle multi-qubit gates
}

impl Gate {
    pub fn new(matrix: Vec<Vec<Complex<f64>>>) -> Self {
        Gate { matrix }
    }

    pub fn apply(&self, qubit: &mut Qubit) {
        let new_state: Vec<Complex<f64>> = self
            .matrix
            .iter()
            .map(|row| row.iter().zip(&qubit.state).map(|(m, q)| m * q).sum())
            .collect();
        qubit.state = new_state;
    }
}

pub fn hadamard(qubit_count: usize) -> Gate {
    let h = 1.0 / (2.0_f64).sqrt();
    let size = 2usize.pow(qubit_count as u32);
    let mut matrix = vec![vec![Complex::new(0.0, 0.0); size]; size];

    for (i, row) in matrix.iter_mut().enumerate().take(size) {
        for (j, elem) in row.iter_mut().enumerate().take(size) {
            let mut product = 1.0;
            for k in 0..qubit_count {
                let i_k = (i >> k) & 1;
                let j_k = (j >> k) & 1;
                if i_k == j_k {
                    product *= h;
                } else {
                    product *= -h;
                }
            }
            *elem = Complex::new(product, 0.0);
        }
    }

    Gate::new(matrix)
}

pub fn pauli_x() -> Gate {
    Gate::new(vec![
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}

pub fn pauli_y() -> Gate {
    Gate::new(vec![
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        vec![Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ])
}

pub fn pauli_z() -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ])
}

pub fn phase(theta: f64) -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![
            Complex::new(0.0, 0.0),
            Complex::new(theta.cos(), theta.sin()),
        ],
    ])
}

// S (Phase) gate: Also known as the S-gate, it is a Clifford gate that introduces a phase.
pub fn s() -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
    ])
}

// Example multi-qubit gate: CNOT gate for a 2-qubit system
pub fn cnot(control: usize, target: usize, num_qubits: usize) -> Gate {
    let size = 2usize.pow(num_qubits as u32);
    let mut matrix = vec![vec![Complex::new(0.0, 0.0); size]; size];

    for i in 0..size {
        for j in 0..size {
            if i == j {
                let control_bit = (i >> control) & 1;
                let _target_bit = (i >> target) & 1; // Prefix with underscore
                if control_bit == 1 {
                    matrix[i ^ (1 << target)][j] = Complex::new(1.0, 0.0);
                } else {
                    matrix[i][j] = Complex::new(1.0, 0.0);
                }
            }
        }
    }

    Gate::new(matrix)
}
