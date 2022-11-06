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

// 敵オブジェクト
#[derive(Component)]
struct Enemy {
    // 生まれてからの生存期間
    lifetime: f32,
}

// スコア表示
#[derive(Component)]
struct ScoreBoard {
    score: usize,
}

// 弾との衝突判定
#[derive(Component)]
struct ShotCollider;

// バウンドの衝突判定
#[derive(Component)]
struct BoundCollider;

// 衝突イベント
#[derive(Default)]
struct CollisionEvent;

// 弾を撃つイベント
#[derive(Default)]
struct ShotEvent;

// 壁の位置
enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    // 壁の位置を返す
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
        }
    }
    // 壁のサイズを返す
    fn size(&self) -> Vec2 {
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
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: BoundCollider,
}

impl WallBundle {
    // 壁オブジェクトをWallLocationから作る
    fn new(location: WallLocation) -> WallBundle {
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

// 速度
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// 弾ヒット時のサウンドエフェクト
struct HitSound(Handle<AudioSource>);

// 弾を撃った時のサウンドエフェクト
struct ShotSound(Handle<AudioSource>);

// ゲームの単位時間
const TIME_STEP: f32 = 1.0 / 60.0;

// 画面のサイズ
const SCREEN_SIZE: Vec2 = Vec2::new(1280., 768.);

// 左の壁(Bevyにおける座標(0,0)は画面の中央)
const LEFT_WALL: f32 = -SCREEN_SIZE.x / 2.;
// 右の壁(Bevyにおける座標(0,0)は画面の中央)
const RIGHT_WALL: f32 = SCREEN_SIZE.x / 2.;
// 上の壁
const TOP_WALL: f32 = SCREEN_SIZE.y / 2.;
// 下の壁
const BOTTOM_WALL: f32 = -SCREEN_SIZE.y / 2.;
// 壁の厚み
const WALL_THICKNESS: f32 = 50.0;
// 壁の色
const WALL_COLOR: Color = Color::rgba(0., 0., 0., 0.);

// 自機の速度
const KABUTO_SPEED: f32 = 500.0;
// 自機のサイズ
const KABUTO_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const KABUTO_PADDING: f32 = 10.0;

// add_startup_system()に渡す最初の準備処理
fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    // 2D画面用のカメラを設定する
    commands.spawn_bundle(Camera2dBundle::default());
    // 自機を初期化する
    setup_kabuto(&mut commands);
    // スコアボードを初期化する
    setup_scoreboard(&mut commands, &mut asset_server);
    // 効果音のリソースを作成(mp3ではダメだったのでoggに変換した)
    let hit_sound = asset_server.load("hit.ogg");
    let shot_sound = asset_server.load("shot.ogg");
    // 効果音と紐付く構造体をリソースとして追加
    commands.insert_resource(HitSound(hit_sound));
    commands.insert_resource(ShotSound(shot_sound));
    // 上下左右の壁を追加
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
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

// 自機が撃つ弾を1個生成する
fn setup_shot(commands: &mut Commands, kabuto_transform: &Transform) {
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

// スコアボードを生成する
fn setup_scoreboard(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    // スコアボードのフォントサイズ
    const SCOREBOARD_FONT_SIZE: f32 = 50.;
    // スコアボードのパディング
    const SCOREBORAD_TEXT_PADDING: Val = Val::Px(10.);
    // スコア表示の色
    const TEXT_COLOR: Color = Color::rgb(1., 1., 1.);
    // テキストスタイル
    let style = TextStyle {
        // フォントはassetsから読み込む
        font: asset_server.load("misaki_gothic.ttf"),
        font_size: SCOREBOARD_FONT_SIZE,
        color: TEXT_COLOR,
    };
    commands.spawn().insert_bundle(
        // テキストは複数のセクションで構成することもできる
        TextBundle::from_sections([
            // 固定で表示する方
            TextSection::new("Score: ", style.clone()),
            // 可変の数値を表示する方
            TextSection::from_style(style.clone()),
        ])
        // テキストの場合はTransformではなくStyleで位置を決める
        .with_style(Style {
            // 絶対位置指定
            position_type: PositionType::Absolute,
            // 画面の左上からパディングの分だけ空けた位置を指定する
            // PositionType::Absoluteだと画面左上が原点(0,0)になる？
            position: UiRect {
                top: SCOREBORAD_TEXT_PADDING,
                left: SCOREBORAD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

// スコアボードを更新するシステム
// TextBundleからはTextを取得することができる
// with_run_criteria()で実行間隔を指定しない場合はいつ実行される？
// ScoreBoardのEntityに変化があったら自動的に呼び出してくれる？
fn update_scoreboard(scoreboard: Res<ScoreBoard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    // TextにはTextSectionで指定した分だけセクション配列が含まれている
    // 2番めのセクションにスコア数値を書き込む
    text.sections[1].value = scoreboard.score.to_string();
}

// 敵オブジェクトを画面に追加する
fn setup_enemy(mut commands: Commands) {
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

// 弾を生成する
fn shoot(
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

// 敵オブジェクトに重力の影響を適用するシステム
fn apply_enemy_gravity(mut query: Query<(&mut Transform, &mut Velocity, &mut Enemy), With<Enemy>>) {
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

// 撃った弾と敵の衝突判定システム
fn shot_enemy_collision(
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

// 敵と壁の衝突判定システム
fn enemy_bound_collision(
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
    bound_collider: Query<(&Transform), With<BoundCollider>>,
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

// 衝突時の効果音を再生するシステム
fn play_hit_sound(
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

// 射撃時の効果音を再生するシステム
fn play_shot_sound(shot_events: EventReader<ShotEvent>, audio: Res<Audio>, sound: Res<ShotSound>) {
    if !shot_events.is_empty() {
        shot_events.clear();
        audio.play(sound.0.clone());
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
                    .with_system(setup_enemy),
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
