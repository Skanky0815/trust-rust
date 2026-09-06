use lapin::Channel;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::application::patient_service::grpc::patient_service_server::PatientService as PatientGrpcService;
use crate::application::patient_service::grpc::{
    PatientListResponse, PatientRequest, PatientResponse,
};
use crate::infrastructure::database::DbPool;
use crate::module::patient::model::{NewPatient, Patient};
use crate::module::patient::service::PatientService;

pub mod grpc {
    tonic::include_proto!("de.trustrust.grpc.patients");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("patients_descriptor");
}

#[derive(Clone)]
pub struct Service {
    patient_server: Arc<PatientService>,
}

impl Service {
    pub fn new(db: DbPool, channel: Channel) -> Self {
        Self {
            patient_server: Arc::new(PatientService::new(db, channel)),
        }
    }
}

#[tonic::async_trait]
impl PatientGrpcService for Service {
    async fn add(
        &self,
        request: Request<PatientRequest>,
    ) -> Result<Response<PatientResponse>, Status> {
        let req = request.into_inner();

        let new_patient = NewPatient::new(req.first_name, req.last_name);

        let patient = self.patient_server.add(new_patient).await.map_err(|e| {
            eprintln!("Error adding patient: {:?}", e);
            Status::internal("Failed to add patient")
        })?;

        let response = patient.to_response();

        Ok(Response::new(response))
    }

    async fn get_all(
        &self,
        _request: Request<()>,
    ) -> Result<Response<PatientListResponse>, Status> {
        println!("Received patient request");

        let patients = self.patient_server.get_all().await.map_err(|e| {
            eprintln!("Error getting patients: {:?}", e);
            Status::internal("Failed to get patients")
        })?;

        let mut records = Vec::new();

        for patient in patients {
            records.push(patient.to_response());
        }

        Ok(Response::new(PatientListResponse { patients: records }))
    }
}

impl Patient {
    fn to_response(&self) -> PatientResponse {
        PatientResponse {
            id: self.id.to_string(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            external_id: self.external_id.clone(),
            date_of_birth: self.date_of_birth.map(|date| date.to_string()),
            insurance_number: self.insurance_number.clone(),
        }
    }
}