//! This module defines the `Circuit` struct and its associated methods for managing
//! and running quantum circuits. A quantum circuit is a sequence of quantum gates
//! applied to qubits.

use crate::gates::Gate;
use crate::qubit::Qubit;

/// A `Circuit` represents a sequence of quantum gates that can be applied to qubits.
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

    /// Adds a quantum gate to the circuit.
    ///
    /// # Arguments
    ///
    /// * `gate` - The quantum gate to add to the circuit.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    /// use quantum_simulator::gates::hadamard;
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard());
    /// ```
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    /// Runs the circuit on a given qubit, applying each gate in sequence.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit to apply the circuit to.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    /// use quantum_simulator::gates::{hadamard, pauli_x};
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let mut circuit = Circuit::new();
    /// circuit.add_gate(hadamard());
    /// circuit.add_gate(pauli_x());
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
    /// Creates a default instance of `Circuit`, which is an empty circuit.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::circuit::Circuit;
    ///
    /// let circuit: Circuit = Default::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
