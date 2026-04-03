# 로그(Log)와 시그널 핸들링(Signal Handling)

실무 수준의 애플리케이션을 개발할 때 필수적인 로그 기록 방법과 운영체제 시그널 처리 방법을 알아보겠습니다.

## 로그(Log) 사용하기

`Rust`에서 로그를 남기기 위해 가장 널리 쓰이는 도구는 `tracing` 크레이트와 `tracing-subscriber`입니다. 기존의 단순한 로그 기록을 넘어, 비동기 작업의 흐름(Span)을 추적하는 기능이 강력합니다.

*   `error!`, `info!`, `debug!`, `trace!`: 로그 레벨별 기록 매크로.
*   `span!`: 특정 작업 단위(범위)를 설정하여 로그에 맥락을 추가합니다.
*   `EnvFilter`: `RUST_LOG` 환경 변수를 통해 실행 시점에 로그 레벨을 동적으로 조절할 수 있게 해줍니다.

## 시그널 핸들링(Signal Handling)

운영체제로부터 전달되는 `SIGINT`(Ctrl+C), `SIGTERM`(종료 요청) 등의 시그널을 우아하게 처리(`Graceful Shutdown`)하는 것은 매우 중요합니다. `signal-hook`과 `signal-hook-tokio`를 사용하면 비동기 환경에서도 안전하게 시그널을 가로챌 수 있습니다.

```rust
// code/etc/main.rs (주요 로직 요약)
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use futures::stream::StreamExt;

async fn handle_signals(mut signals: Signals, term: Arc<AtomicBool>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => println!("설정 파일을 다시 읽어옵니다."),
            SIGTERM | SIGINT | SIGQUIT => {
                println!("프로그램을 안전하게 종료합니다...");
                term.store(true, Ordering::Relaxed);
            }
            _ => unreachable!(),
        }
    }
}
```

시그널 처리를 별도의 태스크(`tokio::spawn`)로 분리하고, 원자적 변수(`AtomicBool`)를 통해 메인 루프에 종료 신호를 보내는 방식이 전형적인 패턴입니다. 이를 통해 작업 중인 데이터를 안전하게 저장하고 자원을 정리한 뒤 종료할 수 있습니다.
