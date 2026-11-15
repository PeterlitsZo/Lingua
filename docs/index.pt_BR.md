# Lingua

Com o Lingua, você mantém documentação multilíngue em um único arquivo-fonte.

Por exemplo, com este documento Lingua, `hello.md.lingua`:

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

Podemos gerar documentos em diferentes idiomas com a ferramenta `lingua`:

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

O comando acima produz:

```markdown
# 你好
```

## Compilação e instalação

Por enquanto, só é possível compilar a partir do código-fonte.

### Compilar a partir do código-fonte

Você precisa de uma toolchain Rust compatível com a edição 2024 e do Cargo.
Execute na raiz do projeto:

```sh
cargo build --release
```

## Uso

### Argumentos da linha de comando

- `-o <OUTPUT-FILE>`: Define o arquivo de saída. Se omitido, remove o sufixo
  `.lingua` do caminho de origem.
- `-v <NAME=VALUE>`: Define uma variável.
- `-h` / `-V`: Exibe a ajuda ou a versão.

### Variáveis

Lingua aceita variáveis dos tipos string, Atom, Boolean e Null. Uma variável
inexistente resulta em `null`. Por exemplo:

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### Sintaxe de templates

Usamos a sintaxe `\(...)` para escrever expressões do Lingua.

As expressões mais simples são os literais. Por exemplo:

```
\("hello")
\("""
  | Hello World
""")
```

Também podemos usar o nome de uma variável para exibir seu valor. Aqui exibimos
o valor de `name`:

```
\(name)
```

Podemos usar `match` para escolher de acordo com um valor. Por exemplo:

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

A primeira opção correspondente é escolhida. `_` é a opção padrão, usada quando
nenhuma corresponde.
