use std::convert::TryFrom;

use crate::domain::user::entity::User;
use crate::error::Error;

pub struct NewUserDTO {
    pub(crate) name: String,
}

impl TryFrom<NewUserDTO> for User {
    type Error = Error;

    fn try_from(dto: NewUserDTO) -> Result<Self, Self::Error> {
        Ok(User {
            id: uuid::Uuid::nil(),
            name: dto.name,
        })
    }
}
