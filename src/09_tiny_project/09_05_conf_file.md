## 설정 파일

지금까지는 main함수에서 모든 입력 데이터에 필요한 객체를 생성해놓고, 모든 입력 데이터를 옵션으로 만들어서 사용했습니다.

```rust
fn main() {
    let productid = ProductID::new(8);
    let customerid = CustomerID::new(4);
    let customertype = CustomerType::new();
    let expiredate = expiredate::ExpireDate::new();
    let mut items: Vec<Box<dyn GenSerialData>> = vec![
        Box::new(customerid),
        Box::new(productid),
        Box::new(customertype),
        Box::new(expiredate),
    ];
    ......생략
```

만약에 모든 입력 데이터가 필요한게 아니라 그 중에 일부만 필요하다면 필요한 객체 생성만 놔두고, 나머지는 코드에서 제거해야됩니다.
이렇게 옵션이 하드코딩되면, 제품마다 별도의 시리얼 코드 생성 프로그램을 만들 수 없게됩니다.

프로그램을 동적으로 설정하는 방법 중에서 가장 흔한 방법이 설정 파일을 이용하는 방법일 것입니다.
이번 장에서는 설정 파일을 만들어서 사용하는 코드를 추가해보겠습니다.

일단 설정 파일부터 만들어보겠습니다.
설정 파일이라고 하면 그냥 필요한 텍스트를 넣어놓고 직접 파싱해서 처리하는 방법도 있겠습니다만 보통은 YAML, TOML, INI, XML 등 이미 널리 사용되는 포맷으로 설정 파일을 만드는 것이 더 효율적입니다.
이미 좋은 파서들이 많이 나와있기 때문입니다.
최근에 제 주변에서 가장 많이 사용되는 설정 파일은 JSON같습니다. 설정 파일 뿐 아니라 통신데이터를 주고받을 때도 많이 사용되므로 JSON 포맷을 사용해보겠습니다.

일단 입력 데이터를 지정하는 설정 파일 serial.conf 파일을 다음과 같이 만들어보겠습니다.

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

옵션으로 만들기 위해서 필요한 특성들이 이름, 자리수, 필수 옵션 등 3가지 이므로 위와 같이 배열로 만들 수 있겠습니다.

그리고 이와같이 JSON 포맷으로 만들어진 파일을 파싱해서 읽어오는 크레이트를 찾아야합니다만, 사실 러스트 언어에서 JSON 파일 처리에는 Serde가 사실상 표준과도 같기 때문에 따로 조사할 필요없이 Serde를 사용하겠습니다. Serde는 직렬화와 역직렬화(SERializing 와 DEserializing)의 영문 약자입니다. 특정한 포맷으로 써진 파일이나 메모리 데이터를 데이터 구조로 읽어오거나, 반대로 특정 데이터 구조 타입의 객체를 파일이나 메모리에 써넣는 일을 합니다. 혹시 다른 언어로 직렬화 처리를 만들어본 적이 없다면 참고 링크를 보시고, 어떤 일을 하는 것인지를 잘 파악해놓으시기 바랍니다. 아주 다양한 분야의 프로그램에서 널리 사용되므로 실무에서 반드시 사용할 기회가 있을 것입니다.

참고 링크
* https://serde.rs/
* https://crates.io/crates/serde

Serde 크레이트를 사용할 때는 Serde 크레이트만 사용하는게 아니라, 항상 Serde 크레이트와 함께 포맷을 처리하는 별도의 크레이트를 같이 사용해야합니다. 우리와 같이 JSON 포맷을 사용하게된다면 Serde 크레이트와 serde_json 크레이트를 같이 사용하는 식입니다.

cargo add 명령을 사용하거나, 직접 Cargo.toml 파일을 수정해서 다음과 같이 serde와 serde_json 크레이트를 Cargo.toml에 추가합니다.
```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

derive라는 기능은 이전에 clap 크레이트의 예제 코드에서 본것과 유사항 기능입니다. 우리가 사용한 데이터 구조에 직렬화/역직렬화를 실행하는 코드를 자동으로 만들어줍니다.

그리고 main.rs 파일에 다음과 같이 SerialData 구조체를 추가합니다.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SerialData {
    name: String,
    digit: usize,
    mandatory: bool,
}
```

derive 구문으로 Deserialize 기능을 직접 구현하지 않아도 자동으로 구현됩니다. 우리가 만든 설정 파일은 각 항목이 "name", "digit", "mandatory" 3개의 하위 필드를 가지고 있습니다. SerialData 구조체도 동일하게 3개의 필드를 가집니다. 그래서 각 항목마다 SerialData 객체로 읽어올 수가 있습니다.

자료 구조가 준비되었으니 이제는 serial.conf를 파일을 읽고 객체를 생성하는 코드를 만들겠습니다.
다음 코드를 main.rs에 추가합니다. 

```rust
use std::fs::File;
use std::io::Read;
use std::io::{stdin, stdout, Write};

fn parse_config() -> Vec<SerialData> {
    // Open the configuration file
    let mut file = match File::open("code/serial_project_step5/serial.conf") {
        Ok(file) => file,
        Err(_) => {
            panic!("Error: Could not find or open 'serial.conf'. Please ensure the file exists.");
        }
    };

    // Read the file contents into a string
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("serial.conf is found"),
        Err(_) => panic!("Failed to read serial.conf file"),
    }

    // Parse the JSON contents into a vector of SerialData structs
    serde_json::from_str(&contents).expect("JSON was not well-formatted")
}
```

std::fs::File의 open 메소드를 사용해서 파일을 열고, read_to_string 메소드로 전체 파일의 내용을 문자열로 읽습니다. 그리고 마지막으로 serde_json의 from_str 메소드를 사용해서 메모리에 있는 문자열을 SerialData타입 객체의 배열로 역직렬화를 실행합니다.

마지막으로 main함수에서 parse_config 함수를 호출하고, 설정 파일에 지정된 옵션만 객체로 생성하면 설정 파일 처리가 완료됩니다.

```rust
fn main() {
    let configs = parse_config();
    let mut items: Vec<Box<dyn GenSerialData>> = Vec::new();
    for config in configs.into_iter() {
        if config.name == "customerid" {
            items.push(Box::new(CustomerID::new(config.digit, config.mandatory)));
        } else if config.name == "productid" {
            items.push(Box::new(ProductID::new(config.digit, config.mandatory)));
        } else if config.name == "customertype" {
            items.push(Box::new(CustomerType::new(config.digit, config.mandatory)));
        } else if config.name == "expiredate" {
            items.push(Box::new(ExpireDate::new(config.digit, config.mandatory)));
        } else {
            panic!("Unexpected config name: {}", config.name);
        }
    }
  ......
```

각 제품마다 별도의 serial.conf 파일을 만들어주면, 하나의 프로그램이 각 제품의 시리얼 코드를 만들 수 있습니다.

### 연습문제

1. 지금 main함수 안에 암호화와 복호화 코드가 같이 있습니다. 이것을 2개의 프로그램으로 분리해서 암호화를 실행해서 시리얼 코드를 만드는 프로그램과, 시리얼 코드를 읽어서 시리얼 코드가 옳바른지 검증하는 프로그램으로 만들어보세요.