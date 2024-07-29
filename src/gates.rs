//! This module defines various quantum gates and their associated methods.

use crate::qubit::Qubit;
use num_complex::Complex;

/// A `Gate` represents a quantum gate with a matrix for multi-qubit operations.
pub struct Gate {
    pub matrix: Vec<Vec<Complex<f64>>>, // Matrix to handle multi-qubit gates
}

impl Gate {
    /// Creates a new `Gate` with the given matrix.
    ///
    /// # Arguments
    ///
    /// * `matrix` - A 2D vector representing the gate matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::gates::Gate;
    /// use num_complex::Complex;
    ///
    /// let matrix = vec![
    ///     vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ///     vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    /// ];
    /// let gate = Gate::new(matrix);
    /// ```
    pub fn new(matrix: Vec<Vec<Complex<f64>>>) -> Self {
        Gate { matrix }
    }

    /// Applies the gate to the given qubit.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit on which to apply the gate.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::gates::{Gate, pauli_x};
    /// use quantum_simulator::qubit::Qubit;
    /// use num_complex::Complex;
    ///
    /// let gate = pauli_x();
    /// let mut qubit = Qubit::new();
    /// gate.apply(&mut qubit);
    /// assert_eq!(qubit.state[0], Complex::new(0.0, 0.0));
    /// assert_eq!(qubit.state[1], Complex::new(1.0, 0.0));
    /// ```
    pub fn apply(&self, qubit: &mut Qubit) {
        let new_state: Vec<Complex<f64>> = self
            .matrix
            .iter()
            .map(|row| row.iter().zip(&qubit.state).map(|(m, q)| m * q).sum())
            .collect();
        qubit.state = new_state;
    }
}

/// Returns a Hadamard gate for the given number of qubits.
///
/// # Arguments
///
/// * `qubit_count` - The number of qubits.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::hadamard;
/// let gate = hadamard(1);
/// ```
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

/// Returns a Pauli-X gate.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::pauli_x;
/// let gate = pauli_x();
/// ```
pub fn pauli_x() -> Gate {
    Gate::new(vec![
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}

/// Returns a Pauli-Y gate.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::pauli_y;
/// let gate = pauli_y();
/// ```
pub fn pauli_y() -> Gate {
    Gate::new(vec![
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        vec![Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ])
}

/// Returns a Pauli-Z gate.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::pauli_z;
/// let gate = pauli_z();
/// ```
pub fn pauli_z() -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ])
}

/// Returns a phase gate with the given angle.
///
/// # Arguments
///
/// * `theta` - The phase angle.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::phase;
/// let gate = phase(std::f64::consts::PI / 2.0);
/// ```
pub fn phase(theta: f64) -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![
            Complex::new(0.0, 0.0),
            Complex::new(theta.cos(), theta.sin()),
        ],
    ])
}

/// Returns an S (Phase) gate, which is a Clifford gate that introduces a phase.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::s;
/// let gate = s();
/// ```
pub fn s() -> Gate {
    Gate::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
    ])
}

/// Returns a CNOT gate for a given control and target qubit in a multi-qubit system.
///
/// # Arguments
///
/// * `control` - The control qubit index.
/// * `target` - The target qubit index.
/// * `num_qubits` - The total number of qubits.
///
/// # Examples
///
/// ```
/// use quantum_simulator::gates::cnot;
/// let gate = cnot(0, 1, 2);
/// ```
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
