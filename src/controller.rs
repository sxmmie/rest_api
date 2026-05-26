use axum::{Extension, Json, extract::Path, http::StatusCode};

use crate::{
    model::{User, UserInfo},
    user_service::UserService,
};

pub async fn list_users(service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_id(service: Extension<UserService>, Path(id): Path<i32>) -> Result<Json<User>, StatusCode> {
    match service.get_users_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(service: Extension<UserService>, Json(user): Json<UserInfo>) -> StatusCode {
    match service.create_user(user).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_user(service: Extension<UserService>, Path(32): axum::extract::Path<i32>, Json(user): Json<User>) -> StatusCode {
    match service.update_user(id, user).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_user(service: Extension<UserService>, Path(id): Path<u32>) -> StatusCode {
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
