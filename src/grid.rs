use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const GRID_CELL_SIZE: f32 = 4.0;
pub const GRID_WIDTH: i32 = 250;
pub const GRID_HEIGHT: i32 = 125;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cells, spawn_grid));
        app.add_systems(PostStartup, pop_density_vec);
        app.add_systems(Update, update_cells);
    }
}

#[derive(Component, Debug)]
pub struct GridDensity {
    pub grid_density_vec: Vec<f32>,
}

impl GridDensity {
    pub fn new(grid_density_vec: Vec<f32>) -> Self {
        Self { grid_density_vec }
    }
}

#[derive(Component, Debug)]
pub struct SimCells;

#[derive(Component, Debug)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    pub grid_density: GridDensity,
}

fn update_cells(
    mut query_cellcolour: Query<&mut Fill, With<SimCells>>,
    query_density: Query<&GridDensity, With<Grid>>,
) {
    let mut count: usize = 0;
    if let Ok(density_vec) = query_density.get_single() {
        for mut cell in query_cellcolour.iter_mut() {
            cell.color = Color::hsl(density_vec.grid_density_vec[count], 100., 100.);
            count += 1;
        }
    }
}

fn spawn_cells(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(GRID_CELL_SIZE),
        ..shapes::RegularPolygon::default()
    };

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        (x - GRID_WIDTH / 2) as f32 * GRID_CELL_SIZE,
                        (y - GRID_HEIGHT / 2) as f32 * GRID_CELL_SIZE,
                        0.0,
                    )),
                    ..default()
                },
                Fill::color(Color::hsl(200.0, 100., 100.)), //red
                SimCells,
            ));
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn((
        GridBundle {
            grid_density: GridDensity::new(Vec::new()),
        },
        Grid,
    ));
}

fn pop_density_vec(mut query: Query<&mut GridDensity, With<Grid>>) {
    if let Ok(mut density_vec) = query.get_single_mut() {
        for _ in 0..GRID_HEIGHT {
            for _ in 0..GRID_WIDTH {
                density_vec.grid_density_vec.push(300.0);
            }
        }
    }
}
