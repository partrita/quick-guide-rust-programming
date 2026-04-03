## 설정 파일을 통한 동적 구성

지금까지는 `main` 함수에서 사용할 플러그인들을 직접 코드로 작성(하드코딩)했습니다. 하지만 제품군이 수백 개로 늘어난다면 어떨까요? 매번 코드를 수정하고 다시 빌드하는 것은 불가능에 가깝습니다.

이번 장에서는 설정 파일을 읽어와서 프로그램의 동작을 실행 시점에 결정하는 구조를 만들어 보겠습니다.

### JSON 설정 파일 (serial.conf)

가장 널리 쓰이는 `JSON` 포맷으로 규칙을 정의해 봅시다.

```json
[
    { "name": "customerid", "digit": 4, "mandatory": true },
    { "name": "productid", "digit": 8, "mandatory": true },
    { "name": "expiredate", "digit": 8, "mandatory": false }
]
```

### Serde를 이용한 파싱

`Rust`의 사실상 표준 직렬화 라이브러리인 `serde`를 사용합니다.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

설정 데이터를 담을 구조체를 정의합니다.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigEntry {
    name: String,
    digit: usize,
    mandatory: bool,
}
```

이제 설정 파일을 읽어 실제 플러그인 객체를 생성하는 팩토리 패턴 (`Factory Pattern`)을 적용합니다.

```rust
fn load_configs() -> Vec<Box<dyn GenSerialData>> {
    let content = std::fs::read_to_string("serial.conf").expect("설정 파일 읽기 실패");
    let configs: Vec<ConfigEntry> = serde_json::from_str(&content).expect("파싱 실패");

    let mut items = Vec::new();
    for cfg in configs {
        match cfg.name.as_str() {
            "customerid" => items.push(Box::new(CustomerID::new(cfg.digit, cfg.mandatory)) as Box<dyn GenSerialData>),
            "productid" => items.push(Box::new(ProductID::new(cfg.digit, cfg.mandatory)) as Box<dyn GenSerialData>),
            "expiredate" => items.push(Box::new(ExpireDate::new()) as Box<dyn GenSerialData>),
            _ => println!("경고: 알 수 없는 플러그인 {}", cfg.name),
        }
    }
    items
}
```

### 최종 진화: 데이터 기반 설계

이제 우리 프로그램은 단순한 도구가 아니라 하나의 플랫폼이 되었습니다.
1.  새로운 데이터 타입이 필요하면 구조체를 만들고 트레이트를 구현합니다. (플러그인 추가)
2.  어떤 데이터로 시리얼을 만들지는 `serial.conf` 파일만 수정하면 됩니다. (동적 설정)

이렇게 데이터가 프로그램의 동작을 결정하는 방식을 데이터 기반 설계 (`Data-driven Design`)라고 합니다. 로직과 데이터를 분리함으로써, 개발자는 핵심 엔진 개발에 집중하고 운영팀은 설정 파일만으로 제품을 관리할 수 있는 이상적인 구조가 완성되었습니다.

### 연습문제

1.  설정 파일의 경로를 명령행 인자로 받도록 수정해 보세요. (예: `--config my_product.json`)
2.  현재는 `match` 문에서 모든 타입을 일일이 나열하고 있습니다. 이를 더 자동화할 방법이 있을까요? (참고: 플러그인 레지스트리 개념)
