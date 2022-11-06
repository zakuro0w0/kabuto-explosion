use crate::{components::*, world::*};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// 敵オブジェクトを画面に追加する
pub fn setup(mut commands: Commands) {
    // ひとまず右方向にまっすぐ進むだけ
    const ENEMY_DIRECTION: Vec2 = Vec2::new(1.0, 0.);
    const ENEMY_SPEED: f32 = 400.;
    commands
        .spawn()
        .insert(Enemy { lifetime: 0. })
        .insert(ShotCollider)
        .insert(BoundCollider)
        .insert(Velocity(ENEMY_DIRECTION.normalize() * ENEMY_SPEED))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                // 画面の左上に出現する
                translation: Vec3::new(-SCREEN_SIZE.x / 2., SCREEN_SIZE.y / 3., 0.),
                scale: Vec3::new(50., 50., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        });
}

// 敵オブジェクトに重力の影響を適用するシステム
pub fn apply_enemy_gravity(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Enemy), With<Enemy>>,
) {
    // 重力加速度(数値の大きさは調整中)
    const GRAVITY: Vec2 = Vec2::new(0., -9.8);
    for (mut transform, mut velocity, mut enemy) in &mut query {
        // 生存期間を増やす
        enemy.lifetime += TIME_STEP;
        // 生存期間が長いほどより強く重力が掛かるようにする
        velocity.y += GRAVITY.normalize().y * enemy.lifetime;
        // 敵オブジェクトのY軸位置に重力の影響を加える
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

// 敵と壁の衝突判定システム
pub fn enemy_bound_collision(
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
    bound_collider: Query<&Transform, With<BoundCollider>>,
) {
    for (mut enemy_velocity, enemy_transform) in &mut enemies {
        let enemy_size = enemy_transform.scale.truncate();
        for bound_transform in &bound_collider {
            let collision = collide(
                enemy_transform.translation,
                enemy_size,
                bound_transform.translation,
                bound_transform.scale.truncate(),
            );

            let print_collision = || {
                // println!("enemy_transform: {}", enemy_transform.translation);
                // println!("enemy_size: {}", enemy_size);
                // println!("bound_transform: {}", bound_transform.translation);
                // println!("bound_size: {}", bound_transform.scale.truncate());
            };

            if let Some(collision) = collision {
                // println!("----------------- collision ----------------");
                let mut reflect_x = false;
                let mut reflect_y = false;
                match collision {
                    Collision::Left => {
                        print_collision();
                        // println!("left");
                        reflect_x = enemy_velocity.x > 0.0
                    }
                    Collision::Right => {
                        print_collision();
                        // println!("right");
                        reflect_x = enemy_velocity.x < 0.0
                    }
                    Collision::Top => {
                        print_collision();
                        // println!("top");
                        reflect_y = enemy_velocity.y < 0.0
                    }
                    Collision::Bottom => {
                        print_collision();
                        // println!("bottom");
                        reflect_y = enemy_velocity.y > 0.0
                    }
                    Collision::Inside => {
                        print_collision();
                        // println!("inside");
                    }
                }
                if reflect_x {
                    // println!("reflect x");
                    // println!(
                    //     "before velocity=({}, {})",
                    //     enemy_velocity.x, enemy_velocity.y
                    // );
                    enemy_velocity.x = -enemy_velocity.x;
                    // println!(
                    //     "after  velocity=({}, {})",
                    //     enemy_velocity.x, enemy_velocity.y
                    // );
                }
                if reflect_y {
                    // println!("reflect y");
                    // println!(
                    //     "before velocity=({}, {})",
                    //     enemy_velocity.x, enemy_velocity.y
                    // );
                    enemy_velocity.y = -enemy_velocity.y;
                    // println!(
                    //     "after  velocity=({}, {})",
                    //     enemy_velocity.x, enemy_velocity.y
                    // );
                }
            }
        }
    }
}
