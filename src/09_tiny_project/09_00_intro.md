# 시리얼 번호 생성기 프로젝트

## 프로젝트 소개

앞서 스마트 포인터 `Box<T>`를 다룰 때, 사용자 ID와 제품 ID를 조합해 시리얼 키를 생성하는 간단한 예제를 만들어 보았습니다. 당시 `GenSerialData` 트레이트를 정의해 공통 인터페이스를 만들고 여러 객체를 하나의 벡터로 관리하는 법을 배웠습니다.

하지만 트레이트의 문법을 아는 것과 이를 실제 설계에 녹여내는 것은 별개의 문제입니다. 이번 장에서는 가장 단순한 형태의 프로그램에서 시작해, 점차 유지보수가 편리하고 실제 제품에 가까운 구조로 발전시켜 나가며 트레이트가 왜 필요한지, 그리고 언제 사용해야 하는지 몸소 체험해 보겠습니다.

먼저 고객 ID(4자리)와 제품 ID(8자리)를 입력받아 단순히 이어 붙이는 초기 버전을 만들어 봅시다.

```rust
// code/serial_project_step1/main.rs
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("입력이 올바르지 않습니다.");
    s.trim().to_string() // 줄바꿈 제거 후 반환
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customerid = get_user_input();

    println!("Please input 8-digits Product ID: ");
    let productid = get_user_input();

    let plain_serial = format!("{}{}", customerid, productid);
    println!("Plain serial: {}", plain_serial);
}
```

이 정도 수준으로도 기능은 동작합니다. 하지만 실무에서는 "요구사항은 반드시 변하고 늘어난다"는 진리가 우리를 기다리고 있습니다.

사용자가 "여기에 암호화 기능을 추가해 주세요", "고객 등급 정보도 넣어주세요"라고 요청한다면 현재의 코드는 금세 복잡해지고 중복으로 가득 차게 될 것입니다. 변화에 유연하게 대응할 수 있는 구조를 고민해야 할 시점입니다.

### 연습문제

1.  본인만의 시리얼 번호 규칙을 설계해 보세요. 어떤 데이터가 포함되어야 할까요?
2.  리눅스 커널의 가상 파일 시스템 (`VFS`)이 어떻게 수많은 파일 시스템을 동일한 인터페이스로 지원하는지 조사해 보세요. 트레이트의 철학을 이해하는 데 큰 도움이 될 것입니다.
3.  동적으로 하드웨어 드라이버를 설치/삭제하는 구조에 대해 알아보세요. 이는 나중에 프로젝트를 플러그인 구조로 확장할 때 좋은 영감을 줄 것입니다.
