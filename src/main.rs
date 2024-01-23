use std::sync::Arc;
use async_std::sync::RwLock;
use geo::Contains;
use tide::{Request, Response};
use tide::http::mime;

use crate::modules::State;

mod modules;
mod library;

// Define state type
pub type ProtectedState = Arc<RwLock<State>>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = State::new();
    let state_arc = Arc::new(RwLock::new(state));

    // Acquire a write lock to mutate the state
    {
        let mut state_write = state_arc.write().await;
        state_write.initiate(Arc::clone(&state_arc));
    }

    let state_for_tide = Arc::clone(&state_arc); // Clone Arc for tide
    let mut app = tide::with_state(state_for_tide);
    let mut admin = tide::with_state(app.state().clone());
    admin.at("/").get(|_| async { Ok("nested app with cloned state") });
    app.at("/").nest(admin);

    app.at("/").get(|req: Request<ProtectedState>| async move {
        let state = req.state().read().await;
        let world = state.world.clone(); // Assuming `world` is cloneable
        let mut response = Response::builder(203)
            .body("<html>Hello</html>")
            .header("custom-header", "value")
            .content_type(mime::HTML)
            .build();
        response.set_body(world.contains(
            // Generate a point outside of the world using a number just outside of the world coordinates
            &geo::Point::new(10000000.09, 1000000.09)
        ).to_string());
        Ok(response)
    });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

