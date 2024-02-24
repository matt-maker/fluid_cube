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

// pass vectors in x.as_mut_slice() (&mut [T])
fn set_bnd(b: i32, x: &mut [f32], n: i32) {
    for _ in 1..(n - 1) {
        for i in 1..(n - 1) {
            x[index(i, 0)] = if b == 2 {
                -x[index(i, 1)]
            } else {
                x[index(i, 1)]
            };

            x[index(i, n - 1)] = if b == 2 {
                -x[index(i, n - 2)]
            } else {
                x[index(i, n - 2)]
            }
        }
    }

    for _ in 1..(n - 1) {
        for j in 1..(n - 1) {
            x[index(0, j)] = if b == 1 {
                -x[index(1, j)]
            } else {
                x[index(1, j)]
            };

            x[index(n - 1, j)] = if b == 1 {
                -x[index(n - 2, j)]
            } else {
                x[index(n - 2, j)]
            }
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

fn index(x: i32, y: i32) -> usize {
    (x + (y * GRID_SIZE)) as usize
}

fn diffuse_all() {
    //
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

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            veloc_x[index(i, j)] -= 0.5 * (p[index(i + 1, j)] - p[index(i - 1, j)]) * n as f32;
            veloc_y[index(i, j)] -= 0.5 * (p[index(i, j + 1)] - p[index(i, j - 1)]) * n as f32;
        }
    }
    set_bnd(1, veloc_x, n);
    set_bnd(2, veloc_y, n);
}

fn advect(
    b: i32,
    d: &mut [f32],
    d0: &mut [f32],
    veloc_x: &mut [f32],
    veloc_y: &mut [f32],
    dt: f32,
    n: i32,
) {
    let (dtx, dty): (f32, f32) = ((dt * (n as f32 - 2.0)), (dt * (n as f32 - 2.0)));
    let nfloat = n;
    let mut jfloat = 1.0;

    for j in 1..n - 1 {
        jfloat += 1.0;
        let mut ifloat = 1.0;
        for i in 1..n - 1 {
            ifloat += 1.0;
            let tmp1 = dtx * veloc_x[index(i, j)];
            let tmp2 = dty * veloc_y[index(i, j)];
            let mut x = ifloat - tmp1;
            let mut y = jfloat - tmp2;

            if x < 0.5 {
                x = 0.5
            };
            if x > nfloat as f32 + 0.5 {
                x = nfloat as f32 + 0.5
            };
            let i0 = x.floor();
            let i1 = i0 + 1.0;

            if y < 0.5 {
                y = 0.5
            };
            if y > nfloat as f32 + 0.5 {
                y = nfloat as f32 + 0.5
            };
            let j0 = y.floor();
            let j1 = j0 + 1.0;

            let s1 = x - i0;
            let s0 = 1.0 - s1;
            let t1 = y - j0;
            let t0 = 1.0 - t1;

            let (i0i, i1i, j0i, j1i): (i32, i32, i32, i32) =
                (i0 as i32, i1 as i32, j0 as i32, j1 as i32);

            d[index(i, j)] = s0 * ((t0 * d0[index(i0i, j0i)]) + (t1 * d0[index(i0i, j1i)]))
                + s1 * ((t0 * d0[index(i1i, j0i)]) + (t1 * d0[index(i1i, j1i)]));
        }
    }
    set_bnd(b, d, n);
}
