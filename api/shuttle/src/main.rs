use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

use shuttle_runtime::CustomError;
use sqlx::Executor;


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/another")]
async fn hello_world_two() -> &'static str {
    "Hello World two!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/patients_schema.sql"))
	.await
	.map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(hello_world_two);
    };

    Ok(config.into())
}
