## 설정 파일

지금까지는 `main` 함수에서 모든 입력 데이터 객체를 직접 생성하고, 이를 옵션으로 만들어 사용했습니다.

```rust,ignore
    let customerid = CustomerID::new(4);
    let customertype = CustomerType::new();
    let expiredate = expiredate::ExpireDate::new();
    let mut items: Vec<Box<dyn GenSerialData>> = vec![
        Box::new(customerid),
        Box::new(productid),
        Box::new(customertype),
        Box::new(expiredate),
    ];
```

만약 모든 입력 데이터가 필요한 것이 아니라 일부만 사용하고 싶다면, 코드에서 직접 객체 생성 부분을 수정해야 합니다. 이렇게 설정이 코드 내부에 하드코딩되어 있으면, 제품마다 다른 시리얼 생성 규칙을 적용하기 위해 매번 코드를 수정하고 다시 빌드해야 하는 번거로움이 생깁니다.

프로그램을 동적으로 설정하는 가장 일반적인 방법은 설정 파일을 활용하는 것입니다. 이번 장에서는 설정 파일을 통해 프로그램을 제어하는 기능을 추가해 보겠습니다.

### JSON 설정 파일 구성

설정 파일 포맷으로는 `YAML`, `TOML`, `INI`, `XML` 등 다양한 방식이 있지만, 여기서는 데이터 교환에 널리 쓰이는 `JSON` 포맷을 사용하겠습니다. 이미 검증된 파서들이 많아 효율적으로 처리할 수 있기 때문입니다.

먼저 입력 데이터를 정의할 `serial.conf` 파일을 다음과 같이 작성합니다.

```json
[
    {
        "name": "customerid",
        "digit": 4,
        "mandatory": true
    },
    {
        "name": "productid",
        "digit": 8,
        "mandatory": true
    },
    {
        "name": "expiredate",
        "digit": 8,
        "mandatory": true
    },
    {
        "name": "customertype",
        "digit": 1,
        "mandatory": true
    }
]
```

각 옵션의 이름, 자릿수, 필수 여부를 배열 형태로 정의했습니다.

### Serde를 이용한 파싱

`Rust`에서 `JSON` 파일을 처리할 때는 사실상 표준 라이브러리처럼 사용되는 `Serde` 크레이트를 사용합니다. `Serde`는 직렬화(Serializer)와 역직렬화(Deserializer)의 약자로, 파일이나 메모리의 데이터를 데이터 구조로 읽어오거나 그 반대의 작업을 수행합니다.

`Serde`를 사용하려면 `serde` 크레이트와 특정 포맷을 처리할 `serde_json` 크레이트가 모두 필요합니다. `Cargo.toml`에 다음과 같이 추가합니다.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

이제 `main.rs`에 설정 데이터를 담을 `SerialData` 구조체를 정의합니다.

```rust,ignore
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SerialData {
    name: String,
    digit: usize,
    mandatory: bool,
}
```

`#[derive(Deserialize)]` 구문을 통해 복잡한 역직렬화 로직을 직접 구현하지 않아도 `JSON` 데이터를 구조체로 바로 읽어올 수 있습니다. 이제 파일을 읽고 객체를 생성하는 `parse_config` 함수를 작성합니다.

```rust,ignore
use std::fs::File;
use std::io::Read;

fn parse_config() -> Vec<SerialData> {
    // 설정 파일 열기
    let mut file = match File::open("serial.conf") {
        Ok(file) => file,
        Err(_) => {
            panic!("Error: 'serial.conf' 파일을 열 수 없습니다. 파일이 존재하는지 확인하세요.");
        }
    };

    // 파일 내용을 문자열로 읽기
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("파일 읽기 실패");

    // JSON 문자열을 SerialData 구조체 벡터로 파싱
    serde_json::from_str(&contents).expect("JSON 포맷이 올바르지 않습니다")
}
```

`std::fs::File::open`으로 파일을 열고 `read_to_string`으로 내용을 읽은 뒤, `serde_json::from_str`를 호출하여 `SerialData` 타입의 벡터로 변환합니다.

마지막으로 `main` 함수에서 이 설정을 바탕으로 객체를 생성하도록 수정합니다.

```rust,ignore
fn main() {
    let configs = parse_config();
    let mut items: Vec<Box<dyn GenSerialData>> = Vec::new();

    for config in configs.into_iter() {
        match config.name.as_str() {
            "customerid" => items.push(Box::new(CustomerID::new(config.digit, config.mandatory))),
            "productid" => items.push(Box::new(ProductID::new(config.digit, config.mandatory))),
            "customertype" => items.push(Box::new(CustomerType::new(config.digit, config.mandatory))),
            "expiredate" => items.push(Box::new(ExpireDate::new(config.digit, config.mandatory))),
            _ => panic!("알 수 없는 설정 이름: {}", config.name),
        }
    }
    // ... 이후 로직 생략
}
```

이제 `serial.conf` 파일 내용만 바꾸면 코드 수정 없이도 시리얼 생성 규칙을 동적으로 변경할 수 있습니다. 각 제품마다 별도의 설정 파일을 제공하면 하나의 프로그램으로 여러 제품의 시리얼 키를 생성할 수 있어 유지보수가 매우 편리해집니다.

### 연습문제

1. 현재 `main` 함수에는 암호화와 복호화 로직이 함께 들어있습니다. 이를 시리얼 코드를 생성하는 프로그램과, 생성된 코드를 검증하는 프로그램으로 각각 분리해 보세요.
