//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

// x coordinates
const LEFT_EDGE: f32 = -450.;
const RIGHT_EDGE: f32 = 450.;
// y coordinates
const BOTTOM_EDGE: f32 = -300.;
const TOP_EDGE: f32 = 300.;

const PLAYER_VELOCITY_X: f32 = 100.;
const PLAYER_VELOCITY_Y: f32 = 100.;

const CIRCLE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const CIRCLE_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const CIRCLE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin        .add_systems(Startup, setup)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32
}

#[derive(Component)]
struct Ring {
    positions: Vec<[f32; 3]>,
    positions2d: Vec<Vec2<>>,
    indices: Vec<[u32; 2]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl Ring {
    fn new() -> Ring {
        let sides = 16;
        let outer_radius = 4.*25.;
        let inner_radius = 4.*20.;
    
        let mut positions = Vec::with_capacity(sides);
        let mut positions2d = Vec::with_capacity(sides);
        let mut indices = Vec::with_capacity(sides);
        let mut normals = Vec::with_capacity(sides);
        let mut uvs = Vec::with_capacity(sides);
    
        let step = std::f32::consts::TAU / sides as f32;
        // outer circle
        for i in 0..sides-1 {
            let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
            let (sin, cos) = theta.sin_cos();
    
            positions.push([cos * outer_radius, sin * outer_radius, 0.0]);
            positions2d.push(Vec2 {x: cos * outer_radius, y: sin * outer_radius});
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }
        // inner circle
        for i in 0..sides-1 {
            let j = sides-2-i;
            let theta = std::f32::consts::FRAC_PI_2 - j as f32 * step;
            let (sin, cos) = theta.sin_cos();
    
            positions.push([cos * inner_radius, sin * inner_radius, 0.0]);
            positions2d.push(Vec2 {x: cos * inner_radius, y: sin * inner_radius});
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }
    
        let first_pos = positions[0];
        positions.push(first_pos);
        let first_pos2d = positions2d[0];
        positions2d.push(first_pos2d);
        let first_normal = normals[0];
        normals.push(first_normal);
        let first_uv = uvs[0];
        uvs.push(first_uv);

        for i in 0..positions2d.len()-1 {
            indices.push([i as u32, i as u32 + 1]);
        }
    
    
        Ring {
            positions: positions,
            positions2d: positions2d,
            indices: indices,
            normals: normals,
            uvs: uvs,
        }
    }

    fn make_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh

    }

}




fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player {x: 0., y: 0., x_velocity: 0., y_velocity: 0.},
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(10.5, 30.5))
    .insert(KinematicCharacterController::default())
    ;

    let ring = Ring::new();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ring.make_mesh()).into(),
            material: materials.add(ColorMaterial::from(CIRCLE_COLOR)),
            transform: Transform::from_translation(CIRCLE_STARTING_POSITION),
            ..default()
        },
    ))
    .insert(RigidBody::KinematicVelocityBased)
    .insert(Collider::polyline(ring.positions2d.clone(), Some(ring.indices.clone())))
    //.insert(Collider::convex_decomposition(&ring.positions2d, &ring.indices))
    .insert(ring)
    .insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: 0.2
    })
    .insert(GravityScale(0.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.,0.,0.)))
    ;

}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut translation = Vec2::new(0.0, 0.0);

    if input.pressed(KeyCode::Right) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::Left) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    if input.pressed(KeyCode::Up) {
        translation.y += time.delta_seconds() * PLAYER_VELOCITY_Y;
    }

    if input.pressed(KeyCode::Down) {
        translation.y += time.delta_seconds() * PLAYER_VELOCITY_Y * -1.0;
    }

    player.translation = Some(translation);
}

