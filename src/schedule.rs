use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    AddDensity,
    AddVelocity,
    UpdateCells,
}

pub struct SchudulePlugin;

impl Plugin for SchudulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                SimulationSet::AddDensity,
                SimulationSet::AddVelocity,
                //flush
                SimulationSet::UpdateCells,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(SimulationSet::AddVelocity)
                .before(SimulationSet::UpdateCells),
        );
    }
}
