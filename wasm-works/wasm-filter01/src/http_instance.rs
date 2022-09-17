use bincode::{config, Decode, Encode};
use bincode::error::EncodeError;
use proxy_wasm::types::Bytes;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct HttpRequestResponse {
    pub request_header: Option<Vec<(String, String)>>,
    pub request_body: Option<Bytes>,
    pub response_header: Option<Vec<(String, String)>>,
    pub response_body: Option<Bytes>,
}

impl HttpRequestResponse {
    pub fn new() -> HttpRequestResponse {
        HttpRequestResponse {
            request_header: Default::default(),
            request_body: Default::default(),
            response_header: Default::default(),
            response_body: Default::default(),
        }
    }
}

impl HttpRequestResponse {
    pub fn encode_contents(content: &HttpRequestResponse) -> Result<Vec<u8>, EncodeError> {
        return bincode::encode_to_vec(content, config::standard());
    }
}
