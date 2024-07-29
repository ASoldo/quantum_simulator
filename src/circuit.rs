//! This module defines the `Circuit` struct and its associated methods for managing and running quantum circuits.

use crate::gates::Gate;
use crate::qubit::Qubit;

/// A `Circuit` represents a sequence of quantum gates to be applied to qubits.
pub struct Circuit {
    gates: Vec<Gate>,
}

impl Circuit {
    /// Creates a new, empty `Circuit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    ///
    /// let circuit = Circuit::new();
    /// ```
    pub fn new() -> Self {
        Circuit { gates: vec![] }
    }

    /// Adds a gate to the circuit.
    ///
    /// # Arguments
    ///
    /// * `gate` - The quantum gate to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    /// use quantum_simulator::gates::hadamard;
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard(1));
    /// ```
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    /// Runs the circuit on the given qubit.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit on which to run the circuit.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    /// use quantum_simulator::gates::hadamard;
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard(1));
    ///
    /// let mut qubit = Qubit::new();
    /// circuit.run(&mut qubit);
    /// ```
    pub fn run(&self, qubit: &mut Qubit) {
        for gate in &self.gates {
            gate.apply(qubit);
        }
    }
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}
