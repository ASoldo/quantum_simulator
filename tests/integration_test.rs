#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use quantum_simulator::circuit::Circuit;
    use quantum_simulator::gates::{hadamard, pauli_x, pauli_y, pauli_z};
    use quantum_simulator::simulator::Simulator;

    #[test]
    fn test_hadamard_pauli_x() {
        let mut circuit = Circuit::new();
        circuit.add_gate(hadamard());
        circuit.add_gate(pauli_x());

        let initial_state = [1.0, 0.0]; // |0> state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert_eq!(final_qubit.state[0], Complex::new(0.7071067811865475, 0.0));
        assert_eq!(final_qubit.state[1], Complex::new(0.7071067811865475, 0.0));
    }

    #[test]
    fn test_pauli_y() {
        let mut circuit = Circuit::new();
        circuit.add_gate(pauli_y());

        let initial_state = [1.0, 0.0]; // |0> state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert_eq!(final_qubit.state[0], Complex::new(0.0, 0.0));
        assert_eq!(final_qubit.state[1], Complex::new(0.0, 1.0));
    }

    #[test]
    fn test_pauli_z() {
        let mut circuit = Circuit::new();
        circuit.add_gate(pauli_z());

        let initial_state = [1.0, 0.0]; // |0> state
        let final_qubit = Simulator::run(&circuit, initial_state);

        assert_eq!(final_qubit.state[0], Complex::new(1.0, 0.0));
        assert_eq!(final_qubit.state[1], Complex::new(0.0, 0.0));
    }
}
