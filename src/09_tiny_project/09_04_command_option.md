## 커맨드 라인 옵션 연동

지금까지는 프로그램 실행 중에 일일이 질문에 답하며 입력을 받았습니다. 하지만 자동화된 시스템이나 숙련된 사용자에게는 매우 번거로운 방식입니다. 이번에는 실행 시점에 모든 정보를 한꺼번에 전달하는 커맨드 라인 옵션 기능을 추가해 보겠습니다.

### Clap 크레이트 활용

`Rust` 생태계의 표준과도 같은 커맨드 라인 파서인 `clap`을 사용합니다. 특히 우리 프로젝트는 제품마다 필요한 옵션이 다를 수 있으므로, 실행 시점에 옵션을 동적으로 구성할 수 있는 `clap`의 유연한 기능을 활용하겠습니다.

먼저 `Cargo.toml`에 기능을 추가합니다.
```toml
[dependencies]
clap = { version = "4.5", features = ["derive", "string"] }
```

### 트레이트 확장

각 플러그인이 자신에게 필요한 커맨드 라인 인자 정보를 제공하도록 트레이트를 수정합니다.

```rust
trait GenSerialData {
    // ... 기존 메서드 생략 ...

    fn get_arg_name(&self) -> &str; // 옵션 이름 (예: "userid")
    fn get_help(&self) -> String;   // 도움말 메시지
    fn get_mandatory(&self) -> bool; // 필수 여부
}
```

이제 각 구조체는 다음과 같이 정보를 제공합니다.
```rust
impl GenSerialData for CustomerID {
    fn get_arg_name(&self) -> &str { "customerid" }
    fn get_help(&self) -> String { format!("고객 ID ({}자리)", self.digit) }
    fn get_mandatory(&self) -> bool { true }
}
```

### 동적 옵션 생성 로직

`main` 함수에서는 등록된 플러그인 리스트를 순회하며 `clap`의 옵션을 자동으로 구성합니다.

```rust
use clap::{Arg, Command};

fn main() {
    let mut items: Vec<Box<dyn GenSerialData>> = vec![ /* 플러그인들 */ ];

    let mut cmd = Command::new("serial")
        .version("1.0")
        .about("시리얼 번호 생성기");

    // 플러그인 정보를 바탕으로 옵션 자동 등록
    for item in items.iter() {
        cmd = cmd.arg(
            Arg::new(item.get_name())
                .long(item.get_arg_name())
                .help(item.get_help())
                .required(item.get_mandatory()),
        );
    }

    let matches = cmd.get_matches();

    // 입력된 값들을 플러그인에 전달
    for item in items.iter_mut() {
        if let Some(val) = matches.get_one::<String>(item.get_name()) {
            item.put_rawdata(val.clone());
        }
    }
}
```

이제 사용자는 다음과 같이 편리하게 프로그램을 실행할 수 있습니다.
```bash
$ ./serial --customerid 1234 --productid qwerasdf
```

여기서 `features = ["string"]` 옵션을 쓴 이유는 라이프타임(`Lifetime`) 문제 때문입니다. `Arg` 객체가 플러그인이 가진 문자열의 참조를 오래 들고 있으면 메모리 안전성 문제가 생길 수 있는데, 소유권이 있는 `String`을 넘겨줌으로써 이를 해결합니다. `Rust` 컴파일러는 이런 세세한 부분까지 꼼꼼하게 검사하여 런타임 에러를 원천 차단합니다.

### 연습문제

1.  특정 옵션을 누락했을 때 `clap`이 자동으로 보여주는 도움말 메시지를 확인해 보세요.
2.  단축 옵션(예: `-c` 대신 `--customerid`)도 지원하도록 `GenSerialData` 트레이트와 메인 로직을 개선해 보세요.
