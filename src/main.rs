mod routes;
mod utils;

use routes::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    run();
}

async fn run() {
    let app = create_router().await;

    // Set the address and the port for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Use Axum to serve up the pages
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Unable to start the Axum webserver");
}

fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect Tokio signal ctrl-c");

    println!("\nSignal shutdown");
}
