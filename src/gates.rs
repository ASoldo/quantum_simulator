//! This module defines the `Gate` struct and its associated methods for creating and applying quantum gates to qubits.
//! It includes implementations of common quantum gates such as the Hadamard, Pauli-X, Pauli-Y, Pauli-Z, and phase gates.

use crate::qubit::Qubit;
use num_complex::Complex;

/// A `Gate` represents a quantum gate, which is a linear transformation applied to a qubit.
pub struct Gate {
    pub matrix: [[Complex<f64>; 2]; 2],
}

impl Gate {
    /// Creates a new `Gate` with the specified transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `matrix` - A 2x2 complex matrix representing the quantum gate.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::gates::Gate;
    /// use num_complex::Complex;
    ///
    /// let matrix = [
    ///     [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ///     [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    /// ];
    /// let gate = Gate::new(matrix);
    /// ```
    pub fn new(matrix: [[Complex<f64>; 2]; 2]) -> Self {
        Gate { matrix }
    }

    /// Applies the gate to a given qubit, updating its state.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit to apply the gate to.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::gates::{Gate, hadamard};
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let gate = hadamard();
    /// let mut qubit = Qubit::new();
    /// gate.apply(&mut qubit);
    /// ```
    pub fn apply(&self, qubit: &mut Qubit) {
        let new_state = [
            self.matrix[0][0] * qubit.state[0] + self.matrix[0][1] * qubit.state[1],
            self.matrix[1][0] * qubit.state[0] + self.matrix[1][1] * qubit.state[1],
        ];
        qubit.state = new_state;
    }
}

/// Returns a Hadamard gate, which creates an equal superposition of states.
pub fn hadamard() -> Gate {
    let h = 1.0 / (2.0_f64).sqrt();
    Gate::new([
        [Complex::new(h, 0.0), Complex::new(h, 0.0)],
        [Complex::new(h, 0.0), Complex::new(-h, 0.0)],
    ])
}

/// Returns a Pauli-X gate, which flips the state of a qubit.
pub fn pauli_x() -> Gate {
    Gate::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}

/// Returns a Pauli-Y gate, which applies a 180-degree rotation around the Y-axis.
pub fn pauli_y() -> Gate {
    Gate::new([
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ])
}

/// Returns a Pauli-Z gate, which applies a 180-degree rotation around the Z-axis.
pub fn pauli_z() -> Gate {
    Gate::new([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ])
}

/// Returns a phase gate, which applies a phase shift of `theta` radians.
pub fn phase(theta: f64) -> Gate {
    Gate::new([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [
            Complex::new(0.0, 0.0),
            Complex::new(theta.cos(), theta.sin()),
        ],
    ])
}
