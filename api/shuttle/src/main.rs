use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use shuttle_runtime::CustomError;
use sqlx::Executor;

use api_lib::health::health;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/patients_schema.sql"))
	.await
	.map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
	cfg.app_data(pool).service(health);
    };

    
    Ok(config.into())
}
