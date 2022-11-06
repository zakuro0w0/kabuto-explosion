use crate::components::*;
use bevy::prelude::*;

// 壁の位置
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    // 壁の位置を返す
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
        }
    }
    // 壁のサイズを返す
    pub fn size(&self) -> Vec2 {
        match self {
            // 左右の壁
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, SCREEN_SIZE.y + WALL_THICKNESS)
            }
            // 上下の壁
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(SCREEN_SIZE.x + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

// 壁オブジェクト
#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub collider: BoundCollider,
}

impl WallBundle {
    // 壁オブジェクトをWallLocationから作る
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: BoundCollider,
        }
    }
}

// ゲームの単位時間
pub const TIME_STEP: f32 = 1.0 / 60.0;

// 画面のサイズ
pub const SCREEN_SIZE: Vec2 = Vec2::new(1280., 768.);

// 左の壁(Bevyにおける座標(0,0)は画面の中央)
pub const LEFT_WALL: f32 = -SCREEN_SIZE.x / 2.;
// 右の壁(Bevyにおける座標(0,0)は画面の中央)
pub const RIGHT_WALL: f32 = SCREEN_SIZE.x / 2.;
// 上の壁
pub const TOP_WALL: f32 = SCREEN_SIZE.y / 2.;
// 下の壁
pub const BOTTOM_WALL: f32 = -SCREEN_SIZE.y / 2.;
// 壁の厚み
pub const WALL_THICKNESS: f32 = 50.0;
// 壁の色
pub const WALL_COLOR: Color = Color::rgba(0., 0., 0., 0.);

// add_startup_system()に渡す最初の準備処理
pub fn setup(mut commands: Commands) {
    // 2D画面用のカメラを設定する
    commands.spawn_bundle(Camera2dBundle::default());
    // 上下左右の壁を追加
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
}

// 速度と時間経過を位置に反映する
// Velocityを関連付けたEntityのTransformとVelocityを引数に取るシステム
pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    // Queryから位置と速度を取り出す
    for (mut transform, velocity) in &mut query {
        // 速度*単位時間を加算して位置を更新する
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
