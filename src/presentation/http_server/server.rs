use std::sync::Arc;
use warp::http;
use warp::Filter;

use crate::application::services::domain::user::service::UserService;
use crate::infrastructure::database::get_db_pool;
use crate::infrastructure::domain::user::repository::UserRepository;

use super::handler;
use super::middleware;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self) {
        let db_pool = get_db_pool()
            .await
            .expect("Unable to connect to the database");
        let user_repository = UserRepository::new(db_pool.clone());
        let user_service = Arc::new(UserService::new(user_repository));

        let cors = warp::cors()
            .allow_any_origin()
            .allow_credentials(true)
            .allow_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .allow_methods(&[
                http::Method::GET,
                http::Method::OPTIONS,
                http::Method::POST,
                http::Method::PUT,
            ]);

        let api = warp::path("api");
        let api_v1 = api.and(warp::path("v1"));
        let users = api_v1.and(warp::path("users"));

        let create_user = users
            .and(warp::post())
            .and(middleware::service::with_service(user_service.clone()))
            .and(warp::body::json())
            .and_then(handler::create_user);

        let find_users = users
            .and(warp::get())
            .and(middleware::service::with_service(user_service.clone()))
            .and_then(handler::find_all);

        warp::serve(create_user.or(find_users).with(cors))
            .run(([127, 0, 0, 1], self.port))
            .await
    }
}
