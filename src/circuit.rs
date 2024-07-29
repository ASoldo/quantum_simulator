use crate::gates::Gate;
use crate::qubit::Qubit;

pub struct Circuit {
    gates: Vec<Gate>,
}

impl Circuit {
    pub fn new() -> Self {
        Circuit { gates: vec![] }
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn run(&self, qubit: &mut Qubit) {
        for gate in &self.gates {
            gate.apply(qubit);
        }
    }
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}
