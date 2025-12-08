use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod fish;
mod util;

use fish::{Fish, FishBodyPart, FishPectoralFinPart, FishVentralFinPart, FishTailFinPart, FishFrontFinPart, update_fish_system, draw_fish_system};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Fish".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_fish_system, draw_fish_system).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let fish_origin = Vec2::ZERO;
    let fish = Fish::new(fish_origin);
    let fin_color = fish.fin_color;
    let body_color = fish.body_color;

    commands.spawn((
        fish,
        SpatialBundle::default(),
    ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle::default()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -10.0)),
                    ..default()
                },
                Fill::color(fin_color),
                Stroke::new(Color::WHITE, 4.0),
                FishVentralFinPart,
            ));

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle::default()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -5.0)),
                    ..default()
                },
                Fill::color(fin_color),
                Stroke::new(Color::WHITE, 4.0),
                FishPectoralFinPart,
            ));

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle::default()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -1.0)),
                    ..default()
                },
                Fill::color(fin_color),
                Stroke::new(Color::WHITE, 4.0),
                FishTailFinPart,
            ));

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle::default()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.1)),
                    ..default()
                },
                Fill::color(body_color),
                Stroke::new(Color::WHITE, 4.0),
                FishBodyPart,
            ));

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle::default()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 1.0)),
                    ..default()
                },
                Fill::color(fin_color),
                Stroke::new(Color::WHITE, 4.0),
                FishFrontFinPart,
            ));
        });
}