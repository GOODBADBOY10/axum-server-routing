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
use std::net::SocketAddr;

use axum::Router;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    
    // Axum 0.6 syntax
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong>World!!!!</strong>")
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