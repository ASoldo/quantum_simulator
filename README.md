![image](https://github.com/user-attachments/assets/43ce8205-2ad9-4c97-bc6d-dd98da1a8de4)

# Quantum Simulator

A Rust-based quantum computing simulator that allows you to model and simulate basic quantum operations on qubits.

## Overview

This project provides a framework to represent qubits, apply quantum gates, construct quantum circuits, and simulate their behavior. The simulator supports a variety of single-qubit and multi-qubit gates, including the Hadamard, Pauli-X, Pauli-Y, Pauli-Z, Phase, and Clifford gates. Additionally, it includes visualization capabilities to render qubits on a Bloch sphere using the Bevy game engine.

## Features

- **Qubit Representation**: Model qubits using complex numbers.
- **Quantum Gates**: Apply basic quantum gates such as Hadamard, Pauli-X, Pauli-Y, Pauli-Z, Phase, and Clifford gates (S-gate).
- **Quantum Circuits**: Construct circuits by chaining gates together.
- **Simulation**: Run circuits on initial qubit states and observe the final states.
- **Measurement**: Measure the state of a qubit.
- **Visualization**: Render qubits on a Bloch sphere using Bevy, with visual aids like arrows to indicate qubit positions.

## Getting Started

### Prerequisites

- Rust programming language (install from [rustup.rs](https://rustup.rs))
- A modern graphics card and drivers for running the Bevy engine

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/ASoldo/quantum_simulator.git
   cd quantum_simulator
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run the project:

   ```sh
   cargo run
   ```

4. Run the tests:
   ```sh
   cargo test
   ```

## Usage

### Running the Simulator

By running the project with `cargo run`, you can observe the simulation of quantum circuits and visualize the qubits on a Bloch sphere. The final state of the qubits, their probabilities, and their positions on the Bloch sphere will be printed to the console.

### Example Quantum Circuit

Here's an example of a quantum circuit setup used in the simulator:

```rust
fn run_quantum_simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_qubits: usize = 4;

    // Define the initial state |0000>
    let initial_state: Vec<Complex<f64>> = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        // ... initialize other states ...
    ];

    let mut circuit = Circuit::new();

    // Apply different gates to each qubit
    circuit.add_gate(hadamard(num_qubits)); // Apply Hadamard to all qubits
    circuit.add_gate(cnot(0, 1, num_qubits)); // Apply CNOT with control=0, target=1
    circuit.add_gate(pauli_x()); // Apply Pauli-X gate to all qubits
    // ... add other gates ...

    let final_qubit = Simulator::run(&circuit, &initial_state);
    println!("Final qubit state: {:?}", final_qubit.state);

    // Display probabilities and visualizations
    // ... visualization code ...
}
Visualization
The visualization includes:

A central white sphere representing the Bloch sphere.
Small black spheres representing the qubits.
Colored arrows indicating the direction from the center to each qubit on the Bloch sphere.
Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.
```
