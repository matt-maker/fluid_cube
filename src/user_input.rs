use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct UserInput;

impl Plugin for UserInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cursor_position);
    }
}

fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
    if let Some(position) = q_windows.single().cursor_position() {
        println!("{:?}", position);
    } else {
        println!("Outside");
    }
}
