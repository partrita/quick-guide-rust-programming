# 향후 과제 및 참고 자료

앞서 다루지 못한 중요한 주제들과 참고할 만한 크레이트(`Crate`) 및 라이브러리들을 정리합니다.

## 주요 크레이트 소개

* **`anyhow`**: 애플리케이션 수준의 에러 처리를 위한 유연한 에러 타입 제공
* **`clap`**: 커맨드 라인 인자 파서(`Command Line Argument Parser`)
    * 참고: [Rust CLI Book](https://rust-cli.github.io/book/index.html)
* **`serde`**: 데이터 직렬화 및 역직렬화(`Serialize`, `Deserialize`)를 위한 프레임워크
* **`tokio`**: 비동기(`Async`) 런타임 및 `HTTP` 서버 구축
* **`tcp` 네트워킹**: 에코 서버/클라이언트 구현 예제
* **파일 `I/O` 및 `grep` 툴**: 표준 라이브러리를 활용한 도구 제작

## 스레드 간 공유 메모리 `Rc`와 `Arc` - 작성 중(WIP)

## `std::pin` - 작성 중(WIP)

`std::pin`은 자기 참조(`Self-reference`)가 있는 구조체(예: 연결 리스트)의 객체가 메모리 위치를 이동하지 않도록 고정하는 역할을 합니다.

어떤 상황에서 메모리 위치가 변하는지는 다음 예제 코드를 통해 확인할 수 있습니다.

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct MyStruct {
    data: String,
    ptr: NonNull<String>,
}

struct MyStructPinned {
    data: String,
    ptr: NonNull<String>,
    _pin: PhantomPinned, // https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html
}

fn get_moved(moved: MyStruct) {
    println!("func-moved addr: {:p}", &moved);
}

fn get_pinned(moved: Pin<Box<MyStructPinned>>) {
    println!("func-pinned addr: {:p}", &*moved);
}

fn main() {
    // 변수가 스택 사이를 이동할 때 메모리 복사가 발생합니다.
    let mut first = MyStruct {
        data: "dummy".to_string(),
        ptr: NonNull::dangling(),
    };
    let slice = NonNull::from(&first.data);
    first.ptr = slice;

    println!("moved addr: {:p} {:p}", &first, first.ptr);
    let second = first;
    println!("moved addr: {:p} {:p}", &second, second.ptr); // 복사로 인해 주소값이 변함
    get_moved(second);

    // Pin된 변수의 경우 메모리 복사가 발생하지 않습니다.
    let first = MyStructPinned {
        data: "dummy".to_string(),
        ptr: NonNull::dangling(),
        _pin: PhantomPinned,
    };
    let mut first_pin = Box::pin(first); // 로컬 변수를 고정(Pin)
    let slice = NonNull::from(&first_pin.data);
    unsafe {
        let mut_ref: Pin<&mut MyStructPinned> = Pin::as_mut(&mut first_pin);
        Pin::get_unchecked_mut(mut_ref).ptr = slice;
    }

    println!("pinned addr: {:p} {:p}", &*first_pin, first_pin.ptr);
    let second_pin = first_pin;
    println!("pinned addr: {:p} {:p}", &*second_pin, second_pin.ptr); // 동일한 주소 유지
    get_pinned(second_pin);
}

/* 실행 결과 예시
moved addr: 0x7ffe3b6c0670 0x7ffe3b6c0670
moved addr: 0x7ffe3b6c0700 0x7ffe3b6c0670
func-moved addr: 0x7ffe3b6c0780
pinned addr: 0x561855b5bae0 0x561855b5bae0
pinned addr: 0x561855b5bae0 0x561855b5bae0
func-pinned addr: 0x561855b5bae0
*/
```

첫 번째 `first` 변수는 스택에 생성된 로컬 변수입니다. 이 객체를 다른 변수에 할당하면 복사가 일어나 `second` 객체의 주소는 `first`와 달라집니다. 만약 내부 포인터가 자신의 멤버를 가리키고 있었다면, 복사 후의 포인터는 엉뚱한 곳(이전 위치)을 가리키게 되어 위험합니다. `Pin`은 이러한 메모리 이동을 방지하여 안전한 참조를 보장합니다.
