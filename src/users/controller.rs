use axum::{
    Router, 
    extract::{Path, Query, Json},
    http::StatusCode, 
    routing::{get, post}
};
use validator::Validate;
use crate::users::dtos::*;

fn json_response(status: StatusCode, body: serde_json::Value) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(body))
}

fn build_validation_response(errors: validator::ValidationErrors) -> (StatusCode, Json<serde_json::Value>) {
    let mut error_messages = Vec::new();

    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = error.message.as_deref().unwrap_or("invalid value");
            error_messages.push(format!("{field}: {message}"));
        }
    }

    json_response(
        StatusCode::BAD_REQUEST,
        serde_json::json!({
            "message": "Invalid user data",
            "errors": error_messages
        }),
    )
}

async fn create_user(Json(user): Json<CreateUserDto>) -> (StatusCode, Json<serde_json::Value>) {
    if let Err(errors) = user.validate() {
        return build_validation_response(errors);
    }

    json_response(
        StatusCode::NOT_IMPLEMENTED,
        serde_json::json!({
            "message": "User created"
        }),
    )
}

async fn get_user(Path(id): Path<String>, Query(filters): Query<UserFilters>) -> (StatusCode, Json<serde_json::Value>) {
    if let Err(errors) = filters.validate() {
        return build_validation_response(errors);
    }

    json_response(
        StatusCode::NOT_IMPLEMENTED,
        serde_json::json!({
            "message": format!("User {id} retrieved"),
            "filters": filters
        }),
    )
}

async fn update_user(Path(id): Path<String>, Json(body): Json<UpdateUserDto>) -> (StatusCode, Json<serde_json::Value>) {
    if let Err(errors) = body.validate() {
        return build_validation_response(errors);
    }

    json_response(
        StatusCode::NOT_IMPLEMENTED,
        serde_json::json!({
            "message": format!("User {id} updated")
        }),
    )
}

async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    json_response(
        StatusCode::NOT_IMPLEMENTED,
        serde_json::json!({
            "message": format!("User {id} deleted")
        }),
    )
}

async fn get_user_payments(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    json_response(
        StatusCode::NOT_IMPLEMENTED,
        serde_json::json!({
            "message": format!("User {id} payments")
        }),
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
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_user() {
        let routes = user_routes();

        let user_data = CreateUserDto {
            full_name: "John Doe".to_string(),
            entity_type: user_entity_type::Natural,
            government_id: "1234567890".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "1234567890".to_string(),
            password: "Password123".to_string(),
            address: "123 Main St".to_string(),
        };

        let response = routes
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&user_data).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        let status = response.status();
        let headers = response.headers().clone();

        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();

        println!("\n\n Test Create User Response:");
        println!("Headers: {:?}", headers);
        println!("Body: {}", body_text);

    }

    #[tokio::test]
    async fn create_invalid_user() {
        let invalid_cases = vec![
            (
                "Empty full name",
                CreateUserDto {
                    full_name: "".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "1234567890".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "1234567890".to_string(),
                    password: "Password123".to_string(),
                    address: "123 Main St".to_string(),
                },
            ),
            (
                "Invalid government ID",
                CreateUserDto {
                    full_name: "John Doe".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "123".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "1234567890".to_string(),
                    password: "Password123".to_string(),
                    address: "123 Main St".to_string(),
                },
            ),
            (
                "Invalid email",
                CreateUserDto {
                    full_name: "John Doe".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "1234567890".to_string(),
                    email: "invalid-email".to_string(),
                    phone: "1234567890".to_string(),
                    password: "Password123".to_string(),
                    address: "123 Main St".to_string(),
                },
            ),
            (
                "Invalid phone",
                CreateUserDto {
                    full_name: "John Doe".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "1234567890".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "123".to_string(),
                    password: "Password123".to_string(),
                    address: "123 Main St".to_string(),
                },
            ),
            (
                "Invalid password",
                CreateUserDto {
                    full_name: "John Doe".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "1234567890".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "1234567890".to_string(),
                    password: "pas".to_string(),
                    address: "123 Main St".to_string(),
                },
            ),
            (
                "Empty address",
                CreateUserDto {
                    full_name: "John Doe".to_string(),
                    entity_type: user_entity_type::Natural,
                    government_id: "1234567890".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "1234567890".to_string(),
                    password: "Password123".to_string(),
                    address: "".to_string(),
                },
            ),
        ];

        for (case_name, invalid_user_data) in invalid_cases {
            let routes = user_routes();

            let response = routes
                .oneshot(
                    Request::builder()
                        .uri("/")
                        .method("POST")
                        .header("Content-Type", "application/json")
                        .body(Body::from(serde_json::to_string(&invalid_user_data).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();

            let status = response.status();
            let headers = response.headers().clone();

            assert_eq!(status, StatusCode::BAD_REQUEST, "Failed: {case_name}");

            let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();

            println!("\n\nTest Create Invalid User Response [{case_name}]");
            println!("Headers: {:?}", headers);
            println!("Body: {}", body_text);
        }
    }
}