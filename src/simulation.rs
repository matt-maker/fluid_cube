use crate::grid::{
    AmountVelX, AmountVelY, Amountd, Density, Diff, Dt, Grid, GridPosX, GridPosY, Visc, GRID_SIZE,
    S, V0X, V0Y, VX, VY,
};
use crate::schedule::SimulationSet;
use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            diffuse_velocities.in_set(SimulationSet::DiffuseVelocities),
        );
        app.add_systems(Update, project_v0.in_set(SimulationSet::ProjectV0));
        app.add_systems(
            Update,
            advect_velocities.in_set(SimulationSet::AdvectVelocities),
        );
        app.add_systems(Update, project_v.in_set(SimulationSet::ProjectV));
        app.add_systems(
            Update,
            diffuse_density.in_set(SimulationSet::DiffuseDensity),
        );
        app.add_systems(Update, advect_density.in_set(SimulationSet::AdvectDensity));
    }
}

fn add_density(
    mut query_density: Query<&mut Density, With<Grid>>,
    query_gridpos: Query<(&GridPosX, &GridPosY), With<Grid>>,
    query_amount: Query<&Amountd, With<Grid>>,
) {
    if let Ok(mut grid_density) = query_density.get_single_mut() {
        if let Ok((gridposx, gridposy)) = query_gridpos.get_single() {
            if let Ok(amount) = query_amount.get_single() {
                grid_density.density[index(gridposx.x, gridposy.y)] += amount.amountd;
            }
        }
    }
}

fn add_velocity(
    mut query_vel: Query<(&mut VX, &mut VY), With<Grid>>,
    query_gridpos: Query<(&GridPosX, &GridPosY), With<Grid>>,
    query_amount: Query<(&AmountVelX, &AmountVelY), With<Grid>>,
) {
    if let Ok((mut grid_vel_x, mut grid_vel_y)) = query_vel.get_single_mut() {
        if let Ok((gridposx, gridposy)) = query_gridpos.get_single() {
            if let Ok((amountx, amounty)) = query_amount.get_single() {
                let index: usize = index(gridposx.x, gridposy.y);
                grid_vel_x.vx[index] += amountx.amountx;
                grid_vel_y.vy[index] += amounty.amounty;
            }
        }
    }
}

// pass vectors in x.as_mut_slice() (&mut [T])
fn set_bnd(b: i32, x: &mut [f32], n: i32) {
    for _ in 1..n - 1 {
        for i in 1..n - 1 {
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

    for _ in 1..n - 1 {
        for j in 1..n - 1 {
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
        for j in 1..n - 1 {
            for i in 1..n - 1 {
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

fn advect_density(
    query_vel: Query<(&VX, &VY), With<Grid>>,
    mut query_s: Query<&mut S, With<Grid>>,
    query_density: Query<&Density, With<Grid>>,
    query_scene: Query<&Dt, With<Grid>>,
) {
    if let Ok((vx, vy)) = query_vel.get_single() {
        if let Ok(density) = query_density.get_single() {
            if let Ok(mut s) = query_s.get_single_mut() {
                if let Ok(dt) = query_scene.get_single() {
                    advect(
                        0,
                        s.s.as_mut_slice(),
                        density.density.as_slice(),
                        vx.vx.as_slice(),
                        vy.vy.as_slice(),
                        dt.dt,
                        GRID_SIZE,
                    );
                }
            }
        }
    }
}

fn advect_velocities(
    mut query_vel: Query<(&mut VX, &mut VY), With<Grid>>,
    query_vel_0: Query<(&V0X, &V0Y), With<Grid>>,
    query_scene: Query<&Dt, With<Grid>>,
) {
    if let Ok((mut vx, mut vy)) = query_vel.get_single_mut() {
        if let Ok((v0x, v0y)) = query_vel_0.get_single() {
            if let Ok(dt) = query_scene.get_single() {
                advect(
                    1,
                    vx.vx.as_mut_slice(),
                    v0x.vx0.as_slice(),
                    v0x.vx0.as_slice(),
                    v0y.vy0.as_slice(),
                    dt.dt,
                    GRID_SIZE,
                );

                advect(
                    2,
                    vy.vy.as_mut_slice(),
                    v0y.vy0.as_slice(),
                    v0x.vx0.as_slice(),
                    v0y.vy0.as_slice(),
                    dt.dt,
                    GRID_SIZE,
                );
            }
        }
    }
}

fn diffuse_density(
    mut query_vel: Query<(&mut S, &mut Density), With<Grid>>,
    query_scene: Query<(&Dt, &Diff), With<Grid>>,
) {
    if let Ok((mut s, mut density)) = query_vel.get_single_mut() {
        if let Ok((dt, diff)) = query_scene.get_single() {
            diffuse(
                0,
                s.s.as_mut_slice(),
                density.density.as_mut_slice(),
                diff.diff,
                dt.dt,
                4,
                GRID_SIZE,
            );
        }
    }
}

fn diffuse_velocities(
    mut query_vel: Query<(&mut VX, &mut VY, &mut V0X, &mut V0Y), With<Grid>>,
    query_scene: Query<(&Visc, &Dt), With<Grid>>,
) {
    if let Ok((mut vx, mut vy, mut v0x, mut v0y)) = query_vel.get_single_mut() {
        if let Ok((visc, dt)) = query_scene.get_single() {
            diffuse(
                1,
                v0x.vx0.as_mut_slice(),
                vx.vx.as_mut_slice(),
                visc.visc,
                dt.dt,
                4,
                GRID_SIZE,
            );

            diffuse(
                2,
                v0y.vy0.as_mut_slice(),
                vy.vy.as_mut_slice(),
                visc.visc,
                dt.dt,
                4,
                GRID_SIZE,
            );
        }
    }
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

fn project_v0(mut query_vel: Query<(&mut V0X, &mut V0Y, &mut VX, &mut VY), With<Grid>>) {
    if let Ok((mut v0x, mut v0y, mut vx, mut vy)) = query_vel.get_single_mut() {
        project(
            v0x.vx0.as_mut_slice(),
            v0y.vy0.as_mut_slice(),
            vx.vx.as_mut_slice(),
            vy.vy.as_mut_slice(),
            4,
            GRID_SIZE,
        );
    }
}

fn project_v(mut query_vel: Query<(&mut V0X, &mut V0Y, &mut VX, &mut VY), With<Grid>>) {
    if let Ok((mut v0x, mut v0y, mut vx, mut vy)) = query_vel.get_single_mut() {
        project(
            vx.vx.as_mut_slice(),
            vy.vy.as_mut_slice(),
            v0x.vx0.as_mut_slice(),
            v0y.vy0.as_mut_slice(),
            4,
            GRID_SIZE,
        );
    }
}

fn advect(b: i32, d: &mut [f32], d0: &[f32], veloc_x: &[f32], veloc_y: &[f32], dt: f32, n: i32) {
    let (dtx, dty): (f32, f32) = ((dt * (n as f32 - 2.0)), (dt * (n as f32 - 2.0)));
    let nfloat = n;
    let mut jfloat = 1.0;

    for j in 1..n - 2 {
        jfloat += 1.0;
        let mut ifloat = 1.0;
        for i in 1..n - 2 {
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
