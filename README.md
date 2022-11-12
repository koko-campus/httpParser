# HTTPパーサー

Rust言語でHTTPリクエストをパースするためのクレート(モジュール)です。  

## ファイル構成

### tester.rs

クレートモジュールをテストするための簡単なWEBサーバです。  
「http_parser」ルートディレクトリに「.env」ファイルを生成して、以下の内容を記述してください。  

```.env
IP_ADDRESS="★★★"
HTTP_PORT="★★★"
```

### http_parser.rs

対象となるHTTPリクエストパースクレートです。  

「http_parser」モジュール作成し、内部に以下の機能を搭載しています。

```rust
HttpRequestStruct (構造体)
parse (関数)
```

「HttpRequestStruct」構造体は以下のメンバを有します。

```rust
method -> HttpMethod列挙型
path -> String
```

## 環境情報

| 機能 | バージョン |
| ---- | ---- |
| Linux / Ubuntu| 20.04 |
| Rust | 1.63.0 |

## 環境構築

```bash
# イロイロ最新に
sudo apt update
sudo apt upgrade
```

## Rust インストール

```bash
# インストールスクリプトの実行
curl https://sh.rustup.rs -sSf | sh
# インストール設定はデフォルト(1)で!!!

# 次に環境変数(PATH)を設定します。
export PATH="$HOME/.cargo/bin:$PATH"

# 最後に正しくインストール、パスの設定がされたか、以下のコマンドで確認します。
cargo --version
# -> cargo 1.63.0
rustc 1.63.0
# -> rustc 1.63.0
rustdoc --version
# -> rustdoc 1.63.0
```

## 実行方法

```bash
# テスト実行
cargo run

# コンパイルして、、、
cargo build --release

# 実行!!!
./target/release/julia_rs
```
