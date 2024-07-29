//! This module defines the `Simulator` struct and its associated methods for running
//! quantum circuits on qubits.

use crate::circuit::Circuit;
use crate::qubit::Qubit;
use num_complex::Complex;

/// The `Simulator` struct provides a method to run quantum circuits on qubits.
pub struct Simulator;

impl Simulator {
    /// Runs the given quantum circuit on an initial qubit state and returns the final qubit state.
    ///
    /// # Arguments
    ///
    /// * `circuit` - A reference to the quantum circuit to be run.
    /// * `initial_state` - An array representing the initial state of the qubit in terms of real numbers.
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
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard());
    /// circuit.add_gate(pauli_x());
    ///
    /// let initial_state = [1.0, 0.0]; // |0⟩ state
    /// let final_qubit = Simulator::run(&circuit, initial_state);
    /// println!("Final qubit state: {:?}", final_qubit.state);
    /// ```
    pub fn run(circuit: &Circuit, initial_state: [f64; 2]) -> Qubit {
        let mut qubit = Qubit::from_state([
            Complex::new(initial_state[0], 0.0),
            Complex::new(initial_state[1], 0.0),
        ]);
        circuit.run(&mut qubit);
        qubit
    }
}
