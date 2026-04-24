use bevy::prelude::*;
use crate::gate::{Gate, GateType};

//this is the actual graph, with each gate being stated in the gates vector and the graph being the actual was to find run through the values
#[derive(Component)]
pub struct Circuit {
    pub gates: Vec<Gate>,
    pub graph: Vec<Vec<Option<usize>>>,
    pub num_inputs: u32,
}


impl Circuit {

    //create a new circuit, probably used for each level
    pub fn new(num_inputs: u32, size: usize) -> Self {
        Self {
            gates: Vec::with_capacity(size),
            graph: vec![vec![None; size]; size],
            num_inputs,
        }
    }
    //to be used when a new gate is added to the "field", uses the new function from 
    //this also inputs a blank row and column to the empty matrix
    pub fn add_gate(&mut self, gate_type: GateType) {
        let mut value: i32 = 0;  
        match gate_type {
            GateType::NOT => value = 1,
            _ => value = 2,
        };

        let insert = Gate::new(gate_type, value as usize);
        self.gates.push(insert);

        for row in self.graph.iter_mut() {
            row.push(None);
        }
        self.graph.push(vec![None; self.gates.len()]);
    }

    //this should be used when a gate is taken off the "field", and this is only possible when the gate is not attached to anything
    pub fn remove_gate(&mut self, get_id: i32) {
        let mut index = 0;
        let mut found = false;
        for (i, value) in self.gates.iter().enumerate() {
            if value.id == get_id {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            self.gates.remove(index);
        }
    }

    pub fn connect_gates(&mut self, from: Gate, to: Gate) {
    
    }
}