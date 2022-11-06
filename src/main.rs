use bevy::{prelude::*, time::FixedTimestep};

mod components;
mod enemy;
mod kabuto;
mod scoreboard;
mod shot;
mod sound;
mod world;
use crate::{components::*, enemy::*, kabuto::*, scoreboard::*, shot::*, sound::*, world::*};

// 任意のプラグイン構造体
pub struct GamePlugin;
// Pluginトレイトを実装させる
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // アプリケーションに初期セットアップ関数を指定する
        app.add_startup_system(scoreboard::setup)
            .add_startup_system(sound::setup)
            .add_startup_system(world::setup)
            .add_startup_system(kabuto::setup)
            // 背景色を指定
            .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.4)))
            // スコアボードを追加
            .insert_resource(ScoreBoard { score: 0 })
            // 衝突イベントがあることを教える
            .add_event::<CollisionEvent>()
            // 射撃イベントがあることを教える
            .add_event::<ShotEvent>()
            // システムを決める
            .add_system_set(
                SystemSet::new()
                    // ゲームの単位時間毎にこのシステムを実行する
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    // 衝突判定システムを追加
                    .with_system(shot_enemy_collision)
                    .with_system(enemy_bound_collision)
                    // 自機の移動システムを追加
                    .with_system(move_kabuto.before(shot_enemy_collision))
                    // 弾の生成システムを追加
                    .with_system(shoot.before(shot_enemy_collision))
                    // 時間経過で移動するオブジェクト向けの位置更新システムを追加
                    .with_system(apply_velocity.before(shot_enemy_collision))
                    // 敵オブジェクトに重力の影響を与えるシステムを追加
                    .with_system(apply_enemy_gravity.before(shot_enemy_collision))
                    // ヒット効果音再生システムを追加
                    .with_system(play_hit_sound.after(shot_enemy_collision))
                    // 射撃効果音再生システムを追加
                    .with_system(play_shot_sound),
            )
            // スコアボード更新システムを追加
            .add_system(update_scoreboard)
            .add_system_set(
                // 一定時間毎に敵オブジェクトを画面に追加するシステム
                SystemSet::new()
                    // 120F毎に実行する
                    .with_run_criteria(FixedTimestep::step(120. / 60.))
                    // 敵オブジェクト生成システム
                    .with_system(enemy::setup),
            );
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kabuto Explosion Game".to_string(),
            width: SCREEN_SIZE.x,
            height: SCREEN_SIZE.y,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
