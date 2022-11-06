use bevy::prelude::*;

// 自機(カブトムシ)
#[derive(Component)]
pub struct Kabuto;

// 自機が撃つ弾
#[derive(Component)]
pub struct Shot;

// 敵オブジェクト
#[derive(Component)]
pub struct Enemy {
    // 生まれてからの生存期間
    pub lifetime: f32,
}

// スコア表示
#[derive(Component)]
pub struct ScoreBoard {
    pub score: usize,
}

// 弾との衝突判定
#[derive(Component)]
pub struct ShotCollider;

// バウンドの衝突判定
#[derive(Component)]
pub struct BoundCollider;

// 速度
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

// 弾ヒット時のサウンドエフェクト
pub struct HitSound(pub Handle<AudioSource>);

// 弾を撃った時のサウンドエフェクト
pub struct ShotSound(pub Handle<AudioSource>);

// 衝突イベント
#[derive(Default)]
pub struct CollisionEvent;

// 弾を撃つイベント
#[derive(Default)]
pub struct ShotEvent;
