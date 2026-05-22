use axum::{Json, extract::Path, http::StatusCode};
use serde_json::Value;

use crate::model::User;

pub async fn list_users() -> (StatusCode, Json<Value>) {
    // get users
}

pub async fn get_user_by_id(Path(id): Path<u64>) -> (StatusCode, Json<Value>) {
    // get user
}

pub async fn create_user(Json(user): Json<User>) -> StatusCode {
    // create user
}

pub async fn update_user(Path(id): axum::extract::Path<u64>, Json(user): Json<User>) -> StatusCode {
    // update user
}

pub async fn delete_user(Path(id): Path<u64>) -> StatusCode {
    // delete user
}
