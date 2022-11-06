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

## WebAssembly化してブラウザ上で動作させる

### 参考にしたURL
- [自作のRust製エミュレーターをWebブラウザーで動くようにした](https://zenn.dev/tanakh/articles/rust-emulator-in-browser)
  - ここがわかりやすかった
- [Introduction - Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
  - Rust公式、順を追って説明しているが少し手間が掛かる
- [Browser (WebAssembly) - Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)
  - Bevy公式、簡単

### 手順

```
rustup target install wasm32-unknown-unknown
```

```
cargo install wasm-server-runner
```

#### .cargo/config.toml
```toml
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```

```
cargo run --target wasm32-unknown-unknown
```

- webブラウザで`localhost:1334`を開くとアプリが動き始める
