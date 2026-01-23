## 커맨드 라인 옵션

표준 인터페이스와 플러그인을 만드는 것은 지금까지는 잘 동작하고 있습니다.
그런데 아직 뭔가 실무에 쓰일만한 프로그램이라고 보기에는 조금 부족한 면이 있습니다.
사실 우리가 만들었던 것과 같이 사용자에게 일일이 뭔가를 물어봐서 입력을 받는 방식으로 동작하는 것은 편리할 때도 있지만, 시리얼 번호를 만드는 경우에는 그다지 좋은 방식이라고는 생각되지 않습니다.
어짜피 사용법을 아는 같은 회사의 직원이 사용하는 프로그램이므로 그렇게까지 친절해야될 필요는 없고, 빨리 실행해서 결과를 얻을 수 있는게 더 좋을것 같습니다.
그래서 이번에는 커맨드 라인에서 입력을 받을 수 있도록 바꿔보겠습니다.

대략 다음과 같은 방식으로 동작하면 될것 같습니다.

```bash
$ serial --help
모든 입력 데이터의 help 메세지 출력
$ serial generate --userid 1234 --productid qwerasdf
..생성
```

### Clap 크레이트 사용법

암호화를 위해서 크레이트를 조사해보고 MagicCrypt 크레이트를 찾아내서 예제 코드를 찾아서 사용법을 알아보는 과정을 이야기했었습니다.
비슷한 방식으로 커맨드 라인 입력을 처리하는 크레이트를 찾아서 사용할 수도 있습니다.
그런데 저는 Clap이라는 크레이트를 예전에 회사 업무로 사용한 경험이 있어서 별다른 조사없이 Clap을 사용하기로 결정했습니다.

먼저 Clap 크레이트의 홈페이지(https://docs.rs/clap/latest/clap/)에 있는 예제 코드를 한번 보면서 대략 어떤 크레이트인지를 알아보겠습니다. 참고로 clap의 예제 코드를 지금 우리가 작업하는 패키지에서 실행하면 기존 코드와 섞이게되니 새로운 패키지를 하나 만들어서 실행해보시기 바랍니다.

다음과 같이 ex-clap이라는 패키지를 만들고, clap 크레이트를 추가하고, derive라는 기능을 활성화합니다.

```bash
$ cargo new ex-clap
$ cd ex-clap
$ cargo add clap --features derive
```

그리고 다음 예제를 main.rs에 복사합니다.
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

Args 구조체의 윗줄에 있는 `#[derive(Parser, Debug)]` 구문은 커맨드 라인 옵션을 처리하는 파서를 자동으로 생성해주라는 의미입니다. Args 구조체의 트레이트 구현으로 옵션 처리 코드를 구현해줍니다. 그래서 main 함수를 보면 커맨드 라인 입력을 처리하는 코드가 전혀 없고 단지 Args구조체의 parse라는 메소드를 호출하는 것 뿐입니다. 이렇게 derive 기능으로 커맨드 라인 옵션 처리 코드를 구현하므로 cargo add 명령을 실행할 때 `--features derive` 옵션을 추가한 것입니다.

그 다음 줄에 있는 `#[command(version, about, long_about = None)]` 구문은 version과 help 옵션을 자동으로 생성해주라는 의미입니다. about이 help에 해당합니다.

그리고 어떤 옵션들이 있는지를 지정하는 것은 Args 구조체의 이 프로그램은 2개의 옵션, --name(-n)과 --cont(-c)를 처리할 수 있다는 것을 알 수 있습니다. Args 구조체에 name과 count필드 위에 `#[arg(short, long)]`이라는 derive가 있기 때문에, clap 크레이트에서 자동으로 각 필드 이름, name과 count에 해당하는 옵션을 만들어줍니다. count필드에는 디폴트 값이 1이라는 표시도 있으므로, 입력이 없으면 자동으로 1을 저장해줍니다. name필드는 디폴트 값이 없으므로 옵션을 지정하지 않으면 프로그램이 실패할 것입니다. 그리고 각 옵션에 대한 설명은 각 필드의 바로 윗줄에 있는 주석에 쓰여있습니다.

예제를 실행해보면서 name과 count 옵션이 어떻게 처리되는지를 확인해보겠습니다.

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/bin-example`
error: the following required arguments were not provided:
  --name <NAME>

Usage: bin-example --name <NAME>

For more information, try '--help'.
$ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/bin-example --help`
Simple program to greet a person

Usage: bin-example [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

아무런 커맨드 라인 옵션을 주지 않으면 필수 옵션이 없다는 안내 메세지가 출력됩니다.
그리고 --help 옵션을 주면 각 옵션에 대한 설명을 출력합니다.
예제 코드에서 본대로 Args 구조체의 각 필드에 써준 주석이 도움말로 출력되는 것을 확인할 수 있습니다. 그래서 아래와 같이 프로그램 옵션을 지정해서 실행하게됩니다.

```bash
$ cargo run -- -n Gioh --count 2
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/bin-example -n Gioh --count 2`
Hello Gioh!
Hello Gioh!
```

### Clap의 다른 사용법

우리가 만들 시리얼 프로그램은 예제와 같이 항상 동일한 옵션을 가지지 않습니다.
어떤 경우에는 지금까지 우리가 만든 4개의 입력 데이터를 모두 사용할 때도 있지만, 어떤 제품은 4개 입력 데이터 중에 한두개만 사용할 수도 있습니다.

따라서 이전 예제의 Args와 같이 고정된 데이터 구조를 만들어서 늘 고정된 옵션을 처리하도록 만들 수가 없습니다. 좀 더 옵션을 유연하게 만들 수 있도록 수정해야합니다.
그렇게 유연한 옵션 처리를 위해서 clap은 다음과 같이 Arg와 Command라는 데이터 구조를 제공하고 있습니다.
아래 예제는 Arg와 Command를 이용해서 이전 예제와 같이 count 옵션에 받은 수만큼 이름을 출력하는 예제입니다.

```rust
use clap::{Arg, Command};

fn main() {
    // clap::Command타입의 객체 생성
    // --version과 --help 옵션에 대한 설정을 추가함
    let mut command = Command::new("serial") // 프로그램의 이름은 serial
        .version("0.1.0") // cargo run -- --version 명령을 실행하면 0.1.0이 출력됨
        .about("Serial number generator"); // cargo run -- --help 명령을 실행하면 출력되는 프로그램 설명

    // "Name"이라는 ID를 가진 Arg타입의 객체 생성
    // --name 혹은 -n 옵션으로 지정가능
    // 디폴트 값은 없고 반드시 옵션으로 지정되어야함
    let arg = Arg::new("NAME")
        .long("name")
        .short('n')
        .help("Name of the person to greet")
        .required(true);
    // command의 arg 메소드를 호출해서 "Name" 옵션을 추가함. arg의 반환값은 새로 생성된 command 객체임
    command = command.arg(arg);
    // "Count"라는 ID를 가진 Arg타입의 객체 셍성
    // --count 혹은 -c 옵션으로 지정가능
    // 커맨드 라인 옵션으로 값을 입력받지 못하면 1값을 디폴트로 사용함
    let arg = Arg::new("COUNT")
        .long("count")
        .short('c')
        .help("Number of times to greet")
        .default_value("1");
    // 옵션 이름으로 Arg타입 객체를 만든 후 command 객체에 추가해줌
    command = command.arg(arg);

    // get_matches함수는 프로그램에 전달된 모든 커맨드 라인 옵션을 읽어옴 - parser와 같은 역할
    let matches = command.get_matches();

    // --name/-n 옵션을 읽어옴
    if let Some(name) = matches.get_one::<String>("NAME") {
        // --count/-c 옵션을 읽어옴
        if let Some(count) = matches.get_one::<String>("COUNT") {
            let count: usize = count.parse::<usize>().unwrap_or(1);
            for _ in 0..count {
                println!("Hello {}!", name);
            }
        }
    }
}
```

가장 처음에 해야 할 일은 Command타입의 객체를 만드는 것입니다.
예제 코드에서는 Command 구조체의 3가지 메소드를 사용하고 있습니다.
* new: Command 타입의 객체를 생성합니다
* version: 프로그램의 버전을 지정합니다
* about: help 옵션을 실행하면 출력될 안내 메세지를 지정합니다.

그 다음은 우리가 만들고자하는 2개의 옵션 `--name`과 `--count`를 위한 Arg타입 객체를 만드는 것입니다.
예제 코드에서 사용하는 메소드는 다음과 같습니다.
* long: `--name`과 같이 옵션의 전체 이름 지정
* short: `-n`과 같이 옵션의 짧은 이름 지정
* help: 각 옵션에 대한 설명
* default_value: 프로그램 사용자가 옵션을 지정하지 않았을 때 기본으로 저장되는 값
* required: 프로그램 사용자가 반드시 지정해야되는 옵션인지 지정

그리고 Command의 get_matches 메소드를 호출합니다.
이 메소드는 프로그램에 전달된 모든 옵션을 파싱해서 저장하는 일을 합니다.
그리고 ArgMatches 타입의 객체를 반환합니다.

최종적으로 각 옵션에 지정된 값을 읽어오는 것이 ArgMatches 객체의 get_one 메소드입니다.
이전에 "NAME"라는 ID를 가진 Arg타입 객체를 만들었는데 get_one메소드에 "NAME"과 같은 ID를 지정해야 옵션을 읽어올 수 있습니다.

프로그램을 실행해보면 이전에 Clap홈페이지에 있는 예제와 동일하게 실행되는 것을 알 수 있습니다.

```bash
% cargo run -- --name gioh --count 2
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/bin-example --name gioh --count 4`
Hello gioh!
Hello gioh!
```

이제 Clap을 적용해서 시리얼 번호 생성에 필요한 입력 데이터를 읽어오도록 만들어보겠습니다.

우선 다음과 같이 Cargo.toml 파일을 수정합니다. features에 "derive"를 "string"으로 바꿉니다.

```
[dependencies]
clap = { version = "4.5.26", features = ["string"] }
```

그리고 GenSerialData 트레이트에 3개의 메소드를 추가합니다.
* get_arg_name: `--productid`와 같이 옵션의 긴 이름을 반환
* get_help: 각 옵션의 설명 반환
* get_mandatory: 필수로 필요한 옵션일 경우 true 반환

```rust
trait GenSerialData {
    fn verify(&self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata().unwrap() == data
    }

    fn get_length(&self) -> usize;
    fn get_rawdata(&self) -> Option<String>;
    fn get_name(&self) -> &str;
    fn put_rawdata(&mut self, data: &str);
    // 명령행 인자 처리를 위해 추가된 함수들
    fn get_arg_name(&self) -> &str;
    fn get_help(&self) -> String;
    fn get_mandatory(&self) -> bool;
}
```

다음은 CustomerID 구조체에 새로운 메소드 3개를 추가하는 예제입니다.

```rust
impl GenSerialData for CustomerID {
...생략

    fn get_arg_name(&self) -> &'static str {
        "customerid"
    }

    fn get_help(&self) -> String {
        format!("Customer ID with {}-digit", self.digit)
    }

    fn get_mandatory(&self) -> bool {
        true
    }
}
```

그리고 main함수를 다음과 같이 수정합니다.

*중복된 코드가 많으므로 모든 코드를 복사해서 옮겨적지 않았습니다. 프로그램의 전체 코드는 https://github.com/gurugio/quick-guide-rust-programming 에서 확인바랍니다.*

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

    // 더 이상 사용자에게 입력을 받지 않고 명령행 인자로 처리
    let mut command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator");

    for item in items.iter() {
        // Cargo.toml에서도 Clap의 string feature를 사용하고 있기 때문에 String을 넣어줘야 함
        // Arg의 new, long, help, required 함수에 String을 넣어주기 위해 to_owned()를 사용
        // 만약에 Clap의 string feature를 사용하지 않는다면 &str을 넣어주면 됨
        // 그런데 &str은 item.get_name()이 &'static str이 아니기 때문에 to_owned()를 사용해야 함
        // 예를 들어 curstomerid 객체의 name은 String이다. 그런데 get_name()으로 name의 레퍼런스를 받아서
        // Arg의 long 함수에 넣어주게되면, Arg가 customerid에 대한 레퍼런스를 가지게 된다.
        // 그러면 Arg와 customerid는 서로 다른 라이프타임을 가지게 되어서 컴파일 에러가 발생한다.
        // 개발자는 Customerid가 arg보다 더 오래 존재한다고 생각하지만, 사실상 Rust 컴파일러가 customerid와 arg 중에
        // 어느 것을 먼저 해지할지는 알 수 없다. 따라서 이러한 의존성으로 생기는 라이프타임 문제를 해결하기 위해서는
        // Arg에 레퍼런스를 넣어주는 것이 아니라 String 객체를 넣어주어서 라이프타임에 대한 의존성을 없애야만한다.
        command = command.arg(
            Arg::new(item.get_name().to_owned())
                .long(item.get_arg_name().to_owned())
                .help(item.get_help())
                .required(item.get_mandatory()),
        );
    }

    let matches = command.get_matches();

    for item in items.iter_mut() {
        if let Some(data) = matches.get_one::<String>(item.get_name()) {
            item.put_rawdata(data.as_str());
        }
    }

    let plain_serial = generate_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter() {
        if let Some(_rawdata) = item.get_rawdata() {
            let len = item.get_length();
            let rawdata = &dec[offset..offset + len];
            println!("Verify {}: {}", item.get_name(), rawdata);
            println!("Verify result: {}", item.verify(rawdata));
            offset += len;
        }
    }
}
```

각 옵션의 ID를 각 입력 데이터 구조체의 get_name 메소드로 얻어옵니다.
옵션 이름도 get_arg_name으로 얻어오고, 도움말 메세지도 get_help로 얻어옵니다.

한가지 특이한게 있는데 제가 주석으로 잔뜩 설명을 써놓은 부분입니다.
Cargo.toml에 clap의 "string" 기능을 사용하도록 `features = ["string"]` 옵션을 추가했습니다.
그래서 Arg 객체의 new, long, help 등의 메소드에 문자열에 대한 레퍼런스가 아니라 String 객체를 인자로 사용하게되었습니다.
```rust
            Arg::new(item.get_name())
                .long(item.get_arg_name())
                .help(item.get_help())
                .required(item.get_mandatory()),
        );
```

왜 문자열에 대한 레퍼런스가 아니라 객체를 그대로 전달해서 소유권을 넘기는게 필요할까요?
이 문제는 눈에 보이지 않는 수명에 관한 문제라서 처음 접하게 되면 당황할 수 있습니다.
new, long, help 메소드에 productid, customerid등의 객체가 가진 name 필드의 레퍼런스를 전달합니다.
그래서 결국 Arg 객체에 다른 객체가 가진 데이터의 레퍼런스가 저장됩니다.
여기서 수명의 문제가 생깁니다.
Arg타입 객체 arg가 productid의 name필드에 대한 레퍼런스를 가지고 있으므로, 두 객체 중에 어느 것이 더 메모리에 오래 있어야될까요?
당연히 productid가 더 오래 존재해야합니다. 만약 productid의 메모리가 해지되고, productid가 존재하던 메모리에 다른 데이터가 저장된다면 arg 객체는 완전히 다른 데이터에 접근하게됩니다.

여기까지는 쉽습니다.
하지만 우리 예제와 같이 arg객체와 productid객체가 main함수의 끝까지 존재한다면 어떻게 될까요?
main함수가 끝난다는 것은 프로그램이 끝난다는 것인데, 프로그램이 끝날 때 main함수가 가지고 있던 다양한 객체들을 해지할 때 arg객체와 productid객체중에 어느 것이 먼저 해지될까요?
사실 프로그램이 종료될 때 프로그램이 가진 메모리를 해지하는 것은 운영체제의 역할이기 때문에, 어느 객체가 먼저 해지될지 알 수 없습니다.
결국 러스트 컴파일러가 서로간에 참조되고 있는 객체들의 수명이 옳바른지를 알 수 없기 때문에, 컴파일 에러가 발생합니다.
그래서 arg객체에 productid객체의 레퍼런스를 저장하지 않도록, new, long, help 메소드에 String객체를 전달할 수 있어야 합니다.
기본적으로 Arg의 메소드들은 문자열에 대한 레퍼런스를 인자로 받습니다. 그게 대부분의 상황에서 더 편리하기 때문입니다.
하지만 우리 프로그램과 같이 옵션의 이름들이 다른 객체에 저장되어있는 경우 new, long, help 메소드들이 String객체를 전달받도록 바꿔야합니다.
Clap 크레이트의 개발자들은 이런 문제를 이미 알고있으므로 `features = ["string"]`를 통해서 메소드들이 받은 인자 타입을 바꿀 수 있도록 지원하고 있습니다.


### 연습문제

1. CustomerID 구조체 외에 다른 입력 데이터에도 get_arg_name, get_help, get_mandatory 메소드를 구현해보세요.

2. Cargo.toml에서 `features = ["string"]` 옵션을 없애고, Arg객체가 문자열의 레퍼런스를 받도록 수정해보세요. 어떤 에러가 나는지 확인해보세요. command나 arg, productid 객체에서 에러가 나는게 아니라 의외의 변수에서 에러가 나는 것을 확인할 수 있습니다. 또 에러 메세지에 수명 lifetime에 대한 에러라는 설명이 없고, 상관없는 에러가 발생하는 것처럼 보일 수 있습니다. `features = ["string"]` 옵션이 아닌 다른 방법으로 해결할 수 있는지 시도해보세요. 오래 다양한 시도를 해볼 수록 더 잘 이해할 수 있습니다.

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

    // 더 이상 사용자에게 입력을 받지 않고 명령행 인자로 처리
    let mut command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator");

    for item in items.iter() {
        // Cargo.toml에서도 Clap의 string feature를 사용하고 있기 때문에 String을 넣어줘야 함
        // Arg의 new, long, help, required 함수에 String을 넣어주기 위해 to_owned()를 사용
        // 만약에 Clap의 string feature를 사용하지 않는다면 &str을 넣어주면 됨
        // 그런데 &str은 item.get_name()이 &'static str이 아니기 때문에 to_owned()를 사용해야 함
        // 예를 들어 curstomerid 객체의 name은 String이다. 그런데 get_name()으로 name의 레퍼런스를 받아서
        // Arg의 long 함수에 넣어주게되면, Arg가 customerid에 대한 레퍼런스를 가지게 된다.
        // 그러면 Arg와 customerid는 서로 다른 라이프타임을 가지게 되어서 컴파일 에러가 발생한다.
        // 개발자는 Customerid가 arg보다 더 오래 존재한다고 생각하지만, 사실상 Rust 컴파일러가 customerid와 arg 중에
        // 어느 것을 먼저 해지할지는 알 수 없다. 따라서 이러한 의존성으로 생기는 라이프타임 문제를 해결하기 위해서는
        // Arg에 레퍼런스를 넣어주는 것이 아니라 String 객체를 넣어주어서 라이프타임에 대한 의존성을 없애야만한다.
        command = command.arg(
            Arg::new(item.get_name())
                .long(item.get_arg_name())
                .help(item.get_help())
                .required(item.get_mandatory()),
        );
    }

    let matches = command.get_matches();

    for item in items.iter_mut() {
        if let Some(data) = matches.get_one::<String>(item.get_name()) {
            item.put_rawdata(data.as_str());
        }
    }

    let plain_serial = generate_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter() {
        if let Some(_rawdata) = item.get_rawdata() {
            let len = item.get_length();
            let rawdata = &dec[offset..offset + len];
            println!("Verify {}: {}", item.get_name(), rawdata);
            println!("Verify result: {}", item.verify(rawdata));
            offset += len;
        }
    }
}
```

```bash
% cargo run --bin serial_project_step4
   Compiling my-rust-book v0.1.0 (/Users/user/study/quick-guide-rust-programming)
error[E0597]: `items` does not live long enough
   --> code/serial_project_step4/main.rs:120:17
    |
108 |     let mut items: Vec<Box<dyn GenSerialData>> = vec![
    |         --------- binding `items` declared here
...
120 |     for item in items.iter() {
    |                 ^^^^^-------
    |                 |
    |                 borrowed value does not live long enough
    |                 argument requires that `items` is borrowed for `'static`
...
167 | }
    | - `items` dropped here while still borrowed

error[E0502]: cannot borrow `items` as mutable because it is also borrowed as immutable
   --> code/serial_project_step4/main.rs:141:17
    |
120 |     for item in items.iter() {
    |                 ------------
    |                 |
    |                 immutable borrow occurs here
    |                 argument requires that `items` is borrowed for `'static`
...
141 |     for item in items.iter_mut() {
    |                 ^^^^^ mutable borrow occurs here

error[E0502]: cannot borrow `items` as mutable because it is also borrowed as immutable
   --> code/serial_project_step4/main.rs:147:40
    |
120 |     for item in items.iter() {
    |                 ------------
    |                 |
    |                 immutable borrow occurs here
    |                 argument requires that `items` is borrowed for `'static`
...
147 |     let plain_serial = generate_serial(&mut items);
    |                                        ^^^^^^^^^^ mutable borrow occurs here

Some errors have detailed explanations: E0502, E0597.
For more information about an error, try `rustc --explain E0502`.
error: could not compile `my-rust-book` (bin "serial_project_step4") due to 3 previous errors
```