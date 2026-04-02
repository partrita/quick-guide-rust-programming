# 비동기 프로그래밍(`Asynchronous programming`) 기본 개념 소개

## 비동기 프로그래밍(`Asynchronous programming`)이란 무엇인가?

서로 다른 여러 태스크(`Task`)를 동시에 실행하는 방식에는 크게 세 가지가 있습니다.
바로 병렬 프로그래밍(`Parallel programming`), 동시성 프로그래밍(`Concurrent programming`), 그리고 비동기 프로그래밍(`Asynchronous programming`)입니다. 각각의 차이를 간략히 알아보겠습니다.

1. **병렬 프로그래밍**: 대표적인 예가 `GPU` 연산입니다. 물리적으로 여러 개의 연산 장치가 서로 다른 데이터를 가지고 동일한 연산을 실제로 동시에 수행하는 방식입니다.
2. **동시성 프로그래밍**: 1개의 코어를 가진 `CPU`가 여러 스레드(`Thread`)를 아주 빠르게 번갈아 가며 실행하여, 사용자에게는 마치 동시에 실행되는 것처럼 보이게 하는 방식입니다. 스레드 전환(`Context Switching`)이 핵심이며, 이를 위해 다양한 락(`Lock`)이나 스케줄링 기법이 필요합니다.
3. **비동기 프로그래밍**: 어떤 작업(`Task`)을 명령한 뒤, 그 작업이 끝날 때까지 기다리지 않고 다른 작업을 수행할 수 있는 방식입니다. 실생활의 예로, 은행에 대출 서류를 제출한 뒤 결과가 나올 때까지 은행 창구에서 기다리지 않고 일상생활을 계속하다가, 연락이 오면 다음 단계를 진행하는 것과 같습니다.

거의 모든 운영체제와 프로그래밍 언어는 나름의 비동기 처리 기법을 제공합니다. 최신 리눅스 커널은 `epoll`이나 `io_uring` 같은 비동기 파일 입출력(`I/O`) 처리 기법을 제공합니다. 전통적인 `read`/`write` 시스템 콜은 데이터 처리가 완료될 때까지 함수가 블록(`Block`)되지만, 비동기 방식은 명령을 내린 후 다른 일을 하며 처리가 끝나기를 기다릴 수 있습니다.

스레드를 이용하는 동시성 프로그래밍과 비슷해 보일 수 있지만, 다음과 같은 중요한 차이가 있습니다.

1. **효율성**: 스레드 기반 방식은 프로세서 개수만큼의 태스크만 실제로 동시에 실행할 수 있으며, 스레드 생성 및 제거 비용이 큽니다. 비동기 프로그래밍은 이러한 제약 없이 더 많은 태스크를 동시에 관리할 수 있습니다.
2. **오버헤드**: 아주 작은 태스크를 수천 개 실행해야 할 때, 스레드를 만들고 전환하는 비용(`Context Switching`)은 실제 작업 시간보다 커질 수 있습니다. 비동기 프로그래밍은 스레드 관리 비용 없이 순수하게 태스크만 효율적으로 처리할 수 있게 해줍니다.

## `Tokio` 소개

러스트에서 비동기 태스크를 만들고 실행하기 위해 가장 널리 사용되는 크레이트가 바로 `Tokio`입니다.
다음과 같이 `cargo add tokio --features=full` 명령으로 `Cargo.toml`에 추가할 수 있습니다.

```toml
[dependencies]
tokio = { version = "1.44.2", features = ["full"] }
```

`Tokio`는 비동기 태스크 실행을 위한 런타임(`Runtime`)을 제공합니다. 런타임은 여러 비동기 태스크 중 어떤 것을 언제 실행할지 판단하는 스케줄러 역할을 합니다. 어떤 태스크가 `I/O`를 기다리느라 중단되었는지, 혹은 데이터가 도착하여 실행 준비가 되었는지 등을 관리합니다.

운영체제가 프로세스를 관리하기 위해 `task_struct`를 가지는 것처럼, 러스트의 비동기 태스크는 `Future`라는 트레이트(`Trait`)를 통해 관리됩니다. 우리가 비동기 함수를 만들면 컴파일러가 자동으로 `Future` 구현체를 생성하며, 런타임은 이 `Future` 객체들을 스케줄링하며 실행합니다. 현재 `Tokio`는 러스트 비동기 생태계의 사실상 표준으로 자리 잡고 있습니다.

## `async`, `await` 키워드와 `Future`

비동기 함수가 어떻게 동작하는지 다음 예제를 통해 살펴보겠습니다.

```rust,ignore
use std::time::Duration;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let future_one = task_one();
    let future_two = task_two();
    println!("Futures are ready but not start yet");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let v1 = future_one.await;
    println!("task_one is finished");
    let v2 = future_two.await;
    println!("task_two is finished");
    println!("v1={} v2={}", v1, v2);
}
```

실행 결과:
```bash
Futures are ready but not start yet
Start task-one
Finish task-one
task_one is finished
Start task-two
Finish task-two
task_two is finished
v1=1 v2=2
```

`task_one`과 `task_two`는 `async` 키워드가 붙은 비동기 함수입니다. 일반적인 함수라면 호출 즉시 실행되겠지만, 비동기 함수는 호출 시점에 실행되지 않고 `Future` 객체(정확히는 `impl Future<Output=i32>`)를 반환할 뿐입니다.

`Future`는 아직 완료되지 않은 연산 작업을 나타내는 구조체입니다. 비동기 함수가 실제로 실행을 시작하는 지점은 반환된 `Future` 객체에 `await` 키워드를 사용할 때입니다. `await`은 비동기 함수의 실행을 시작할 뿐만 아니라, 그 작업이 완료될 때까지 기다려 결과값을 받아오는 역할을 합니다.

예제에서 `task_one()`을 호출했을 때는 아무 일도 일어나지 않다가, `future_one.await`을 만나는 순간 작업이 시작되고 완료를 기다립니다. 따라서 비동기 함수를 호출만 하고 `await`하지 않으면 실제 작업은 수행되지 않습니다.

여기서 몇 가지 특이한 점이 보입니다.

1. `#[tokio::main]`: `main` 함수가 `Tokio` 런타임을 시작하도록 설정하는 매크로 속성입니다.
2. `async fn main`: `await` 키워드를 사용하는 함수는 반드시 그 자신도 `async` 함수여야 합니다.

## 비동기 함수들을 진정으로 비동기하게 실행하기

이전 예제는 비동기 함수들을 순서대로 `await` 했기 때문에 결과적으로 동기 방식과 차이가 없었습니다. 이번에는 여러 비동기 작업을 동시에 처리하는 방법을 알아보겠습니다.

```rust,ignore
#[tokio::main]
async fn main() {
    let future_one = task_one();
    let future_two = task_two();
    println!("Futures are ready but not start yet");
    
    // 두 future를 동시에 실행하고 모두 끝날 때까지 기다림
    let (v1, v2) = tokio::join!(future_one, future_two);
    println!("v1={} v2={}", v1, v2);
}
```

`tokio::join!` 매크로는 여러 개의 `Future`를 인자로 받아 하나의 비동기 태스크 내에서 동시에 실행합니다. 이를 통해 각 작업을 병행 처리할 수 있습니다.

여기서 한 가지 중요한 점이 있습니다. 비동기 함수 내에서 대기할 때는 `std::thread::sleep`이 아닌 `tokio::time::sleep`을 사용해야 한다는 것입니다.

* `std::thread::sleep`: 현재 실행 중인 **스레드 전체**를 멈춥니다. 런타임 자체가 멈추므로 해당 스레드에서 돌아가는 다른 모든 비동기 태스크도 중단됩니다.
* `tokio::time::sleep`: 현재 **비동기 블록(함수)**의 실행만 중단하고 제어권을 런타임에 넘깁니다. 런타임은 그동안 다른 비동기 태스크를 실행할 수 있습니다.

비동기 프로그래밍의 이점을 살리려면 파일 입출력(`tokio::fs`)이나 네트워크 통신 등 외부 라이브러리도 비동기 방식을 지원하는 것을 사용해야 합니다.

만약 동기 방식으로 동작하는 코드를 비동기 환경에서 실행해야 한다면, 다음과 같이 `tokio::task::spawn`을 사용하여 별도의 비동기 태스크로 분리할 수 있습니다.

```rust,ignore
#[tokio::main]
async fn main() {
    let one = tokio::task::spawn(async {
        task_one_thread_sleep().await;
    });
    let two = tokio::task::spawn(async {
        task_one_async_sleep().await;
    });
    let _ = tokio::join!(one, two);
}
```

`spawn`을 통해 생성된 태스크는 런타임에 의해 별도로 스케줄링되므로, 한 태스크가 블록되더라도 다른 태스크의 실행에 영향을 덜 주게 됩니다.

## 비동기 테스트(`Async Test`)

비동기 함수를 테스트할 때도 일반적인 테스트 프레임워크로는 한계가 있습니다. `Tokio`는 비동기 테스트를 위해 `#[tokio::test]` 속성을 제공합니다.

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_func() {
        let v = task_one().await;
        assert_eq!(v, 1);
    }

    #[tokio::test]
    async fn test_async_task() {
        let future_one = task_one();
        let future_two = task_two();
        let (v1, v2) = tokio::join!(future_one, future_two);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
```

테스트 함수 앞에 `async`를 붙이고 `#[tokio::test]` 속성을 추가하면, 테스트 코드 내에서도 자유롭게 `await`을 사용하여 비동기 함수를 검증할 수 있습니다.

## 좀 더 나아가기 위해 할 일

러스트를 이용한 비동기 프로그래밍은 현대적인 고성능 애플리케이션 개발에서 필수적인 요소가 되고 있습니다. 이 장에서는 `async`, `await`, `Future`의 기본 개념과 `Tokio` 런타임의 기초 사용법을 살펴보았습니다.

더 깊이 있는 공부를 원하신다면 직접 `Future` 트레이트를 구현해 보거나, `Tokio` 내부의 스케줄링 원리를 파헤쳐 보는 것을 추천합니다. 또한 `axum`이나 `warp` 같은 비동기 웹 프레임워크를 사용하여 실제 프로젝트를 구성해 보는 것도 좋은 학습 방법이 될 것입니다.
