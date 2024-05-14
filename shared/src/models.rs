use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Patient {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub name: String,
    pub gender: String,
    pub date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreatePatient {
    pub name: String,
    pub gender: String,
    pub date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
}

impl Patient {

    pub fn date_of_birth_string(&self) -> String {
	match self.date_of_birth {
	    Some(date_of_birth) => date_of_birth.format("%Y-%m-%d").to_string(),
	    None => "Not recorded".to_string()
	}
    }
}
