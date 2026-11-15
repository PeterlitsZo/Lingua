# Lingua

Mit Lingua verwaltest du mehrsprachige Dokumentation in einer Quelldatei.

Zum Beispiel mit diesem Lingua-Dokument `hello.md.lingua`:

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

Mit dem Werkzeug `lingua` erzeugen wir Dokumente in verschiedenen Sprachen:

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

Dieser Befehl erzeugt:

```markdown
# 你好
```

## Bauen und installieren

Derzeit wird nur das Bauen aus dem Quellcode unterstützt.

### Aus dem Quellcode bauen

Du brauchst eine Rust-Toolchain mit Unterstützung für Edition 2024 und Cargo.
Führe im Projektverzeichnis Folgendes aus:

```sh
cargo build --release
```

## Verwendung

### Kommandozeilenargumente

- `-o <OUTPUT-FILE>`: Legt die Ausgabedatei fest. Ohne diese Option wird die
  Endung `.lingua` vom Quellpfad entfernt.
- `-v <NAME=VALUE>`: Setzt eine Variable.
- `-h` / `-V`: Zeigt die Hilfe oder Version an.

### Variablen

Lingua unterstützt Variablen der Typen String, Atom, Boolean und Null. Nicht
vorhandene Variablen ergeben `null`. Zum Beispiel:

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### Vorlagensyntax

Lingua-Ausdrücke werden mit der Syntax `\(...)` geschrieben.

Die einfachsten Ausdrücke sind Literale. Zum Beispiel:

```
\("hello")
\("""
  | Hello World
""")
```

Über den Variablennamen kannst du ihren Wert ausgeben. Hier wird der Wert von
`name` ausgegeben:

```
\(name)
```

Mit `match` kannst du anhand eines Werts einen Zweig auswählen. Zum Beispiel:

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

Der erste passende Zweig wird gewählt. `_` ist der Standardzweig, falls kein
anderer passt.
