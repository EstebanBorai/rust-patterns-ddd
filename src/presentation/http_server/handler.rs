use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::reject::reject;
use warp::reply::json;
use warp::{Rejection, Reply};

use crate::application::services::domain::user::{dto::NewUserDTO, service::UserService};
use crate::infrastructure::domain::user::repository::UserRepository;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: Uuid,
    name: String,
}

pub async fn find_all(
    user_service: Arc<UserService<UserRepository>>,
) -> Result<impl Reply, Rejection> {
    match user_service.find().await {
        Ok(users) => Ok(json(
            &users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    name: u.name,
                })
                .collect::<Vec<UserResponse>>(),
        )),
        Err(e) => {
            error!("{:?}", e);
            Err(reject())
        }
    }
}

pub async fn create_user(
    user_service: Arc<UserService<UserRepository>>,
    body: CreateUserPayload,
) -> Result<impl Reply, Rejection> {
    match user_service
        .create_user(NewUserDTO { name: body.name })
        .await
    {
        Ok(created_user) => Ok(json(&UserResponse {
            id: created_user.id,
            name: created_user.name,
        })),
        Err(e) => {
            error!("{:?}", e);
            Err(reject())
        }
    }
}
