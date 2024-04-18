/*
use axum::{response::Html, routing::get, Router};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Wow, let's!</h1>") }))
        .route("/json", get(|| async { axum::Json(json!({"message": "Hello, World!"})) }))
            .layer(LiveReloadLayer::new().custom_prefix("/live-reloading").reload_interval(Duration::from_millis(200)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
 */

use std::time::Duration;

use axum::{async_trait, http::StatusCode, Json, Router, routing::{get, post}};
use axum::extract::{FromRequest, Request};
use axum::extract::rejection::JsonRejection;
use serde::{Deserialize, Serialize};
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))

        .layer(LiveReloadLayer::new().custom_prefix("/live-reloading").reload_interval(Duration::from_millis(200)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello world!"
}

#[derive(Serialize)]
struct FailedResponse {
    errors: Vec<String>,
}

#[derive(Serialize)]
struct SuccessResponse {
    id: u64,
    message: String,
}

type Res<T> = (StatusCode, Json<Result<T, FailedResponse>>);


fn success<A: Serialize>(a: A) -> Res<A> {
    (StatusCode::CREATED, Json(Ok(a)))
}

fn failure<A>(failed_response: FailedResponse) -> Res<A> {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(Err(failed_response)))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    payload: CreateUser,
) -> Res<SuccessResponse> {
    // insert your application logic here
    if payload.username.len() > 6 {
        let user = User {
            id: 1337,
            username: payload.username,
        };

        // this will be converted into a JSON response
        // with a status code of `201 Created`
        success(SuccessResponse { id: user.id, message: "User created".to_string() })
    } else {
        failure(FailedResponse {
            errors: vec!["username must be at least 7 characters long".to_string()],
        })
    }
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequest<S> for CreateUser {
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<CreateUser>::from_request(req, state).await {
            Ok(user) => Ok(user.0),
            Err(rejection) => Err(rejection),
        }
    }
}


// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}