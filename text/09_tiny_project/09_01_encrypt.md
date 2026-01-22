## 암호화

트레이트에 대한 본격적인 이야기를 하기 전에 이번 장에서는 보통 어떻게 새로운 크레이트(다른 언어에서 라이브러리라고 부르는)을 추가하고 사용하는지를 소개하겠습니다. 앞으로 Rust로 개발을 하면서 모든 걸 직접 만들 수 없으니까요. 분명히 어떤 기능이 필요할 때마다 저와 유사한 방식으로 이미 구현된 크레이트가 있는지 조사하고 사용하시게 될 것입니다.

일단 가장 많이 해볼 것은 역시 구글에서 검색하는 것입니다.
암호화가 필요하니까 구글에 "rust encryption"이라고 검색해보면 많은 자료들이 나올 것입니다.
지금 제가 이 글을 쓰고 있는 2025년 1월 8일에 검색해본 결과 첫번째 글은 러스트 언어 홈페이지에 있는 토론 글입니다.
왜 러스트에서 문자열을 암호화하는게 어려운지에 대한 글입니다.
좋은 토론이긴 하지만, 일단 우리는 당장 구현에  필요한 자료가 필요하니까 넘어갑니다.
그리고 2번째가 MagicCrypt 메뉴얼 사이트 https://docs.rs/magic-crypt/latest/magic_crypt/ 입니다.
3번째는 Openssl의 encrypt 모듈에 대한 메뉴얼 사이트 https://docs.rs/openssl/latest/openssl/encrypt/index.html 가 나오고 있습니다.
그 외에도 여러가지 크레이트들이 검색됩니다.

일단 가장 먼저 검색되는 2개의 크레이트를 조사해서 우리가 쓸만한 것인지, 어느 것을 사용할지 결정해보겠습니다.
러스트에서 사용하는 크레이트들을 관리하는 https://crates.io/ 사이트에서 해당 크레이트가 얼마나 오래동안 개발이 되고 현재도 개발이 활발하게 진행되고 있는지, 안정적인 버전을 출시했는지, 사용자가 얼마나 많은지 등을 확인해볼 수 있습니다.
우선 magic_crypt를 검색해서 https://crates.io/crates/magic-crypt 페이지를 열어보겠습니다.
페이지의 왼쪽에는 크레이트의 소개와 사용법등을 보여주고, 오른쪽에는 가장 최신 버전이 언제 출시되었는지, 라이선스는 무엇인지, 그리고 Cargo를 이용해서 어떻게 설치할 수 있는지 등을 보여줍니다.
해당 크레이트의 Github 주소도 있습니다.
그리고 페이지 아래쪽을 보면 "Stats Overview"라는 섹션에 있고 현재 391,263번 다운로드가 되었다고 알려줍니다.
총 31개의 릴리즈가 있었다고도 보여주네요.
제가 crates.io 사이트에서 crypt나 encrypt로 검색해보면 대부분의 크레이트들의 다운로드 횟수가 몇만번이 안되는 것을 보면 MagicCrypt가 많이 사용되는 크레이트라는 것을 알 수 있었습니다.
crates.io에서 openssl를 검색해보면 당연하게도 다운로드 횟수가 훨씬 높습니다. 1억번이 넘네요. 당연한 결과겠지요.

저는 다운로드 횟수 다음으로 예제 코드를 확인해봅니다. 각 크레이트의 홈페이지에 들어가보면 예제 코드를 보여줍니다.
Openssl의 encryp 모듈의 예제 코드와 MagicCrypt의 예제 코드를 보면 확연하게 MagicCrypt의 예제 코드가 단순하다는 것을 알 수 있습니다.
기능이 풍부한 크레이트는 사용법이 복잡한 경우가 많고, 기능이 단순한 크레이트가 사용법이 쉬운 경우가 많습니다.
저는 보통 시작할 때는 나한테 꼭 필요한 기능만 있다면 사용법이 쉬운 크레이트를 먼저 선택합니다.
개발을 하면서 내가 어떤 기능이 필요한가를 더 잘 이해하게되고, 사용중인 크레이트에 부족한 기능일 때 다른 크레이트로 변경하는 것을 선호합니다.
현재 저는 암호화에 대해서 별로 아는 것도 없으니 선택할 옵션이 없습니다.
그냥 어떻게든 암호화만 되면 충분합니다. 나중에 개발을 하면서 암호화에 대해 제가 더 잘 이해하게되고, 그러면서 더 세세한 옵션 선택을 할 수 있는 상황이 되면 좀 더 기능이 많은 크레이트를 사용할 수 있겠지요.
결국 지금은 시작 단계이니 Openssl보다 더 단순하고 직관적인 MagicCrypt를 사용하겠습니다.

사용할 크레이트가 결정되었으니 첫번째로 할 일은 Cargo를 써서 MagicCrypt 크레이트를 설치하는 것입니다.
https://crates.io/crates/magic-crypt 에서 2가지 설치 방법을 알려주고 있습니다.
첫번째는 `cargo add magic-crypt` 명령을 실행하는 것입니다.
이 명령은 cargo가 알아서 가장 최신 버전을 확인해서 Cargo.toml에 추가해줍니다.
두번째 방법은 개발자가 직접 Cargo.toml 파일에 `magic-crypt = "4.0.1"`를 입력하는 방법입니다.
최신 버전이 아니라 이전 버전을 사용해야될 때는 직접 Cargo.toml을 수정하는게 편리합니다.
우리는 특정 버전이 필요한 상황이 아니므로 cargo 툴을 사용해서 설치하겠습니다.

다음은 cargo툴을 이용해서 magic-crypt 크레이트를 추가한 결과입니다.

```bash
$ cargo add magic-crypt
    Updating crates.io index
      Adding magic-crypt v4.0.1 to dependencies
             Features:
             + std
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 22 packages to latest compatible versions
      Adding aes v0.8.4
      Adding base64 v0.22.1
      Adding block-buffer v0.10.4
      Adding block-padding v0.3.3
      Adding cbc v0.1.2
      Adding cfg-if v1.0.0
      Adding cipher v0.4.4
      Adding cpufeatures v0.2.16
      Adding crc-any v2.5.0
      Adding crypto-common v0.1.6
      Adding debug-helper v0.3.13
      Adding des v0.8.1
      Adding digest v0.10.7
      Adding generic-array v0.14.7 (latest: v1.1.1)
      Adding inout v0.1.3
      Adding libc v0.2.169
      Adding magic-crypt v4.0.1
      Adding md-5 v0.10.6
      Adding sha2 v0.10.8
      Adding tiger v0.2.1
      Adding typenum v1.17.0
      Adding version_check v0.9.5
```

magic-crypt의 4.0.1 버전을 설치한 후, 그 다음에 magic-crypt가 동작하기 위해 필요한 다른 크레이트들을 같이 설치해줍니다. 설치가 완료되었으면 Cargo.toml가 어떻게 바꼈는지를 보겠습니다.

```bash
$ git diff Cargo.toml
diff --git a/Cargo.toml b/Cargo.toml
index 5272213..2e4f18d 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -6,6 +6,7 @@ edition = "2021"
 # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

 [dependencies]
+magic-crypt = "4.0.1"
```

dependencies 섹션에 magic-crypt가 추가된 것을 볼 수 있습니다. 크레이트의 설치가 되었으니 이제 소스 코드에서 크레이트를 사용할 수 있습니다.

참고로 다음과 같이 메이저 버전만 지정해서 사용하는 경우도 많습니다. 아주 세부적으로 특정한 마이너 버전이 필요한 경우라면 메이저 버전만 지정하는 것도 좋은 방법입니다.

```
 [dependencies]
+magic-crypt = "4"
```

설치가 완료되었으니 MagicCrypt가 대략 어떻게 구현되었는지, 어떻게 사용하면 될지를 알아야겠지요.
가장 먼저 홈페이지에 있는 예제 코드를 분석해보겠습니다.
여러분이 확인해보실 시점에는 바뀔 수도 있겠지만, 제가 이 글을 쓰는 시점에는 다음과 같은 예제 코드를 보여주고 있습니다.

```rust
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

let mc = new_magic_crypt!("magickey", 256);

let base64 = mc.encrypt_str_to_base64("http://magiclen.org");

assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);

assert_eq!("http://magiclen.org", mc.decrypt_base64_to_string(&base64).unwrap());
```

첫줄에서 use 구문을 써서 new_magic_crypt와 MagicCryptTrait라는 것을 사용한다고 컴파일러에게 알려주고 있습니다.
new_magic_crypt는 바로 2번째 줄에서 사용하고 있는데요, !를 써서 호출하는 것을 보면 매크로 함수인 것을 알 수 있습니다.
그럼 MagicCryptTrait는 뭘까요? 이름에 Trait가 들어간 것을 보면 트레이트같은데 정말 트레이트가 맞을까요?
트레이트라면 어떤 함수들을 구현하는 트레이트일까요?
그걸 확인하기 위해서 crates.io에서 매뉴얼 페이지를 잘 봐야합니다.
https://crates.io/crates/magic-crypt 사이트에 들어가서 오른쪽 중간에 있는 Documentation 사이트의 링크(https://docs.rs/magic-crypt/4.0.1)를 누릅니다.
그러면 MagicCrypt의 매뉴얼 사이트에 접속하게됩니다.
웹페이지의 가장 윗쪽에 검색 메뉴가 있습니다. 여기에 MagicCryptTrait를 입력해보겠습니다.
그러면 https://docs.rs/magic-crypt/4.0.1/magic_crypt/trait.MagicCryptTrait.html 페이지에 접속하게됩니다.
이제 MagicCryptTrait라는게 정말 트레이트가 맞고, 다음과 같이 정의되어있다는 것을 확인할 수 있습니다.

```rust
pub trait MagicCryptTrait {
    // Required methods
    fn new<S: AsRef<[u8]>, V: AsRef<[u8]>>(key: S, iv: Option<V>) -> Self;
    fn encrypt_to_bytes<T: ?Sized + AsRef<[u8]>>(&self, data: &T) -> Vec<u8>;
    fn encrypt_reader_to_bytes(
        &self,
        reader: &mut dyn Read,
    ) -> Result<Vec<u8>, MagicCryptError>;
    fn encrypt_reader_to_writer2<N: ArrayLength<u8> + PartialDiv<U16> + IsGreaterOrEqual<U16, Output = True>>(
        &self,
        reader: &mut dyn Read,
        writer: &mut dyn Write,
    ) -> Result<(), MagicCryptError>;
    fn decrypt_bytes_to_bytes<T: ?Sized + AsRef<[u8]>>(
        &self,
        bytes: &T,
    ) -> Result<Vec<u8>, MagicCryptError>;
    fn decrypt_reader_to_bytes(
        &self,
        reader: &mut dyn Read,
    ) -> Result<Vec<u8>, MagicCryptError>;
......
```

메소드가 여러개가 있지만, 일단 예제에서 사용하고 있는 encrypt_str_to_base64 함수만 확인해보겠습니다.

```rust
    fn encrypt_str_to_base64<S: AsRef<str>>(&self, string: S) -> String { ... }
```

함수 정의만 보면 
* 인자로 받는 것은 `&str` 타입(`AsRef<str>`은 간단하게 생각하면 `&str`과 같습니다)이다.
* 반환값은 `String` 타입이다. 즉 새로운 `String` 객체를 생성해준다.

*AsRef도 트레이트입니다. 제가 MagicCryptTrait에 대해서 설명하고 있는 순서대로 메뉴얼 페이지에서 직접 검색해보시길 추천합니다.*

이제 예제 코드가 좀 이해가 됩니다. new_magic_crypt 매크로를 써서 MagicCryptTrait를 구현하는 객체(혹은 핸들러라고도 부르는)를 만듭니다.
그리고 encrypt_str_to_base64 메소드에 문자열 레퍼런스를 전달해서 최종적으로 암호화된 데이터가 있는 String을 생성하게됩니다.
encrypt_str_to_bytes는 문자열 레퍼런스를 받아서, 암호화된 데이터가 있는 바이트 배열을 반환하겠네요.
우리는 간단하게 바로 암호화된 결과물을 터미널에 출력해볼 것이니 BASE64 인코딩으로 암호문을 출력할 수 있게 만들어주는 encrypt_str_to_base64 함수가 가장 적합하겠습니다.
이 함수가 없었다면 encrypt_str_to_bytes를 호출해서 바이트 배열을 얻은 후 별도의 BASE64 관련 크레이트를 찾아서 사용했어야 했을 것입니다.
아니면 BASE64가 아닌 다른 인코딩을 하고 싶었더라면 다른 인코딩 크레이트를 찾아야겠지요.

어쨌든 우리는 굳이 다른 인코딩을 사용할 필요가 없으므로 예제 코드를 그대로 시리얼 키 생성에 사용하겠습니다.
이전에 평문으로 만들었던 serial이라는 문자열 변수를 그대로 encrypt_str_to_base64에 전달하면 되겠네요.
이전장에서 단순히 입력 데이터를 합쳐서 시리얼로 보여주었지만, 이번 장에서는 아래와같이 합쳐진 입력 데이터를 암호화해서 시리얼을 생성하겠습니다.

```rust
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customerid = Some(get_user_input());

    println!("Please input 8-digits Product ID: ");
    let productid = Some(get_user_input());

    let plain_serial = format!("{}{}", customerid.unwrap(), productid.unwrap());
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);
    let verify_customerid = &dec[0..4];
    let verify_productid = &dec[4..12];
    println!("Verify Customer ID: {}", verify_customerid);
    println!("Verify Product ID: {}", verify_productid);
}
```

```bash
$ cargo run --bin serial_project_step2
   Compiling cfg-if v1.0.0
   Compiling debug-helper v0.3.13
   Compiling base64 v0.22.1
......
   Compiling cbc v0.1.2
   Compiling des v0.8.1
   Compiling magic-crypt v4.0.1
   Compiling my-rust-book v0.1.0 (/Users/user/study/quick-guide-rust-programming)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.97s
     Running `target/debug/project_step1`
Please input 4-digits Customer ID: 
1234
Please input 8-digits Product ID: 
qwerasdf
Plain serial: 1234qwerasdf
Encrypted serial: 3OvuVy1IXj5veDI61Mszjg==
Decrypted serial: 1234qwerasdf
Verify Customer ID: 1234
Verify Product ID: qwerasdf
```

최초로 실행할 때는 MagicCrypt 크레이트와 그 외 연관 크레이트들을 빌드한 후 프로그램을 실행합니다.
두개의 입력 데이터를 합친 plain_serial의 레퍼런스를 encrypt_str_to_base64에 전달해서 암호화되고 BASE64로 인코딩된 시리얼 키를 생성했습니다.
예를 들어 사용자 번호 1234와 제품 번호 qwerasdf를 입력해서 나온 시리얼 키는 GPghOzaNUn7G7FKiAkhKQQ==입니다.
마지막으로 decrypt_base64_to_string에 시리얼 키를 전달하면 시리얼 키를 복호화해서 원래 데이터 1234abcdabcd를 보여줍니다.
암호화, 복호화가 잘 동작하는 것을 확인할 수 있었습니다.

### 연습문제

1. 참고로 트레이트의 메뉴얼을 찾아보는 과정에 대해서 소개했습니다만 new_magic_crypt라는 매크로에 대해서는 소개를 안했습니다.
직접 한번 메뉴얼 페이지를 검색해서 어떤 일을 하는 매크로인지 찾아보시기 바랍니다.
어떤 타입의 객체를 생성하는 것인지, 2개의 인자는 각각 어떤 의미를 갖는지를 확인해보시면, 나중에 좀 더 다양한 옵션을 사용하는데 도움이 될 것입니다.

2. BASE64에 대해서도 조사해보세요.
특히 위 예제에서 생성한 시리얼 키 GPghOzaNUn7G7FKiAkhKQQ==에서 마지막에 있는 "=="가 어떤 의미인지를 확인해보시기 바랍니다.
보통의 시리얼 키에는 "="라는 문자가 없는데 왜 우리가 만든 시리얼 키에는 "="가 있을까요? 사실은 "=="를 생략해도 괸찮습니다만 왜 그럴까요?

3. 저는 구글에서 암호화 크레이트를 검색해봤습니다만 사실 요즘에는 더 좋은 개발 보조 도구가 있습니다.
ChatGPT나 Copilot등에 암호화에 대해 좋은 크레이트를 추천해달라고 요청해보세요.
예제 코드도 만들어달라고하면 좋은 예제를 보여주고, 예제에 대한 상세한 설명도 해줍니다.