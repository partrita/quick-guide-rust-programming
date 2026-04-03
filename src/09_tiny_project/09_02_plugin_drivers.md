## 플러그인 구조 설계

요구사항은 언제나 바뀝니다. 고객 ID가 8자리로 늘어날 수도 있고, 제품마다 각기 다른 추가 정보가 필요할 수도 있습니다. 매번 코드를 수정해 다시 빌드하는 대신, 필요에 따라 기능을 끼워 넣는 플러그인 (`Plugin`) 혹은 드라이버 (`Driver`) 구조를 만들어 보겠습니다.

플러그인 설계의 핵심은 표준 인터페이스입니다. 마치 모든 전자기기가 정해진 규격의 플러그를 통해 전기를 공급받는 것처럼, 우리 프로그램도 정해진 규칙(트레이트)을 따르는 데이터라면 무엇이든 처리할 수 있어야 합니다.

### 트레이트 정의: GenSerialData

시리얼 번호에 포함될 모든 데이터 타입이 가져야 할 공통 동작을 정의합니다.

```rust
trait GenSerialData {
    // 사용자로부터 데이터를 입력받는 기본 로직
    fn get_input_from_user(&mut self) {
        println!("Please input {}-digits for {}: ", self.get_length(), self.get_name());
        let input = get_user_input();
        assert_eq!(input.len(), self.get_length(), "길이가 일치하지 않습니다.");
        self.put_rawdata(input);
    }

    // 데이터 검증 로직
    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    // 아래 메서드들은 각 구조체에서 반드시 구현해야 합니다.
    fn get_length(&self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, data: String);
}
```

트레이트 안에서 `get_input_from_user`와 `verify`를 미리 구현해 두었습니다. 이를 디폴트 구현 (`Default Implementation`)이라고 합니다. 이렇게 하면 각 플러그인(구조체)은 핵심 데이터 처리에만 집중하면 되고, 공통 로직은 트레이트에 맡길 수 있습니다.

### 플러그인 구현: CustomerID와 ProductID

이제 `GenSerialData`를 구현하는 구체적인 구조체들을 만듭니다.

```rust
pub struct CustomerID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl GenSerialData for CustomerID {
    fn get_length(&self) -> usize { self.digit }
    fn get_rawdata(&self) -> String { self.id.clone().unwrap() }
    fn get_name(&self) -> String { self.name.clone() }
    fn put_rawdata(&mut self, data: String) { self.id = Some(data); }
}
```

`ProductID` 구조체도 동일한 방식으로 구현할 수 있습니다. 각 구조체가 어떤 필드를 가졌는지는 중요하지 않습니다. 오직 트레이트가 요구하는 인터페이스만 충족하면 됩니다.

### 메인 루프: 트레이트 객체 활용

이제 `main` 함수에서는 이 객체들을 `Box<dyn GenSerialData>`에 담아 한꺼번에 관리합니다.

```rust
fn main() {
    let mut items: Vec<Box<dyn GenSerialData>> = vec![
        Box::new(CustomerID::new(4)),
        Box::new(ProductID::new(8)),
    ];

    // 모든 아이템에 대해 사용자 입력 받기
    for item in items.iter_mut() {
        item.get_input_from_user();
    }

    // 데이터 취합 및 시리얼 생성
    let mut plain_serial = String::new();
    for item in items.iter() {
        plain_serial.push_str(&item.get_rawdata());
    }
    // ... 이후 암호화 로직 ...
}
```

이것이 바로 프레임워크와 플러그인의 관계입니다. 메인 프로그램(프레임워크)은 각 아이템의 구체적인 타입을 알 필요가 없습니다. 그저 "시리얼 데이터를 생성할 줄 아는 놈들"이라는 사실만 믿고 일을 시킵니다. 덕분에 나중에 새로운 데이터 타입이 추가되어도 메인 로직은 전혀 수정할 필요가 없습니다.

### 연습문제

1.  `CustomerID` 외에 시리얼 번호에 넣고 싶은 또 다른 정보를 상상해 보고, 직접 구조체와 트레이트 구현을 시도해 보세요.
2.  트레이트의 디폴트 구현을 특정 구조체에서만 다르게 동작하게 만들려면 어떻게 해야 할까요? (힌트: 오버라이딩)
