# kabuto-explosion

## やりたいこと
- マリオRPGのミニゲーム「爆裂カブトムシ」的な2Dシューティングゲームを自分で作ってみる
- 使ってみたい技術駆動で実現方法を考える

## 使ってみたい技術
- プログラミング言語: Rust
- ゲームエンジン: [Bevy](https://github.com/bevyengine/bevy)
- wasm化してwebブラウザ上でも動かしてみたい

## 参考にしたドキュメント類
### Rust関連
- [Rust日本語ドキュメント](https://doc.rust-jp.rs/)
- [std - Rust](https://doc.rust-lang.org/std/index.html)
  - 標準crateとかのドキュメント
- [Docs.rs](https://docs.rs/)
  - Cargo.tomlに追加するcrateを探す時に

### Bevy関連
- [Bevy - Bevy Examples in WebGL2](https://bevyengine.org/examples/)
- [Rustで3Dサンドボックスゲームを作る #1 Bevy](https://zenn.dev/publictheta/articles/034a9e28073dfe)
- [Rust製ゲームエンジンBevyの使い方](https://zenn.dev/hansel/scraps/ae80c102c129d2)
  - シンプルなやりたいことベースで解説されてるのでわかりやすかった
- [Rust Bevy ECS 入門](https://zenn.dev/hideakitai/articles/rust_bevy_ecs_introduction_ht)
- [[WIP] Entity-Component-System 実装！](https://zenn.dev/toyboot4e/books/making-toy-ecs)
  - ECSはゲーム制作における代表的なアーキテクチャ
  - BevyもECSアーキテクチャに沿った設計・実装が行われている