# Lingua

Con Lingua puedes mantener documentación multilingüe en un solo archivo fuente.

Por ejemplo, con este documento de Lingua, `hello.md.lingua`:

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

Podemos generar documentos en distintos idiomas con la herramienta `lingua`:

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

El comando anterior produce:

```markdown
# 你好
```

## Compilación e instalación

Por ahora, solo se admite la compilación desde el código fuente.

### Compilar desde el código fuente

Necesitas una cadena de herramientas de Rust compatible con la edición 2024 y
Cargo. Ejecuta esto desde la raíz del proyecto:

```sh
cargo build --release
```

## Uso

### Argumentos de línea de comandos

- `-o <OUTPUT-FILE>`: Especifica el archivo de salida. Si se omite, se elimina
  el sufijo `.lingua` de la ruta de origen.
- `-v <NAME=VALUE>`: Define una variable.
- `-h` / `-V`: Muestra la ayuda o la versión.

### Variables

Lingua admite variables de tipo cadena, Atom, Boolean y Null. Una variable que
no existe se evalúa como `null`. Por ejemplo:

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### Sintaxis de plantillas

Usamos la sintaxis `\(...)` para escribir expresiones de Lingua.

Las expresiones más sencillas son los literales. Por ejemplo:

```
\("hello")
\("""
  | Hello World
""")
```

También podemos usar un nombre de variable para mostrar su valor. Aquí
mostramos el valor de `name`:

```
\(name)
```

Podemos usar `match` para elegir según un valor. Por ejemplo:

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

Se elige la primera rama que coincida. `_` es la rama predeterminada y se usa
si ninguna coincide.
