use crate::components::*;
use bevy::prelude::*;

// add_startup_system()に渡す最初の準備処理
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 効果音のリソースを作成(mp3ではダメだったのでoggに変換した)
    let hit_sound = asset_server.load("hit.ogg");
    let shot_sound = asset_server.load("shot.ogg");
    // 効果音と紐付く構造体をリソースとして追加
    commands.insert_resource(HitSound(hit_sound));
    commands.insert_resource(ShotSound(shot_sound));
}

// 衝突時の効果音を再生するシステム
pub fn play_hit_sound(
    // 衝突イベント
    collision_events: EventReader<CollisionEvent>,
    // Audioリソース
    audio: Res<Audio>,
    // 衝突効果音
    sound: Res<HitSound>,
) {
    if !collision_events.is_empty() {
        // 衝突イベントが空っぽでない場合
        // 衝突イベントを空にする
        collision_events.clear();
        // 衝突効果音を再生する
        audio.play(sound.0.clone());
    }
}
