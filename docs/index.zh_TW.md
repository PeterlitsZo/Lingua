# Lingua

使用 Lingua，在一個原始檔中維護多語言文件。

例如，以下 Lingua 文件 `hello.md.lingua`：

```
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

我們可以透過 Lingua 提供的 `lingua` 工具產生不同語言的文件：

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

上述命令會產生：

```markdown
# 你好
```

## 建置與安裝

目前僅支援從原始碼建置。

### 從原始碼建置

需要安裝支援 Rust 2024 edition 的 Rust 工具鏈與 Cargo。在專案根目錄執行：

```sh
cargo build --release
```

## 使用

### 命令列參數

- `-o <OUTPUT-FILE>`: 指定輸出檔案。省略時，移除來源路徑最後的 `.lingua` 副檔名
  作為輸出路徑。
- `-v <NAME=VALUE>`: 設定變數。
- `-h` / `-V`: 顯示說明或版本。

### 變數

Lingua 支援多種變數型別，包括字串、Atom、Boolean 和 Null。不存在的變數求值為
`null`。例如：

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### 範本語法

我們使用 `\(...)` 語法表示 Lingua 的特殊運算式。

最簡單的運算式是字面值，例如：

```
\("hello")
\("""
  | Hello World
""")
```

也可以使用變數名稱來輸出變數，以下會輸出 `name` 變數的值：

```
\(name)
```

可以使用 `match` 進行比對，例如：

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

它會依序選擇符合的分支。`_` 是預設分支，沒有符合項目時使用。
