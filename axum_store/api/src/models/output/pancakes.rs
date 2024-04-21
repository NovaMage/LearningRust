use serde::Serialize;
use axum_store_derive::Output;

#[derive(Serialize, Output)]
pub struct Pancakes;
