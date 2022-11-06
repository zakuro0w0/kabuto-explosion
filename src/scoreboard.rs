use crate::components::*;
use bevy::prelude::*;

// スコアボードを生成する
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
pub fn update_scoreboard(scoreboard: Res<ScoreBoard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    // TextにはTextSectionで指定した分だけセクション配列が含まれている
    // 2番めのセクションにスコア数値を書き込む
    text.sections[1].value = scoreboard.score.to_string();
}
