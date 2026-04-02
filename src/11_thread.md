# 스레드(`Thread`)

## 스레드를 사용하기 위해 미리 알아두어야 할 것들

먼저 러스트(`Rust`)에서 제공하는 동기화 기법들을 소개하고, 그 다음 스레드(`Thread`)를 생성하여 사용하는 방법을 알아보겠습니다.

### 여러 스레드가 공유하여 사용할 수 있는 스마트 포인터 `Arc`

`Arc`는 `Atomic Reference Counter`의 약자입니다.
여러 스레드가 하나의 데이터를 공유할 때, 몇 번 공유되고 있는지를 관리합니다.
이미 `C`/`C++` 계열에서 직접 구현하여 사용하는 경우도 많고, 자바(`Java`) 등 가비지 컬렉터(`Garbage Collector`)가 있는 언어에서는 내부 메모리 관리 기법으로 사용되므로 익숙한 개념일 것입니다.

러스트에서 `Arc`가 아주 중요하고 스레드를 사용할 때 필수적인 이유는 러스트의 소유권을 관리하기 위해서입니다.
`C`/`C++` 언어에서는 뮤텍스(`Mutex`)와 같은 락(`Lock`)만 있으면 여러 스레드에서 데이터를 공유할 수 있습니다.
하지만 러스트는 소유권을 관리해야 하며, 데이터가 언제 해제되어야 하는지를 추적해야 하므로 별도의 레퍼런스 카운터가 필요합니다.

사용법은 간단합니다.
다음과 같이 `Arc`의 `new` 메서드를 이용하여 공유하고자 하는 데이터를 위한 `Arc` 객체를 만듭니다.
그리고 `clone`을 호출하면 레퍼런스 카운터가 증가하면서 각 스레드가 데이터에 접근할 수 있는 포인터가 생성됩니다.

```rust
fn main() {
    let myarc: std::sync::Arc<Vec<i32>> = std::sync::Arc::new(vec![1, 2, 3, 4, 5]);
    let myarc_ref: std::sync::Arc<Vec<i32>> = myarc.clone();
    println!("{:?}", myarc_ref);
}
```

```bash
[1, 2, 3, 4, 5]
```

주의해야 할 점은 `Arc`로 공유하는 데이터는 기본적으로 변경할 수 없다는 것입니다.
러스트 메모리 관리의 가장 중요한 규칙은 가변 레퍼런스(`&mut`)는 단 하나만 존재해야 한다는 것입니다.
여러 스레드가 `Arc`로 공유되는 데이터를 변경하려 한다면 가변 레퍼런스가 여러 개 존재하게 되므로 러스트의 규칙을 어기게 됩니다.

`Arc` 매뉴얼을 보면 가변 레퍼런스를 반환하는 `get_mut` 메서드가 있습니다.
하지만 이 메서드는 같은 데이터에 대한 다른 `Arc` 공유가 없을 때만 가변 레퍼런스를 얻을 수 있다고 명시되어 있습니다.
위 예제에서 `myarc`만 있을 때는 데이터가 공유되지 않은 상태이므로 가변 레퍼런스를 얻을 수 있지만, `myarc_ref`가 생성된 후에는 불가능합니다.
따라서 여러 스레드가 하나의 데이터를 수정하려면 단 하나의 가변 레퍼런스만 존재하도록 보장하는 락(`Lock`)을 사용해야 합니다.

### 기본 자료형을 공유할 수 있는 아토믹(`Atomic`) 타입

`usize`, `i32` 등 기본 자료형을 공유할 때는 `AtomicUsize`, `AtomicI32` 등의 아토믹 타입을 사용합니다.
다음과 같은 단계로 아토믹 변수의 공유 객체를 생성하여 사용할 수 있습니다.

1. `AtomicUsize::new` 등 `new` 메서드를 이용해 아토믹 변수를 만듭니다.
2. 스레드 간 공유를 위해 `Arc::new` 메서드로 아토믹 변수를 감싸는 `Arc` 객체를 만듭니다.
3. `Arc`의 `clone` 메서드를 사용해 각 스레드에 `Arc` 객체를 전달합니다.
4. `Arc` 타입 변수에서 `store`, `load` 등 아토믹 타입의 메서드를 그대로 사용합니다.

```rust
fn main() {
    let atomic_usize = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(1));
    let arc_usize = atomic_usize.clone();

    arc_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    assert_eq!(atomic_usize.load(std::sync::atomic::Ordering::Relaxed), 0);
}
```

주의할 점은 `store`, `load` 등의 메서드에 메모리 오더링(`Memory Ordering`)을 위한 `std::sync::atomic::Ordering` 타입을 전달해야 한다는 것입니다.
일반적으로 하나의 아토믹 변수만 읽고 쓰는 상황에서는 `Ordering::Relaxed`를 사용할 수 있습니다.

#### 메모리 오더링(`Memory Ordering`)에 대한 짧은 소개

메모리 오더링은 컴파일러가 아토믹 변수에 접근하는 명령어를 어떻게 배치하는지 지정하는 것입니다.
제품 출시 단계에서 반드시 고려해야 할 내용이므로 기본 개념을 이해할 필요가 있습니다.
여기서는 짧게 소개하지만, `C++`의 `Memory Ordering`이나 리눅스 커널의 `Memory Barrier` 자료를 찾아 상세히 이해하시길 추천합니다.

```rust
fn main() {
    let mut a;
    let mut b;
    let mut c;
    a = 1;
    b = 2;
    c = 3;
    println!("{}", a + b + c);
}
```

우리는 위와 같이 `a`, `b`, `c` 순서로 메모리에 값을 쓰려 합니다.
하지만 컴파일러는 최적화를 위해 이 순서를 지킬 필요가 없다고 판단할 수 있습니다.
`c`, `b`, `a` 순서로 써도 결과가 같기 때문입니다.
컴파일러는 성능을 위해 메모리 접근 순서를 바꿀 수 있는데, 이를 완전히 컴파일러에게 맡길 때 사용하는 것이 `Ordering::Relaxed`입니다.

```rust
fn main() {
    let atomic_usize = std::sync::atomic::AtomicUsize::new(1);
    atomic_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(1, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(2, std::sync::atomic::Ordering::Relaxed);
    println!(
        "{}",
        atomic_usize.load(std::sync::atomic::Ordering::Relaxed)
    );
}
```

위와 같이 하나의 변수만 사용할 때는 문제가 없습니다.
하지만 두 개 이상의 스레드에서 여러 아토믹 변수를 읽고 쓸 때는 순서가 중요해집니다.
예를 들어 특정 플래그(`Flag`) 값에 따라 동작하는 경우가 그렇습니다.

(`std::thread::spawn` 함수에 대해서는 뒤에서 자세히 다룹니다.)

```rust
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let mut handles = vec![];
    let atomic_flag = Arc::new(AtomicUsize::new(0));
    let atomic_data = Arc::new(AtomicUsize::new(0));

    let thr1_flag = atomic_flag.clone();
    let thr2_flag = atomic_flag.clone();
    let thr1_data = atomic_data.clone();
    let thr2_data = atomic_data.clone();

    // 스레드 1
    let handle_thr1 = thread::spawn(move || {
        thr1_data.store(1234, Ordering::Relaxed);
        thr1_flag.store(1, Ordering::Relaxed);
    });
    handles.push(handle_thr1);

    // 스레드 2
    let handle_thr2 = thread::spawn(move || {
        loop {
            if thr2_flag.load(Ordering::Relaxed) == 1 {
                println!(
                    "Do something with data {}",
                    thr2_data.load(Ordering::Relaxed)
                );
                break;
            }
        }
    });
    handles.push(handle_thr2);

    for h in handles {
        let _ = h.join();
    }
}
```

스레드 1을 생산자, 스레드 2를 소비자라고 생각하면 편합니다.
우리가 기대하는 순서는 생산자가 데이터를 쓰고 플래그를 세우면, 소비자가 이를 확인하고 데이터를 사용하는 것입니다.

1. `Thread-1`: `data = 1234`
2. `Thread-1`: `flag = 1`
3. `Thread-2`: `if flag == 1`
4. `Thread-2`: `use data`

하지만 `Ordering::Relaxed`를 사용하면 컴파일러가 순서를 바꿀 수 있습니다.

1. `Thread-1`: `flag = 1`
2. `Thread-2`: `if flag == 1`
3. `Thread-2`: `use data`
4. `Thread-1`: `data = 1234`

이 경우 소비자는 아직 0인 데이터를 처리하게 될 수도 있습니다.
이런 문제가 발생하면 디버깅이 매우 어렵습니다.

따라서 `Release`와 `Acquire` 타입이 필요합니다.
`Release`는 해당 연산 이전의 메모리 쓰기 명령들이 절대로 그 이후로 밀리지 않도록 보장합니다.

```rust
fn main() {
    let atomic_usize = std::sync::atomic::AtomicUsize::new(1);
    atomic_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(1, std::sync::atomic::Ordering::Release); // 이 지점을 기준으로 이전 쓰기 완료 보장
    atomic_usize.store(2, std::sync::atomic::Ordering::Relaxed);
}
```

`Acquire`는 메모리 읽기 연산에 대한 순서를 보장합니다.
`Acquire`로 읽는 지점 이후의 읽기 명령들이 그 이전으로 당겨지지 않도록 합니다.

따라서 두 스레드 간에 데이터를 올바르게 전달하려면 플래그를 쓸 때는 `Release`를, 읽을 때는 `Acquire`를 사용해야 합니다.

```rust
// ... 스레드 1
thr1_data.store(1234, Ordering::Relaxed);
thr1_flag.store(1, Ordering::Release);

// ... 스레드 2
if thr2_flag.load(Ordering::Acquire) == 1 {
    println!("{}", thr2_data.load(Ordering::Relaxed));
}
```

이렇게 하면 우리가 의도한 대로 항상 순서가 보장됩니다.

러스트의 `Ordering`은 5가지가 있습니다.
* `Relaxed`
* `Release`
* `Acquire`
* `AcqRel` (Acquire + Release)
* `SeqCst` (Sequential Consistency: 가장 강력한 보장)

잘 모르겠다면 모든 연산을 `SeqCst`로 처리한 뒤 성능에 따라 최적화하는 것도 방법입니다.

### 뮤텍스(`Mutex`)

뮤텍스는 하나의 스레드만 공유 데이터에 접근할 수 있게 하는 락(`Lock`)입니다.
러스트의 뮤텍스는 다른 언어와 달리 데이터와 뮤텍스가 결합되어 있습니다.
뮤텍스를 생성할 때 데이터를 내부에 저장하며, 이는 락을 걸지 않고 데이터에 접근하는 것을 원천적으로 방지합니다.

```rust
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let data: usize = 0;
    let data_lock = Mutex::new(data);
    let data_lock_share = Arc::new(data_lock);

    println!("Original data is {}", data);

    let data_thr1 = data_lock_share.clone();
    {
        // 스레드 1
        let mut data: MutexGuard<usize> = data_thr1.lock().unwrap();
        *data += 1;
        println!("Thread-1: data is {}", *data);
        // 여기서 자동으로 언락됨
    }

    let data_thr2 = data_lock_share.clone();
    {
        // 스레드 2
        let mut data: MutexGuard<usize> = data_thr2.lock().unwrap();
        *data -= 1;
        println!("Thread-2: data is {}", data);
    }
}
```

`lock` 메서드를 호출하면 `MutexGuard` 객체를 반환합니다.
이 객체는 `Deref`, `DerefMut` 트레이트를 구현하여 스마트 포인터처럼 데이터에 접근하게 해줍니다.
중요한 점은 `unlock` 메서드가 따로 없으며, `MutexGuard` 객체가 스코프를 벗어나 소멸될 때 자동으로 락이 해제된다는 것입니다.
이를 `RAII`(`Resource Acquisition Is Initialization`) 패턴이라고 부릅니다.

또한 `Lock Poisoning` 문제도 중요합니다. 락을 잡은 스레드가 패닉(`Panic`)으로 죽으면 락이 풀리지 않을 수 있습니다. 러스트의 뮤텍스는 이를 감지하여 `lock().unwrap()` 시에 에러를 발생시킵니다. 필요하다면 `into_inner()`를 통해 데이터를 복구할 수도 있습니다.

### 채널(`Channel`)

채널은 스레드 간 데이터를 주고받는 편리한 도구입니다.
러스트 표준 라이브러리는 `MPSC`(`Multi-Producer, Single-Consumer`) 방식의 채널을 제공합니다.

```rust
use std::sync::mpsc::{channel, Sender};

fn main() {
    let (sender, receiver) = channel();

    let sender_thr1: Sender<i32> = sender.clone();
    {
        // 스레드 1
        sender_thr1.send(1).unwrap();
    }

    let sender_thr2: Sender<i32> = sender.clone();
    {
        // 스레드 2
        sender_thr2.send(2).unwrap();
    }

    for _ in 0..2 {
        let t: i32 = receiver.recv().unwrap();
        println!("Main received {}", t);
    }
}
```

## 스레드(`Thread`) 생성 방법

`std::thread::spawn` 함수에 클로저나 함수를 전달하여 스레드를 생성할 수 있습니다.
반환값인 `JoinHandle`의 `join` 메서드를 호출하면 스레드가 종료될 때까지 기다릴 수 있습니다.

```rust
use std::{thread, time};

fn main() {
    let handle = thread::spawn(|| {
        let onesecond = time::Duration::from_millis(1000);
        for i in 0..5 {
            println!("In thread {}", i);
            thread::sleep(onesecond);
        }
    });

    let _ = handle.join();
}
```

스레드에 데이터를 전달할 때는 소유권(`Ownership`)을 고려해야 합니다.
참조를 전달하려 할 경우, 스레드가 원본 데이터보다 오래 살아남을 수 있다는 우려 때문에 컴파일 에러가 발생합니다.
이때 `move` 키워드를 사용하여 변수의 소유권을 스레드 내부로 완전히 넘겨야 합니다.

```rust
let handle = thread::spawn(move || thread_func_with_ref(&counter));
```

## 스레드 결과값 받는 방법

`JoinHandle`의 `join` 메서드는 `std::thread::Result<T>`를 반환합니다.
이는 스레드가 정상 종료되었는지, 아니면 패닉이 발생했는지를 포함합니다.

```rust
let handle = thread::spawn(|| thread_func(1));
let ret = handle.join();
match ret {
    Ok(r) => match r {
        Ok(v) => println!("Value: {}", v),
        Err(e) => println!("Error: {}", e.message),
    },
    Err(e) => println!("Panic!: {:?}", e),
}
```

## 스레드 사용을 위해 필요한 트레이트

스레드 간 데이터를 전달하거나 공유하려면 `Send`와 `Sync` 트레이트가 필요합니다.
* `Send`: 다른 스레드로 소유권을 보낼 수 있는 타입
* `Sync`: 여러 스레드에서 참조를 공유해도 안전한 타입

대부분의 기본 타입은 이를 자동으로 구현하지만, 로우 포인터(`Raw Pointer`)와 같이 컴파일러가 안전을 보장할 수 없는 경우에는 직접 마크 트레이트(`Mark Trait`)를 구현해주어야 합니다.

```rust
unsafe impl Send for MyData {}
unsafe impl Sync for MyData {}
```

싱글 스레드용 스마트 포인터인 `Rc`, `RefCell`, `Cell` 등은 `Send`나 `Sync`를 구현하지 않으므로 멀티 스레드 환경에서 사용할 수 없습니다.
이들을 멀티 스레드에서 쓰려면 `Arc`, `Mutex`, `RwLock` 등을 사용하도록 설계를 변경해야 합니다.
