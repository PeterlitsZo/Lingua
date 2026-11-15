# Lingua

Avec Lingua, gérez une documentation multilingue dans un seul fichier source.

Prenons ce document Lingua, `hello.md.lingua` :

```text
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

L’outil `lingua` permet de générer des documents dans différentes langues :

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

Cette commande produit :

```markdown
# 你好
```

## Compilation et installation

Pour le moment, seule la compilation depuis les sources est disponible.

### Compiler depuis les sources

Il faut une chaîne Rust compatible avec l’édition 2024 et Cargo. À la racine du
projet, exécutez :

```sh
cargo build --release
```

## Utilisation

### Arguments de ligne de commande

- `-o <OUTPUT-FILE>`: Indique le fichier de sortie. Sinon, le suffixe `.lingua`
  est retiré du chemin source.
- `-v <NAME=VALUE>`: Définit une variable.
- `-h` / `-V`: Affiche l’aide ou la version.

### Variables

Lingua accepte les variables de type chaîne, Atom, Boolean et Null. Une
variable inexistante vaut `null`. Par exemple :

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### Syntaxe des modèles

Les expressions Lingua s’écrivent avec la syntaxe `\(...)`.

Les expressions les plus simples sont les littéraux. Par exemple :

```
\("hello")
\("""
  | Hello World
""")
```

On peut aussi afficher une variable en utilisant son nom. Ici, on affiche la
valeur de `name` :

```
\(name)
```

On peut utiliser `match` pour choisir selon une valeur. Par exemple :

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

La première branche correspondante est choisie. `_` est la branche par défaut,
utilisée si aucune ne correspond.
