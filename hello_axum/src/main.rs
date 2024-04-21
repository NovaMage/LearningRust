use tracing_subscriber::fmt;

mod models;

use axum::{async_trait, http::StatusCode, Json, Router, routing::{get, post}};
use axum::body::Body;
use axum::extract::{FromRequest, Request};
use axum::extract::rejection::JsonRejection;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    // initialize tracing
    fmt::init();

    // build application routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> CreateOutput {
    CreateOutput::new(70)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    payload: CreateUser,
) -> Result<CreateOutput, FailedResponse> {
    // insert your application logic here
    if payload.username.len() > 6 {
        println!("User created: {:?}", payload);
        // this will be converted into a JSON response
        // with a status code of `201 Created`
        let output = CreateOutput::new(42);
        Ok(output)
    } else {
        Err(FailedResponse {
            errors: vec!["username must be at least 7 characters long".to_string()],
        })
    }
}

#[derive(Serialize)]
struct FailedResponse {
    errors: Vec<String>,
}

impl<A: Serialize> From<FailedResponse> for Result<A, FailedResponse> {
    fn from(value: FailedResponse) -> Self {
        Err(value)
    }
}

impl IntoResponse for FailedResponse {
    fn into_response(self) -> Response<Body> {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
struct CreateOutput {
    id: u64,
}

impl CreateOutput {
    fn new(id: u64) -> Self {
        Self {
            id
        }
    }
}

impl IntoResponse for CreateOutput {
    fn into_response(self) -> Response<Body> {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}


// the input to our `create_user` handler
#[derive(Debug, Deserialize)]
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

