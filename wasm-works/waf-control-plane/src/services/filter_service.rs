use tonic::{Request, Response, Status};

use crate::generated::filter_constraints::filter_constraints_server::FilterConstraints;
use crate::generated::filter_constraints::{Constraints, RequestWrapper};
use crate::services::constraints::{ActionEnum, FilteringConstraints, RequestHeadersConstraint};

#[derive(Default)]
pub struct FilterConstraintsService {}

#[tonic::async_trait]
impl FilterConstraints for FilterConstraintsService {
    async fn get_constraints(&self, _: Request<RequestWrapper>) -> Result<Response<Constraints>, Status> {
        let filter = Self::create_control_filter();
        let bytes = FilteringConstraints::encode(filter);
        let len = bytes.len();
        let response = Constraints { byte_vector: bytes };
        println!("sent {}", len);
        Ok(Response::new(response))
    }
}

impl FilterConstraintsService {
    fn create_control_filter() -> FilteringConstraints {
        let header_constraint = RequestHeadersConstraint {
            allow_constraints: Some(String::from("match(index(split(head(lowercase(\"content-type\")), \";\"), 0), \"application/json\")")),
            deny_constraints: Some(String::from("Denyeee Expression")),
            log_constraints: Some(String::from("Logee Expression")),
        };
        let filter = FilteringConstraints { request: Some(header_constraint), default: ActionEnum::Log };
        filter
    }
}