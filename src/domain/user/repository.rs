use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;

use super::entity::User;

#[async_trait]
pub trait Repository {
    async fn create(&self, user: &User) -> Result<User>;
    async fn find(&self) -> Result<Vec<User>>;
    async fn find_one(&self, id: &Uuid) -> Result<User>;
    async fn update(&self, id: &Uuid, user: &User) -> Result<User>;
}
