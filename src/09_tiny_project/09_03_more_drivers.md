## 추가 플러그인 구현

우리가 만든 플러그인 구조가 얼마나 유연한지 확인하기 위해, 이전과는 전혀 다른 형태의 데이터 두 가지를 더 추가해 보겠습니다.

### 1. 고객 유형 (CustomerType)

구매 고객의 성격(개인, 학생, 기업 등)을 시리얼 번호에 담고 싶습니다. 단순히 숫자만 받는 게 아니라, 내부적으로 열거형(`Enum`)을 사용해 관리해 보겠습니다.

```rust
#[derive(Clone, Debug)]
enum CustomerKind { Business, Student, Company }

impl From<&CustomerKind> for usize {
    fn from(item: &CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Student => 2,
            CustomerKind::Company => 3,
        }
    }
}

pub struct CustomerType {
    customer_type: Option<CustomerKind>,
    digit: usize,
    name: String,
}
```

`CustomerType`은 트레이트의 `get_input_from_user` 디폴트 구현을 그대로 쓸 수 없습니다. 사용자에게 어떤 번호가 어떤 유형인지 설명해 주어야 하기 때문입니다.

```rust
impl GenSerialData for CustomerType {
    fn get_input_from_user(&mut self) {
        println!("고객 유형을 입력하세요 (1: 개인, 2: 학생, 3: 기업): ");
        let input = get_user_input();
        self.put_rawdata(input);
    }

    // ... 다른 메서드 생략 ...
}
```

이렇게 트레이트의 디폴트 로직이 내 상황에 맞지 않을 때는 직접 구현하여 오버라이딩(`Overriding`)하면 됩니다.

### 2. 사용 기한 (ExpireDate)

제품의 만료 날짜를 'YYYYMMDD' 형식의 8자리 숫자로 포함합니다. 내부적으로는 연, 월, 일 데이터를 정수형으로 쪼개서 관리하고 싶습니다.

```rust
pub struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
    name: String,
}

impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("만료 날짜를 입력하세요 (YYYYMMDD): ");
        let raw = get_user_input();
        // 파싱 및 유효성 검사 로직
        self.year = raw[0..4].parse().expect("연도 오류");
        self.month = raw[4..6].parse().expect("월 오류");
        self.day = raw[6..8].parse().expect("일 오류");
    }

    fn get_rawdata(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }
    // ... 나머지 구현 ...
}
```

`ExpireDate`는 내부적으로 문자열이 아닌 숫자 데이터를 다루지만, 최종적으로는 트레이트 인터페이스에 맞춰 문자열을 반환합니다. `verify` 메서드 또한 날짜 논리에 맞춰 재정의할 수 있습니다.

### 트레이트의 위력

새로운 데이터 타입을 추가하면서 메인 프로그램의 로직을 고쳤나요? 아니요, 그저 새로운 구조체를 만들고 `GenSerialData` 트레이트를 입혀준 뒤, 벡터에 `Box::new()`로 추가했을 뿐입니다.

이것이 바로 객체지향의 다형성(`Polymorphism`)이 주는 강력한 혜택입니다. 표준 인터페이스만 잘 정의해 두면, 새로운 기능을 추가하는 작업은 마치 레고 블록을 조립하는 것처럼 즐거운 일이 됩니다.

### 연습문제

1.  `get_rawdata`에서 소유권 문제로 고생해 본 적이 있나요? `clone()`을 쓰지 않고 참조만 넘기는 방식으로 설계를 바꿀 수 있을지 고민해 보세요.
2.  `Option` 타입을 사용해 아직 데이터가 입력되지 않은 상태를 안전하게 처리하는 로직을 보강해 보세요.
