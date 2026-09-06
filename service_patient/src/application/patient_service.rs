use chrono::NaiveDate;
use lapin::Channel;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::application::patient_service::grpc::patient_service_server::PatientService as PatientGrpcService;
use crate::application::patient_service::grpc::{
    get_patient_request, GetPatientRequest, PatientListResponse, PatientRequest, PatientResponse,
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

        let new_patient = req.to_new_patient();

        let patient = self.patient_server.add(new_patient).await.map_err(|e| {
            eprintln!("Error adding patient: {:?}", e);
            Status::internal("Failed to add patient")
        })?;

        let response = patient.to_response();

        Ok(Response::new(response))
    }

    async fn get(
        &self,
        request: Request<GetPatientRequest>,
    ) -> Result<Response<PatientResponse>, Status> {
        let search = request
            .into_inner()
            .search
            .expect("Search parameter is required");

        let patient = match search {
            get_patient_request::Search::ExternalId(external_id) => {
                self.patient_server.get_by_external_id(external_id).await
            }
            get_patient_request::Search::SearchCriteria(criteria) => {
                let search_date_of_birth =
                    NaiveDate::parse_from_str(&criteria.date_of_birth, "%Y-%m-%d")
                        .map_err(|_| Status::invalid_argument("Invalid date format. It must be in the format YYYY-MM-DD"))?;

                self.patient_server
                    .get_by_search_criteria(search_date_of_birth, criteria.insurance_number)
                    .await
            }
        }
        .map_err(|e| {
            eprintln!("Error getting patient: {:?}", e);
            Status::not_found("No Patient found for the given criteria")
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

impl PatientRequest {
    fn to_new_patient(&self) -> NewPatient {
        NewPatient::new(
            self.first_name.clone(),
            self.last_name.clone(),
        )
    }
}