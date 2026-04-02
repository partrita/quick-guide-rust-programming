# 처음 시작하기

`Rust`를 경험해보기 위한 환경을 준비하고, 앞으로 소개할 예제들을 실행해 볼 수 있는 간단한 프로젝트를 만들어보겠습니다.

## 개발 환경

### Rust 컴파일러 설치

최신 언어 트렌드에 걸맞게 컴파일러 설치와 개발 환경 설정은 매우 간단합니다. `Rust` 언어의 개발 환경을 설치하기 위한 `rustup`이라는 설치 관리자가 있으며, <https://rustup.rs/>에 접속하면 `rustup` 툴을 설치하는 방법을 안내합니다.

사이트에 접속하시면 사용 중인 운영체제에 따라 운영체제별로 다른 메시지나 명령어를 보여줍니다. 리눅스/유닉스 계열이라면 다음과 같은 명령어를 안내할 것입니다.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

다음은 윈도우에서 `WSL`을 설치한 후 위 명령을 실행한 결과입니다.

```bash
gurugio@AL01945427:~$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/gurugio/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/gurugio/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/gurugio/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/gurugio/.profile
  /home/gurugio/.bashrc

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:

   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2024-02-08, rust version 1.76.0 (07dca489a 2024-02-04)
info: downloading component 'cargo'
  8.5 MiB /   8.5 MiB (100 %)   1.2 MiB/s in  7s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)   1.4 MiB/s in 12s ETA:  0s
info: downloading component 'rust-std'
 23.9 MiB /  23.9 MiB (100 %) 485.6 KiB/s in 41s ETA:  0s
info: downloading component 'rustc'
 62.3 MiB /  62.3 MiB (100 %)   2.6 MiB/s in 40s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)  10.4 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 23.9 MiB /  23.9 MiB (100 %)  18.0 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 62.3 MiB /  62.3 MiB (100 %)  19.6 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.76.0 (07dca489a 2024-02-04)

Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

중간에 설치 환경 설정을 디폴트로 진행할지, 아니면 사용자가 직접 수정할지 물어보는데 1번을 입력하여 디폴트 환경으로 설치를 진행했습니다. 디폴트 환경을 선택하면 `rustc` 같은 컴파일러와 `cargo`, `clippy`, `rustfmt` 등 개발에 필수적인 툴들을 자동으로 설치하고 환경 변수까지 설정해줍니다.

설치가 완료되면 아래와 같이 `.cargo/env` 파일이 생성됩니다.

```bash
gurugio@AL01945427:~$ cat .cargo/env
#!/bin/sh
# rustup shell setup
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"$HOME/.cargo/bin":*)
        ;;
    *)
        # Prepending path in case a system-installed rustc needs to be overridden
        export PATH="$HOME/.cargo/bin:$PATH"
        ;;
esac
```

이 파일은 실행 파일을 찾는 환경 변수 `PATH`에 `.cargo/bin`을 추가하는 역할을 합니다. `.cargo/bin` 디렉토리에는 `cargo`, `rustc` 등 `Rust` 개발에 필요한 실행 파일들이 저장되어 있습니다.

이제 `source .bashrc` 명령을 실행하면 새로운 환경 변수가 적용되어 `cargo` 등 실행 파일을 바로 사용할 수 있습니다.

```bash
gurugio@AL01945427:~$ source .bashrc
gurugio@AL01945427:~$ cargo --version
cargo 1.76.0 (c84b36747 2024-01-18)
gurugio@AL01945427:~$ rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
```

## Cargo 툴 소개

`Rust` 언어로 개발할 때 가장 자주 사용하게 되는 도구는 `cargo`입니다. 프로젝트 생성과 빌드, 외부 라이브러리 다운로드 등 프로젝트 관리 전반을 담당합니다. 도움말을 확인해 보면 `new`, `build`, `clean` 등 이름만 봐도 용도를 짐작할 수 있는 명령들이 가득합니다.

```bash
% cargo help
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]

Options:
  -V, --version             Print version info and exit
      --list                List installed commands
      --explain <CODE>      Run `rustc --explain CODE`
  -v, --verbose...          Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet               Do not print cargo log messages
      --color <WHEN>        Coloring: auto, always, never
  -C <DIRECTORY>            Change to DIRECTORY before doing anything (nightly-only)
      --frozen              Require Cargo.lock and cache are up to date
      --locked              Require Cargo.lock is up to date
      --offline             Run without accessing the network
      --config <KEY=VALUE>  Override a configuration value
  -Z <FLAG>                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                Print help

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.
```

가장 먼저 사용하게 될 명령어는 `cargo new`입니다. 새로운 프로젝트를 생성하는 명령입니다. `Rust`에서는 여러 개의 실행 파일과 라이브러리가 포함된 프로젝트 단위를 '패키지'라고 부릅니다. 그래서 `new` 명령의 설명에 "새로운 `cargo` 패키지를 만든다"는 내용이 있는 것입니다.
`new` 명령을 다음과 같이 실행하면 새로운 디렉토리와 하위 구조를 자동으로 생성해줍니다. `Rust` 언어의 패키지 및 프로젝트 관리에 대해서는 나중에 다시 자세히 설명하겠습니다.

```bash
% cargo new cargo-new-test
     Created binary (application) `cargo-new-test` package
% cd cargo-new-test
% ls
Cargo.toml  src
% ls -R
Cargo.toml  src

./src:
main.rs
```

`cargo new` 명령으로 `cargo-new-test` 디렉토리와 그 하위의 `src` 디렉토리가 생성됩니다. 또한 `Cargo.toml` 파일과 `src/main.rs` 파일이 함께 만들어집니다.

참고로 이미 존재하는 디렉토리를 `Rust` 프로젝트로 만들고 싶다면 `cargo init` 명령을 사용하면 됩니다. `Cargo.toml` 파일을 생성하고, 기존 파일 구조를 유지하면서 프로젝트를 초기화합니다.

`Cargo.toml`은 현재 디렉토리에 있는 `main.rs` 파일을 빌드하여 `cargo-init-test`라는 실행 파일을 만들도록 설정됩니다.

```bash
user@AL02279337 cargo-init_test % ls
main.rs
user@AL02279337 cargo-init_test % cargo init
     Created binary (application) package
user@AL02279337 cargo-init-test % ls
Cargo.toml main.rs
user@AL02279337 cargo-init-test % cat Cargo.toml
[package]
name = "cargo-init-test"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bin]]
name = "cargo-init-test"
path = "main.rs"
```

지금은 개발 환경이 제대로 설치되었는지 확인하는 단계이므로 `cargo`의 모든 명령어를 상세히 다루지는 않겠습니다. `apt`나 `yum` 같은 패키지 관리자를 써본 경험이 있다면, `cargo help` 메시지만 읽어봐도 충분히 이해할 수 있을 만큼 직관적이고 강력한 도구입니다. 또한 문법이 간단한 `toml` 포맷을 사용하므로 금방 적응하실 수 있을 것입니다.

## Hello, World! 구현하기

`cargo new` 명령을 사용해서 간단한 바이너리 파일을 빌드하기 위한 패키지를 생성해 보겠습니다. 다음은 `hello`라는 이름의 패키지를 만드는 방법입니다.

```bash
% cargo new hello
     Created binary (application) `hello` package
% cd hello
% ls
Cargo.toml  src
% ls -R
Cargo.toml  src

./src:
main.rs
```

생성된 패키지를 확인해 보면 최상위 디렉토리에 `src` 디렉토리와 `Cargo.toml` 파일이 보입니다. `src` 디렉토리 안에는 `main.rs` 파일이 들어 있습니다.

`Cargo.toml` 파일의 가장 중요한 역할은 패키지 이름과 버전 관리, 그리고 패키지에서 참조할 외부 라이브러리를 관리하는 것입니다. 파일을 한 번 열어 보겠습니다.

```bash
% cat Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

`package` 섹션의 `name`과 `version`은 말 그대로 패키지의 이름과 버전입니다. `edition` 항목은 `Rust` 언어의 명세 버전입니다. 이 책을 쓰는 시점에서 가장 최신 버전은 `2021`이며, 그 이전에는 `2018` 버전이 있었습니다. 지금 단계에서는 `cargo`가 기본으로 지정해주는 값을 그대로 사용하면 됩니다.

`dependencies` 섹션은 이 패키지에서 사용할 외부 라이브러리를 지정합니다. 이 책에서는 주로 표준 라이브러리만 사용하겠지만, 외부 라이브러리를 추가하고 싶다면 `cargo add` 명령을 사용하면 편리합니다. 아래는 `anyhow` 라이브러리를 추가한 예시입니다.

```bash
$ cargo add anyhow
    Updating crates.io index
      Adding anyhow v1.0.80 to dependencies.
             Features:
             + std
             - backtrace
    Updating crates.io index
$ cat Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.80"
```

`cargo add` 실행 시 `crates.io` 인덱스를 업데이트한다는 메시지가 나오는데, 이는 <https://crates.io/>라는 사이트에서 `anyhow` 라이브러리 정보를 가져오기 때문입니다. 해당 사이트에서 검색해 보면 최신 버전을 직접 확인할 수 있습니다.

개발 환경 설정이 모두 끝났으니 이제 `Hello, World!`를 출력해 보겠습니다. 다음과 같이 `main.rs`를 수정해 봅니다.

```rust
fn main() {
    println!("Hello, World!");
    println!("{}", "Hello, World again!");
    println!("{:x}", 65535);
}
```

```bash
% cargo run
   Compiling hello v0.1.0 (/Users/user/study/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.98s
     Running `target/debug/hello`
Hello, World!
Hello, World again!
ffff
```

`main` 함수는 프로그램의 시작점입니다. 위의 예제에서는 인자와 반환값이 없지만, 필요에 따라 추가할 수도 있습니다. `println!()`은 문자열을 출력하는 매크로 함수입니다. 함수 이름 끝에 `!`가 붙은 것은 일반 함수가 아닌 매크로로 정의되었음을 의미합니다. `println!`에서 `ln`은 `line`의 약자입니다. `C` 언어의 `printf`를 쓸 때 `\n`을 빠뜨리는 실수를 자주 하곤 하는데, `Rust`는 아예 줄 바꿈이 포함된 `println!`을 제공하여 그런 번거로움을 덜어줍니다. (물론 `!`를 빠뜨리는 새로운 실수를 하게 될 수도 있습니다.)

출력 포맷을 지정할 때는 `{}`를 사용하며, 이는 `Python`과 유사합니다. `{}` 사이에 형식을 지정하는 표시를 넣을 수도 있는데, `{:x}`는 16진수로 출력하라는 의미입니다.
