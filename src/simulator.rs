use crate::circuit::Circuit;
use crate::qubit::Qubit;
use num_complex::Complex;

pub struct Simulator;

impl Simulator {
    pub fn run(circuit: &Circuit, initial_state: [f64; 2]) -> Qubit {
        let mut qubit = Qubit::from_state([
            Complex::new(initial_state[0], 0.0),
            Complex::new(initial_state[1], 0.0),
        ]);
        circuit.run(&mut qubit);
        qubit
    }
}
