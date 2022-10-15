use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// 自機(カブトムシ)
#[derive(Component)]
struct Kabuto;

// 自機が撃つ弾
#[derive(Component)]
struct Shot;

// 衝突判定
#[derive(Component)]
struct Collider;

// 速度
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// ゲームの単位時間
const TIME_STEP: f32 = 1.0 / 60.0;
// 左の壁
const LEFT_WALL: f32 = -450.;
// 右の壁
const RIGHT_WALL: f32 = 450.;
// 壁の厚み
const WALL_THICKNESS: f32 = 10.0;

// 自機の速度
const KABUTO_SPEED: f32 = 500.0;
// 自機のサイズ
const KABUTO_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const KABUTO_PADDING: f32 = 10.0;

// add_startup_system()に渡す最初の準備処理
fn setup(mut commands: Commands) {
    // 2D画面用のカメラを設定する
    commands.spawn_bundle(Camera2dBundle::default());
    // 自機を初期化する
    setup_kabuto(&mut commands);
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

// 自機のセットアップ
fn setup_kabuto(commands: &mut Commands) {
    commands
        // Entityを1個生成する
        .spawn()
        // 生成したEntityにComponentを関連付ける
        // ここで関連付けておくことで、Queryを引数に取る際にWith<T: Component>で
        // 特定のComponentだけを対象にした処理を記述することができる
        .insert(Kabuto)
        // 衝突判定用のComponentを関連付ける
        .insert(Collider)
        // 外観となるスプライトを設定する
        .insert_bundle(SpriteBundle {
            // Transformはスプライトの位置と大きさを定義する
            transform: Transform {
                // 2D画面の場合、zは0.0固定
                translation: Vec3::new(0.0, -300.0, 0.0),
                scale: Vec3::new(25.0, 25.0, 0.0),
                // デフォルト値で良いものは以下のように記述を省略できる
                ..Default::default()
            },
            sprite: Sprite {
                // スプライトの色を指定する
                color: Color::rgb(0.3, 0.3, 0.7),
                ..Default::default()
            },
            ..Default::default()
        });
}

// 自機が撃つ弾を1個生成する
fn setup_shot(commands: &mut Commands, kabuto_transform: &Transform) {
    // 弾の方向は真上
    const SHOT_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);
    // 弾の速度
    const SHOT_SPEED: f32 = 600.0;
    commands
        .spawn()
        .insert(Shot)
        .insert(Collider)
        // 弾は時間経過で動かす必要がある
        // 弾の基本ベクトルに速度を掛けたものを適用する
        .insert(Velocity(SHOT_DIRECTION.normalize() * SHOT_SPEED))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: kabuto_transform.translation + Vec3::new(0.0, 50.0, 0.0),
                scale: Vec3::new(5.0, 5.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}

// SystemSet::with_system()に渡す「自機を移動させるシステム」
// 引数の型と並びはほぼ任意
fn move_kabuto(
    // キーボード入力
    keyboard_input: Res<Input<KeyCode>>,
    // KabutoコンポーネントのTransformを引数に取る
    mut query: Query<&mut Transform, With<Kabuto>>,
) {
    // 自機の位置をQueryから取り出す
    let mut kabuto_transform = query.single_mut();
    let mut direction = 0.0;
    // 押下されてるキーに応じて方向を決める
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }
    // 現在の自機の位置に方向*速度*単位時間を加算して新しい位置を決める
    let new_kabuto_position = kabuto_transform.translation.x + direction * KABUTO_SPEED * TIME_STEP;
    // 左右の壁の位置を計算
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + KABUTO_SIZE.x / 2.0 + KABUTO_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - KABUTO_SIZE.x / 2.0 - KABUTO_PADDING;
    // 自機の位置を新しい位置(左右の限界値で丸め済み)に置き換える
    kabuto_transform.translation.x = new_kabuto_position.clamp(left_bound, right_bound);
}

// 弾を生成する
fn shoot(
    // キーボード入力
    keyboard_input: Res<Input<KeyCode>>,
    // 自機の位置
    mut query: Query<&mut Transform, With<Kabuto>>,
    mut commands: Commands,
) {
    let kabuto_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        // スペースキーが押下されていたら自機の位置を元に弾を1個生成する
        setup_shot(&mut commands, &kabuto_transform);
    }
}

// 速度と時間経過を位置に反映する
// Velocityを関連付けたEntityのTransformとVelocityを引数に取るシステム
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    // Queryから位置と速度を取り出す
    for (mut transform, velocity) in &mut query {
        // 速度*単位時間を加算して位置を更新する
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

// 任意のプラグイン構造体
pub struct GamePlugin;
// Pluginトレイトを実装させる
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // アプリケーションに初期セットアップ関数を指定する
        app.add_startup_system(setup)
            // 背景色を指定
            .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.4)))
            // システムを決める
            .add_system_set(
                SystemSet::new()
                    // ゲームの単位時間毎にこのシステムを実行する
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    // 自機の移動システムを追加
                    .with_system(move_kabuto)
                    // 弾の生成システムを追加
                    .with_system(shoot)
                    // 時間経過で移動するオブジェクト向けの位置更新システムを追加
                    .with_system(apply_velocity),
            );
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kabuto Explosion Game".to_string(),
            width: 1080.,
            height: 768.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
