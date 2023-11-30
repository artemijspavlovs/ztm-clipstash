pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// these exports allow to use these types directly from the crate
// using crate::Time syntax
// ShortCode will be utilised by the server and domain modules
pub use data::DataError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use domain::time::Time;
pub use service::ServiceError;

use data::AppDatabase;
use domain::maintenance::Maintenance;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::hitcounter::HitCounter;
use web::renderer::Renderer;

// build a rocket server
pub fn build_a_rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .manage::<Maintenance>(config.maintenance)
        .mount("/api/clip", web::api::routes())
        .mount("/", web::http::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}

// RocketConfig represents the server configuration
pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
    pub maintenance: Maintenance,
}

#[cfg(test)]
pub mod test {
    // new_async_runtime is helper function to spawn a new tokio runtime for tests
    // this is required in all tests related with database because the db runtime is async
    pub fn new_async_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime")
    }
}
