use std::convert::TryFrom;
use uuid::Uuid;

use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository as UserRepository;
use crate::error::Result;

use super::dto;

pub struct UserService<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, user: dto::NewUserDTO) -> Result<User> {
        let user = User::try_from(user)?;
        let user = self.user_repository.create(&user).await?;

        Ok(user)
    }

    pub async fn find(&self) -> Result<Vec<User>> {
        self.user_repository.find().await
    }

    pub async fn find_one(&self, id: &Uuid) -> Result<User> {
        self.user_repository.find_one(id).await
    }

    pub async fn update(&self, id: &Uuid, user: dto::NewUserDTO) -> Result<User> {
        let user = User::try_from(user)?;

        self.user_repository.update(id, &user).await
    }
}
