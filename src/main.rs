use std::fs;

use axum::{
    routing::get,
    Router,
    response::Html
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:6969".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    return Html(fs::read_to_string("html/index.html").unwrap());
}

async fn about() -> Html<String> {
    return Html(fs::read_to_string("html/about.html").unwrap());
}