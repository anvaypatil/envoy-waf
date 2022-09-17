use std::borrow::Borrow;
use bincode::config;
use bincode::{Encode, Decode};
use bincode::error::DecodeError;
use tonic::{Request, Response, Status};
use crate::generated::logger::{BinaryWrapper, LogAck};
use crate::generated::logger::logger_server::Logger;


pub type Bytes = Vec<u8>;

#[derive(Encode, Decode, PartialEq, Debug)]
struct HttpCall {
    request_header: Option<Vec<(String, String)>>,
    request_body: Option<Bytes>,
    response_header: Option<Vec<(String, String)>>,
    response_body: Option<Bytes>,
}

#[derive(Default)]
pub struct LogCollectorService {}

#[tonic::async_trait]
impl Logger for LogCollectorService {
    async fn log(&self, request: Request<BinaryWrapper>) -> Result<Response<LogAck>, Status> {
        let wrapper = request.into_inner();
        let bytes = wrapper.byte_vector;
        Self::unwrap_request(bytes);
        let response = LogAck { ack: false };
        Ok(Response::new(response))
    }
}

impl LogCollectorService {
    fn decode_bytes(bytes: Vec<u8>) -> Result<(HttpCall, usize), DecodeError> {
        bincode::decode_from_slice(bytes.borrow(), config::standard())
    }

    fn extract_headers(headers: Option<Vec<(String, String)>>) -> String {
        let optional_headers = headers.as_ref();
        if let Some(headers) = optional_headers {
            return format!("{:?}", headers);
        }
        String::new()
    }

    fn extract_body(body: Option<Bytes>) -> String {
        if let Some(request) = body {
            if let Ok(request_body) = String::from_utf8(request) {
                return format!("{}", request_body);
            }
        }
        String::new()
    }

    fn extract_http_call(http_call: HttpCall) {
        let request_headers = Self::extract_headers(http_call.request_header);
        let request_body = Self::extract_body(http_call.request_body);
        let response_headers = Self::extract_headers(http_call.response_header);
        let response_body = Self::extract_body(http_call.response_body);
        println!("Request:\n Headers: {} \n Body: {}", request_headers, request_body);
        println!("Response:\n Headers: {} \n Body: {}", response_headers, response_body);
    }

    fn unwrap_request(bytes: Vec<u8>) {
        let decode = Self::decode_bytes(bytes);
        if let Ok((http_call, _)) = decode {
            Self::extract_http_call(http_call);
        }
    }
}
