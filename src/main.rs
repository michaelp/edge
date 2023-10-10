use std::net::SocketAddr;

use axum::{
  routing::get, Router
};
use tracing::{instrument, info};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(subscriber);
    // build our application with a single route
    let app = Router::new()
                  .route("/", get( root));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[instrument]
async fn root() -> &'static str {
  info!("inside my_function!");
  "Hello, World!"
}
