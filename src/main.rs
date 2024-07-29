use num_complex::Complex;
use quantum_simulator::circuit::Circuit;
use quantum_simulator::gates::{hadamard, pauli_x, pauli_y, pauli_z, phase};
use quantum_simulator::simulator::Simulator;
use std::io;

fn main() {
    let mut circuit = Circuit::new();

    println!("Enter initial qubit state (|0⟩, |1⟩, or custom [a, b] where |a|^2 + |b|^2 = 1):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let initial_state: Vec<Complex<f64>> = match input.trim() {
        "|0>" => vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], // |0⟩
        "|1>" => vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)], // |1⟩
        custom => {
            let values: Vec<f64> = custom
                .trim_matches(|p| p == '[' || p == ']')
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let norm = (values[0].powi(2) + values[1].powi(2)).sqrt();
            vec![
                Complex::new(values[0] / norm, 0.0),
                Complex::new(values[1] / norm, 0.0),
            ]
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

    let final_qubit = Simulator::run(&circuit, [initial_state[0].re, initial_state[1].re]);
    println!("Final qubit state: {:?}", final_qubit.state);

    // Calculate angles for Bloch sphere representation
    let (theta, phi) = calculate_angles(final_qubit.state);
    println!("Theta: {}, Phi: {}", theta, phi);

    // Calculate Cartesian coordinates for visualization
    let (x, y, z) = bloch_sphere_coordinates(theta, phi);
    println!(
        "Cartesian coordinates on Bloch sphere: (x: {}, y: {}, z: {})",
        x, y, z
    );

    println!("Command: Measure");
    let measurement = final_qubit.measure();
    println!("Measurement result: |{}⟩", measurement);
}

fn calculate_angles(state: [Complex<f64>; 2]) -> (f64, f64) {
    let alpha = state[0];
    let beta = state[1];

    let theta = 2.0 * alpha.norm().acos();
    let phi = beta.arg() - alpha.arg();

    (theta, phi)
}

fn bloch_sphere_coordinates(theta: f64, phi: f64) -> (f64, f64, f64) {
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    (x, y, z)
}
