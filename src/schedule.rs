use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    UpdateCells,
}

pub struct SchudulePlugin;

impl Plugin for SchudulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                //SimulationSet::OtherSystem,
                //flush
                SimulationSet::UpdateCells,
            )
                .chain(),
        );
        //.add_systems(
        //Update,
        //apply_deferred
        //.after(SimulationSet::OtherSystem)
        //.before(SimulationSet::UpdateCells),
        //);
    }
}
