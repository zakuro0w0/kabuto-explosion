use crate::{components::*, kabuto::*};
use bevy::{prelude::*, sprite::collide_aabb::collide};

// 自機が撃つ弾を1個生成する
pub fn setup_shot(commands: &mut Commands, kabuto_transform: &Transform) {
    // 弾の方向は真上
    const SHOT_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);
    // 弾の速度
    const SHOT_SPEED: f32 = 600.0;
    commands
        .spawn()
        .insert(Shot)
        // 弾は時間経過で動かす必要がある
        // 弾の基本ベクトルに速度を掛けたものを適用する
        .insert(Velocity(SHOT_DIRECTION.normalize() * SHOT_SPEED))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: kabuto_transform.translation + Vec3::new(0.0, KABUTO_SIZE.y, 0.0),
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

// 弾を生成する
pub fn shoot(
    // キーボード入力
    keyboard_input: Res<Input<KeyCode>>,
    // マウス入力
    mouse_input: Res<Input<MouseButton>>,
    // 射撃イベント
    mut shot_events: EventWriter<ShotEvent>,
    // 自機の位置
    mut query: Query<&mut Transform, With<Kabuto>>,
    mut commands: Commands,
) {
    let kabuto_transform = query.single_mut();
    // スペースキーまたは左クリックのリリースを検出する
    if keyboard_input.just_released(KeyCode::Space) || mouse_input.just_released(MouseButton::Left)
    {
        // 対象のキーが押下>>リリースされていたら自機の位置を元に弾を1個生成する
        setup_shot(&mut commands, &kabuto_transform);
        // 射撃イベントを発行する
        shot_events.send_default();
    }
}

// 撃った弾と敵の衝突判定システム
pub fn shot_enemy_collision(
    // Entityの追加や削除を行いたいのでコマンドを貰うことにする
    mut commands: Commands,
    // 撃った弾のクエリ(衝突判定に使うためのTransformと、画面から削除するためのEntityをShotコンポーネントについて集める)
    shot_query: Query<(Entity, &Transform), With<Shot>>,
    // 衝突判定を持ったEntityのクエリ(EntityがEnemyだった場合に別の処理ができるようにOptionでEnemyか否かを判定できるようにする)
    collider_query: Query<(Entity, &Transform, Option<&Enemy>), With<ShotCollider>>,
    // 他のシステム関数が衝突イベントを検出できるようにするためのイベント発行オブジェクト
    mut collision_events: EventWriter<CollisionEvent>,
    // スコアボードEntity
    mut scoreboad: ResMut<ScoreBoard>,
) {
    // 撃った弾のクエリからEntityとTransformを取り出す
    for (shot_entity, shot_transform) in &shot_query {
        // 撃った弾のサイズをTransformから取り出す
        let shot_size = shot_transform.scale.truncate();
        // 衝突判定持ちのクエリからEntity, Transformを取り出して順次処理する
        for (collider_entity, collider_transform, maybe_enemy) in &collider_query {
            // 弾との衝突判定をBevyのcollide()関数で行う
            // 衝突判定には2つのEntityの位置(translation)とサイズが必要
            let collision = collide(
                shot_transform.translation,
                shot_size,
                collider_transform.translation,
                collider_transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                // 衝突が発生した場合
                if maybe_enemy.is_some() {
                    // 弾が当たったのがEnemyだった場合
                    // 衝突イベントを発行して他のシステムにも知らせる
                    collision_events.send_default();
                    // Enemyを画面から消去する
                    commands.entity(collider_entity).despawn();
                    // スコアを加算する(今はとりあえず1個破壊毎に100点)
                    scoreboad.score += 100;
                }
            }
        }
    }
}

// 射撃時の効果音を再生するシステム
pub fn play_shot_sound(
    shot_events: EventReader<ShotEvent>,
    audio: Res<Audio>,
    sound: Res<ShotSound>,
) {
    if !shot_events.is_empty() {
        shot_events.clear();
        audio.play(sound.0.clone());
    }
}
