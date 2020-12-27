use async_trait::async_trait;

use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository;
use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::dto::UserDTO;

pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Repository for UserRepository {
    async fn create(&self, user: &User) -> Result<User> {
        let result: UserDTO = sqlx::query_as("INSERT INTO users(name) VALUES ($1) RETURNING *")
            .bind(&user.name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(User::from(result))
    }

    async fn find(&self) -> Result<Vec<User>> {
        let result: Vec<UserDTO> = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(result.into_iter().map(User::from).collect())
    }

    async fn find_one(&self, id: &uuid::Uuid) -> Result<User> {
        let result: UserDTO = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(User::from(result))
    }

    async fn update(&self, id: &uuid::Uuid, user: &User) -> Result<User> {
        let result: UserDTO =
            sqlx::query_as("UPDATE users SET name = $1 WHERE id = $2 RETURNING *")
                .bind(&user.name)
                .bind(&id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(User::from(result))
    }
}
