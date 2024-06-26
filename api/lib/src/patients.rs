use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreatePatient, Patient};
use uuid::Uuid;

use crate::patient_repository::PatientRepository;

pub fn service<R: PatientRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/patients")
            .route("", web::get().to(get_all::<R>))
            .route("/{patient_id}", web::get().to(get::<R>))
            .route("", web::post().to(post::<R>))
            .route("", web::put().to(put::<R>))
            .route("/{patient_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: PatientRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_patients().await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: PatientRepository>(
    patient_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.get_patient(&patient_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn post<R: PatientRepository>(
    create_patient: web::Json<CreatePatient>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_patient(&create_patient).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn put<R: PatientRepository>(
    patient: web::Json<Patient>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.update_patient(&patient).await {
        Ok(patient) => HttpResponse::Ok().json(patient),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: PatientRepository>(
    patient_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.delete_patient(&patient_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
