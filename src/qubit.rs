//! This module defines the `Qubit` struct and its associated methods.

use num_complex::Complex;

/// A `Qubit` represents a quantum bit, which can exist in a superposition of states.
pub struct Qubit {
    pub state: Vec<Complex<f64>>,
}

impl Qubit {
    /// Creates a new `Qubit` initialized to the `|0⟩` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    /// use num_complex::Complex;
    ///
    /// let qubit = Qubit::new();
    /// assert_eq!(qubit.state[0], Complex::new(1.0, 0.0));
    /// assert_eq!(qubit.state[1], Complex::new(0.0, 0.0));
    /// ```
    pub fn new() -> Self {
        Qubit {
            state: vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        }
    }

    /// Creates a new `Qubit` initialized to the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - A vector representing the qubit state.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    /// use num_complex::Complex;
    ///
    /// let state = vec![Complex::new(0.707, 0.0), Complex::new(0.707, 0.0)];
    /// let qubit = Qubit::from_state(state);
    /// assert_eq!(qubit.state[0], Complex::new(0.707, 0.0));
    /// assert_eq!(qubit.state[1], Complex::new(0.707, 0.0));
    /// ```
    pub fn from_state(state: Vec<Complex<f64>>) -> Self {
        Qubit { state }
    }

    /// Measures the qubit, collapsing its state to one of the basis states.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_simulator::qubit::Qubit;
    ///
    /// let qubit = Qubit::new();
    /// let result = qubit.measure();
    /// assert!(result == 0 || result == 1);
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
    /// use num_complex::Complex;
    ///
    /// let qubit = Qubit::default();
    /// assert_eq!(qubit.state[0], Complex::new(1.0, 0.0));
    /// assert_eq!(qubit.state[1], Complex::new(0.0, 0.0));
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
