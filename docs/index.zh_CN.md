# Lingua

使用 Lingua，在一个源文件中维护多语言文档。

例如，对于如下 Lingua 文档 `hello.md.lingua`：

```
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

我们可以通过 Lingua 提供的 `lingua` 工具来生成不同语言的文档：

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

如上的命令中，它会渲染得到：

```markdown
# 你好
```

## 构建和安装

目前我们仅支持从源码中构建。

### 从源码构建

需要安装支持 Rust 2024 edition 的 Rust 工具链和 Cargo。在项目根目录运行：

```sh
cargo build --release
```

## 使用

### 命令行参数

- `-o <OUTPUT-FILE>`: 指定输出文件。不指定时，移除源路径最后的 `.lingua` 后缀作
  为输出路径。
- `-v <NAME=VALUE>`: 设置变量。
- `-h` / `-V`: 显示帮助或版本。

### 变量

Lingua 支持多种类型的变量，包括字符串、Atom、Boolean 和 Null。不存在的变量求值为
`null`。例如：

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### 模版语法

我们使用 `\(...)` 语法来表示 Lingua 特殊的表达式。

最简单的表达式为字面量，例如：

```
\("hello")
\("""
  | Hello World
""")
```

我们也可以使用变量名来渲染变量，如下我们会渲染 `name` 变量的值：

```
\(name)
```

我们支持使用 `match` 来进行匹配，例如：

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

它会按照顺序选择对应的分支。其中 `_` 为默认分支，没有匹配时使用这个分支。
