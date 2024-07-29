#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use quantum_simulator::circuit::Circuit;
    use quantum_simulator::gates::{cnot, hadamard, pauli_x, pauli_y, pauli_z, phase};
    use quantum_simulator::simulator::Simulator;

    const TOLERANCE: f64 = 1e-10;

    fn complex_approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
        (a - b).norm() < tol
    }

    #[test]
    fn test_hadamard_pauli_x() {
        let mut circuit = Circuit::new();
        circuit.add_gate(hadamard(1));
        circuit.add_gate(pauli_x());

        let initial_state = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]; // |0⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(0.7071067811865475, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(0.7071067811865475, 0.0),
            TOLERANCE
        ));
    }

    #[test]
    fn test_pauli_y() {
        let mut circuit = Circuit::new();
        circuit.add_gate(pauli_y());

        let initial_state = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]; // |0⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(0.0, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(0.0, 1.0),
            TOLERANCE
        ));
    }

    #[test]
    fn test_pauli_z() {
        let mut circuit = Circuit::new();
        circuit.add_gate(pauli_z());

        let initial_state = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]; // |0⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(1.0, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(0.0, 0.0),
            TOLERANCE
        ));
    }

    #[test]
    fn test_phase() {
        let mut circuit = Circuit::new();
        circuit.add_gate(phase(std::f64::consts::PI / 2.0));

        let initial_state = vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]; // |1⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(0.0, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(6.123233995736766e-17, 1.0),
            TOLERANCE
        ));
    }

    #[test]
    fn test_hadamard_multi_qubit() {
        let mut circuit = Circuit::new();
        circuit.add_gate(hadamard(3));

        let initial_state = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ]; // |000⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        let expected_amplitude = 1.0 / (2.0_f64).sqrt();
        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(expected_amplitude, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(expected_amplitude, 0.0),
            TOLERANCE
        ));
    }

    #[test]
    fn test_cnot() {
        let mut circuit = Circuit::new();
        circuit.add_gate(hadamard(3));
        circuit.add_gate(cnot(0, 1, 3));

        let initial_state = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ]; // |000⟩ state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert!(complex_approx_eq(
            final_qubit.state[0],
            Complex::new(0.5, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[1],
            Complex::new(0.5, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[2],
            Complex::new(0.0, 0.0),
            TOLERANCE
        ));
        assert!(complex_approx_eq(
            final_qubit.state[3],
            Complex::new(0.0, 0.0),
            TOLERANCE
        ));
    }
}
