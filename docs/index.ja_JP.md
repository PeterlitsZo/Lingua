# Lingua

Lingua を使うと、ひとつのソースファイルで多言語ドキュメントを管理できます。

たとえば、次の Lingua ドキュメント `hello.md.lingua` を用意します。

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

`lingua` コマンドで、言語ごとにドキュメントを生成できます。

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

このコマンドの出力は次のとおりです。

```markdown
# 你好
```

## ビルドとインストール

現在はソースからのビルドのみ対応しています。

### ソースからビルド

Rust 2024 edition に対応した Rust ツールチェーンと Cargo が必要です。プロジェク
トのルートで実行します。

```sh
cargo build --release
```

## 使い方

### コマンドライン引数

- `-o <OUTPUT-FILE>`: 出力ファイルを指定します。省略すると、入力パスの末尾から
  `.lingua` を除いたパスに出力します。
- `-v <NAME=VALUE>`: 変数を設定します。
- `-h` / `-V`: ヘルプまたはバージョンを表示します。

### 変数

Lingua の変数には文字列、Atom、Boolean、Null を使えます。存在しない変数は `null`
として評価されます。例：

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### テンプレート構文

Lingua の式は `\(...)` で記述します。

最も単純な式はリテラルです。例：

```
\("hello")
\("""
  | Hello World
""")
```

変数名を指定すると、その値を出力できます。次の例では `name` の値を出力します。

```
\(name)
```

`match` で値に応じた分岐ができます。例：

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

上から順に一致する分岐を選びます。`_` は、どれにも一致しない場合のデフォルトの分
岐です。
