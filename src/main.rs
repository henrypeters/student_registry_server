// mod container;
// use crate::container::{Cont, CreateUser};

// use std::net::TcpListener;

// use axum::{
//     routing::{get, post},
//     http::StatusCode,
//     Json, Router,
// };
// use serde::{Deserialize, Serialize};

// #[tokio::main]
// async fn main() {
//     let mut container = Cont::new();

//     let user = User { id: 123, username: String::from("Henry") };

//     // initialize tracing
//     tracing_subscriber::fmt::init();

//     // build our application with a route
//     let app = Router::new()
//         // `GET /` goes to `root`
//         .route("/", get(henry_base_url))
//         // `POST /users` goes to `create_user`
//         .route("/users", post(container.create_user))
//         .route("/get_users", get(container.add(user)));

//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     println!("Server is listening from {:?}", listener);
//     tracing::info!("Server running on {PORT}");
//     axum::serve(listener, app).await.unwrap();
// }

// // basic handler that responds with a static string
// async fn henry_base_url() -> &'static str {
//     "GM Rust!"
// }

// // async fn create_user(
// //     // this argument tells axum to parse the request body
// //     // as JSON into a `CreateUser` type
// //     Json(payload): Json<CreateUser>,
// // ) -> (StatusCode, Json<User>) {
// //     // insert your application logic here
// //     let user = User {
// //         id: 1337,
// //         username: payload.username,
// //     };

// //     let mut container = Cont::new();
// //     container.add(user.clone());

// //     // this will be converted into a JSON response
// //     // with a status code of `201 Created`
// //     (StatusCode::CREATED, Json(user))
// // }

// async fn get_users() -> (StatusCode, Json<Vec<User>>) {

// }

// // the input to our `create_user` handler
// // #[derive(Deserialize)]
// // struct CreateUser {
// //     username: String,
// // }

// // the output to our `create_user` handler
// #[derive(Debug, Clone, Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }













mod handlers;
mod routes;
mod schema;
mod utils;

const PORT: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = routes::axum_router().await;
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();

    tracing::info!("server running on {PORT}");
    axum::serve(listener, app).await.unwrap();
}