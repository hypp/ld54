
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

const RING_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const RING_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "LD54 - Limited Space. A failed attempt by Mathias Olsson".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
       // .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin        .add_systems(Startup, setup)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, change_scale_direction, scale_ring))
        .run();
}


#[derive(Component)]
struct Scaling {
    scale_direction: Vec3,
    scale_speed: f32,
    max_element_size: f32,
    min_element_size: f32,
}

// Implement a simple initialization.
impl Scaling {
    fn new() -> Self {
        Scaling {
            scale_direction: Vec3 { x: 1., y: 1., z: 0.} ,
            scale_speed: 0.1,
            max_element_size: 3.0,
            min_element_size: 0.5,
        }
    }
}

#[derive(Component)]
struct Player {
//    x: f32,
//    y: f32,
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
//        Player {x: 0., y: 0., x_velocity: 0., y_velocity: 0.},
        Player {x_velocity: 0., y_velocity: 0.},
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(10.5, 30.5))
//    .insert(KinematicCharacterController::default())
    .insert(GravityScale(0.0))
    .insert(Ccd::enabled())
    .insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: 0.0
    })    
    ;

    let ring = Ring::new();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ring.make_mesh()).into(),
            material: materials.add(ColorMaterial::from(RING_COLOR)),
            transform: Transform::from_translation(RING_STARTING_POSITION).with_scale(Vec3 {x: 3., y: 3., z: 1.}),
            ..default()
        },
        Scaling::new()
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::polyline(ring.positions2d.clone(), Some(ring.indices.clone())))
    .insert(ring)
    .insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: 0.8
    })
    .insert(GravityScale(0.0))
    .insert(Ccd::enabled())
    ;

    // Bottom floor
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(LEFT_EDGE.abs()+RIGHT_EDGE.abs(), 1.)),
            ..default()
        },
        ..default()
    })        
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5*(LEFT_EDGE.abs()+RIGHT_EDGE.abs()), 0.5))
    .insert(TransformBundle::from_transform(Transform::from_xyz(0., BOTTOM_EDGE, 0.)))
    ;

    // Top floor
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(LEFT_EDGE.abs()+RIGHT_EDGE.abs(), 1.)),
            ..default()
        },
        ..default()
    })        
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5*(LEFT_EDGE.abs()+RIGHT_EDGE.abs()), 0.5))
    .insert(TransformBundle::from_transform(Transform::from_xyz(0., TOP_EDGE, 0.)))
    ;

    // Left wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(1., TOP_EDGE.abs()+BOTTOM_EDGE.abs())),
            ..default()
        },
        ..default()
    })        
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5*(TOP_EDGE.abs()+BOTTOM_EDGE.abs())))
    .insert(TransformBundle::from_transform(Transform::from_xyz(LEFT_EDGE, 0., 0.)))
    ;

    // Right wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(1., TOP_EDGE.abs()+BOTTOM_EDGE.abs())),
            ..default()
        },
        ..default()
    })        
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5*(TOP_EDGE.abs()+BOTTOM_EDGE.abs())))
    .insert(TransformBundle::from_transform(Transform::from_xyz(RIGHT_EDGE, 0., 0.)))
    ;


}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    let (mut vel, mut player) = query.single_mut();

    let mut velocity = Vec2 {x: 0., y: 0.};

    let mut changed = false;

    if input.pressed(KeyCode::Right) {
        velocity.x = time.delta_seconds() * PLAYER_VELOCITY_X;
        player.x_velocity += time.delta_seconds() * PLAYER_VELOCITY_X;
        changed = true;
    }

    if input.pressed(KeyCode::Left) {
        velocity.x = time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        player.x_velocity += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        changed = true;
    }

    if input.pressed(KeyCode::Up) {
        velocity.y = time.delta_seconds() * PLAYER_VELOCITY_Y;
        player.y_velocity += time.delta_seconds() * PLAYER_VELOCITY_Y;
        changed = true;
    }

    if input.pressed(KeyCode::Down) {
        velocity.y = time.delta_seconds() * PLAYER_VELOCITY_Y * -1.0;
        player.y_velocity += time.delta_seconds() * PLAYER_VELOCITY_Y * -1.0;
        changed = true;
    }

    player.x_velocity = player.x_velocity.clamp(-PLAYER_VELOCITY_X,PLAYER_VELOCITY_X);
    player.y_velocity = player.y_velocity.clamp(-PLAYER_VELOCITY_Y,PLAYER_VELOCITY_Y);

    if changed {
        vel.linvel = Vec2 {x: player.x_velocity, y: player.y_velocity};
    }
}

fn change_scale_direction(mut rings: Query<(&mut Transform, &mut Scaling)>) {
    for (mut transform, mut ring) in &mut rings {
        if transform.scale.max_element() > ring.max_element_size {
            ring.scale_direction *= -1.0;
            transform.scale = Vec3 {x: ring.max_element_size, y:ring.max_element_size, z: 1.};
        }
        if transform.scale.min_element() < ring.min_element_size {
            ring.scale_direction *= -1.0;
            transform.scale = Vec3 {x: ring.min_element_size, y:ring.min_element_size, z: 1.};
        }
    }
}

fn scale_ring(mut rings: Query<(&mut Transform, &Scaling)>, timer: Res<Time>) {
    for (mut transform, ring) in &mut rings {
        transform.scale += ring.scale_direction * ring.scale_speed * timer.delta_seconds();
    }
}

