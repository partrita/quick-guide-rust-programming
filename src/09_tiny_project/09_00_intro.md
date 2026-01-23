# 시리얼번호 생성기 프로젝트

## 프로젝트 소개

이전에 스마트 포인터 Box를 소개하면서 잠깐 시리얼 키를 생성하는 예제를 만들었습니다. 프로그램 사용자가 사용자 ID와 제품 ID를 입력하면, 프로그램은 입력받은 두 문자열을 합쳐서 하나의 시리얼 키를 생성하는 예제였습니다. GenSerialData라는 트레이트를 만들어서 사용자 ID와 제품 ID를 위한 공통 인터페이스를 만들어서 사용자 입력을 받고 시리얼을 생성하는데 사용했었습니다.

트레이트를 만들면 공통 인터페이스를 만들 수 있고, 트레이트 객체를 만들어서 특정 트레이트를 구현한 객체들을 한꺼번에 관리할 수 있습니다. 이렇게 트레이트의 기능에 대한 이해를 했다고 해서 곧바로 트레이트를 적용해서 프로그램을 만들기는 쉽지 않습니다. 그래서 어쩌라는 건가 생각이 들 수도 있습니다. 이번장에서는 시리얼 키를 생성하는 프로그램을 아주 단순한 형태부터 점점 유지보수가 편리할 실제 제품에 가까운 형태로 발전시켜나가면서 트레이트를 언제 어떻게 사용해야하고, 그래서 왜 트레이트가 중요한지에 대해서 이야기를 해보려고 합니다.

우선 가장 단순하게 시작해보겠습니다. 만약에 팀장님이나 다른 동료로부터 시리얼 키를 만드는 프로그램을 만들어달라고 부탁을 받으면 어떻게 시작하셨겠나요? 저라면 어떤 입력 데이터를 가지고 시리얼 키를 만들면 되는지 물어보겠습니다. 예를 들어 고객ID와 제품ID가 있다는 말을 들으면 아마 다음과 같이 만들었을것입니다.

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

제가 만약 고객ID와 제품ID라는게 사실 처음 그런게 있다는걸 알았을때에는 어떻게 처리해야될지 모를 수도 있습니다. 어디에선가 어설프게나마 배운데로 영업부서나 마케팅팀에 가서 고객ID와 제품ID가 무엇인지를 조사하면서 요구사항을 수집했을 것입니다. 만약 예를 들어서 각각 4글자, 8글자의 문자와 숫자로된 데이터라는 것을 알았으면 위와 같이 프로그램 사용자에게 4개의 문자로 된 고객 번호와 8개의 문자로 된 제품 번호를 입력받는 프로그램을 만들었을 것입니다. 

이 프로그램은 단순히 두개의 입력 문자열을 하나로 붙여서 시리얼 번호를 만들어서 보여줍니다. 이 예제는 사실상 시리얼 키를 생성하는 기능은 전혀 없고 단순히 2가지 입력 데이터를 받기 위한 예제였습니다만 어쨌든 대략 이렇게 시작했을 것입니다. 이정도로 만들어서 잘 동작하는 것을 확인하고, 이 프로그램을 사용할 사용자에게 이정도로 동작하는 프로그램이면 어떠냐고 물어보고, 나름 요구사항등을 수집하려는 시도를 했을 것입니다.

프로그램 사용자가 이정도면 쓸 수 있겠다는 피드백과 함께 암호화만 추가해달라고 리뷰를 해줬다고 생각해보겠습니다. 그런데 우리가 일을 하다보면 항상 겪는 일이 있습니다. 동료에게 요구사항을 받고, 리뷰까지 받고 개발을 하다보면 어떤 일이 벌어지나요? 항상, 예 정말 말 그대로 항상 요구사항은 변합니다. 보통은 변할 뿐 아니라 늘어나기까지 합니다. 예전에 이정도면 충분하다고 리뷰를 해주지 않았냐고 하소연을 해봐도 거의 항상 요구사항은 변하고, 변하는 것만이 아니라 새로운 요구사항이 들어오고, 결국 프로그램은 복잡해집니다. 아마 조금이라도 그런 경험을 해보신 분이라면 지금 저 프로그램은 전혀 요구사항 변화를 수용할 수 없다는 것을 이해하실 것입니다. 아직 그런 경험을 해보지 못하신, 이제 막 소프트웨어 개발에 입문하신 분이라면 미리 마음의 준비를 하시길 부탁드립니다. 요구사항은 절대 고정될 수 없고, 소프트웨어 개발 중만이 아니라 다 끝나고 출시된 이후에도 늘 반드시 변화하고, 늘어난다는 것을 기억해주세요.

### 연습문제

1. 혹시 시리얼 번호를 생성하는 프로그램을 만들어본 적이 없다면 프로그램을 디자인해보세요. 어떠한 요구사항이 있고, 어떤 설계로 프로그램을 만들 수 있을지 생각해보세요.

2. 리눅스 커널의 가상 파일 시스템 (Virtual Filesystems)이 어떻게 여러개의 파일시스템(FAT32, EXT4, Btrfs)등을 동시에 지원할 수 있는지 생각해보신 적이 있나요? 관련 자료를 찾아서 읽어보세요.

3. 리눅스 커널은 모놀리틱 커널입니다. 커널이라는 프로그램은 하나의 바이너리 파일입니다. 그런데 어떻게 동적으로 하드웨어 드라이버를 설치하거나 삭제할 수 있을까요? 관련 자료를 찾아서 읽어보세요. 추후에 각자의 현업에 적용할 수 있는 아이디어를 얻을 수 있을지도 모릅니다.