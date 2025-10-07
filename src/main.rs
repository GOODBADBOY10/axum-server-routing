// #![allow(unused)] // for beginner only.
// use std::net::SocketAddr;

// use axum::Router;
// use axum::routing::get;
// use axum::response::Html;

// #[tokio::main]
// async fn main() {
//     let routes_hello = Router::new().route(
//         "/hello",
//         get(|| async {
//             Html("Hellow <strong>World!!!!</strong>")
//         })
//     );

//     //region: --- start server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("->> LISTENING on {addr}\n");
//     axum::Server::bind(&addr)
//     .serve(routes_hello.into_make_service())
//     .await
//     .unwrap();

//     // end-region: ----- start serve
// }

#![allow(unused)]
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use axum::extract::Query;
use axum::extract::Path;
use axum::Router;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::get_service;
use serde::Deserialize;

#[tokio::main]
// async fn main() {
//     let routes_hello = Router::new()
//         .route("/hello", 
//         get(handler_hello))
//         .route("/hello2/:name", 
//         get(handler_hello2));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("Listening on http://{}", addr);
    
//     // Axum 0.6 syntax
//     axum::Server::bind(&addr)
//         .serve(routes_hello.into_make_service())
//         .await
//         .unwrap();
// }

async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .fallback_service(routes_static());


    // region ----- start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    
    // Axum 0.6 syntax
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


//region ----- Routes Hello--
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}


async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}





// #![allow(unused)] // for beginner only.
// use std::net::SocketAddr;

// use axum::Router;
// use axum::response::Html;
// use axum::response::IntoResponse;
// use axum::routing::get;

// #[tokio::main]
// async fn main() {
//     let routes_hello = Router::new().route("/hello", get(handler_hello));

//     // Add these lines to actually start the server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("Listening on http://{}", addr);
    
//     let listener = tokio::net::TcpListener::bind(addr)
//         .await
//         .unwrap();
    
//     axum::serve(listener, routes_hello.into_make_service())
//         .await
//         .unwrap();
// }

// async fn handler_hello() -> impl IntoResponse {
//     println!("-->> {:<12} - handler_hello", "HANDLER");
//     Html("Hello <strong>World!!!!</strong>")
// }