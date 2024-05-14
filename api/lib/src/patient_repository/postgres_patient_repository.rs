use super::{PatientRepository, PatientResult};
use shared::models::{CreatePatient, Patient};
use chrono::offset::Utc;

pub struct PostgresPatientRepository {
    pool: sqlx::PgPool,
}

impl PostgresPatientRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PatientRepository for PostgresPatientRepository {
    async fn get_patients(&self) -> PatientResult<Vec<Patient>> {
        sqlx::query_as::<_, Patient>(
            r#"
      SELECT id, name, date_of_birth, gender, created_at, updated_at
      FROM patients
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_patient(&self, patient_id: &uuid::Uuid) -> PatientResult<Patient> {
        sqlx::query_as::<_, Patient>(
            r#"
      SELECT id, name, date_of_birth, gender, created_at, updated_at
      FROM patients
      WHERE id = $1
      "#,
        )
        .bind(patient_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_patient(&self, create_patient: &CreatePatient) -> PatientResult<Patient> {
        sqlx::query_as::<_, Patient>(
            r#"
      INSERT INTO patients (name, date_of_birth, gender)
      VALUES ($1, $2, $3)
      RETURNING id, name, date_of_birth, gender, created_at, updated_at
      "#,
        )
        .bind(&create_patient.name)
        .bind(&create_patient.date_of_birth)
        .bind(&create_patient.gender)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_patient(&self, patient: &Patient) -> PatientResult<Patient> {
        sqlx::query_as::<_, Patient>(
            r#"
      UPDATE patients
      SET name = $2, date_of_birth = $3, gender = $4, updated_at = $5
      WHERE id = $1
      RETURNING id, name, date_of_birth, gender, created_at, updated_at
      "#,
        )
        .bind(patient.id)
        .bind(&patient.name)
        .bind(&patient.date_of_birth)
            .bind(&patient.gender)
            .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_patient(&self, patient_id: &uuid::Uuid) -> PatientResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
      DELETE FROM patients
      WHERE id = $1
      RETURNING id
      "#,
        )
        .bind(patient_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
