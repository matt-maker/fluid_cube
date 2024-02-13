use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod camera_2d;
mod grid;
mod schedule;
mod simulation;

use camera_2d::CameraPlugin;
use grid::GridPlugin;
use schedule::SchudulePlugin;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(SchudulePlugin)
        .run();
}
