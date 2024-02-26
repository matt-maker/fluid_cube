use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    DiffuseVelocities,
    ProjectV0,
    UpdateCells,
}

pub struct SchudulePlugin;

impl Plugin for SchudulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                SimulationSet::DiffuseVelocities,
                SimulationSet::ProjectV0,
                //flush
                SimulationSet::UpdateCells,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(SimulationSet::ProjectV0)
                .before(SimulationSet::UpdateCells),
        );
    }
}
