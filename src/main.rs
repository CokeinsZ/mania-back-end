use axum::{Router};

mod users;
mod tools;
mod invoices;
mod payments;
use users::controller::user_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/users", user_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}