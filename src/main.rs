use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    // commands.spawn_bundle(block());
    commands
        .spawn()
        .insert(Kabuto)
        .insert(Collider)
        .insert_bundle(kabuto());
}

// ブロックのスプライト
fn block(translation: Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.00, 0.25, 0.25),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    }
}

// 自機のスプライト
fn kabuto() -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: Vec3::new(25.0, 25.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.7),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[derive(Component)]
struct Kabuto;

#[derive(Component)]
struct Collider;

const PADDLE_SPEED: f32 = 500.0;
const TIME_STEP: f32 = 1.0 / 60.0;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const WALL_THICKNESS: f32 = 10.0;
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_PADDING: f32 = 10.0;

fn move_kabuto(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Kabuto>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn shoot_kabuto(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Kabuto>>,
    mut commands: Commands,
) {
    let kabuto_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn().insert_bundle(block(
            kabuto_transform.translation + Vec3::new(0.0, 50.0, 0.0),
        ));
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.7)))
            .add_system_set(SystemSet::new().with_system(move_kabuto))
            .add_system_set(SystemSet::new().with_system(shoot_kabuto));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
