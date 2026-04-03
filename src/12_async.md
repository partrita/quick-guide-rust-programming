# 비동기 프로그래밍 (Asynchronous Programming)

## 비동기 프로그래밍이란?

여러 작업을 효율적으로 처리하는 방식에는 크게 세 가지가 있습니다.

1.  병렬 프로그래밍 (`Parallel`): `GPU` 연산처럼 물리적으로 여러 코어가 동시에 서로 다른 연산을 수행합니다.
2.  동시성 프로그래밍 (`Concurrent`): `CPU`가 여러 스레드를 아주 빠르게 번갈아 실행하여 동시에 돌아가는 것처럼 보이게 합니다.
3.  비동기 프로그래밍 (`Asynchronous`): 작업(I/O 등)을 요청한 뒤 결과가 올 때까지 기다리지 않고 다른 일을 하다가, 신호가 오면 하던 작업을 이어 나갑니다.

비동기 방식은 스레드를 직접 생성하고 전환하는 비용(`Context Switching`)이 거의 없어, 수만 개의 동시 연결을 처리해야 하는 네트워크 서버 등에서 압도적인 효율을 자랑합니다.

## Tokio 런타임

`Rust` 표준 라이브러리는 비동기 구문(`async`/`await`)만 정의할 뿐, 이를 실제로 실행할 스케줄러(런타임)는 포함하지 않습니다. 현재 `Rust` 생태계의 표준 런타임은 `Tokio`입니다.

```toml
[dependencies]
tokio = { version = "1.44", features = ["full"] }
```

## async, await와 Future

`async` 키워드가 붙은 함수는 호출 즉시 실행되지 않고, 나중에 실행될 작업 명세서인 `Future` 객체를 반환합니다.

```rust
async fn my_task() -> i32 { 10 }

#[tokio::main]
async fn main() {
    let future = my_task(); // 아직 실행 안 됨
    let val = future.await; // 여기서 실행 시작 및 완료 대기
    println!("{}", val);
}
```

`await`는 비동기 작업이 끝날 때까지 현재 실행 흐름을 일시 정지(Yield)시키고 제어권을 런타임에 넘깁니다. 런타임은 그동안 다른 비동기 작업들을 처리하므로 시스템 전체 효율이 올라갑니다.

## 진정한 비동기 실행: join!

여러 비동기 작업을 동시에 시작하고 모두 끝나기를 기다릴 때는 `tokio::join!` 매크로를 사용합니다.

```rust
let (res1, res2) = tokio::join!(task_one(), task_two());
```

비동기 환경에서 주의할 점은 `std::thread::sleep`을 쓰면 안 된다는 것입니다. 이는 런타임 전체를 멈춰버리기 때문입니다. 대신 반드시 비동기 전용인 `tokio::time::sleep`을 사용해야 합니다.

`Rust`의 비동기 프로그래밍은 처음에는 생소할 수 있지만, 한 번 익혀두면 고성능 서버 개발이나 복잡한 시스템을 구축할 때 강력한 무기가 될 것입니다.
