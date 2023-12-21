use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const BOUNDS: Vec2 = Vec2::new(80., 80.);
const PLAYER_WIDTH: f32 = 14.;
const PLAYER_HEGHT: f32 = 9.;

#[derive(Component)]
struct Player {
    movement_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut my_2d_camera_bundle = Camera2dBundle::default();
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(10.0);

    commands.spawn(my_2d_camera_bundle);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(BOUNDS.x * 2., BOUNDS.y * 2.)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Slime-0001.png"),
            ..default()
        },
        Player {
            movement_speed: 100.0,
        },
    ));
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let (slime, mut transform) = query.single_mut();
    let mut cam_transform = camera.single_mut();

    let mut dx = 0.0;
    let mut dy = 0.0;

    if keys.pressed(KeyCode::W) {
        dy += slime.movement_speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        dx -= slime.movement_speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::S) {
        dy -= slime.movement_speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        dx += slime.movement_speed * time.delta_seconds();
    }

    transform.translation.x += dx;
    transform.translation.y += dy;

    if transform.translation.x < -BOUNDS.x + PLAYER_WIDTH / 2. && dx < 0.0 {
        transform.translation.x = -BOUNDS.x + PLAYER_WIDTH / 2.;
    }

    if transform.translation.x > BOUNDS.x - PLAYER_WIDTH / 2. && dx > 0.0 {
        transform.translation.x = BOUNDS.x - PLAYER_WIDTH / 2.;
    }

    if transform.translation.y < -BOUNDS.y + PLAYER_HEGHT / 2. && dy < 0.0 {
        transform.translation.y = -BOUNDS.y + PLAYER_HEGHT / 2.;
    }

    if transform.translation.y > BOUNDS.y - PLAYER_HEGHT / 2. && dy > 0.0 {
        transform.translation.y = BOUNDS.y - PLAYER_HEGHT / 2.;
    }
    cam_transform.translation.x = transform.translation.x;
    cam_transform.translation.y = transform.translation.y;

    println!("{}", transform.translation);
}
