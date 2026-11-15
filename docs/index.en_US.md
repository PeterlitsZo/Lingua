# Lingua

Use Lingua to maintain multilingual documentation in a single source file.

For example, given this Lingua document, `hello.md.lingua`:

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

We can generate documents in different languages using the `lingua` tool:

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

The command above produces:

```markdown
# 你好
```

## Build and install

Currently, we only support building from source.

### Build from source

You need a Rust toolchain that supports edition 2024 and Cargo. Run this from
the project root:

```sh
cargo build --release
```

## Usage

### Command-line arguments

- `-o <OUTPUT-FILE>`: Set the output file. If omitted, remove the final
  `.lingua` suffix from the source path to get the output path.
- `-v <NAME=VALUE>`: Set a variable.
- `-h` / `-V`: Show help or version information.

### Variables

Lingua supports String, Atom, Boolean, and Null variables. A variable that does
not exist evaluates to `null`. For example:

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### Template syntax

We use `\(...)` to write Lingua expressions.

The simplest expressions are literals. For example:

```
\("hello")
\("""
  | Hello World
""")
```

We can also render a variable by name. This renders the value of `name`:

```
\(name)
```

We support matching values with `match`. For example:

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

The first matching branch is selected. `_` is the default branch, used when
nothing else matches.
