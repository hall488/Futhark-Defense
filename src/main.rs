use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const BOUNDS: Vec2 = Vec2::new(206.0, 114.0);

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
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(3.0);

    commands.spawn(my_2d_camera_bundle);
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
) {
    let (slime, mut transform) = query.single_mut();

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

    if transform.translation.x < -BOUNDS.x && dx < 0.0
        || transform.translation.x > BOUNDS.x && dx > 0.0
    {
        dx = 0.0;
    }

    if transform.translation.y < -BOUNDS.y && dy < 0.0
        || transform.translation.y > BOUNDS.y && dy > 0.0
    {
        dy = 0.0;
    }

    transform.translation.x += dx;
    transform.translation.y += dy;

    println!("{}", transform.translation);
}
