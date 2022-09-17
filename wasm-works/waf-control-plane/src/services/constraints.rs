use std::borrow::Borrow;
use bincode::{Encode, Decode, config};
use bincode::error::DecodeError;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct RequestHeadersConstraint {
    pub allow_constraints: Option<String>,
    pub deny_constraints: Option<String>,
    pub log_constraints: Option<String>,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct FilteringConstraints {
    pub request: Option<RequestHeadersConstraint>,
    pub default: ActionEnum
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ActionEnum {
    Allow,
    Deny,
    Log,
}

#[allow(dead_code)]
impl FilteringConstraints {
    pub fn encode(filter: FilteringConstraints) -> Vec<u8> {
        bincode::encode_to_vec(filter, config::standard()).unwrap()
    }
    pub fn decode_filters(bytes: Vec<u8>) -> Result<(FilteringConstraints, usize), DecodeError> {
        bincode::decode_from_slice(bytes.borrow(), config::standard())
    }
}