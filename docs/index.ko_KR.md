# Lingua

Lingua로 하나의 소스 파일에서 다국어 문서를 관리할 수 있습니다.

예를 들어 다음과 같은 Lingua 문서 `hello.md.lingua`가 있다고 합시다.

```
# \(match lang
  :zh_CN "你好"
  _      "Hello")
```

`lingua` 도구로 언어별 문서를 생성할 수 있습니다.

```text
lingua hello.md.lingua -v lang=:zh_CN -o hello.md
```

위 명령의 출력은 다음과 같습니다.

```markdown
# 你好
```

## 빌드 및 설치

현재는 소스에서 빌드하는 방법만 지원합니다.

### 소스에서 빌드

Rust 2024 edition을 지원하는 Rust 도구 체인과 Cargo가 필요합니다. 프로젝트
루트에서 실행하세요.

```sh
cargo build --release
```

## 사용법

### 명령줄 인수

- `-o <OUTPUT-FILE>`: 출력 파일을 지정합니다. 생략하면 소스 경로 끝의
  `.lingua`를 제거한 경로에 출력합니다.
- `-v <NAME=VALUE>`: 변수를 설정합니다.
- `-h` / `-V`: 도움말 또는 버전을 표시합니다.

### 변수

Lingua는 문자열, Atom, Boolean, Null 타입의 변수를 지원합니다. 존재하지 않는
변수는 `null`로 평가됩니다. 예를 들면:

```sh
lingua hello.md.lingua -v 'name="Ada"' -v lang=:en_US -v \
  enabled=true -v optional=null
```

### 템플릿 문법

Lingua 표현식은 `\(...)` 문법으로 작성합니다.

가장 간단한 표현식은 리터럴입니다. 예를 들면:

```
\("hello")
\("""
  | Hello World
""")
```

변수 이름으로 값을 출력할 수도 있습니다. 다음은 `name` 변수의 값을 출력합니다.

```
\(name)
```

`match`로 값에 따라 분기할 수 있습니다. 예를 들면:

```
\(match lang
  :zh_CN "你好"
  _      "Hello")
```

순서대로 일치하는 분기를 선택합니다. `_`는 일치하는 항목이 없을 때 사용하는
기본 분기입니다.
