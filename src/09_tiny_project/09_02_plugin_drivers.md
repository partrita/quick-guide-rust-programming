## 플러그인

이제 본격적으로 트레이트를 활용하는 것에 대해서 이야기를 해보겠습니다.
이전에 강조했듯이 요구사항은 언제나 바뀝니다.
시리얼 키 프로그램의 요구사항은 키 생성에 들어가는 입력 데이터의 종류나 크기일 것입니다.
지금은 고객 ID가 4글자이지만 고객이 많아지면 고객ID가 8글자가 될 수 있을 것입니다.
회사에 제품이 처음에는 1개만 있지만, 제품의 종류가 많아지면 제품 ID외에 제품의 종류에 대한 정보가 추가될 수도 있습니다.
나중에는 단순히 제품의 ID만 입력할 게 아니라, 계약 만기 날짜나 고객의 국가 번호 등등 모든 제품마다 시리얼 키에 들어갈 입력 데이터가 다를 수도 있습니다.

그럼 제품마다 시리얼 키를 만드는 프로그램을 별도로 만들어야할까요?
그렇게 만든다고하면 사실상 동일한 코드가 여러번 사용될 것이 분명하기 때문에 효율이 좋은 방법은 아닐 것입니다.
무엇보다도 실제로 매출을 낼 수 있는 제품도 아닌 단지 시리얼 키를 생성하는 프로그램이 여러개가 되고, 그것을 내가 관리해야된다면 큰 부담이 될 수 있습니다.
제품 10개가 각기 다른 시리얼 프로그램을 가지고 있는데, 10개의 프로그램을 하나하나 고객 ID를 8글자로 바꾼다고 생각해보세요.
일이 어려운것은 아니지만, 오히려 일이 어려운게 아니기 때문에 거기에 들어가는 시간과 노력이 좀 아깝지 않을까요?
이왕이면 같은 시간에 좀 더 연구할 가치가 있는 일을 하는게 좋다고 생각합니다.

사용자가 편리하게 사용하는 프로그램이 당연히 가장 좋은 프로그램이지만,
그 프로그램을 개발하는 개발자가 달라지는 요구사항에 기민하게 대처할 수 있어야만 사용하기 편리한 프로그램을 만들 수도 있는 것입니다.
그래서 우리는 여러가지 입력 데이터에 대한 처리 코드를 미리 만들어넣고, 필요에 따라 입력 데이터를 고를 수 있도록 옵션을 처리할 수 있게 만들어 보겠습니다.

보통 제가 만들어보려고하는 형태의 코드를 플러그인이나 드라이버 형태의 코드라고 부릅니다.
예를 들어 고객 ID가 하나의 플러그인(혹은 드라이버)가 됩니다.
시리얼 키에 사용자 ID를 넣으려면 플러그인을 사용하면 됩니다. 시리얼 키에 사용자 ID가 안들어간다면 플러그인을 사용하지 않으면 됩니다.
플러그인이라는 말 그대로 필요에 따라 사용하기도하고 안쓰기도하면 됩니다.
그래서 플러그인 혹은 드라이버라고 부르는 것입니다.
우리같은 개발자는 다양한 플러그인을 준비해주면 됩니다.
그리고 프로그램의 사용자(여기에서는 시리얼 키를 만드는 영업부서 직원이 될 수 있겠지요)가 필요한 플러그인을 골라서 사용하면 됩니다.
개발을 모르는 사용자도 플러그인을 잘 골라서 사용할 수 있도록 만드는 것도 중요하겠지요.

그럼 플러그인을 만들어보겠습니다.
우리가 전기를 사용하기 위한 플러그를 보면 모두 같은 형태를 가지고 있습니다.
원통형의 쇠 막대기 2개가 표준으로 길이가 정해진 길기와 간격으로 튀어나와있고, 파워 서플라이에는 그런 플러그를 딱 맞게 고정될 수 있도록 꼽아넣을 수 있도록 만들어져있습니다.
모든 플러그가 공통적인 모습을 가지고 있는 것처럼, 플러그를 만들 때 가장 먼저 할 일은 우리가 사용할 각 입력 데이터의 공통적인 특성을 뽑아내는 것입니다.
지금 우리가 사용한 2개의 입력 데이터, 사용자 ID과 제품 ID의 특성들을 일단 먼저 나열해보겠습니다.
이제 본격적으로 트레이트를 활용하는 것에 대해서 이야기를 해보겠습니다.
이전에 강조했듯이 요구사항은 언제나 바뀝니다.
시리얼 키 프로그램의 요구사항은 키 생성에 들어가는 입력 데이터의 종류나 크기일 것입니다.
지금은 고객 ID가 4글자이지만 고객이 많아지면 고객ID가 8글자가 될 수 있을 것입니다.
회사에 제품이 처음에는 1개만 있지만, 제품의 종류가 많아지면 제품 ID외에 제품의 종류에 대한 정보가 추가될 수도 있습니다.
나중에는 단순히 제품의 ID만 입력할 게 아니라, 계약 만기 날짜나 고객의 국가 번호 등등 모든 제품마다 시리얼 키에 들어갈 입력 데이터가 다를 수도 있습니다.

그럼 제품마다 시리얼 키를 만드는 프로그램을 별도로 만들어야할까요?
그렇게 만든다고하면 사실상 동일한 코드가 여러번 사용될 것이 분명하기 때문에 효율이 좋은 방법은 아닐 것입니다.
무엇보다도 실제로 매출을 낼 수 있는 제품도 아닌 단지 시리얼 키를 생성하는 프로그램이 여러개가 되고, 그것을 내가 관리해야된다면 큰 부담이 될 수 있습니다.
제품 10개가 각기 다른 시리얼 프로그램을 가지고 있는데, 10개의 프로그램을 하나하나 고객 ID를 8글자로 바꾼다고 생각해보세요.
일이 어려운것은 아니지만, 오히려 일이 어려운게 아니기 때문에 거기에 들어가는 시간과 노력이 좀 아깝지 않을까요?
이왕이면 같은 시간에 좀 더 연구할 가치가 있는 일을 하는게 좋다고 생각합니다.

사용자가 편리하게 사용하는 프로그램이 당연히 가장 좋은 프로그램이지만,
그 프로그램을 개발하는 개발자가 달라지는 요구사항에 기민하게 대처할 수 있어야만 사용하기 편리한 프로그램을 만들 수도 있는 것입니다.
그래서 우리는 여러가지 입력 데이터에 대한 처리 코드를 미리 만들어넣고, 필요에 따라 입력 데이터를 고를 수 있도록 옵션을 처리할 수 있게 만들어 보겠습니다.

보통 제가 만들어보려고하는 형태의 코드를 플러그인이나 드라이버 형태의 코드라고 부릅니다.
예를 들어 고객 ID가 하나의 플러그인(혹은 드라이버)가 됩니다.
시리얼 키에 사용자 ID를 넣으려면 플러그인을 사용하면 됩니다. 시리얼 키에 사용자 ID가 안들어간다면 플러그인을 사용하지 않으면 됩니다.
플러그인이라는 말 그대로 필요에 따라 사용하기도하고 안쓰기도하면 됩니다.
그래서 플러그인 혹은 드라이버라고 부르는 것입니다.
우리같은 개발자는 다양한 플러그인을 준비해주면 됩니다.
그리고 프로그램의 사용자(여기에서는 시리얼 키를 만드는 영업부서 직원이 될 수 있겠지요)가 필요한 플러그인을 골라서 사용하면 됩니다.
개발을 모르는 사용자도 플러그인을 잘 골라서 사용할 수 있도록 만드는 것도 중요하겠지요.

그럼 플러그인을 만들어보겠습니다.
우리가 전기를 사용하기 위한 플러그를 보면 모두 같은 형태를 가지고 있습니다.
원통형의 쇠 막대기 2개가 표준으로 길이가 정해진 길기와 간격으로 튀어나와있고, 파워 서플라이에는 그런 플러그를 딱 맞게 고정될 수 있도록 꼽아넣을 수 있도록 만들어져있습니다.
모든 플러그가 공통적인 모습을 가지고 있는 것처럼, 플러그를 만들 때 가장 먼저 할 일은 우리가 사용할 각 입력 데이터의 공통적인 특성을 뽑아내는 것입니다.
지금 우리가 사용한 2개의 입력 데이터, 사용자 ID과 제품 ID의 특성들을 일단 먼저 나열해보겠습니다.

사용자 ID
* 길이 4글자
* 숫자나 알파벳으로 이루어짐

제품 ID
* 길이 8글자
* 숫자나 알파벳으로 이루어짐

이렇게 나열하고 보니 공통적인 특성을 하나 곧바로 알 수 있습니다.
숫자나 알파벳으로 이루어졌다는 공통 특성이 나왔습니다.
그런데 길이가 다르네요. 근데 길이 다르다고 더 이상 공통 특성이 없는 것일까요? 아닙니다.
길이라는 특성을 가지고 있다는 것, 그리고 그 길이가 미리 정해져있다는 것 자체도 너무나 중요한 공통의 특성입니다.
이렇게 나열하고 보니 공통적인 특성을 하나 곧바로 알 수 있습니다.
숫자나 알파벳으로 이루어졌다는 공통 특성이 나왔습니다.
그런데 길이가 다르네요. 근데 길이 다르다고 더 이상 공통 특성이 없는 것일까요? 아닙니다.
길이라는 특성을 가지고 있다는 것, 그리고 그 길이가 미리 정해져있다는 것 자체도 너무나 중요한 공통의 특성입니다.
예를 들어 네트워크로 입력받는 데이터같은 경우는 길이가 미리 정해져있지 않거나, 너무나 큰 데이터일 수 있습니다.
그런 데이터는 길이라는 특성이 없다고 볼 수도 있습니다. 하지만 우리가 사용할 데이터는 크기가 쉽게 다룰 수 있도록 작고 미리 정해져있습니다.

이제 데이터 자체에 대한 특성을 봤으니 그 다음으로는 각 데이터를 처리하는 방법에는 어떤 공통적인 특성이 있는지 확인해볼 수 있습니다.
그런 데이터는 길이라는 특성이 없다고 볼 수도 있습니다. 하지만 우리가 사용할 데이터는 크기가 쉽게 다룰 수 있도록 작고 미리 정해져있습니다.

이제 데이터 자체에 대한 특성을 봤으니 그 다음으로는 각 데이터를 처리하는 방법에는 어떤 공통적인 특성이 있는지 확인해볼 수 있습니다.
일단 두 데이터를 입력받는 방법이 같습니다. 프로그램 사용자에게 입력을 달라는 메세지를 터미널에 출력하고, 터미널에서 입력을 받습니다.
입력을 받을 때, 각 입력 데이터의 특성들을 활용할 수 있으므로, 입력을 받는 코드 또한 공통적으로 사용할 수 있는 부분이 많을 것입니다.

*사실 이 과정은 프로그래밍을 많이 하다보면 무의식적으로도 반복하게되는 과정입니다. 코드를 최소화하기 위해 공통적인 데이터나 처리 코드를 찾아내서 데이터 구조나 함수를 만들어내서 재사용하는 과정과 같습니다. 그런 코드와 데이터의 재사용을 잘 하기 위한 방법론들이 객체지향이나 함수형 패러다임들이라고 생각합니다. 이번 장에서 이야기하는 것들은 객체지향이나 함수형 패러다임이라는 것이 나타나기 전 경력많은 개발자들이 널리 사용하던 패러다임들이 아직 뭐라 이름이 지어지기 전 시대의 지혜라고 생각해도 좋습니다.*

*사실 이 과정은 프로그래밍을 많이 하다보면 무의식적으로도 반복하게되는 과정입니다. 코드를 최소화하기 위해 공통적인 데이터나 처리 코드를 찾아내서 데이터 구조나 함수를 만들어내서 재사용하는 과정과 같습니다. 그런 코드와 데이터의 재사용을 잘 하기 위한 방법론들이 객체지향이나 함수형 패러다임들이라고 생각합니다. 이번 장에서 이야기하는 것들은 객체지향이나 함수형 패러다임이라는 것이 나타나기 전 경력많은 개발자들이 널리 사용하던 패러다임들이 아직 뭐라 이름이 지어지기 전 시대의 지혜라고 생각해도 좋습니다.*

그럼 두 입력 데이터를 위한 구조체부터 만들어보겠습니다.
각 데이터는 길이와 숫자/알파벳으로 이루어진 문자열이니 다음과 같은 구조체로 데이터를 표현할 수 있겠습니다.

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

고객ID와 제품ID가 각각 특정한 길이를 가진 문자열이기 때문에 동일한 필드를 가진 이름만 다른 구조체로 표현되고 있습니다.
digit필드와 id필드는 각각 자리수와 입력 데이터를 저장하는 문자열입니다.
name필드가 왜 필요한지는 다음 코드를 보면 알 수 있습니다.
이렇게 데이터 구조의 형태가 동일하니 사용자에게서 두 데이터를 입력받거나, 구조체에 있는 데이터를 읽어오는 코드가 동일하게됩니다.
```

고객ID와 제품ID가 각각 특정한 길이를 가진 문자열이기 때문에 동일한 필드를 가진 이름만 다른 구조체로 표현되고 있습니다.
digit필드와 id필드는 각각 자리수와 입력 데이터를 저장하는 문자열입니다.
name필드가 왜 필요한지는 다음 코드를 보면 알 수 있습니다.
이렇게 데이터 구조의 형태가 동일하니 사용자에게서 두 데이터를 입력받거나, 구조체에 있는 데이터를 읽어오는 코드가 동일하게됩니다.
같은 코드에 단지 다른 데이터를 적용할 수 있게 됩니다. 서로 다른 구조체에 같은 코드가 있다면 바로 트레이트로 만들 수 있습니다. 
다음은 GenSerialData라는 트레이트를 구현한 코드입니다.
다음은 GenSerialData라는 트레이트를 구현한 코드입니다.

```rust
trait GenSerialData {
    fn get_input_from_user(&mut self) {
        let input: String;

        let input: String;

        println!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            self.get_name()
            self.get_length(),
            self.get_name()
        );
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    fn get_length(&mut self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, _data: String);
    fn get_length(&mut self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, _data: String);
}
```

CustomerID와 ProductID가 같은 형태를 가지므로 프로그램 사용자에게서 입력 데이터를 받는 함수인 get_input_from_user함수는 사실 각 구조체마다 다르게 구현할 필요가 없습니다.
암호화된 시리얼을 복호화한 후 원본 데이터와 동일한지 검증하는 verify 함수도 CustomerID와 ProductID가 동일하게 구현하는 함수입니다.
이렇게 이 트레이트를 사용하는 구조체마다 동일하게 구현될수 있는 함수는 미리 구현해놓을 수 있습니다.
CustomerID와 ProductID가 같은 형태를 가지므로 프로그램 사용자에게서 입력 데이터를 받는 함수인 get_input_from_user함수는 사실 각 구조체마다 다르게 구현할 필요가 없습니다.
암호화된 시리얼을 복호화한 후 원본 데이터와 동일한지 검증하는 verify 함수도 CustomerID와 ProductID가 동일하게 구현하는 함수입니다.
이렇게 이 트레이트를 사용하는 구조체마다 동일하게 구현될수 있는 함수는 미리 구현해놓을 수 있습니다.
이렇게 트레이트 정의에 미리 구현되어있는 코드를 트레이트의 디폴트 구현이라고 부릅니다.
앞으로 GenSerialData 트레이트를 사용하는 구조체들은 get_input_from_user와 verify 함수를 따로 구현할 필요가 없습니다.
단지 디폴트 구현이 없는 다른 함수들만 구현하면 됩니다.
물론 디폴트 구현에 맞지 않는 구조체라면 직접 구현할 수도 있습니다.
앞으로 GenSerialData 트레이트를 사용하는 구조체들은 get_input_from_user와 verify 함수를 따로 구현할 필요가 없습니다.
단지 디폴트 구현이 없는 다른 함수들만 구현하면 됩니다.
물론 디폴트 구현에 맞지 않는 구조체라면 직접 구현할 수도 있습니다.

디폴트 구현이 있는 get_input_from_user의 코드를 자세히보면 get_length, get_name, put_rawdata 등의 함수들을 호출하는데 모두 각 구조체가 별도로 구현해야하는 트레이트 함수들입니다.
CustomerID 구조체와 ProductID가 GenSerialData 트레이트를 구현하는 코드를 보겠습니다.
디폴트 구현이 있는 get_input_from_user의 코드를 자세히보면 get_length, get_name, put_rawdata 등의 함수들을 호출하는데 모두 각 구조체가 별도로 구현해야하는 트레이트 함수들입니다.
CustomerID 구조체와 ProductID가 GenSerialData 트레이트를 구현하는 코드를 보겠습니다.

```rust
impl GenSerialData for CustomerID {
    fn get_length(&mut self) -> usize {
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
    fn get_length(&mut self) -> usize {
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
impl GenSerialData for CustomerID {
    fn get_length(&mut self) -> usize {
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
    fn get_length(&mut self) -> usize {
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

사실 CustomerID 구조체가 id, digit, name이라는 필드를 가질 필요는 없습니다.
구조체가 어떻게 정의되었는지는 전혀 상관없습니다.
단지 트레이트 구현에 필요한 get_length, get_rawdata, get_name, put_rawdata 이 4개의 함수를 구현하기만 하면 됩니다.

그럼 main함수의 구현을 보겠습니다.
사실 CustomerID 구조체가 id, digit, name이라는 필드를 가질 필요는 없습니다.
구조체가 어떻게 정의되었는지는 전혀 상관없습니다.
단지 트레이트 구현에 필요한 get_length, get_rawdata, get_name, put_rawdata 이 4개의 함수를 구현하기만 하면 됩니다.

그럼 main함수의 구현을 보겠습니다.

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

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter_mut() {
        let len = item.get_length();
        let rawdata = &dec[offset..offset + len];
        println!("Verify {}: {}", item.get_name(), rawdata);
        println!("Verify result: {}", item.verify(rawdata));
        offset += len;
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

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
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

```bash
g$ cargo run --bin serial_project_step3
   Compiling my-rust-book v0.1.0 (/home/gkim/study/quick-guide-rust-programming)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/serial_project_step3`
Please input 4-digits for UserID: 
1234
Please input 8-digits for UserID: 
qwerasdf
Plain serial: 1234qwerasdf
Encrypted serial: 3OvuVy1IXj5veDI61Mszjg==
Decrypted serial: 1234qwerasdf
Verify User ID: 1234
Verify Product ID: qwerasdf
```

collect_data함수와 generate_serial함수는 이전에 Box를 소개할 때 사용한 예제 코드와 동일한 코드입니다.
GenSerialData트레이트를 구현한 트레이트 객체의 배열을 인자로 받습니다.
그리고 각 객체가 별도로 구현한 get_input_from_user함수나 get_rawdata함수를 사용해서 사용자에게 트레이트 객제가 가진 이름과 자리수를 출력해주고 터미널 입력을 받습니다.
그렇게 입력받은 데이터를 각 트레이트 객체에 저장하는 put_rawdata 함수를 이용하는 것입니다.
generate_serial 함수도 각 트레이트 객체가 구현한 get_rawdata 메소드를 호출해서 각 트레이트 객체가 가진 데이터를 얻어옵니다.
그리고 모든 데이터를 다 합쳐서 하나의 문자열로 만들어서 반환합니다.

다시 한번 정리를 해보겠습니다.
이번 장의 목표는 언제 플러그인(드라이버)를 사용할 수 있는지, 어떻게 시작해야할지를 이야기하는 것이었습니다.
먼저 우리가 처리해야할 데이터들이 무엇이있고 각 데이터가 어떤 특성들을 가지고 있는지 파악합니다.
그리고 공통 특성들이 무엇인지를 골라내서, 그 공통 특성들을 저장하고 얻어낼 수 있는 트레이트를 만듭니다.
각 데이터를 데이터 구조로 표현한 후, 각 데이터 구조들이 트레이트를 구현하게되면 모든 데이터를 처리할 수 있는 트레이트가 만들어집니다.

결국 제가 이야기하고 싶은 것은 트레이트가 플러그인(드라이버)를 구현하는데 핵심적인 역할을 한다는 것입니다.
트레이트는 각 구조체가 어떻게 구현되었는지를 전혀 알 필요가 없습니다.
단지 트레이트는 get_length, get_rawdata, get_name, put_rawdata등의 표준적인 인터페이스를 제공해서, 각 구조체가 별도로 각 구조체의 정의에 맞게 구현하도록합니다.
그리고 그렇게 구현된 각 메소드들을 사용해서 대부분의 구조체들에 적용될 수 있는 get_input_from_user와 verify함수같은 공통 구현을 만들 수도 있습니다.
GenSerialData 트레이트와 같이 인터페이스를 제공해주는 쪽을 프레임워크라고 부르고, CustomerID와 같이 인터페이스를 구현하는 쪽을 플러그인(드라이버)라고 부르는 경우가 많습니다.

비유를 해보자면 GenSerialData라는 트레이트는 멀티탭과 같은 것입니다.
get_length 등의 함수들은 멀티탭에 있는 2개의 구멍과 같은 것입니다.
그리고 CustomerID에서 구현한 get_length등의 메소드들은 2개의 금속 막대기라고 생각할 수 있습니다.
이 2개의 금속 막대기 끝에 있는 전자제품이 전기 주전자인지 전자레인지인지 드릴인지는 멀티탭에게 아무 상관이 없습니다.
단지 서로 미리 정한 인터페이스(2개의 금속 막대기와 그것들이 끼워질 구멍)에 맞게 서로가 구현되었으면 각자의 할 일을 할 뿐인 것입니다.

### 연습문제

1. CustomerID와 ProductID, 2개의 입력 데이터를 만들어봤습니다.
그 외에 시리얼 번호를 생성하기 위해서 어떤 입력 데이터가 있을 수 있을지 한번 생각해보세요.
그리고 한번 구현까지 해보세요.
다음장에서 2개의 입력데이터를 추가해볼 예정입니다만 그 전에 직접 고민해보는 것을 추천합니다.
다음 장을 읽어보면서 제가 제시한 방법보다 더 좋은 방법을 생각해보시기 바랍니다.