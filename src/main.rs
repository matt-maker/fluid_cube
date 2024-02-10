use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod camera_2d;
mod grid;

use camera_2d::CameraPlugin;
use grid::GridPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup_system)
        .add_plugins(GridPlugin)
        .run();
}

fn setup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(4.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        material: materials.add(ColorMaterial::from(Color::rgb(100.0, 0.0, 0.0))),
        spatial: SpatialBundle::from_transform(Transform::from_xyz(100.0, 0.0, 0.0)),
        ..default()
    },));

    commands.spawn((ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        material: materials.add(ColorMaterial::from(Color::rgb(100.0, 0.0, 0.0))),
        spatial: SpatialBundle::from_transform(Transform::from_xyz(150.0, 0.0, 0.0)),
        ..default()
    },));
}
