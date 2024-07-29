//! This module defines the `Qubit` struct and its associated methods for representing
//! and manipulating quantum bits (qubits). It includes functions for creating new qubits,
//! initializing qubits from a given state, and measuring qubits.

use num_complex::Complex;

/// A `Qubit` represents a quantum bit, which can exist in a superposition of states.
pub struct Qubit {
    pub state: [Complex<f64>; 2],
}

impl Qubit {
    /// Creates a new `Qubit` initialized to the `|0⟩` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let qubit = Qubit::new();
    /// ```
    pub fn new() -> Self {
        Qubit {
            state: [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        }
    }

    /// Creates a new `Qubit` initialized to the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - A 2-dimensional complex vector representing the qubit state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    /// use num_complex::Complex;
    ///
    /// let state = [Complex::new(0.707, 0.0), Complex::new(0.707, 0.0)];
    /// let qubit = Qubit::from_state(state);
    /// ```
    pub fn from_state(state: [Complex<f64>; 2]) -> Self {
        Qubit { state }
    }

    /// Measures the qubit, collapsing its state to either `|0⟩` or `|1⟩`.
    ///
    /// # Returns
    ///
    /// * `0` if the qubit collapses to the `|0⟩` state.
    /// * `1` if the qubit collapses to the `|1⟩` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let qubit = Qubit::new();
    /// let result = qubit.measure();
    /// println!("Measurement result: {}", result);
    /// ```
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
    /// Creates a default instance of `Qubit`, which is initialized to the `|0⟩` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let qubit: Qubit = Default::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
