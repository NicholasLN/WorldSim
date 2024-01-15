use std::sync::Arc;
use std::time::Duration;
use async_std::task;
use async_std::sync::Mutex;
use modules::State;
use tide::{Request, Server};


mod modules;     // Modules are for organizing code
mod library;     // Lib is for general purpose 



type ProtectedState = Arc<Mutex<State>>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut state: State = State::new();
    state.initiate();
    let state = Arc::new(Mutex::new(state));

    let mut app = Server::with_state(state);

    app.at("/").get(|_| async move { Ok("Hello, world!") });
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
