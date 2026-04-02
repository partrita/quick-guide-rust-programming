## 추가 플러그인

### CustomerType 추가

플러그인은 서로 다른 형태의 데이터를 동일한 인터페이스로 추가할 수 있도록 높은 호환성을 가져야 합니다. 지금까지 우리가 만든 플러그인은 사실상 이름만 다르고 데이터 구조가 동일한 두 개의 타입(`CustomerID`, `ProductID`)에만 적용되었습니다. 정말로 다른 형태의 데이터 구조에도 사용할 수 있는지 확인해 보고, 만약 부족한 점이 있다면 보완하기 위해 두 가지 새로운 플러그인을 추가해 보겠습니다.

첫 번째로 추가할 입력 데이터는 고객 유형입니다. 제품을 판매하다 보면 일반 구매 고객, 학생용 무료 버전을 사용하는 고객, 그리고 회사 단체 구매 고객 등이 있을 수 있습니다. 이 세 가지 유형을 시리얼 번호에 포함해 보겠습니다.

먼저 세 가지 고객 유형을 표현할 수 있는 `enum` 타입을 정의합니다.

```rust
#[derive(Clone, Debug)]
enum CustomerKind {
    Business,
    Student,
    Company,
}

impl From<CustomerKind> for usize {
    fn from(item: CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1, // 개인 구매
            CustomerKind::Student => 2,  // 학생 무료 버전
            CustomerKind::Company => 3,  // 회사 단체 구매
        }
    }
}
```

`CustomerKind`라는 `enum` 타입을 만들고, 이를 `usize` 타입의 정수로 변환할 수 있도록 `From` 트레이트를 구현했습니다. 프로그램 사용자에게 1, 2, 3 중 하나의 숫자를 입력받아 이를 고객 유형으로 저장해야 하므로, 정수와 `CustomerKind` 사이의 변환 로직이 필요합니다.

다음으로 고객 유형을 나타내는 `CustomerType` 구조체를 정의합니다.

```rust
pub struct CustomerType {
    customer_type: Option<CustomerKind>,
    digit: usize,
    name: String,
}

impl CustomerType {
    pub fn new() -> Self {
        CustomerType {
            name: "CustomerType".to_owned(),
            digit: 1,
            customer_type: None,
        }
    }
}
```

필요한 숫자가 1, 2, 3이므로 한 자리 수면 충분하며, `digit` 필드는 1로 초기화합니다. 이제 `GenSerialData` 트레이트를 구현해 보겠습니다.

```rust
impl GenSerialData for CustomerType {
    fn get_input_from_user(&mut self) {
        println!("Please input customer type: ");
        print!(
            "{}-{:?}, ",
            usize::from(CustomerKind::Business),
            CustomerKind::Business
        );
        print!(
            "{}-{:?}, ",
            usize::from(CustomerKind::Student),
            CustomerKind::Student
        );
        println!(
            "{}-{:?}",
            usize::from(CustomerKind::Company),
            CustomerKind::Company
        );
        let input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        if let Some(kind) = &self.customer_type {
            return format!("{}", usize::from((*kind).clone()));
        } else {
            return "0".to_owned();
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        let kind = match data.as_str() {
            "1" => CustomerKind::Business,
            "2" => CustomerKind::Student,
            "3" => CustomerKind::Company,
            _ => CustomerKind::Business,
        };
        self.customer_type = Some(kind);
    }
}
```

가장 큰 변화는 `get_input_from_user` 메서드를 디폴트 구현 대신 직접 구현한 점입니다. 디폴트 구현은 자릿수와 이름만 출력하지만, `CustomerType`은 각 고객 유형에 대한 안내가 필요하기 때문입니다.

`get_rawdata` 메서드 구현에서 한 가지 눈여겨볼 점은 `From` 트레이트의 `from` 메서드를 활용하는 방식입니다. `from` 메서드는 인자로 `CustomerKind` 타입의 소유권을 가져갑니다. 따라서 `self.customer_type`에서 값을 읽어올 때 소유권 문제를 해결하기 위해 `clone()`을 사용했습니다.

```rust,ignore
    fn get_rawdata(&self) -> String {
        if let Some(kind) = &self.customer_type {
            return format!("{}", usize::from((*kind).clone()));
        } else {
            return "0".to_owned();
        }
    }
```

러스트의 소유권 개념은 익숙해지기 전까지는 컴파일 에러로 인해 난감할 때가 많습니다. 특히 가변 참조(`mutable reference`)가 여러 번 발생하거나 소유권 이동이 명확하지 않을 때 그렇습니다. 하지만 이는 잠재적인 메모리 문제를 컴파일러가 미리 찾아주는 과정입니다. 저 또한 수년 동안 러스트를 사용해 왔지만, 여전히 소유권 관련 에러를 마주할 때가 있습니다. 하지만 에러를 해결하고 나면 컴파일러가 방지해 준 메모리 안전성 문제의 가치를 깨닫게 됩니다.

### ExpireDate 추가

다음으로 제품의 사용 기한을 시리얼 번호에 추가해 보겠습니다. 예를 들어 2025년 12월 31일을 표현하기 위해 '20251231'과 같은 8글자 정보를 사용하겠습니다.

```rust
pub struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
    name: String,
}

impl ExpireDate {
    pub fn new() -> Self {
        ExpireDate {
            name: "ExpireDate".to_owned(),
            year: 0,
            month: 0,
            day: 0,
        }
    }
}
```

`GenSerialData` 트레이트 구현은 다음과 같습니다.

```rust
impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYYMMDD) (e.g. 20250123): ");
        let rawdata = get_user_input();
        assert_eq!(rawdata.len(), 8);

        self.year = rawdata[0..4].parse().unwrap();
        assert!(self.year >= 2021, "The year must be 2021 or later.");
        self.month = rawdata[4..6].parse().unwrap();
        assert!(
            (1..=12).contains(&self.month),
            "The month must be between 1 and 12."
        );
        self.day = rawdata[6..8].parse().unwrap();
        assert!(
            (1..=31).contains(&self.day),
            "The day must be between 1 and 31."
        );
    }

    fn verify(&mut self, data: &str) -> bool {
        let year: u32 = data[0..4].parse().unwrap();
        let month: u32 = data[4..6].parse().unwrap();
        let day: u32 = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
    }

    fn get_length(&self) -> usize {
        8
    }

    fn get_rawdata(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, _data: String) {
        unimplemented!()
    }
}
```

`ExpireDate` 역시 `get_input_from_user` 메서드를 직접 구현했습니다. 8글자의 입력을 받아 연, 월, 일로 나누어 저장합니다. 또한 `verify` 메서드도 재정의했습니다. 다른 데이터들은 단순히 문자열 비교로 검증할 수 있지만, `ExpireDate`는 내부적으로 정수 데이터를 다루기 때문입니다.

마지막의 `put_rawdata` 메서드는 구현하지 않았습니다. `ExpireDate`는 `get_input_from_user`에서 직접 필드 값을 설정하므로 이 메서드가 필요하지 않기 때문입니다.

이렇게 새로운 플러그인 2개를 추가해 보았습니다. `GenSerialData` 트레이트가 충분히 유연한 플러그인 인터페이스 역할을 하고 있음을 알 수 있습니다. 물론 앞으로 더 다양한 데이터를 추가하다 보면 트레이트의 정의를 조금씩 수정해야 할 수도 있습니다. 하지만 이런 표준 인터페이스가 존재한다는 것만으로도 새로운 기능을 추가할 때 어디서부터 시작해야 할지 명확한 실마리를 얻을 수 있습니다.

### 연습문제

1. `get_rawdata`에서 `(*kind).clone()`을 호출하지 않도록 코드를 수정해 보세요. `From` 트레이트 구현의 어느 부분을 바꾸면 될지 생각해 보세요.

2. `get_rawdata` 구현에서 `if let Some(kind) = self.customer_type`과 같이 소유권이 이동되도록 수정해 보세요. 어떤 컴파일 에러가 발생하는지 확인하고 그 이유를 고찰해 보세요.

### 답안

1.
```rust
impl From<&CustomerKind> for usize {
    fn from(item: &CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Student => 2,
            CustomerKind::Company => 3,
        }
    }
}
```
