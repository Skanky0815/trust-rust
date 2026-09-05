use crate::infrastructure::database::DbPool;
use crate::module::patient::model::{NewPatient, Patient, PatientEvent};
use crate::schema::patients;
use diesel::RunQueryDsl;
use lapin::Channel;
use std::sync::Arc;

#[derive(Clone)]
pub struct PatientService {
    db: DbPool,
    channel: Arc<Channel>,
}

impl PatientService {
    pub fn new(db: DbPool, channel: Channel) -> Self {
        Self {
            db,
            channel: Arc::new(channel),
        }
    }

    fn validate_patient(new_patient: &NewPatient) -> Result<(), String> {
        if new_patient.first_name.is_empty() {
            return Err("First name cannot be empty".to_string());
        }
        if new_patient.last_name.is_empty() {
            return Err("Last name cannot be empty".to_string());
        }
        Ok(())
    }

    pub async fn add(&self, new_patient: NewPatient) -> Result<Patient, String> {
        let db = self.db.clone();
        let patient = tokio::task::spawn_blocking(move || {
            Self::validate_patient(&new_patient)?;

            let mut conn = db.get().map_err(|_| "Connection pool error".to_string())?;

            println!("Adding patient: {:?}", new_patient);
            diesel::insert_into(patients::table)
                .values(&new_patient)
                .get_result::<Patient>(&mut conn)
                .map_err(|_| "Insert error".to_string())
        })
        .await
        .map_err(|_| "Task error".to_string())??;

        self.publish_patient_event(&patient).await?;

        Ok(patient)
    }

    pub async fn get_all(&self) -> Result<Vec<Patient>, String> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = db.get().map_err(|_| "Connection pool error".to_string())?;

            patients::table
                .load::<Patient>(&mut conn)
                .map_err(|_| "Load error".to_string())
        })
        .await
        .map_err(|_| "Task error".to_string())?
    }

    async fn publish_patient_event(&self, patient: &Patient) -> Result<(), String> {
        let event = PatientEvent::new(patient.clone());
        let payload = serde_json::to_vec(&event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;

        self.channel
            .basic_publish(
                "".into(),
                "new_patient".into(),
                lapin::options::BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default(),
            )
            .await
            .map_err(|e| format!("RabbitMQ publish failed: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_validate_empty_first_name() {
        let patient = NewPatient {
            id: Uuid::new_v4(),
            first_name: String::new(),
            last_name: "Doe".to_string(),
        };

        let result = PatientService::validate_patient(&patient);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "First name cannot be empty".to_string());
    }

    #[test]
    fn test_validate_empty_last_name() {
        let patient = NewPatient {
            id: Uuid::new_v4(),
            first_name: "John".to_string(),
            last_name: String::new(),
        };

        let result = PatientService::validate_patient(&patient);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Last name cannot be empty".to_string());
    }

    #[test]
    fn test_validate_valid_patient() {
        let patient = NewPatient {
            id: Uuid::new_v4(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };

        let result = PatientService::validate_patient(&patient);
        assert!(result.is_ok());
    }
}
