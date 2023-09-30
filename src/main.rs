//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::mesh::*;
use bevy::sprite::*;

// x coordinates
const LEFT_EDGE: f32 = -450.;
const RIGHT_EDGE: f32 = 450.;
// y coordinates
const BOTTOM_EDGE: f32 = -300.;
const TOP_EDGE: f32 = 300.;

const CIRCLE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const CIRCLE_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const CIRCLE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32
}

fn make_circle_segment() -> Mesh {
    let sides = 16;
    let outer_radius = 4.*25.;
    let inner_radius = 4.*20.;

    let mut positions = Vec::with_capacity(sides);
    let mut normals = Vec::with_capacity(sides);
    let mut uvs = Vec::with_capacity(sides);

    let step = std::f32::consts::TAU / sides as f32;
    // outer circle
    for i in 0..sides-1 {
        let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
        let (sin, cos) = theta.sin_cos();

        positions.push([cos * outer_radius, sin * outer_radius, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
    }
    // inner circle
    for i in 0..sides-1 {
        let j = sides-2-i;
        let theta = std::f32::consts::FRAC_PI_2 - j as f32 * step;
        let (sin, cos) = theta.sin_cos();

        positions.push([cos * inner_radius, sin * inner_radius, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
    }

    let first_pos = positions[0];
    positions.push(first_pos);
    let first_normal = normals[0];
    normals.push(first_normal);
    let first_uv = uvs[0];
    uvs.push(first_uv);


    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh    
}

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player {x: 100., y: 0., x_velocity: 0., y_velocity: 0.},
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));

    //make_circle_segment()

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(make_circle_segment()).into(),
            material: materials.add(ColorMaterial::from(CIRCLE_COLOR)),
            transform: Transform::from_translation(CIRCLE_STARTING_POSITION),
            ..default()
        },
    ));

}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Player, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {

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

    for (mut player, mut transform) in &mut sprite_position {
        player.x_velocity += x_velocity;
        player.x += player.x_velocity * time.delta_seconds();
        if player.x > RIGHT_EDGE {
            player.x = RIGHT_EDGE;
            player.x_velocity *= -1.;
        }
        if player.x < LEFT_EDGE {
            player.x = LEFT_EDGE;
            player.x_velocity *= -1.;
        }

        player.y_velocity += y_velocity;
        player.y += player.y_velocity * time.delta_seconds();
        if player.y > TOP_EDGE {
            player.y = TOP_EDGE;
            player.y_velocity *= -1.;
        }
        if player.y < BOTTOM_EDGE {
            player.y = BOTTOM_EDGE;
            player.y_velocity *= -1.;
        }

        transform.translation.x = player.x;
        transform.translation.y = player.y;

    }
}
