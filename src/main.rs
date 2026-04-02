use rust_demo::app;

#[tokio::main]
async fn main() {
    let app = app();
    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on http://{}", addr);
    
    // Axum 0.8 的新方式
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}