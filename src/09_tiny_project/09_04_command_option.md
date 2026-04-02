## 커맨드 라인 옵션

표준 인터페이스와 플러그인을 만드는 작업은 지금까지 순조롭게 진행되었습니다. 하지만 실제 실무에서 쓰이기에는 아직 부족한 면이 있습니다. 사용자에게 일일이 질문하여 입력을 받는 방식은 친절하긴 하지만, 시리얼 번호를 자주 생성해야 하는 개발자나 운영팀 입장에서는 다소 번거로울 수 있습니다. 실행 시점에 필요한 정보를 한꺼번에 전달하여 빠르게 결과를 얻는 방식이 더 효율적입니다. 그래서 이번에는 커맨드 라인 옵션을 통해 입력을 받도록 수정해 보겠습니다.

대략 다음과 같은 방식으로 동작하게 만들 것입니다.

```bash
$ serial --help
# 모든 입력 데이터에 대한 도움말 메시지 출력
$ serial generate --userid 1234 --productid qwerasdf
# 시리얼 번호 생성
```

### Clap 크레이트 사용법

암호화 기능을 위해 `MagicCrypt`를 조사했던 것처럼, 커맨드 라인 인자 처리를 위한 크레이트를 찾아볼 수 있습니다. 러스트 생태계에서 가장 널리 사용되는 도구는 `Clap`입니다.

먼저 `Clap`의 공식 문서(https://docs.rs/clap/latest/clap/) 예제 코드를 보며 기본 사용법을 알아보겠습니다. 테스트를 위해 `ex-clap`이라는 새로운 프로젝트를 생성하고 `clap` 크레이트를 추가해 봅시다. 이때 `derive` 기능을 활성화해야 합니다.

```bash
$ cargo new ex-clap
$ cd ex-clap
$ cargo add clap --features derive
```

그리고 다음 코드를 `main.rs`에 작성합니다.

```rust,ignore
use clap::Parser;

/// 간단한 인사 프로그램
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 인사할 대상의 이름
    #[arg(short, long)]
    name: String,

    /// 인사 반복 횟수
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

`Args` 구조체 위의 `#[derive(Parser, Debug)]` 구문은 커맨드 라인 옵션을 처리하는 파서를 자동으로 생성하도록 지시합니다. `main` 함수에서는 복잡한 파싱 로직 없이 `Args::parse()`만 호출하면 됩니다. `#[arg(short, long)]` 속성은 필드 이름을 기반으로 `-n`, `--name` 같은 옵션을 자동으로 만들어 줍니다. 각 필드 위의 주석은 도움말 메시지로 활용됩니다.

예제를 실행하여 어떻게 동작하는지 확인해 봅시다.

```bash
$ cargo run -- --help
Simple program to greet a person

Usage: ex-clap [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

필수 인자인 `--name`을 누락하면 에러 메시지가 출력되며, `--help` 옵션을 통해 사용 가능한 옵션과 설명을 확인할 수 있습니다.

### Clap의 유연한 사용법

우리가 만드는 시리얼 프로그램은 항상 동일한 옵션을 가지지 않습니다. 제품마다 필요한 입력 데이터가 다를 수 있기 때문입니다. 따라서 `Args` 구조체를 고정하는 방식보다는, 실행 시점에 옵션을 동적으로 구성할 수 있는 방식이 더 적합합니다. `Clap`은 이를 위해 `Arg`와 `Command`라는 도구를 제공합니다.

```rust,ignore
use clap::{Arg, Command};

fn main() {
    let mut command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator");

    // "NAME" 옵션 정의
    let arg_name = Arg::new("NAME")
        .long("name")
        .short('n')
        .help("Name of the person to greet")
        .required(true);
    
    command = command.arg(arg_name);

    // "COUNT" 옵션 정의
    let arg_count = Arg::new("COUNT")
        .long("count")
        .short('c')
        .help("Number of times to greet")
        .default_value("1");

    command = command.arg(arg_count);

    let matches = command.get_matches();

    if let Some(name) = matches.get_one::<String>("NAME") {
        if let Some(count_str) = matches.get_one::<String>("COUNT") {
            let count: usize = count_str.parse().unwrap_or(1);
            for _ in 0..count {
                println!("Hello {}!", name);
            }
        }
    }
}
```

`Command::new`로 프로그램을 정의하고, `Arg::new`로 개별 옵션을 생성한 뒤 `command.arg()`로 추가합니다. 최종적으로 `get_matches()`를 호출하여 입력된 인자들을 파싱합니다. 이 방식은 코드 실행 중에 옵션을 동적으로 추가할 수 있어 우리 프로젝트에 더 적합합니다.

### 시리얼 프로젝트에 Clap 적용하기

이제 우리 프로젝트에 `Clap`을 적용해 봅시다. 먼저 `Cargo.toml`에 `string` 기능을 추가합니다.

```toml
[dependencies]
clap = { version = "4.5", features = ["string"] }
```

그리고 `GenSerialData` 트레이트에 커맨드 라인 인자 처리를 위한 메서드들을 추가합니다.

```rust
trait GenSerialData {
    // ... 기존 메서드들 생략

    // 명령행 인자 처리를 위해 추가된 메서드들
    fn get_arg_name(&self) -> &str;
    fn get_help(&self) -> String;
    fn get_mandatory(&self) -> bool;
}
```

`CustomerID` 구조체에서의 구현 예시는 다음과 같습니다.

```rust
impl GenSerialData for CustomerID {
    // ... 생략
    fn get_arg_name(&self) -> &str { "customerid" }
    fn get_help(&self) -> String { format!("Customer ID ({} digits)", self.digit) }
    fn get_mandatory(&self) -> bool { true }
}
```

`main` 함수에서는 등록된 모든 `items`를 순회하며 `Clap`의 `Arg`를 생성하고 추가합니다.

```rust,ignore
fn main() {
    // ... 아이템 초기화 생략

    let mut command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator");

    for item in items.iter() {
        command = command.arg(
            Arg::new(item.get_name().to_owned())
                .long(item.get_arg_name().to_owned())
                .help(item.get_help())
                .required(item.get_mandatory()),
        );
    }

    let matches = command.get_matches();

    for item in items.iter_mut() {
        if let Some(data) = matches.get_one::<String>(item.get_name()) {
            item.put_rawdata(data);
        }
    }
    // ... 나머지 로직 생략
}
```

여기서 `to_owned()`를 사용하여 `String` 객체를 전달하는 이유는 **라이프타임(Lifetime)** 문제 때문입니다. `Arg` 객체가 다른 객체(`item`)가 가진 데이터의 참조(&str)를 가지고 있으면, `item`이 메모리에서 해제된 후에도 `Arg`가 해당 메모리에 접근하려 할 위험이 있습니다. `Rust` 컴파일러는 이러한 위험을 방지하기 위해 참조 관계에 있는 객체들의 수명을 엄격히 검사합니다. `Arg`에 소유권이 있는 `String`을 넘겨줌으로써 이러한 의존성 문제를 해결할 수 있습니다. `Clap`의 `features = ["string"]` 옵션은 바로 이런 상황에서 편리하게 사용할 수 있도록 지원되는 기능입니다.

### 연습문제

1. `CustomerID` 외의 다른 데이터 타입들(`ProductID`, `CustomerType`, `ExpireDate`)에도 `get_arg_name`, `get_help`, `get_mandatory` 메서드를 구현해 보세요.

2. `Cargo.toml`에서 `features = ["string"]` 옵션을 제거하고 코드를 수정해 보세요. 어떤 라이프타임 에러가 발생하는지 확인하고, 이를 해결하기 위한 다른 방법이 있는지 시도해 보시기 바랍니다.
