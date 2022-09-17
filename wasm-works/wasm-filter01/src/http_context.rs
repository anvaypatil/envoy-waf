use std::borrow::Borrow;
use std::rc::Rc;

use log::info;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;
use crate::expr_matcher::{append_context_vec, bool_check, get_expression_context};
use crate::filtering_constraints::{ActionEnum, FilteringConstraints};

use crate::http_instance::HttpRequestResponse;

pub struct HttpCallContext {
    log_req_res: HttpRequestResponse,
    log_queue_token: u32,
    filters: Rc<FilteringConstraints>,
}

impl HttpCallContext {
    pub fn new(log_queue_token: u32, filters: Rc<FilteringConstraints>) -> HttpCallContext {
        HttpCallContext {
            log_req_res: HttpRequestResponse::new(),
            log_queue_token,
            filters,
        }
    }

    fn header_filtering(&mut self, headers: Vec<(String, String)>) -> Option<ActionEnum> {
        let mut header_context = get_expression_context();
        let head_str = format!("{:?}", &headers);
        let header_context = append_context_vec(&mut header_context, headers);
        let filter = &self.filters;
        if let Some(header_filter) = &filter.request {
            if let Some(expression) = &header_filter.allow_constraints {
                info!("Allow Expression: {}", expression);
                info!("Debug Headers {}", head_str);
                let result = bool_check(expression, &header_context);
                if let Ok(v) = result {
                    info!("Ok Expression: {}", v);
                } else {
                    info!("Expression Err: {}", result.err().unwrap());
                }
                return Some(ActionEnum::Allow);
            }
            if let Some(expression) = &header_filter.deny_constraints {
                info!("Deny Expression: {}",expression);
                return Some(ActionEnum::Deny);
            }
            if let Some(expression) = &header_filter.log_constraints {
                info!("Log Expression: {}",expression);
                return Some(ActionEnum::Log);
            }
        }
        None
    }
}

impl Context for HttpCallContext {}

impl HttpContext for HttpCallContext {
    fn on_http_request_headers(&mut self, _: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            let headers = self.get_http_request_headers();
            let _filter_option = self.header_filtering(headers);
            let headers = self.get_http_request_headers();
            self.log_req_res.request_header = Some(headers);
        }
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            let body_object = self.get_http_request_body(0, _body_size);
            self.log_req_res.request_body = body_object;
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            let headers = self.get_http_request_headers();
            self.log_req_res.response_header = Option::Some(headers);
        }
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            let body_object = self.get_http_response_body(0, _body_size);
            self.log_req_res.response_body = body_object;
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        let result = HttpRequestResponse::encode_contents(&self.log_req_res);
        if let Ok(bin) = result {
            let o = bin.borrow();
            let _ = self.enqueue_shared_queue(self.log_queue_token, Option::Some(o));
        }
    }
}

