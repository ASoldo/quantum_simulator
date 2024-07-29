use quantum_simulator::circuit::Circuit;
use quantum_simulator::gates::{hadamard, pauli_x, pauli_y, pauli_z, phase};
use quantum_simulator::simulator::Simulator;

fn main() {
    let mut circuit = Circuit::new();
    circuit.add_gate(hadamard());
    circuit.add_gate(pauli_x());
    circuit.add_gate(pauli_y());
    circuit.add_gate(pauli_z());
    circuit.add_gate(phase(1.0));

    let initial_state = [1.0, 0.0]; // |0> state
    let final_qubit = Simulator::run(&circuit, initial_state);

    println!("Final qubit state: {:?}", final_qubit.state);
}
