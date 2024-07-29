![image](https://github.com/user-attachments/assets/e3896286-665a-46c6-92d2-ca0195a022b3)

# Quantum Simulator

A Rust-based quantum computing simulator that allows you to model and simulate basic quantum operations on qubits.

## Overview

This project provides a framework to represent qubits, apply quantum gates, construct quantum circuits, and simulate their behavior. The simulator supports a variety of single-qubit gates, including the Hadamard, Pauli-X, Pauli-Y, and Pauli-Z gates, as well as the phase gate.

## Features

- **Qubit Representation**: Model qubits using complex numbers.
- **Quantum Gates**: Apply basic quantum gates such as Hadamard, Pauli-X, Pauli-Y, Pauli-Z, and phase gates.
- **Quantum Circuits**: Construct circuits by chaining gates together.
- **Simulation**: Run circuits on initial qubit states and observe the final states.
- **Measurement**: Measure the state of a qubit.

## Getting Started

### Prerequisites

- Rust programming language (install from [rustup.rs](https://www.rustup.rs/))

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
