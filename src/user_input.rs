use crate::grid::{AddDensityVelocity, Grid, GridPosX, GridPosY, GRID_CELL_SIZE, GRID_SIZE};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct UserInput;

impl Plugin for UserInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cursor_position);
    }
}

fn cursor_position(
    window: Query<&Window>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut query_add_bool: Query<&mut AddDensityVelocity, With<Grid>>,
    mut query_grid_pos: Query<(&mut GridPosX, &mut GridPosY), With<Grid>>,
) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let window_centre: (f32, f32) = (width / 2.0, height / 2.0);
    let simulation_boundaries: (f32, f32, f32, f32) = (
        window_centre.0 - ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE),
        window_centre.0 + ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE),
        window_centre.1 - ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE),
        window_centre.1 + ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE),
    );
    if let Ok((mut grid_x, mut grid_y)) = query_grid_pos.get_single_mut() {
        if let Ok(mut add_density_velocity) = query_add_bool.get_single_mut() {
            if let Some(position) = q_windows.single().cursor_position() {
                let mut in_boundaries: bool = false;
                add_density_velocity.add_density_velocity = false;

                if position[0] > simulation_boundaries.0
                    && position[0] < simulation_boundaries.1
                    && position[1] > simulation_boundaries.2
                    && position[1] < simulation_boundaries.3
                {
                    in_boundaries = true;
                }
                if buttons.pressed(MouseButton::Left) && in_boundaries == true {
                    grid_x.x = ((position[0]
                        - (window_centre.0 - ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE)))
                        / GRID_CELL_SIZE)
                        .floor() as i32;

                    grid_y.y = ((position[1]
                        - (window_centre.1 - ((GRID_SIZE / 2) as f32 * GRID_CELL_SIZE)))
                        / GRID_CELL_SIZE)
                        .floor() as i32;
                    add_density_velocity.add_density_velocity = true;
                }
            }
        }
    }
}
