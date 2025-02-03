use axum::{extract::Json, http::{request, response, StatusCode}, routing::{get, post}, Router};
use serde::Deserialize;

struct Content{
    title: String,
    content: String
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/",get(||async {"Hello World"})).route("/post",post(writeInFile));
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn write_in_file(request: axum::http::Request<axum::body::Body>){
    let body = request.into_body();
    let limit = 2048usize;
    let bytes = axum::body::to_bytes(body, limit).await.unwrap();
    let body_string = String::from_utf8(bytes.to_vec()).unwrap();
    println!("Body: {}", body_string);
    
}
