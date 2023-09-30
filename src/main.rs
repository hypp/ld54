//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

// x coordinates
const LEFT_EDGE: f32 = -450.;
const RIGHT_EDGE: f32 = 450.;
// y coordinates
const BOTTOM_EDGE: f32 = -300.;
const TOP_EDGE: f32 = 300.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Ball {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Ball {x: 100., y: 0., x_velocity: 0., y_velocity: 0.},
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Ball, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {

    let mut x_velocity: f32 = 0.;
    let mut y_velocity: f32 = 0.;

    if keyboard_input.pressed(KeyCode::Left) {
        x_velocity -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        x_velocity += 1.0;
    }
    x_velocity = x_velocity.clamp(-5.,5.);

    if keyboard_input.pressed(KeyCode::Down) {
        y_velocity -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        y_velocity += 1.0;
    }
    y_velocity = y_velocity.clamp(-5.,5.);

    for (mut ball, mut transform) in &mut sprite_position {
        ball.x_velocity += x_velocity;
        ball.x += ball.x_velocity * time.delta_seconds();
        if ball.x > RIGHT_EDGE {
            ball.x = RIGHT_EDGE;
            ball.x_velocity *= -1.;
        }
        if ball.x < LEFT_EDGE {
            ball.x = LEFT_EDGE;
            ball.x_velocity *= -1.;
        }

        ball.y_velocity += y_velocity;
        ball.y += ball.y_velocity * time.delta_seconds();
        if ball.y > TOP_EDGE {
            ball.y = TOP_EDGE;
            ball.y_velocity *= -1.;
        }
        if ball.y < BOTTOM_EDGE {
            ball.y = BOTTOM_EDGE;
            ball.y_velocity *= -1.;
        }

        transform.translation.x = ball.x;
        transform.translation.y = ball.y;

    }
}
