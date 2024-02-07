use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub const GRID_CELL_SIZE: Vec2 = Vec2::new(8.0, 8.0);
pub const GRID_WIDTH: i32 = 150;
pub const GRID_HEIGHT: i32 = 100;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct SimCell;

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
        app.add_systems(Startup, (spawn_cells, spawn_grid));
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(GRID_CELL_SIZE)))
                        .into(),
                    transform: Transform::from_xyz(
                        (x - GRID_WIDTH / 2) as f32 * GRID_CELL_SIZE[0],
                        (y - GRID_HEIGHT / 2) as f32 * GRID_CELL_SIZE[1],
                        0.0,
                    ),
                    material: materials
                        .add(ColorMaterial::from(Color::rgba(255.0, 255.0, 255.0, 1.0))),
                    ..default()
                },
                SimCell,
            ));
        }
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
