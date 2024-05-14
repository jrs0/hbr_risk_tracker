use shared::models::{CreatePatient, Patient};
use uuid::Uuid;

mod postgres_patient_repository;

pub use postgres_patient_repository::PostgresPatientRepository;

pub type PatientError = String;
pub type PatientResult<T> = Result<T, PatientError>;

#[async_trait::async_trait]
pub trait PatientRepository: Send + Sync + 'static {
    async fn get_patients(&self) -> PatientResult<Vec<Patient>>;
    async fn get_patient(&self, id: &Uuid) -> PatientResult<Patient>;
    async fn create_patient(&self, id: &CreatePatient) -> PatientResult<Patient>;
    async fn update_patient(&self, id: &Patient) -> PatientResult<Patient>;
    async fn delete_patient(&self, id: &Uuid) -> PatientResult<Uuid>;
}
