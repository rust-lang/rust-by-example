# Rust by Example 日本語版 翻訳ガイド

日本語版の翻訳は https://github.com/rust-lang-ja/rust-by-example
にて日本語のレビューを行います。
翻訳に貢献される方は以下のフローに従ってください。

- https://github.com/rust-lang/rust-by-example を自身のアカウントにフォークする
- `po/ja.po` に訳文を追加・変更する
- https://github.com/rust-lang-ja/rust-by-example
  （の`ja`ブランチ）にプルリクエストを出す
  - 直接オリジナルのリポジトリに出しても訳文自体のレビューができないので
    rust-lang-ja で一旦レビューします
  - 上記リポジトリは`ja`ブランチがデフォルトなので通常通りプルリクエストを作成すれば`ja`ブランチ向けになります

rust-lang-ja のレビュワーは以下のように作業します。

- プルリクエストをレビューし、問題なければ`ja`ブランチにマージ
- https://github.com/rust-lang/rust-by-example にプルリクエストを出す

最終的に https://github.com/rust-lang/rust-by-example
でマージされれば公開されます。

## 翻訳の方法

### 全般的なこと

- 文体は「です・ます」調
- 記号類は原則として全角（括弧`（）`やコロン`：`など）

### 対訳表

- カタカナ語のままで違和感のない用語はカタカナ語のまま使う
  - 気持としては「無理矢理和訳」を避けたい。そのための基準。
  - カタカナ語の方が用語として認識しやすい
- カタカナ語の末尾の長音記号「ー」は省く(JIS規格)
- 構文キーワードなどはそのままアルファベットを使う

| English                        | 日本語                                       |
| :----------------------------- | :------------------------------------------- |
| (lockの) acquire               | 獲得                                         |
| (lockの) release               | 解放                                         |
| Intrinsics                     | Intrinsic                                    |
| Lang Items                     | Lang Item                                    |
| Universal Function Call Syntax | 共通の関数呼び出し構文                       |
| abort                          | アボート                                     |
| activity                       | 実践                                         |
| aggregate type                 | 合成型                                       |
| alignment                      | アラインメント                               |
| allocator                      | アロケータ                                   |
| antipattern                    | アンチパターン                               |
| application                    | アプリケーション                             |
| argument type                  | 引数タイプ                                   |
| arity                          | アリティ                                     |
| array                          | 配列                                         |
| assignment                     | 代入                                         |
| associated -                   | 関連-                                        |
| atomic                         | アトミック                                   |
| attribute                      | アトリビュート                               |
| binary                         | バイナリ                                     |
| binding                        | 束縛                                         |
| block                          | ブロック                                     |
| borrow checker                 | 借用チェッカー                               |
| borrowing                      | 借用                                         |
| bounds                         | 境界                                         |
| bug                            | バグ                                         |
| capture                        | キャプチャ                                   |
| case analysis                  | 場合分け                                     |
| casting                        | キャスト                                     |
| channel                        | チャネル                                     |
| closure                        | クロージャ                                   |
| code bloat                     | コードの膨張                                 |
| coercion                       | 型強制                                       |
| color model                    | カラーモデル                                 |
| combinator                     | コンビネータ                                 |
| comma                          | カンマ                                       |
| command line                   | コマンドライン                               |
| compile-time error             | コンパイル時エラー                           |
| compiler                       | コンパイラ                                   |
| composable                     | 合成可能                                     |
| computer science               | コンピュータサイエンス                       |
| concurrency                    | 並行性                                       |
| constant                       | 定数                                         |
| constructor                    | コンストラクタ                               |
| continuous integration         | 継続的インテグレーション                     |
| crate                          | クレート                                     |
| custom type                    | カスタム型                                   |
| dangling                       | ダングリング                                 |
| data race                      | データ競合                                   |
| deadlock                       | デッドロック                                 |
| declaration statement          | 宣言文                                       |
| dereferencing                  | 参照外し                                     |
| derive                         | 導出                                         |
| designator                     | 識別子                                       |
| destructor                     | デストラクタ                                 |
| destructuring                  | 分配                                         |
| directive                      | ディレクティブ                               |
| directory                      | ディレクトリ                                 |
| discriminant                   | 判別子                                       |
| distribution                   | 配布物                                       |
| diverge                        | ダイバージ                                   |
| diverging                      | ダイバージング                               |
| diverging function             | 発散する関数                                 |
| documentation comment          | ドキュメンテーションコメント                 |
| documentation test             | ドキュメンテーションテスト                   |
| early return                   | 早期リターン                                 |
| empty tuple                    | 空タプル                                     |
| encode                         | エンコード                                   |
| endpoint                       | エンドポイント                               |
| entry point                    | エントリポイント                             |
| enum                           | 列挙型                                       |
| equality                       | 等値性                                       |
| ergonomic                      | エルゴノミック（人間にとって扱いやすいもの） |
| error                          | エラー                                       |
| error handling                 | エラーハンドリング                           |
| executable                     | 実行可能形式                                 |
| existentially quantified type  | 存在量型                                     |
| expression statement           | 式文                                         |
| exterior                       | 外側の                                       |
| feature                        | フィーチャ                                   |
| field                          | フィールド                                   |
| foreign                        | 他言語                                       |
| free-standing function         | フリースタンディングな関数                   |
| full path                      | 絶対パス                                     |
| generic parameter              | ジェネリックパラメータ                       |
| generics                       | ジェネリクス                                 |
| glob                           | グロブ                                       |
| growable                       | 伸張可能                                     |
| guard                          | ガード                                       |
| handle                         | ハンドル                                     |
| hash                           | ハッシュ                                     |
| hash set                       | ハッシュ集合                                 |
| higher order functions         | 高階関数                                     |
| identifier                     | 識別子                                       |
| immutability                   | イミュータビリティ                           |
| immutable                      | イミュータブル                               |
| implement                      | 実装する                                     |
| initialize                     | 初期化する                                   |
| input lifetime                 | 入力ライフタイム                             |
| install                        | インストール                                 |
| installer                      | インストーラ                                 |
| interior                       | 内側の                                       |
| interpolate                    | インターポーレートする                       |
| interpolation                  | インターポーレーション                       |
| key                            | キー                                         |
| keyword                        | キーワード                                   |
| leak                           | リーク                                       |
| least significant bit          | 最下位ビット                                 |
| lending                        | 貸付け                                       |
| library                        | ライブラリ                                   |
| lifetime                       | ライフタイム                                 |
| lifetime coercion              | ライフタイムの圧縮                           |
| lifetime elision               | ライフタイムの省略                           |
| lifetime parameter             | ライフタイムパラメータ                       |
| link                           | リンク                                       |
| lint                           | リント                                       |
| mangling                       | マングリング                                 |
| match                          | マッチ                                       |
| memory                         | メモリ                                       |
| method                         | メソッド                                     |
| monomorphization               | 単相化                                       |
| move                           | ムーブ                                       |
| mutability                     | ミュータビリティ                             |
| mutable                        | ミュータブル                                 |
| mutable binding                | ミュータブルな束縛                           |
| mutual-exclusion               | 相互排他                                     |
| null                           | ヌル                                         |
| numeric literal                | 数値リテラル                                 |
| object-safe                    | オブジェクト安全                             |
| offline                        | オフライン                                   |
| opaque                         | オペーク                                     |
| open source                    | オープンソース                               |
| option                         | オプション                                   |
| output lifetime                | 出力ライフタイム                             |
| output type                    | アウトプット型                               |
| overflow                       | オーバーフロー                               |
| owner                          | 所有者                                       |
| ownership                      | 所有権                                       |
| panic                          | パニック                                     |
| parameter                      | パラメータ                                   |
| parametric polymorphism        | パラメトリック多相                           |
| parse                          | パース、パースする                           |
| partial moves                  | 部分的ムーブ                                 |
| patch                          | パッチ                                       |
| pattern                        | パターン                                     |
| performance                    | パフォーマンス                               |
| phantom type                   | 幽霊型                                       |
| platform                       | プラットフォーム                             |
| pointer                        | ポインタ                                     |
| primitive type                 | プリミティブ型                               |
| private                        | プライベート                                 |
| process                        | プロセス                                     |
| public                         | パブリック                                   |
| r-value                        | 右辺値                                       |
| range                          | レンジ                                       |
| raw pointer                    | 生ポインタ                                   |
| raw identifier                 | 生識別子                                     |
| re-assignment                  | 再代入                                       |
| rebind                         | 再束縛                                       |
| reference count                | 参照カウント                                 |
| regression                     | リグレッション                               |
| release                        | リリース                                     |
| return                         | 返す                                         |
| return type                    | リターン型                                   |
| return value                   | 戻り値                                       |
| runtime                        | 実行時                                       |
| safe                           | 安全                                         |
| safety check                   | 安全性検査                                   |
| scope                          | スコープ                                     |
| scoped                         | スコープ化された                             |
| script                         | スクリプト                                   |
| semantics                      | セマンティクス                               |
| shadow                         | 覆い隠す                                     |
| shadowing                      | シャドーイング                               |
| signature                      | シグネチャ                                   |
| signed                         | 符号付き                                     |
| slice                          | スライス                                     |
| slicing                        | スライシング                                 |
| specialized                    | 特殊化された                                 |
| standard library               | 標準ライブラリ                               |
| string                         | 文字列                                       |
| string interpolation           | 文字列インターポーレーション                 |
| struct                         | 構造体                                       |
| structure                      | 構造体                                       |
| sum type                       | 直和型                                       |
| subtrait                       | サブトレイト                                 |
| supertrait                     | スーパートレイト                             |
| symbol                         | シンボル                                     |
| syntactic sugar                | 糖衣構文                                     |
| syntax tree                    | 構文木                                       |
| system                         | システム                                     |
| tagged union                   | タグ付き共用体                               |
| term                           | 項                                           |
| thread-locality                | スレッドローカル性                           |
| threadsafe                     | スレッドセーフ                               |
| tick                           | クオート                                     |
| token trees                    | トークン木                                   |
| trait                          | トレイト                                     |
| transmute                      | トランスミュート                             |
| tuple                          | タプル                                       |
| tuple struct                   | タプル                                       |
| type alias                     | 型エイリアス                                 |
| type erasure                   | 型消去                                       |
| type family                    | 型族                                         |
| type inference                 | 型推論                                       |
| type parameter                 | 型パラメータ                                 |
| uninstall                      | アンインストール                             |
| unit 注: `()` の読み           | ユニット                                     |
| unsafe                         | アンセーフ                                   |
| unsigned                       | 符号無し                                     |
| unsized type                   | サイズ不定型                                 |
| unwinding                      | 巻き戻し                                     |
| unwrap                         | アンラップ                                   |
| value constructor              | 値コンストラクタ                             |
| variable                       | 変数                                         |
| variable binding               | 変数束縛                                     |
| variant                        | ヴァリアント                                 |
| vector                         | ベクタ                                       |
| version                        | バージョン                                   |
| warning                        | ウォーニング                                 |
| wildcard                       | ワイルドカード                               |
| wrapper                        | ラッパ                                       |
