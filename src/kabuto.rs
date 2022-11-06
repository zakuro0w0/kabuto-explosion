use crate::{components::*, world::*};
use bevy::prelude::*;

// 自機の速度
pub const KABUTO_SPEED: f32 = 500.0;
// 自機のサイズ
pub const KABUTO_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
pub const KABUTO_PADDING: f32 = 10.0;

// 自機のセットアップ
pub fn setup(mut commands: Commands) {
    commands
        // Entityを1個生成する
        .spawn()
        // 生成したEntityにComponentを関連付ける
        // ここで関連付けておくことで、Queryを引数に取る際にWith<T: Component>で
        // 特定のComponentだけを対象にした処理を記述することができる
        .insert(Kabuto)
        // 外観となるスプライトを設定する
        .insert_bundle(SpriteBundle {
            // Transformはスプライトの位置と大きさを定義する
            transform: Transform {
                // 2D画面の場合、zは0.0固定
                translation: Vec3::new(
                    0.0,
                    BOTTOM_WALL + WALL_THICKNESS / 2. + KABUTO_SIZE.y / 2. + KABUTO_PADDING,
                    0.0,
                ),
                scale: KABUTO_SIZE,
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

// SystemSet::with_system()に渡す「自機を移動させるシステム」
// 引数の型と並びはほぼ任意
pub fn move_kabuto(
    // キーボード入力
    keyboard_input: Res<Input<KeyCode>>,
    // KabutoコンポーネントのTransformを引数に取る
    mut query: Query<&mut Transform, With<Kabuto>>,
) {
    // 自機の位置をQueryから取り出す
    let mut kabuto_transform = query.single_mut();
    let mut direction = 0.0;
    // 押下されてるキーに応じて方向を決める
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
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
