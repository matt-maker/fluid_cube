use crate::grid::GRID_SIZE;
use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, test_index);
    }
}

fn index(x: usize, y: usize) -> usize {
    x + (y * GRID_SIZE as usize)
}

fn test_index() {
    let position = index(10, 1);
    println!("{}", position);
}

//
