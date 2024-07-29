//! This module defines the `Simulator` struct and its associated methods for running quantum circuits on qubits.

use crate::circuit::Circuit;
use crate::qubit::Qubit;
use num_complex::Complex;

/// The `Simulator` struct provides functionality to run quantum circuits on qubits.
pub struct Simulator;

impl Simulator {
    /// Runs the given quantum circuit on an initial qubit state and returns the final qubit state.
    ///
    /// # Arguments
    ///
    /// * `circuit` - A reference to the quantum circuit to be run.
    /// * `initial_state` - A reference to a vector representing the initial state of the qubit.
    ///
    /// # Returns
    ///
    /// * A `Qubit` representing the final state after the circuit has been applied.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    /// use quantum_simulator::gates::{hadamard, pauli_x};
    /// use quantum_simulator::simulator::Simulator;
    /// use num_complex::Complex;
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard(1));
    /// circuit.add_gate(pauli_x());
    ///
    /// let initial_state = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]; // |0⟩ state
    /// let final_qubit = Simulator::run(&circuit, &initial_state);
    /// assert_eq!(final_qubit.state.len(), 2);
    /// assert!(final_qubit.state.iter().all(|&c| c.im == 0.0)); // Check if all imaginary parts are zero
    /// ```
    pub fn run(circuit: &Circuit, initial_state: &[Complex<f64>]) -> Qubit {
        let mut qubit = Qubit::from_state(initial_state.to_vec());
        circuit.run(&mut qubit);
        qubit
    }
}
