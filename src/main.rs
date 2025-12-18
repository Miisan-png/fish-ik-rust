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
                resolution: (1200, 800).into(),
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
    commands.spawn((Camera2d, Msaa::Sample4));

    let fish_origin = Vec2::ZERO;
    let fish = Fish::new(fish_origin);
    let fin_color = fish.fin_color;
    let body_color = fish.body_color;

    commands.spawn((
        fish,
        Transform::default(),
        Visibility::default(),
    ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBuilder::with(&shapes::Circle::default())
                    .fill(fin_color)
                    .stroke((Color::WHITE, 4.0))
                    .build(),
                Transform::from_xyz(0.0, 0.0, -10.0),
                FishVentralFinPart,
            ));

            parent.spawn((
                ShapeBuilder::with(&shapes::Circle::default())
                    .fill(fin_color)
                    .stroke((Color::WHITE, 4.0))
                    .build(),
                Transform::from_xyz(0.0, 0.0, -5.0),
                FishPectoralFinPart,
            ));

            parent.spawn((
                ShapeBuilder::with(&shapes::Circle::default())
                    .fill(fin_color)
                    .stroke((Color::WHITE, 4.0))
                    .build(),
                Transform::from_xyz(0.0, 0.0, -1.0),
                FishTailFinPart,
            ));

            parent.spawn((
                ShapeBuilder::with(&shapes::Circle::default())
                    .fill(body_color)
                    .stroke((Color::WHITE, 4.0))
                    .build(),
                Transform::from_xyz(0.0, 0.0, 0.1),
                FishBodyPart,
            ));

            parent.spawn((
                ShapeBuilder::with(&shapes::Circle::default())
                    .fill(fin_color)
                    .stroke((Color::WHITE, 4.0))
                    .build(),
                Transform::from_xyz(0.0, 0.0, 1.0),
                FishFrontFinPart,
            ));
        });
}
