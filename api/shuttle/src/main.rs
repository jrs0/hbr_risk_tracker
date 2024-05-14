use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/patients_schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let patient_repository = api_lib::patient_repository::PostgresPatientRepository::new(pool);
    let patient_repository = actix_web::web::Data::new(patient_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(patient_repository)
            .configure(api_lib::health::service)
            .configure(api_lib::patients::service::<api_lib::patient_repository::PostgresPatientRepository>);
    };

    Ok(config.into())
}
