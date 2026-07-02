use std::sync::Arc;

use async_trait::async_trait;

use crate::users::{dtos::CreateUserDto, repository::UserRepositoryTrait};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn create_user(&self, dto: CreateUserDto) -> Result<String, String>;
}

pub struct UserService {
    repository: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(&self, dto: CreateUserDto) -> Result<String, String> {  
        self.repository.save_user(&dto).await?;      
        println!("Guardando en base de datos al usuario: {}", dto.email);
        Ok(format!("Usuario {} creado con éxito", dto.full_name))
    }
}


#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn service_placeholder_test() {
        assert!(true);
    }
}