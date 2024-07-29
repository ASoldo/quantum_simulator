use bevy::color::palettes::tailwind::{BLUE_500, GREEN_500, RED_500};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use num_complex::Complex;
use quantum_simulator::circuit::Circuit;
use quantum_simulator::gates::{cnot, hadamard, pauli_x, pauli_y, pauli_z, s};
use quantum_simulator::simulator::Simulator;

// Components
#[derive(Component)]
struct QubitSphere;

#[derive(Component)]
struct Position;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_camera_and_light)
        .add_systems(Startup, run_quantum_simulation)
        .add_systems(Update, gizmo_draw)
        .run();
}

fn setup_camera_and_light(mut commands: Commands) {
    // Setup camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera::default(),
    ));

    // Setup light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..Default::default()
    });
}

fn run_quantum_simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_qubits: usize = 4;

    // Define the initial state |000>
    let initial_state: Vec<Complex<f64>> = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    let mut circuit = Circuit::new();

    // Apply different gates to each qubit
    circuit.add_gate(hadamard(num_qubits)); // Apply Hadamard to all qubits
    circuit.add_gate(cnot(0, 1, num_qubits)); // Apply CNOT with control=0, target=1
    circuit.add_gate(pauli_x()); // Apply Pauli-X gate to all qubits
    circuit.add_gate(cnot(1, 2, num_qubits)); // Apply CNOT with control=1, target=2
    circuit.add_gate(pauli_y()); // Apply Pauli-Y gate to all qubits
    circuit.add_gate(cnot(2, 3, num_qubits)); // Apply CNOT with control=2, target=3
    circuit.add_gate(s()); // Apply S gate to all qubits
    circuit.add_gate(pauli_z()); // Apply Pauli-Z gate to all qubits

    let final_qubit = Simulator::run(&circuit, &initial_state);
    println!("Final qubit state: {:?}", final_qubit.state);

    // Display the probabilities of each basis state
    let probabilities: Vec<f64> = final_qubit.state.iter().map(|amp| amp.norm_sqr()).collect();
    for (index, prob) in probabilities.iter().enumerate() {
        println!("|{}>: {:.4}", index, prob);
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.0).mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    });

    // Calculate and display Bloch sphere coordinates for each qubit
    for qubit_index in 0..num_qubits {
        let reduced_state = get_reduced_state(&final_qubit.state, qubit_index, num_qubits);
        let (theta, phi) = calculate_angles(reduced_state);
        println!("Qubit {}: Theta: {}, Phi: {}", qubit_index, theta, phi);
        let (x, y, z) = bloch_sphere_coordinates(theta, phi);
        println!(
            "Qubit {}: Cartesian coordinates on Bloch sphere: (x: {}, y: {}, z: {})",
            qubit_index, x, y, z
        );

        // Add sphere for each qubit
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1).mesh()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.0, 0.0),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                ..Default::default()
            })
            .insert(QubitSphere)
            .insert(Position);
    }

    println!("Command: Measure");
    let measurement = final_qubit.measure();
    println!("Measurement result: |{}>", measurement);
}

fn get_reduced_state(
    state: &[Complex<f64>],
    qubit_index: usize,
    _num_qubits: usize,
) -> [Complex<f64>; 2] {
    let mut reduced_state = [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)];
    let mask = 1 << qubit_index;

    for (i, amplitude) in state.iter().enumerate() {
        if (i & mask) == 0 {
            reduced_state[0] += amplitude;
        } else {
            reduced_state[1] += amplitude;
        }
    }

    let norm = (reduced_state[0].norm_sqr() + reduced_state[1].norm_sqr()).sqrt();
    if norm > 0.0 {
        reduced_state[0] /= norm;
        reduced_state[1] /= norm;
    }

    reduced_state
}

fn calculate_angles(state: [Complex<f64>; 2]) -> (f64, f64) {
    let alpha = state[0];
    let beta = state[1];

    let theta = 2.0 * beta.norm().acos();
    let phi = if alpha.norm() == 0.0 {
        0.0
    } else {
        alpha.arg() - beta.arg()
    };

    (theta, phi)
}

fn bloch_sphere_coordinates(theta: f64, phi: f64) -> (f64, f64, f64) {
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    (x, y, z)
}

fn gizmo_draw(mut gizmos: Gizmos) {
    gizmos
        .grid_3d(
            Vec3::ZERO,
            Quat::IDENTITY,
            UVec3::new(2, 0, 2),
            Vec3::splat(1.),
            RED_500,
        )
        .outer_edges();
    gizmos
        .grid_3d(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0),
            UVec3::new(2, 0, 2),
            Vec3::splat(1.),
            GREEN_500,
        )
        .outer_edges();

    gizmos
        .grid_3d(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.0),
            UVec3::new(2, 0, 2),
            Vec3::splat(1.),
            BLUE_500,
        )
        .outer_edges();

    gizmos.arrow(Vec3::ZERO, Vec3::X * 2.0, RED_500);
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 2.0, GREEN_500);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 2.0, BLUE_500);
}
