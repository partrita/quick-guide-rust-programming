# 스레드 (Thread)

## 동기화 도구 이해하기

`Rust`에서 스레드를 안전하게 다루기 위해 반드시 알아야 할 도구들을 먼저 소개합니다.

### 1. 스레드 간 공유를 위한 Arc

`Arc`는 `Atomic Reference Counter`의 약자입니다. 여러 스레드가 하나의 데이터를 안전하게 공유할 수 있게 해주며, 데이터가 더 이상 사용되지 않을 때 메모리를 자동으로 해제합니다.

```rust
let shared_data = Arc::new(vec![1, 2, 3]);
let thread_ref = shared_data.clone(); // 레퍼런스 카운트 증가
```

주의할 점은 `Arc` 자체만으로는 데이터를 수정할 수 없다는 것입니다. `Rust`의 안전성 규칙 때문인데, 여러 스레드에서 동시에 데이터를 고치려 하면 데이터 경합이 생길 수 있기 때문입니다. 수정을 원한다면 `Mutex`나 `RwLock`과 조합해야 합니다.

### 2. 기본 자료형을 위한 Atomic 타입

`usize`, `i32` 같은 단순한 숫자를 공유할 때는 `AtomicUsize`, `AtomicI32` 등을 사용합니다.

```rust
let counter = Arc::new(AtomicUsize::new(0));
counter.store(1, Ordering::SeqCst); // 값 저장
let val = counter.load(Ordering::SeqCst); // 값 읽기
```

여기서 `Ordering`은 메모리 연산의 순서를 결정하는 정책입니다. 잘 모를 때는 가장 강력한 보장을 제공하는 `Ordering::SeqCst`를 사용하는 것이 안전합니다.

### 3. 뮤텍스 (Mutex)

한 번에 단 하나의 스레드만 데이터에 접근하도록 제어하는 락(`Lock`)입니다. `Rust`의 뮤텍스는 데이터와 락이 하나로 묶여 있어, 락을 얻지 않고는 데이터에 접근하는 것이 문법적으로 불가능합니다.

```rust
let data = Arc::new(Mutex::new(0));
{
    let mut num = data.lock().unwrap(); // 락 획득
    *num += 1;
} // 스코프를 벗어나면 자동으로 락이 해제됨 (RAII 패턴)
```

### 4. 채널 (Channel)

스레드 간에 메시지를 주고받는 통로입니다. `Rust` 표준 라이브러리는 `MPSC`(`Multi-Producer, Single-Consumer`) 방식의 채널을 제공합니다. 여러 곳에서 보내고 한 곳에서 받는 구조에 적합합니다.

## 스레드 생성과 결과 받기

`std::thread::spawn` 함수로 스레드를 생성합니다.

```rust
let handle = thread::spawn(move || {
    // move 키워드는 주변 변수의 소유권을 스레드 안으로 가져옵니다.
    "작업 완료"
});

let result = handle.join(); // 스레드가 끝날 때까지 대기하고 결과 받기
```

## Send와 Sync 트레이트

`Rust` 컴파일러가 스레드 안전성을 검사하는 핵심 기준입니다.
*   `Send`: 소유권을 다른 스레드로 안전하게 넘길 수 있는 타입.
*   `Sync`: 여러 스레드에서 참조(`&T`)를 동시에 사용해도 안전한 타입.

우리가 만든 구조체가 이 트레이트들을 만족하지 못하면 멀티 스레드 환경에서 아예 컴파일조차 되지 않습니다. `Rust`는 이처럼 언어 차원에서 데이터 경합을 원천 봉쇄합니다.
