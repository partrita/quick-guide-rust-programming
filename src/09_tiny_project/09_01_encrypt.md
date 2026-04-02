## 암호화

트레이트에 대한 본격적인 이야기를 하기 전에, 이번 장에서는 새로운 `Crate`(다른 언어의 라이브러리와 유사한 개념)를 추가하고 사용하는 방법을 소개하겠습니다. `Rust`로 개발할 때 모든 기능을 직접 구현할 수는 없기 때문입니다. 필요한 기능이 생길 때마다 이미 구현된 크레이트가 있는지 조사하고 이를 활용하게 될 것입니다.

우선 가장 먼저 할 수 있는 일은 구글 검색입니다. 암호화 기능이 필요하므로 구글에 "rust encryption"이라고 검색해 보면 많은 자료가 나올 것입니다. 이 글을 쓰는 시점에 검색해 본 결과, 첫 번째 글은 `Rust` 공식 홈페이지의 토론 글이었습니다. "왜 러스트에서 문자열 암호화가 어려운가"에 대한 내용인데, 좋은 토론이지만 당장 구현이 급한 우리에게는 적합하지 않으므로 넘어갑니다. 두 번째 검색 결과는 `MagicCrypt` 매뉴얼 사이트(https://docs.rs/magic-crypt/latest/magic_crypt/)였고, 세 번째는 `OpenSSL`의 `encrypt` 모듈에 대한 매뉴얼 사이트(https://docs.rs/openssl/latest/openssl/encrypt/index.html)였습니다. 그 외에도 여러 크레이트가 검색됩니다.

검색된 크레이트 중 상위 2개를 조사하여 어떤 것을 사용할지 결정해 보겠습니다. 러스트 크레이트 관리 사이트인 https://crates.io/에서 해당 크레이트가 얼마나 오랫동안 개발되었는지, 현재 활발히 유지보수되고 있는지, 안정적인 버전을 출시했는지, 그리고 사용자가 얼마나 많은지 등을 확인할 수 있습니다.

`magic-crypt`를 검색하여 페이지를 열어 보면, 왼쪽에는 크레이트 소개와 사용법이, 오른쪽에는 최신 버전 출시일, 라이선스 정보, 그리고 `Cargo`를 이용한 설치 방법 등이 나옵니다. 해당 크레이트의 GitHub 주소도 확인할 수 있습니다. 페이지 하단의 "Stats Overview" 섹션에 따르면 현재 391,263번 다운로드되었고 총 31개의 릴리스가 있었습니다. `crypt`나 `encrypt`로 검색했을 때 다른 크레이트들의 다운로드 횟수가 수만 번 정도인 것과 비교하면, `MagicCrypt`는 상당히 많이 사용되는 크레이트임을 알 수 있습니다. 참고로 `OpenSSL`은 1억 번 이상 다운로드되었습니다.

다운로드 횟수만큼 중요한 것은 예제 코드입니다. 각 크레이트의 홈페이지에서 제공하는 예제 코드를 비교해 보면, `OpenSSL`보다 `MagicCrypt`의 예제 코드가 확연히 단순하다는 것을 알 수 있습니다. 기능이 풍부한 크레이트는 사용법이 복잡한 경우가 많고, 단순한 크레이트는 사용법이 쉬운 편입니다.

저는 보통 시작할 때는 나에게 꼭 필요한 기능만 있다면 사용법이 쉬운 크레이트를 먼저 선택합니다. 개발을 진행하며 필요한 기능을 더 잘 이해하게 되었을 때, 부족함이 느껴진다면 그때 다른 크레이트로 교체하는 것을 선호합니다. 현재는 암호화에 대해 깊은 지식이 없으므로, 일단 암호화 기능만 제대로 동작하면 충분합니다. 나중에 암호화에 대해 더 잘 알게 되면 세세한 옵션을 선택할 수 있는 더 풍부한 기능의 크레이트를 사용할 수 있을 것입니다. 따라서 이번에는 시작 단계에 적합한 직관적인 `MagicCrypt`를 사용하기로 결정했습니다.

사용할 크레이트가 결정되었으면, 가장 먼저 `Cargo`를 이용해 `magic-crypt` 크레이트를 설치해야 합니다. `crates.io` 페이지에서는 두 가지 설치 방법을 안내합니다. 첫 번째는 `cargo add magic-crypt` 명령을 실행하는 것입니다. 이 명령은 `Cargo`가 자동으로 최신 버전을 확인하여 `Cargo.toml`에 추가해 줍니다. 두 번째 방법은 개발자가 직접 `Cargo.toml` 파일의 `[dependencies]` 섹션에 `magic-crypt = "4.0.1"`을 입력하는 방식입니다. 특정 버전을 고정해야 할 때는 직접 수정하는 것이 편리합니다. 여기서는 `Cargo` 도구를 사용하여 설치하겠습니다.

`Cargo`를 이용해 `magic-crypt` 크레이트를 추가한 결과는 다음과 같습니다.

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

`magic-crypt` 4.0.1 버전을 설치하면서, 해당 크레이트가 동작하는 데 필요한 다른 의존성 크레이트들도 함께 설치됩니다. 설치가 완료된 후 `Cargo.toml` 파일이 어떻게 바뀌었는지 확인해 보겠습니다.

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

`dependencies` 섹션에 `magic-crypt`가 추가된 것을 볼 수 있습니다. 이제 소스 코드에서 이 크레이트를 사용할 수 있습니다. 참고로 다음과 같이 메이저 버전만 지정하여 사용하는 경우도 흔합니다.

```toml
 [dependencies]
+magic-crypt = "4"
```

설치가 끝났으니 `MagicCrypt`가 대략 어떻게 구현되었고 어떻게 사용하는지 알아야겠죠. 홈페이지의 예제 코드를 분석해 보겠습니다.

```rust
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

let mc = new_magic_crypt!("magickey", 256);

let base64 = mc.encrypt_str_to_base64("http://magiclen.org");

assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);

assert_eq!("http://magiclen.org", mc.decrypt_base64_to_string(&base64).unwrap());
```

첫 줄에서 `use` 구문을 사용하여 `new_magic_crypt`와 `MagicCryptTrait`를 사용하겠다고 컴파일러에 알립니다. `new_magic_crypt`는 두 번째 줄에서 사용되는데, 끝에 `!`가 붙은 것을 보아 매크로 함수임을 알 수 있습니다. 그렇다면 `MagicCryptTrait`는 무엇일까요? 이름으로 보아 트레이트인 것 같습니다. 어떤 메서드들을 정의하고 있는지 확인하려면 매뉴얼 페이지를 살펴봐야 합니다.

`docs.rs`의 `MagicCrypt` 페이지(https://docs.rs/magic-crypt/4.0.1/magic_crypt/trait.MagicCryptTrait.html)에서 해당 트레이트가 다음과 같이 정의되어 있음을 확인할 수 있습니다.

```rust
pub trait MagicCryptTrait {
    // 필수 메서드들
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

메서드가 여러 개 있지만, 예제에서 사용 중인 `encrypt_str_to_base64` 메서드만 확인해 보겠습니다.

```rust
    fn encrypt_str_to_base64<S: AsRef<str>>(&self, string: S) -> String { ... }
```

정의를 보면 인자로 `&str` 타입을 받고(정확히는 `AsRef<str>`), 반환값으로 새로운 `String` 객체를 생성해 줍니다. 참고로 `AsRef` 또한 트레이트입니다.

이제 예제 코드가 이해됩니다. `new_magic_crypt` 매크로를 사용해 `MagicCryptTrait`를 구현한 객체를 생성합니다. 그리고 `encrypt_str_to_base64` 메서드에 문자열 참조를 전달하여 암호화된 `String` 데이터를 얻습니다. 만약 `encrypt_str_to_bytes`를 썼다면 바이트 배열(`Vec<u8>`)을 반환했을 것입니다. 우리는 암호화 결과를 터미널에 쉽게 출력하고 싶으므로, BASE64 인코딩을 적용해 주는 `encrypt_str_to_base64` 메서드가 가장 적합합니다.

이제 이 코드를 시리얼 키 생성에 적용해 보겠습니다. 이전 장에서 평문으로 만들었던 `plain_serial` 문자열을 그대로 암호화 메서드에 전달하면 됩니다.

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

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // 복호화 후 원래 데이터 복원
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

두 입력 데이터를 합친 `plain_serial`을 `encrypt_str_to_base64`에 전달하여 암호화된 시리얼 키를 생성했습니다. 마지막으로 `decrypt_base64_to_string`에 시리얼 키를 전달하여 원래의 데이터가 잘 복원되는 것을 확인할 수 있었습니다.

### 연습문제

1. 트레이트 매뉴얼을 찾아보는 방법은 소개했지만, `new_magic_crypt` 매크로에 대해서는 설명하지 않았습니다. 직접 매뉴얼 페이지를 검색하여 어떤 일을 하는 매크로인지 찾아보세요. 어떤 타입의 객체를 생성하며, 두 인자는 각각 어떤 의미인지 확인해 보는 것이 나중에 다양한 옵션을 사용하는 데 도움이 될 것입니다.

2. BASE64 인코딩에 대해서도 조사해 보세요. 특히 예제에서 생성한 시리얼 키 `GPghOzaNUn7G7FKiAkhKQQ==`의 마지막 "=="가 무엇을 의미하는지 확인해 보세요. 왜 시리얼 키에 "=" 문자가 포함되는지, 그리고 이를 생략해도 괜찮은 이유는 무엇인지 조사해 보시기 바랍니다.

3. 구글 검색뿐만 아니라 ChatGPT나 Copilot 같은 AI 도구를 활용해 보세요. 암호화에 적합한 크레이트를 추천받거나, 상세한 설명과 함께 예제 코드를 요청하여 학습해 보시기 바랍니다.
