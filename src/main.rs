use rust_demo::app;

#[tokio::main]
async fn main() {
    let app = app();
    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
