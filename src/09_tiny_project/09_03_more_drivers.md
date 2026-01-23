## 추가 플러그인

### CustomerType 추가

플러그인은 서로 다른 형태의 데이터들을 동일한 인터페이스로 추가할 수 있도록 높은 호환성을 가저야합니다.
지금까지 우리가 만든 플러그인은 사실상 이름만 다르고 데이터 구조가 동일한 2개의 타입에만 적용되었습니다.
정말로 다른 형태의 데이터 구조에도 사용할 수 있는건지 알아보고, 만약 부족한 점이 있다면 보완하기위해 2가지의 새로운 플러그인을 추가해보겠습니다.

첫번째로 추가해볼 입력 데이터는 고객 유형입니다.
제품을 판매하다보면 일반 구매 고객이 있고, 학생용 무료 버전을 사용하는 고객이 있을 수 있습니다.
또 회사에서 단체로 구매해서 사용하는 경우도 있습니다.
이렇게 3가지 유형을 시리얼 번호에도 추가해보겠습니다.

먼저 3가지 고객 유형을 표현할 수 있는 enum 타입으로 만들어보겠습니다.

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
            CustomerKind::Business => 1, // 개인이 구매해서 사용하는 경우
            CustomerKind::Student => 2,  // 학생이 무료버전을 사용하는 경우
            CustomerKind::Company => 3,  // 회사에서 단체 구매한 경우
        }
    }
}
```

CustomerKind라는 enum타입을 만들었을 뿐 아니라 CustomerKind를 usize타입의 정수로 바꿀 수 있는 From 트레이트까지 구현했습니다.
프로그램 사용자에게 입력을 요청할 때 1, 2, 3 중에 하나의 숫자를 입력받아서 그것을 고객 유형으로 저장해야합니다.
그래서 정수와 CustomerKind를 서로 변환해줄 수 있는 From 트레이트를 구현하는 것입니다.

그 다음으로 실제로 고객 유형을 표현하는 CustomerType구조체를 만듭니다.

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

CustomerType이라는 구조체를 만들고 customer_type 필드에 CustomerKind을 저장합니다.
필요한 숫자는 1,2,3이므로 지금은 한자리 수만 필요하므로 digit필드는 1로 초기화합니다.

이제 정말 중요한 GenSerialData트레이트의 구현이 필요합니다.

```rust
impl GenSerialData for CustomerType {
    fn get_input_from_user(&mut self) {
        let input: String;

        print!("Please input customer type: ");
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
        print!(
            "{}-{:?}",
            usize::from(CustomerKind::Company),
            CustomerKind::Company
        );
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&mut self) -> usize {
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

가장 큰 변화는 get_input_from_user메소드를 디폴트 구현을 사용하지 않고, 직접 구현해준 것입니다.
get_input_from_user 메소드의 디폴트 구현은 구조체의 길이와 이름만 출력하지만 CustomerType의 구현은 각 고객 유형을 출력해줘야하기 때문에 직접 구현을 했습니다.

다른 메소드들의 구현은 간단하므로 일일이 설명하지는 않겠습니다만 get_rawdata메소드에 한가지 특이한 것이 있어서 추가 설명을 하겠습니다. From트레이트의 from 메소드를 구현해서 CustomerKind 타입으로부터 usize타입의 정수를 만들어낼 수 있습니다. from메소드의 함수를 자세히 보면 인자가 CustomerKind 타입입니다. &CustomerKind와 같은 레퍼런스 타입이 아닙니다. 따라서 CustomerKind 변수를 전달하면 소유권이 from메소드로 넘어가서 함수 실행이 완료된 후에는 변수를 사용할 수가 없습니다. 그래서 get_rawdata 메소드에 다음과 같이 clone()메소드를 사용한 것입니다.

```rust
    fn get_rawdata(&self) -> String {
        if let Some(kind) = &self.customer_type {
            return format!("{}", usize::from((*kind).clone()));
        } else {
            return "0".to_owned();
        }
    }
```

kind변수는 `if let Some(kind) = &self.customer_type` 코드에서 생성되었습니다.
self의 레퍼런스에서 값을 읽어오므로 kind에는 self.customer_type 변수의 레퍼런스가 저장됩니다.
만약에 `if let Some(kind) = self.customer_type`와 같이 객체에서 바로 특정 필드를 읽어오면 kind에는 레퍼런스가 아닌 self.customer_type 변수의 소유권이 이동합니다.
데이터 구조체가 가진 여러 필드중에서 한 필드의 소유권이 이동해도 데이터 구조체를 사용하는데는 지장이 없습니다만, 소유권이 이동한 필드는 더 이상 사용할 수 없습니다.
따라서 `if let` 구문을 사용할 때는 보통 `if let Some(kind) = &self.customer_type`와 같이 레퍼런스를 참조하도록 사용하게됩니다.

소유권이라는 컨셉이 사실 자세히 따져보면 크게 어려운 것은 아닙니다.
하지만 이와 같이 일부분만 소유권이 이동하거나, 어디서 소유권이 이동하는지 바로 눈에 보이지 않는 경우가 자주 생겨서, 컴파일 에러는 발생하지만 어디를 어떻게 고쳐야할지 난감한 경우가 많습니다.
특히 mutable reference가 여러번 발생하는 경우와 같이 보통의 프로그래밍 언어에서는 아무런 문제도 아닌 것들이 러스트 언어에서만 문제가 되는 경우가 많고, 해결하기 위해서는 여러 함수를 고치거나 모듈 전체를 다시 디자인해야되는 경우도 생갑니다.
당얀히 불편할 수밖에 없습니다.
하지만 다르게 생각해보면, 잠재적으로 발생할 수 있는 메모리 문제를 러스트 컴파일러가 미리 찾아준 것이기도 합니다.
저는 여가 시간에 러스트 언어를 사용한게 3~4년 정도 되었지만, 아직까지도 종종 소유권에 대한 알 수 없는 에러들 때문에 당황할 때가 있습니다.
하지만 막상 해결하고나면 제가 생각하지 못했던 곳에 메모리 문제가 있을 수 있었고, 그것을 컴파일러가 미리 방지해줬다는 것을 이해하게됩니다.

### ExpireDate 추가

제품의 사용 기한을 시리얼 번호에 추가해보겠습니다.
2025년 12월 31일을 표현하기 위해서 20251231과 같이 8글자의 정보를 넣어보겠습니다.
ExpireDate라는 이름의 데이터 구조를 만들고 간단하게 연, 월, 일을 저장하는 3가지 정수만 저장해보겠습니다.

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

GenSerialData트레이트의 구현은 다음과 같습니다.

```rust
impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYYMMDD) (e.g. 20250123) : ",);
        let rawdata = get_user_input();
        assert_eq!(rawdata.len(), 8); // 입력받은 데이터의 길이가 10인지 검증, YYYYMMDD에 /가 2개 들어가므로 10개임

        // 입력받은 날짜를 분리해서 year, month, day 필드에 저장
        // 동시에 year, month, day 필드에 저장된 값이 올바른지 검증
        self.year = rawdata[0..4].parse().unwrap();
        assert!(self.year >= 2021, "The year must be 2021 or later.");
        self.month = rawdata[4..6].parse().unwrap();
        assert!(
            self.month >= 1 && self.month <= 12,
            "The month must be between 1 and 12."
        );
        self.day = rawdata[6..8].parse().unwrap();
        assert!(
            self.day >= 1 && self.day <= 31,
            "The day must be between 1 and 31."
        );
    }

    fn verify(&mut self, data: &str) -> bool {
        let year = data[0..4].parse().unwrap();
        let month = data[4..6].parse().unwrap();
        let day = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
    }

    fn get_length(&mut self) -> usize {
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

이번에도 get_input_from_user 메소드를 직접 구현했습니다.
사용자로부터 8글자의 숫자를 입력받습니다.
8글자중에 4자리는 연도, 2자리는 월, 2자리는 일로 나눠서 ExpireDate 구조체에 저장합니다.

그리고 디폴트 구현을 사용하지 않는 메소드가 하나 더 생겼습니다.
verify 메소드를 직접 구현하고 있습니다.
지금까지 다른 입력 데이터들은 단순히 기존에 사용자로부터 입력받아서 저장한 문자열과, 시리얼 번호를 복호화해서 얻은 문자열을 단순 비교하는 것으로 검증하기 때문에 verify 메소드의 디폴트 구현으로 충분했습니다.
하지만 ExpireDate 구조체는 연/월/일 정보를 정수로 저장하고 있으므로, 단순 문자열 비교를 사용할 수 없습니다.
verify메소드의 기본 구조는 get_input_from_user 메소드와 동일합니다.
시리얼을 복호화한 후 얻은 8글자의 문자열에서 4글자를 연도와 비교하고, 2글자를 월과 비교하고, 2글자를 일자와 비교합니다.
get_input_from_user에서 사용자가 입력한 문자열을 날짜로 변환하는 과정과 동일합니다.

GenSerialData 트레이트 구현의 마지막에 있는 put_rawdata 메소드는 구현하지 않습니다. 이 메소드는 get_input_from_user에서 사용하는 메소드인데, ExpireDate에서 사용하지 않습니다.

이렇게 새로운 플러그인을 2개 추가해봤습니다. 어떤가요?
GenSerialData라는 트레이트가 충분히 유연한 플러그인의 역할을 하는것 같은가요?
제가 생각할 수 있는 4가지 입력 데이터를 위해서는 제 역할을 하는 것으로 보입니다만, 계속 다양한 입력 데이터를 추가하다보면 언젠가는 GenSerialData 트레이트의 메소드들을 조금씩 봐꿔주어야하는 순간이 올 것입니다.
그러면 이미 구현한 입력 데이터들을 수정해야될 수도 있습니다.
그래도 이런 표준 인터페이스가 있다는 것만으로도 새로운 유형이 데이터를 추가할 때 어떻게 무엇으로 시작해야될지 실마리를 얻을 수 있습니다.
그것만으로도 개발을 시작하는데 큰 도움이 됩니다.

### 연습문제

1. get_rawdata에서 (*kind).clone()과 같이 clone메소드를 호출하지 않도록 고쳐보세요. From트레이트 구현의 어디를 바꾸면 될지 생각해보세요.

2. get_rawdata 메소드의 구현에서 `if let Some(kind) = self.customer_type`와 같이 소유권이 이동되도록 구현을 바꿔보세요. 어디에서 어떤 에러가 나는지 확인해보고 그 의미를 생각해보세요.


### 답안

1.
```rust
impl From<&CustomerKind> for usize {
    fn from(item: &CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1, // 개인이 구매해서 사용하는 경우
            CustomerKind::Student => 2,  // 학생이 무료버전을 사용하는 경우
            CustomerKind::Company => 3,  // 회사에서 단체 구매한 경우
        }
    }
}
```
