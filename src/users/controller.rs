use axum::{
    Router, 
    extract::{Path, Query, State, Json},
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post, put, delete}
};
use validator::Validate;
use crate::users::dtos::{CreateUserDto, UpdateUserDto, UserFilters};

async fn create_user(Json(body): Json<CreateUserDto>) -> impl IntoResponse {
    if body.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "message": "Invalid user data"
            }))
        );
    }

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "message": "User created"
        })),
    )
}

async fn get_user(Path(id): Path<String>, Query(filters): Query<UserFilters>) -> impl IntoResponse {
    if filters.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "message": "Invalid user filters"
            }))
        );
    } else {
        return (
            StatusCode::NOT_IMPLEMENTED,
            Json(serde_json::json!({
                "message": format!("User {id} retrieved"),
                "filters": filters
            })),
        );
    }
}

async fn update_user(Path(id): Path<String>, Json(body): Json<UpdateUserDto>) -> impl IntoResponse {
    if body.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "message": "Invalid user data"
            }))
        )
    }

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "message": "User updated"
        }))
    )
}

async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "message": "User deleted"
        })),
    )
}

async fn get_user_payments(Path(id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "message": format!("User {id} payments")
        })),
    )
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/{id}/payments", get(get_user_payments))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn user_routes_builds() {
        let routes = user_routes();

        // Act: Hacemos un request a / en memoria
        let response = routes
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        eprintln!("Headers: {:?}", response.headers());
        
        // Assert: Esperamos un 200 OK
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);

    }
}