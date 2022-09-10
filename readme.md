# HTTPパーサー

Rust言語でHTTPリクエストをパースするためのクレート(モジュール)です。  
リクエストライン(メソッド・パス)、ヘッダー、ボディ部を解析します。  


# ファイル構成

## tester.rs

クレートモジュールをテストするための簡単なWEBサーバです。  
「http_parser」ルートディレクトリに「.env」ファイルを生成して、以下の内容を記述してください。  

```.env
IP_ADDRESS="★★★"
HTTP_PORT="★★★"
```


## http_parser.rs

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


