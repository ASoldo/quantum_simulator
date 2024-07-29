use quantum_simulator::circuit::Circuit;
use quantum_simulator::gates::{hadamard, pauli_x, pauli_y, pauli_z, phase};
use quantum_simulator::simulator::Simulator;
use std::io;

fn main() {
    let mut circuit = Circuit::new();

    println!("Enter initial qubit state (|0⟩, |1⟩, or custom [a, b] where |a|^2 + |b|^2 = 1):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let initial_state: [f64; 2] = match input.trim() {
        "|0>" => [1.0, 0.0],
        "|1>" => [0.0, 1.0],
        custom => {
            let values: Vec<f64> = custom
                .trim_matches(|p| p == '[' || p == ']')
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            [values[0], values[1]]
        }
    };

    loop {
        println!("Enter gate to add (Hadamard, Pauli-X, Pauli-Y, Pauli-Z, Phase [angle]) or 'Run' to execute:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed == "Run" {
            break;
        } else if trimmed.starts_with("Phase") {
            let angle: f64 = trimmed.split_whitespace().nth(1).unwrap().parse().unwrap();
            circuit.add_gate(phase(angle));
        } else {
            match trimmed {
                "Hadamard" => circuit.add_gate(hadamard()),
                "Pauli-X" => circuit.add_gate(pauli_x()),
                "Pauli-Y" => circuit.add_gate(pauli_y()),
                "Pauli-Z" => circuit.add_gate(pauli_z()),
                _ => println!("Unknown command"),
            }
        }
    }

    let final_qubit = Simulator::run(&circuit, initial_state);
    println!("Final qubit state: {:?}", final_qubit.state);

    println!("Command: Measure");
    let measurement = final_qubit.measure();
    println!("Measurement result: |{}⟩", measurement);
}
