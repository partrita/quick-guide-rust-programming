# 시리얼 번호 생성기 프로젝트

## 프로젝트 소개

이전에 스마트 포인터 `Box`를 소개하면서 잠깐 시리얼 키를 생성하는 예제를 만들었습니다. 프로그램 사용자가 사용자 ID와 제품 ID를 입력하면, 프로그램은 입력받은 두 문자열을 합쳐서 하나의 시리얼 키를 생성하는 예제였습니다. `GenSerialData`라는 트레이트를 정의하여 사용자 ID와 제품 ID를 위한 공통 인터페이스를 만들고, 사용자 입력을 받아 시리얼 키를 생성하는 데 사용했었습니다.

트레이트를 활용하면 공통 인터페이스를 구축할 수 있고, 트레이트 객체를 생성하여 특정 트레이트를 구현한 객체들을 한꺼번에 관리할 수 있습니다. 하지만 트레이트의 기능에 대해 이해했다고 해서 곧바로 실제 프로그램에 적용하기는 쉽지 않습니다. 그래서 "실제로 어떻게 활용해야 하는가"라는 의문이 생길 수 있습니다. 이번 장에서는 시리얼 키를 생성하는 프로그램을 아주 단순한 형태부터 시작하여, 점점 유지보수가 편리하고 실제 제품에 가까운 형태로 발전시켜 나가면서 트레이트를 언제 어떻게 사용해야 하는지, 그리고 왜 트레이트가 중요한지에 대해 이야기해 보려 합니다.

우선 가장 단순하게 시작해 보겠습니다. 만약 팀장님이나 동료로부터 시리얼 키 생성 프로그램을 만들어 달라는 부탁을 받으면 어떻게 시작하시겠습니까? 저라면 우선 어떤 입력 데이터를 가지고 시리얼 키를 만들면 되는지 물어볼 것입니다. 예를 들어 고객 ID와 제품 ID가 필요하다는 답변을 들었다면, 아마 다음과 같이 구현했을 것입니다.

```rust
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

    let verify_customerid = &plain_serial[0..4];
    let verify_productid = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customerid);
    println!("Verify Product ID: {}", verify_productid);
}
```

```bash
$ cargo run --bin serial_project_step1
   Compiling my-rust-book v0.1.0 (/home/gkim/study/quick-guide-rust-programming)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/serial_project_step1`
Please input 4-digits Customer ID: 
1234
Please input 8-digits Product ID: 
qwerasdf
Plain serial: 1234qwerasdf
Verify Customer ID: 1234
Verify Product ID: qwerasdf
```

만약 고객 ID와 제품 ID라는 개념을 처음 접했다면 어떻게 처리해야 할지 막막할 수도 있습니다. 이럴 때는 영업 부서나 마케팅 팀에 문의하여 고객 ID와 제품 ID가 무엇인지 조사하며 요구사항을 수집해야 합니다. 예를 들어 각각 4글자, 8글자의 문자와 숫자로 된 데이터라는 것을 파악했다면, 위와 같이 사용자에게 4자리의 고객 번호와 8자리의 제품 번호를 입력받는 프로그램을 만들 수 있습니다.

이 프로그램은 단순히 두 개의 입력 문자열을 하나로 붙여 시리얼 번호를 생성하고 보여줍니다. 사실상 본격적인 시리얼 키 생성 기능은 없지만, 어쨌든 두 가지 데이터를 입력받는 것으로 시작할 수 있습니다. 이 정도 수준으로 구현하여 잘 동작하는 것을 확인한 후, 사용자에게 피드백을 받아 요구사항을 구체화해 나가는 시도가 필요합니다.

사용자가 "이 정도면 쓸 수 있겠다"라는 피드백과 함께 암호화 기능만 추가해 달라고 요청했다고 가정해 봅시다. 그런데 실무에서는 항상 겪는 일이 있습니다. 동료에게 요구사항을 받고 리뷰를 거쳐 개발을 진행하다 보면, 정말 말 그대로 **항상** 요구사항은 변하기 마련입니다. 보통은 변할 뿐만 아니라 늘어나기까지 합니다. "예전에 이 정도면 충분하다고 하지 않았느냐"고 하소연해 봐도 소용없습니다. 결국 프로그램은 복잡해집니다. 조금이라도 개발 경험이 있는 분이라면 현재의 코드가 요구사항 변화를 유연하게 수용할 수 없다는 점을 이해하실 것입니다. 이제 막 소프트웨어 개발에 입문하신 분이라면 미리 마음의 준비를 하시기 바랍니다. 요구사항은 절대 고정될 수 없으며, 개발 중은 물론 출시 이후에도 반드시 변화하고 늘어난다는 점을 기억해 주세요.

### 연습문제

1. 혹시 시리얼 번호 생성 프로그램을 만들어 본 적이 없다면 직접 설계해 보세요. 어떠한 요구사항이 필요하고, 어떤 설계로 프로그램을 만들 수 있을지 생각해 보세요.

2. 리눅스 커널의 가상 파일 시스템(Virtual Filesystems)이 어떻게 여러 개의 파일 시스템(FAT32, EXT4, Btrfs 등)을 동시에 지원할 수 있는지 조사해 보세요.

3. 리눅스 커널은 모놀리틱 커널이며 하나의 바이너리 파일입니다. 그런데 어떻게 동적으로 하드웨어 드라이버를 설치하거나 삭제할 수 있을까요? 관련 자료를 찾아 읽어 보시기 바랍니다. 이는 나중에 현업에 적용할 수 있는 좋은 아이디어가 될 수 있습니다.
