# 로그(`Log`)와 시그널 핸들링(`Signal handling`)

## 로그(`Log`) 사용하기

러스트에서 로그를 남기기 위해 `tracing` 크레이트와 `tracing-subscriber`를 자주 사용합니다. `error!`, `info!`, `debug!`, `trace!` 등의 매크로를 사용하여 로그 레벨별로 메시지를 남길 수 있습니다.

## 시그널 핸들링(`Signal handling`)

`signal-hook`이나 `signal-hook-tokio` 크레이트를 사용하여 `SIGHUP`, `SIGTERM`, `SIGINT` 등 운영체제의 시그널을 비동기적으로 처리할 수 있습니다.

```rust,ignore
use std::collections::HashMap;
//use std::env;

use tracing::info;
use tracing::span;
use tracing::trace;
use tracing::Instrument;
use tracing::Level;
use tracing_subscriber::EnvFilter;

//use std::io::Error;

use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct Context {
    variables: HashMap<String, String>,
    dry_run: bool,
}

impl Context {
    async fn from_path(path: &str) -> anyhow::Result<Self> {
        let mut variables = HashMap::new();
        tracing::error!("Creating context");
        info!("Creating context from path: {}", path);
        tracing::debug!("Creating context");
        trace!(?path);

        // test span again
        // If log level is info, process_recipe() prints log message like
        // 2025-02-23T15:10:29.315933Z  INFO cook{path="/path/to/file"}: rust_log: process_recipe::Ingredients: ["Pasta", "Eggs", "Bacon", "Parmesan"]
        // If log level is debug, process_recipe() prints log message like
        // 2025-02-23T15:09:40.495049Z  INFO my_span_main{path="/path/to/file"}:cook{path="/path/to/file"}: rust_log: process_recipe::Ingredients: ["Pasta", "Eggs", "Bacon", "Parmesan"]
        // because the span of "my_span_main" is added for the debug level.
        let span = span!(Level::INFO, "cook", ?path);
        process_recipe().instrument(span).await?;

        variables.insert("path".to_owned(), path.to_owned());
        Ok(Self {
            variables,
            dry_run: false,
        })
    }
}

async fn process_recipe() -> anyhow::Result<()> {
    let recipe = "Pasta Carbonara";
    let ingredients = vec!["Pasta", "Eggs", "Bacon", "Parmesan"];
    let steps = vec![
        "Cook the pasta",
        "Fry the bacon",
        "Mix the eggs and cheese",
        "Combine everything",
    ];

    tracing::error!("process_recipe::Recipe: {}", recipe);
    tracing::info!("process_recipe::Ingredients: {:?}", ingredients);
    tracing::debug!("Steps: {:?}", steps);

    Ok(())
}

async fn handle_signals(mut signals: Signals, term: Arc<AtomicBool>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration
                // Reopen the log file
                println!("SIGHUP received");
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                println!("SIGTERM, SIGINT, or SIGQUIT received");
                term.store(true, Ordering::Relaxed);
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // error,rust_log=info: rust_log crate can print info-level log messages, others can print only error level
    // But env RUST_LOG can override this setting, for example) RUST_LOG=debug cargo run
    let envfilter = EnvFilter::builder()
        .try_from_env()
        .unwrap_or_else(|_| EnvFilter::new("error,rust_log=info")); // try with rust_log=debug and rust_log=info to test two span! calls
    tracing_subscriber::fmt().with_env_filter(envfilter).init();

    tracing::error!("This is an error message");
    tracing::info!("This is an info message"); // This is not printed because the filter is set to info.
    tracing::debug!("This is a debug message"); // This is not printed because the filter is set to info.

    let path = "/path/to/file".to_owned();

    // ADD "my_span_main{path="/path/to/file"}" to the log message if the log level is DEBUG
    let span = span!(Level::DEBUG, "my_span_main", ?path);

    let context = Context::from_path(&path).instrument(span.clone()).await?;
    tracing::info!("info level message context={:?}", context);
    tracing::debug!("debug message");

    let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();

    let term = Arc::new(AtomicBool::new(false));
    let signals_task = tokio::spawn(handle_signals(signals, term.clone()));

    // Execute your main program logic

    while !term.load(Ordering::Relaxed) {
        // Do some time-limited stuff here
        // (if this could block forever, then there's no guarantee the signal will have any
        // effect).
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("sleeping");
    }

    // Terminate the signal stream.
    handle.close();
    signals_task.await?;

    Ok(())
}
```
