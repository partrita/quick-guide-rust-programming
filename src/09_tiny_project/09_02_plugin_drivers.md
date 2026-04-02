## 플러그인

이제 본격적으로 트레이트를 활용하는 방법에 대해 이야기해 보겠습니다. 이전에 강조했듯이, 요구사항은 언제나 바뀝니다. 시리얼 키 프로그램의 경우, 키 생성에 들어가는 입력 데이터의 종류나 크기가 주요 요구사항일 것입니다. 지금은 고객 ID가 4글자이지만 고객이 많아지면 8글자로 늘어날 수 있고, 제품의 종류가 다양해지면 제품 ID 외에 다른 정보가 추가될 수도 있습니다. 나중에는 계약 만기 날짜나 고객의 국가 번호 등 제품마다 서로 다른 입력 데이터가 필요할 수도 있습니다.

그렇다면 제품마다 시리얼 키 생성 프로그램을 별도로 만들어야 할까요? 그렇게 하면 사실상 동일한 코드가 여러 번 중복되어 사용될 것이 분명하므로 효율적인 방법이 아닙니다. 무엇보다 단순히 시리얼 키를 생성하는 프로그램이 수십 개로 늘어나고 이를 일일이 관리해야 한다면 개발자에게 큰 부담이 됩니다. 예를 들어 10개의 프로그램에서 각각 고객 ID를 8글자로 수정해야 한다고 생각해 보세요. 작업 자체는 어렵지 않지만, 불필요한 시간과 노력이 낭비됩니다.

사용자가 편리하게 사용하는 프로그램이 좋은 프로그램이지만, 개발자가 변화하는 요구사항에 기민하게 대처할 수 있어야만 그런 프로그램을 지속적으로 만들어낼 수 있습니다. 그래서 우리는 여러 가지 입력 데이터 처리 코드를 미리 구현해 두고, 필요에 따라 선택할 수 있는 구조를 만들어 보겠습니다.

이런 형태의 코드를 보통 **플러그인** 또는 **드라이버**라고 부릅니다. 예를 들어 고객 ID 처리를 하나의 플러그인으로 만드는 것입니다. 시리얼 키에 고객 ID가 필요하면 해당 플러그인을 사용하고, 필요 없으면 사용하지 않으면 됩니다. 말 그대로 필요에 따라 끼웠다 뺐다 할 수 있는 구조입니다. 개발자는 다양한 플러그인을 준비해 두고, 사용자가 필요한 플러그인을 골라 사용할 수 있게 하면 됩니다.

플러그인을 설계할 때는 공통적인 인터페이스가 중요합니다. 전기를 사용하기 위한 플러그들이 모두 표준화된 형태를 가진 것처럼, 우리도 각 입력 데이터의 공통적인 특성을 뽑아내야 합니다. 사용자 ID와 제품 ID의 특성을 나열해 보겠습니다.

**사용자 ID**
* 길이: 4글자
* 구성: 숫자와 알파벳

**제품 ID**
* 길이: 8글자
* 구성: 숫자와 알파벳

두 데이터 모두 숫자와 알파벳으로 이루어졌다는 공통점이 있습니다. 길이는 다르지만, '길이'라는 특성을 가지고 있으며 그 길이가 미리 정해져 있다는 점 또한 중요한 공통 특성입니다. 반면 네트워크를 통해 들어오는 데이터는 길이가 가변적이거나 매우 클 수 있는데, 우리가 다룰 데이터는 크기가 작고 고정되어 있습니다.

데이터 자체의 특성을 파악했으니, 이제 처리 방식의 공통점을 찾아봅시다. 두 데이터 모두 사용자에게 입력을 요청하는 메시지를 터미널에 출력하고 입력을 받습니다. 이 과정에서 공통적으로 사용할 수 있는 코드가 많을 것입니다.

이러한 과정은 코드를 최소화하기 위해 공통적인 데이터나 처리 로직을 찾아내어 재사용하는 과정과 같습니다. 이를 더 체계적으로 수행하기 위한 방법론이 바로 객체지향이나 함수형 패러다임입니다. 여기서는 트레이트를 활용하여 이를 구현해 보겠습니다.

우선 두 입력 데이터를 위한 구조체부터 정의해 보겠습니다.

```rust
pub struct CustomerID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl CustomerID {
    pub fn new(digit: usize) -> Self {
        CustomerID {
            name: "UserID".to_owned(),
            digit,
            id: None,
        }
    }
}

pub struct ProductID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        ProductID {
            name: "ProductID".to_owned(),
            digit,
            id: None,
        }
    }
}
```

고객 ID와 제품 ID가 각각 특정한 길이를 가진 문자열이므로, 동일한 필드를 가진 구조체로 표현되었습니다. `digit` 필드는 자릿수를, `id` 필드는 입력받은 데이터를 저장합니다. `name` 필드는 사용자에게 안내 메시지를 출력할 때 사용됩니다. 이제 이 두 구조체에 공통으로 적용할 인터페이스인 `GenSerialData` 트레이트를 정의하겠습니다.

```rust
trait GenSerialData {
    fn get_input_from_user(&mut self) {
        println!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            self.get_name()
        );
        let input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    fn get_length(&self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, data: String);
}
```

`CustomerID`와 `ProductID`가 유사한 형태이므로, 사용자로부터 입력을 받는 `get_input_from_user` 메서드나 데이터를 검증하는 `verify` 메서드는 각 구조체마다 다르게 구현할 필요가 없습니다. 이렇게 트레이트 정의 단계에서 미리 구현해 두는 코드를 **디폴트 구현(Default Implementation)**이라고 부릅니다. 이 트레이트를 구현하는 구조체들은 디폴트 구현이 없는 필수 메서드들만 구현하면 되며, 필요한 경우 디폴트 구현을 오버라이딩(재정의)할 수도 있습니다.

이제 각 구조체에서 `GenSerialData` 트레이트를 구현해 보겠습니다.

```rust
impl GenSerialData for CustomerID {
    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        self.id = Some(data);
    }
}

impl GenSerialData for ProductID {
    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        self.id = Some(data);
    }
}
```

각 구조체가 어떤 필드를 가졌는지는 중요하지 않습니다. 오직 트레이트에서 요구하는 4개의 메서드를 구현하기만 하면 `GenSerialData`로서 동작할 수 있습니다. 이제 `main` 함수의 구현을 살펴봅시다.

```rust
fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input_from_user();
    }
}

fn generate_serial(items: &mut Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter_mut() {
        data.push_str(&item.get_rawdata());
    }
    data
}

fn main() {
    let productid = ProductID::new(8);
    let customerid = CustomerID::new(4);
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(customerid), Box::new(productid)];

    collect_data(&mut items);
    let plain_serial = generate_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256);
    let serial = mc.encrypt_str_to_base64(&plain_serial);
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap();
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter_mut() {
        let len = item.get_length();
        let rawdata = &dec[offset..offset + len];
        println!("Verify {}: {}", item.get_name(), rawdata);
        println!("Verify result: {}", item.verify(rawdata));
        offset += len;
    }
}
```

`collect_data`와 `generate_serial` 함수는 `GenSerialData` 트레이트 객체의 벡터를 인자로 받습니다. 각 객체가 구현한 메서드들을 통해 사용자 입력을 받고 시리얼 키를 생성합니다. 여기서 중요한 점은 트레이트가 플러그인 구조를 구현하는 데 핵심적인 역할을 한다는 것입니다.

트레이트는 각 구조체의 내부 구현을 알 필요가 없습니다. 오직 표준 인터페이스를 제공하여 각 구조체가 자신의 역할에 맞게 동작하도록 유도합니다. 이러한 인터페이스 제공자를 **프레임워크**라 부르고, 이를 구현하는 쪽을 **플러그인**(또는 드라이버)이라고 부릅니다.

비유하자면 `GenSerialData` 트레이트는 멀티탭과 같습니다. 메서드들은 멀티탭의 구멍과 같고, 각 구조체의 구현은 플러그의 금속 막대기와 같습니다. 멀티탭에 연결된 기기가 주전자인지 드릴인지는 중요하지 않습니다. 정해진 규격(인터페이스)에 맞기만 하면 각자의 기능을 수행할 수 있는 것과 같은 원리입니다.

### 연습문제

1. `CustomerID`와 `ProductID` 외에 시리얼 번호 생성에 필요한 또 다른 입력 데이터는 무엇이 있을지 생각해 보세요. 그리고 직접 구현까지 해보시기 바랍니다.

2. 다음 장에서는 새로운 입력 데이터 2개를 추가해 볼 예정입니다. 제가 제시할 방법보다 더 효율적인 설계가 있을지 미리 고민해 보는 것을 추천합니다.
