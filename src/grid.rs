use crate::schedule::SimulationSet;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const GRID_CELL_SIZE: f32 = 4.0;
pub const GRID_SIZE: i32 = 120;

pub struct GridPlugin;

//app.add_systems(Update, integrate.in_set(SimulationSet::Integrate));
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cells, spawn_grid));
        app.add_systems(Update, update_cells.in_set(SimulationSet::UpdateCells));
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
pub struct Amountd {
    pub amountd: f32,
}

impl Amountd {
    pub fn new(amountd: f32) -> Self {
        Self { amountd }
    }
}

#[derive(Component, Debug)]
pub struct AmountVelX {
    pub amountx: f32,
}

impl AmountVelX {
    pub fn new(amountx: f32) -> Self {
        Self { amountx }
    }
}

#[derive(Component, Debug)]
pub struct AmountVelY {
    pub amounty: f32,
}

impl AmountVelY {
    pub fn new(amounty: f32) -> Self {
        Self { amounty }
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
pub struct B {
    pub b: i32,
}

impl B {
    pub fn new(b: i32) -> Self {
        Self { b }
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
pub struct VX {
    pub vx: Vec<f32>,
}

impl VX {
    pub fn new(vx: Vec<f32>) -> Self {
        Self { vx }
    }
}

#[derive(Component, Debug)]
pub struct VY {
    pub vy: Vec<f32>,
}

impl VY {
    pub fn new(vy: Vec<f32>) -> Self {
        Self { vy }
    }
}

#[derive(Component, Debug)]
pub struct V0X {
    pub vx0: Vec<f32>,
}

impl V0X {
    pub fn new(vx0: Vec<f32>) -> Self {
        Self { vx0 }
    }
}

#[derive(Component, Debug)]
pub struct V0Y {
    pub vy0: Vec<f32>,
}

impl V0Y {
    pub fn new(vy0: Vec<f32>) -> Self {
        Self { vy0 }
    }
}

#[derive(Component, Debug)]
pub struct GridPosX {
    pub x: i32,
}

impl GridPosX {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

#[derive(Component, Debug)]
pub struct GridPosY {
    pub y: i32,
}

impl GridPosY {
    pub fn new(y: i32) -> Self {
        Self { y }
    }
}

#[derive(Component, Debug)]
pub struct AddDensityVelocity {
    pub add_density_velocity: bool,
}

impl AddDensityVelocity {
    pub fn new(add_density_velocity: bool) -> Self {
        Self {
            add_density_velocity,
        }
    }
}

#[derive(Component, Debug)]
pub struct SimCells;

#[derive(Component, Debug)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    pub add_density_velocity: AddDensityVelocity,
    pub dt: Dt,
    pub diff: Diff,
    pub visc: Visc,

    pub s: S,
    pub b: B,
    pub density: Density,

    pub vx: VX,
    pub vy: VY,

    pub v0x: V0X,
    pub v0y: V0Y,

    pub grid_density: GridDensity,

    pub grid_pos_x: GridPosX,
    pub grid_pos_y: GridPosY,
    pub amountd: Amountd,
    pub amountvel_x: AmountVelX,
    pub amountvel_y: AmountVelY,
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
            add_density_velocity: AddDensityVelocity::new(false),
            b: B::new(0),
            dt: Dt::new(0.0),
            diff: Diff::new(0.0),
            visc: Visc::new(0.0),
            amountd: Amountd::new(2.0),

            amountvel_x: AmountVelX::new(3.0),
            amountvel_y: AmountVelY::new(3.0),
            grid_pos_x: GridPosX::new(0),
            grid_pos_y: GridPosY::new(0),

            s: S::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),
            density: Density::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),

            vx: VX::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),
            vy: VY::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),

            v0x: V0X::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),
            v0y: V0Y::new(vec![0.0; (GRID_SIZE * GRID_SIZE) as usize]),

            grid_density: GridDensity::new(vec![300.0; (GRID_SIZE * GRID_SIZE) as usize]),
        },
        Grid,
    ));
}
