use crate::grid::{AmountVel, Amountd, Density, Grid, GridPos, GRID_SIZE, V};
use crate::schedule::SimulationSet;
use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_density.in_set(SimulationSet::AddDensity));
        app.add_systems(Update, add_velocity.in_set(SimulationSet::AddVelocity));
    }
}

fn add_density(
    mut query_density: Query<&mut Density, With<Grid>>,
    query_gridpos: Query<&GridPos, With<Grid>>,
    query_amount: Query<&Amountd, With<Grid>>,
) {
    if let Ok(mut grid_density) = query_density.get_single_mut() {
        if let Ok(gridpos) = query_gridpos.get_single() {
            if let Ok(amount) = query_amount.get_single() {
                grid_density.density[index(gridpos.x, gridpos.y)] += amount.amountd;
            }
        }
    }
}

fn add_velocity(
    mut query_vel: Query<&mut V, With<Grid>>,
    query_gridpos: Query<&GridPos, With<Grid>>,
    query_amount: Query<&AmountVel, With<Grid>>,
) {
    if let Ok(mut grid_vel) = query_vel.get_single_mut() {
        if let Ok(gridpos) = query_gridpos.get_single() {
            if let Ok(amount) = query_amount.get_single() {
                let index: usize = index(gridpos.x, gridpos.y);
                grid_vel.vx[index] += amount.amountx;
                grid_vel.vy[index] += amount.amounty;
            }
        }
    }
}

// *** helper funcitons ***
// pass vectors in x.as_mut_slice() (&mut [T])
fn set_bnd(b: i32, x: &mut [f32], n: i32) {
    for j in 1..(n - 1) {
        for i in 1..(n - 1) {
            x[index(i, j)] = if b == 3 {
                -x[index(i, j)]
            } else {
                x[index(i, j)]
            };
        }
    }
    x[index(0, 0)] = 0.33 * (x[index(1, 0)] + x[index(0, 1)] + x[index(0, 0)]);
    x[index(0, n - 1)] = 0.33 * (x[index(1, n - 1)] + x[index(0, n - 2)] + x[index(0, n - 1)]);
    x[index(n - 1, 0)] = 0.33 * (x[index(n - 2, 0)] + x[index(n - 1, 1)] + x[index(n - 1, 0)]);
    x[index(n - 1, n - 1)] =
        0.33 * (x[index(n - 2, n - 1)] + x[index(n - 1, n - 2)] + x[index(n - 1, n - 1)]);
}

fn lin_solve(b: i32, x: &mut [f32], x0: &mut [f32], a: f32, c: f32, iter: i32, n: i32) {
    let c_recip = 1.0 / c;
    for _ in 0..iter {
        for j in 0..n {
            for i in 0..n {
                x[index(i, j)] = (x0[index(i, j)]
                    + a * (x[index(i + 1, j)]
                        + x[index(i - 1, j)]
                        + x[index(i, j + 1)]
                        + x[index(i, j - 1)]))
                    * c_recip;
            }
        }
    }
    set_bnd(b, x, n);
}

fn diffuse(b: i32, x: &mut [f32], x0: &mut [f32], diff: f32, dt: f32, iter: i32, n: i32) {
    let a: f32 = dt * diff * (n - 2) as f32 * (n - 2) as f32;
    lin_solve(b, x, x0, a, 1.0 + 6.0 * a, iter, n)
}

fn project(
    veloc_x: &mut [f32],
    veloc_y: &mut [f32],
    p: &mut [f32],
    div: &mut [f32],
    iter: i32,
    n: i32,
) {
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            div[index(i, j)] = -0.5
                * (veloc_x[index(i + 1, j)] - veloc_x[index(i - 1, j)] + veloc_y[index(i, j + 1)]
                    - veloc_y[index(i, j - 1)])
                / n as f32;
            p[index(i, j)] = 0.0;
        }
    }
    set_bnd(0, div, n);
    set_bnd(0, p, n);
    lin_solve(0, p, div, 1.0, 6.0, iter, n);
    // second for loop
}

fn index(x: i32, y: i32) -> usize {
    (x + (y * GRID_SIZE)) as usize
}
