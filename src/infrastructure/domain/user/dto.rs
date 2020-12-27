use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::user::entity::User;

#[derive(Debug, FromRow)]
pub(crate) struct UserDTO {
    id: Uuid,
    name: String,
}

impl From<UserDTO> for User {
    fn from(dto: UserDTO) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
        }
    }
}
