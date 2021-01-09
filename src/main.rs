use std::sync::{Arc, atomic::{Ordering, AtomicU32}};
use tide::{sse, Result};

static INDEX: AtomicU32 = AtomicU32::new(0);

/// State of the Koggie Server
#[derive(Clone)]
struct State {
    /// Temporary stereo 48KHz 16LE PCM 10 second buffer.
    seconds: [Arc<[[u8; 4]; 48_000]>; 10],
}

/// Koggie raw stereo 16LE PCM broadcast.
async fn broadcast(mut request: tide::Request<State>) -> Result<String> {
    Ok("".to_string())
}

/// Opus Stream For The Website and App.
async fn listen(mut request: tide::Request<State>) -> Result<String> {
    Ok("".to_string())
}

/// Configure what passwords can be used to broadcast.
async fn config(mut request: tide::Request<State>) -> Result<String> {
    Ok("".to_string())
}

/// Start the webserver.
#[async_std::main]
async fn main() -> Result<()> {
    let state = State {
        seconds: [Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000]), Arc::new([[0; 4]; 48_000])],
    };

    tide::log::start();
    let mut app = tide::with_state(state);
    app.at("/listen").get(listen);
    app.at("/koggie").post(broadcast);
    app.at("/config").post(config);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
