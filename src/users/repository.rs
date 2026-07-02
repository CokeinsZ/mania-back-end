use async_trait::async_trait;
use crate::users::dtos::*;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn save_user(&self, user_data: &CreateUserDto) -> Result<(), String>;
    async fn email_exists(&self, email: &str) -> Result<bool, String>;
}

pub struct PostgresUserRepository {
    db_pool: sqlx::PgPool, 
}

impl PostgresUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { db_pool: pool }
    }
}

// 3. Le enseñamos al struct real a cumplir el contrato
#[async_trait]
impl UserRepositoryTrait for PostgresUserRepository {
    async fn save_user(&self, user_data: &CreateUserDto) -> Result<(), String> {
        // Aquí iría el query real a Postgres usando sqlx
        
        println!("[DB] Insertando usuario {} en PostgreSQL...", user_data.email);
        Ok(())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, String> {
        // Query real
        println!("[DB] Verificando si {} existe...", email);
        Ok(false) // Simulamos que no existe
    }
}