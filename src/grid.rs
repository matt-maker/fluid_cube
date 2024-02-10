use bevy::prelude::*;

pub const GRID_CELL_SIZE: Vec2 = Vec2::new(8.0, 8.0);
pub const GRID_WIDTH: i32 = 150;
pub const GRID_HEIGHT: i32 = 100;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridSmoke {
    pub grid_smoke_vec: Vec<f32>,
}

impl GridSmoke {
    pub fn new(grid_smoke_vec: Vec<f32>) -> Self {
        Self { grid_smoke_vec }
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub grid_smoke: GridSmoke,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn((
        GridBundle {
            grid_smoke: GridSmoke::new(Vec::new()),
        },
        //Grid,
    ));
}
