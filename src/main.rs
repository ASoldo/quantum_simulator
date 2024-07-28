use quantum_simulator::circuit::Circuit;
use quantum_simulator::gates::{hadamard, pauli_x};
use quantum_simulator::simulator::Simulator;

fn main() {
    let mut circuit = Circuit::new();
    circuit.add_gate(hadamard());
    circuit.add_gate(pauli_x());

    let initial_state = [1.0, 0.0]; // |0> state
    let final_qubit = Simulator::run(&circuit, initial_state);

    println!("Final qubit state: {:?}", final_qubit.state);
}
