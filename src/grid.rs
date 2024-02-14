use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const GRID_CELL_SIZE: f32 = 4.0;
pub const GRID_SIZE: i32 = 120;

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
pub struct Size {
    pub size: i32,
}

impl Size {
    pub fn new(size: i32) -> Self {
        Self { size }
    }
}

#[derive(Component, Debug)]
pub struct Dt {
    pub dt: f32,
}

impl Dt {
    pub fn new(dt: f32) -> Self {
        Self { dt }
    }
}

#[derive(Component, Debug)]
pub struct Diff {
    pub diff: f32,
}

impl Diff {
    pub fn new(diff: f32) -> Self {
        Self { diff }
    }
}

#[derive(Component, Debug)]
pub struct Visc {
    pub visc: f32,
}

impl Visc {
    pub fn new(visc: f32) -> Self {
        Self { visc }
    }
}

#[derive(Component, Debug)]
pub struct S {
    pub s: Vec<f32>,
}

impl S {
    pub fn new(s: Vec<f32>) -> Self {
        Self { s }
    }
}

#[derive(Component, Debug)]
pub struct Density {
    pub density: Vec<f32>,
}

impl Density {
    pub fn new(density: Vec<f32>) -> Self {
        Self { density }
    }
}

#[derive(Component, Debug)]
pub struct Vx {
    pub vx: Vec<f32>,
}

impl Vx {
    pub fn new(vx: Vec<f32>) -> Self {
        Self { vx }
    }
}

#[derive(Component, Debug)]
pub struct Vy {
    pub vy: Vec<f32>,
}

impl Vy {
    pub fn new(vy: Vec<f32>) -> Self {
        Self { vy }
    }
}

#[derive(Component, Debug)]
pub struct Vz {
    pub vz: Vec<f32>,
}

impl Vz {
    pub fn new(vz: Vec<f32>) -> Self {
        Self { vz }
    }
}

#[derive(Component, Debug)]
pub struct Vx0 {
    pub vx0: Vec<f32>,
}

impl Vx0 {
    pub fn new(vx0: Vec<f32>) -> Self {
        Self { vx0 }
    }
}

#[derive(Component, Debug)]
pub struct Vy0 {
    pub vy0: Vec<f32>,
}

impl Vy0 {
    pub fn new(vy0: Vec<f32>) -> Self {
        Self { vy0 }
    }
}

#[derive(Component, Debug)]
pub struct Vz0 {
    pub vz0: Vec<f32>,
}

impl Vz0 {
    pub fn new(vz0: Vec<f32>) -> Self {
        Self { vz0 }
    }
}

#[derive(Component, Debug)]
pub struct SimCells;

#[derive(Component, Debug)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    pub size: Size,
    pub dt: Dt,
    pub diff: Diff,
    pub visc: Visc,

    pub s: S,
    pub density: Density,

    pub vx: Vx,
    pub vy: Vy,
    pub vz: Vz,

    pub vx0: Vx0,
    pub vy0: Vy0,
    pub vz0: Vz0,

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

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        (x - GRID_SIZE / 2) as f32 * GRID_CELL_SIZE,
                        (y - GRID_SIZE / 2) as f32 * GRID_CELL_SIZE,
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
            size: Size::new(0),
            dt: Dt::new(0.0),
            diff: Diff::new(0.0),
            visc: Visc::new(0.0),

            s: S::new(Vec::new()),
            density: Density::new(Vec::new()),

            vx: Vx::new(Vec::new()),
            vy: Vy::new(Vec::new()),
            vz: Vz::new(Vec::new()),

            vx0: Vx0::new(Vec::new()),
            vy0: Vy0::new(Vec::new()),
            vz0: Vz0::new(Vec::new()),

            grid_density: GridDensity::new(Vec::new()),
        },
        Grid,
    ));
}

fn pop_density_vec(mut query: Query<&mut GridDensity, With<Grid>>) {
    if let Ok(mut density_vec) = query.get_single_mut() {
        for _ in 0..GRID_SIZE {
            for _ in 0..GRID_SIZE {
                density_vec.grid_density_vec.push(300.0);
            }
        }
    }
}
