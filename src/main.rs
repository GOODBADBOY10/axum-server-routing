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



#![allow(unused)] // for beginner only.
use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use axum::response::Html;
use axum::response::IntoResponse;


#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handle_hello)
    );

    // region: ----- Handler Hello
    async fn handle_hello() -> impl IntoResponse {
        println!("-->> {:<12} - handler_hello", "HANDLER");
        Html("Hellow <strong>World!!!!</strong>")
    }
}