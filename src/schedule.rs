use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    DiffuseVelocities,
    ProjectV0,
    AdvectVelocities,
    ProjectV,
    DiffuseDensity,
    AdvectDensity,
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
                SimulationSet::AdvectVelocities,
                SimulationSet::ProjectV,
                SimulationSet::DiffuseDensity,
                SimulationSet::AdvectDensity,
                //flush
                SimulationSet::UpdateCells,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(SimulationSet::AdvectDensity)
                .before(SimulationSet::UpdateCells),
        );
    }
}
