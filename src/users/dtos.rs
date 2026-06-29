use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub enum user_entity_type {
    Natural,
    Juridical
}

#[derive(Debug, Deserialize, Serialize)]
pub enum user_state {
    Active,
    Inactive,
    Blocked,
    Deleted
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub full_name: String,
    pub entity_type: user_entity_type,
    pub government_id: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: user_state
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, max = 255, message = "El nombre debe tener entre 1 y 255 caracteres"))]
    pub full_name: String,

    pub entity_type: user_entity_type, 

    #[validate(length(min = 10, max = 13, message = "El documento de identificación debe tener entre 10 y 13 caracteres"))]
    pub government_id: String,
    
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    
    #[validate(length(min = 1, max = 13, message = "El teléfono debe tener entre 1 y 13 caracteres"))]
    pub phone: String,

    #[validate(length(min = 5, max = 100, message = "La contraseña debe tener entre 5 y 100 caracteres"))]
    pub password: String,

    #[validate(length(min = 1, message = "La dirección debe tener mas de 1 caracter"))]
    pub address: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 1, max = 255, message = "El nombre debe tener entre 1 y 255 caracteres"))]
    pub full_name: String,

    pub entity_type: user_entity_type, 

    #[validate(length(min = 10, max = 13, message = "El documento de identificación debe tener entre 10 y 13 caracteres"))]
    pub government_id: String,
    
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    
    #[validate(length(min = 1, max = 13, message = "El teléfono debe tener entre 1 y 13 caracteres"))]
    pub phone: String,

    #[validate(length(min = 1, message = "La dirección debe tener mas de 1 caracter"))]
    pub address: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangeUserPasswordDto {
    #[validate(length(min = 5, max = 100, message = "La contraseña debe tener entre 5 y 100 caracteres"))]
    pub password: String,

    #[validate(length(equal = 6, message = "El código debe tener 6 caracteres"))]
    pub code: String,
}

#[derive( Serialize, Deserialize, Validate )]
pub struct UserFilters {
    #[validate(length(min = 1, max = 255, message = "El nombre debe tener entre 1 y 255 caracteres"))]
    pub name: Option<String>,

    #[validate(length(min = 10, max = 13, message = "El documento de identificación debe tener entre 10 y 13 caracteres"))]
    pub government_id: Option<String>,
    
    #[validate(length(min = 1, max = 13, message = "El teléfono debe tener entre 1 y 13 caracteres"))]
    pub phone: Option<String>
}