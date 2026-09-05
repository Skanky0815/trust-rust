use crate::schema::patients;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Patient {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = patients)]
pub struct NewPatient {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl NewPatient {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientEvent {
    pub datetime: DateTime<Utc>,
    pub trace_id: Uuid,
    pub source: String,
    pub payload: Patient,
}

impl PatientEvent {
    pub fn new(patient: Patient) -> Self {
        Self {
            datetime: Utc::now(),
            trace_id: Uuid::new_v4(),
            source: "patient-service".to_string(),
            payload: patient,
        }
    }
}
