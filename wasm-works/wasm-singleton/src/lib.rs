mod generated;

use std::borrow::Borrow;
use std::time::{Duration};
use protobuf::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use crate::generated::filter_constraints::{Constraints, RequestWrapper};
use crate::generated::logger::BinaryWrapper;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(WasmRootContext::default()) });
}}

struct WasmRootContext {
    log_token: u32,
    filter_token: u32,
    filter_grpc: u32,
    log_grpc: u32,
}

impl Default for WasmRootContext {
    fn default() -> WasmRootContext {
        WasmRootContext {
            log_token: 0,
            filter_grpc: 0,
            log_grpc: 0,
            filter_token: 0,
        }
    }
}

impl Context for WasmRootContext {
    fn on_grpc_call_response(&mut self, token_id: u32, _status_code: u32, response_size: usize) {
        if self.filter_grpc == token_id {
            self.pump_to_worker_queue(response_size);
        }
    }
}

impl RootContext for WasmRootContext {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        self.register_queues();
        self.set_tick_period(Duration::from_secs(5));
        true
    }
    fn on_tick(&mut self) {
        self.fetch_filtering_rules();
    }

    fn on_queue_ready(&mut self, _queue_id: u32) {
        if _queue_id == self.log_token {
            self.grpc_log_line();
        }
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl WasmRootContext {
    fn grpc_log_line(&mut self) {
        if let Ok(optional_bytes) = self.dequeue_shared_queue(self.log_token) {
            if let Some(bytes) = optional_bytes {
                let mut req = BinaryWrapper::new();
                req.byteVector = bytes;
                let result = self.dispatch_grpc_call(
                    "guarding_grpc", // grpc call endpoint
                    "logger.Logger", // service name
                    "log",  // method
                    Vec::<(&str, &[u8])>::new(),
                    Some(req.write_to_bytes().unwrap().as_slice()),
                    Duration::from_secs(1),
                );
                if let Ok(token) = result {
                    self.log_grpc = token;
                }
            }
        }
    }

    fn register_queues(&mut self) {
        self.log_token = self.register_shared_queue("log-queue");
        self.filter_token = self.register_shared_queue("filter-queue");
    }

    fn pump_to_worker_queue(&mut self, response_size: usize) {
        let result = self.get_grpc_call_response_body(0, response_size);
        if let Some(bytes) = result {
            let result_wrap = Constraints::parse_from_bytes(bytes.borrow());
            if let Ok(wrapper) = result_wrap {
                let _ =
                    self.enqueue_shared_queue(
                        self.filter_token,
                        Some(wrapper.byteVector.borrow()));
            }
        }
    }

    fn fetch_filtering_rules(&mut self) {
        let mut req = RequestWrapper::new();
        req.byteVector = Vec::from("FetchFilteringRules".as_bytes());
        let result = self.dispatch_grpc_call(
            "guarding_grpc", // grpc call endpoint
            "filter_constraints.FilterConstraints", // service name
            "getConstraints",  // method
            Vec::<(&str, &[u8])>::new(),
            Some(req.write_to_bytes().unwrap().as_slice()),
            Duration::from_secs(1),
        );
        if let Ok(token) = result {
            self.filter_grpc = token;
        }
    }
}
