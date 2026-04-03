## 암호화

모든 기능을 직접 구현하기보다는 이미 검증된 라이브러리(크레이트, `Crate`)를 활용하는 것이 현대적인 개발 방식입니다. 시리얼 번호를 안전하게 보호하기 위해 암호화 기능을 추가해 보겠습니다.

### 크레이트 선정

[crates.io](https://crates.io)에서 "rust encryption"을 검색하면 수많은 결과가 나옵니다. 우리는 다음과 같은 기준으로 `magic-crypt`를 선택했습니다.

*   사용자 수: 수십만 건의 다운로드 횟수로 검증됨.
*   난이도: `OpenSSL`보다 훨씬 직관적이고 예제 코드가 단순함.
*   활발한 관리: 지속적으로 업데이트가 이루어지고 있음.

초기 단계에서는 나에게 꼭 필요한 기능만 담고 있으면서 사용법이 쉬운 도구를 고르는 것이 좋습니다. 나중에 더 정밀한 제어가 필요해지면 그때 더 복잡한 크레이트로 교체하면 됩니다.

### 라이브러리 설치

`cargo add` 명령으로 프로젝트에 추가합니다.

```bash
$ cargo add magic-crypt
```

이 명령을 실행하면 `Cargo.toml`의 `[dependencies]` 섹션에 `magic-crypt = "4.0.1"`과 같이 자동으로 기록됩니다.

### 코드 적용

`MagicCrypt` 홈페이지의 예제 코드를 참고하여 우리 프로젝트에 적용해 보겠습니다. `AES256` 알고리즘을 사용해 데이터를 암호화하고, 출력하기 편하도록 `BASE64` 인코딩을 적용합니다.

```rust
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn main() {
    // ... 입력 받는 로직 생략 ...
    let plain_serial = format!("{}{}", customerid, productid);

    // AES256 알고리즘 객체 생성
    let mc = new_magic_crypt!("magickey", 256);

    // 암호화 후 BASE64 인코딩된 문자열 반환
    let serial = mc.encrypt_str_to_base64(&plain_serial);
    println!("Encrypted serial: {}", serial);

    // 복호화하여 원래 데이터 확인
    let dec = mc.decrypt_base64_to_string(&serial).unwrap();
    println!("Decrypted serial: {}", dec);
}
```

이제 단순한 평문이 아닌, 암호화된 시리얼 키가 생성됩니다. 외부에서 시리얼 키만 봐서는 원래의 고객 ID나 제품 ID를 유추할 수 없게 되었습니다.

### 연습문제

1.  `new_magic_crypt!` 매크로의 인자 의미를 매뉴얼에서 찾아보세요. 키의 길이와 보안성 사이의 관계를 조사해 보면 좋습니다.
2.  `BASE64` 인코딩이 무엇인지, 그리고 암호화된 결과 끝에 붙는 `==`는 어떤 역할을 하는지 알아보세요.
3.  `ChatGPT`나 `Copilot`에게 다른 암호화 라이브러리를 추천받고 예제 코드를 비교해 보세요.
