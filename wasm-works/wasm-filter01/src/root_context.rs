use std::rc::Rc;
use std::time::Duration;

use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;

use crate::filtering_constraints::FilteringConstraints;
use crate::http_context::HttpCallContext;

const MASTER_VM_ID: &str = "wasm-master-vm";
const LOG_QUEUE: &str = "log-queue";
const FILTER_QUEUE: &str = "filter-queue";


pub struct WasmRootContext {
    log_queue_token: u32,
    filter_queue_token: u32,
    filtering_constraints: Rc<FilteringConstraints>
}

impl WasmRootContext {
    pub fn new() -> WasmRootContext {
        WasmRootContext {
            log_queue_token: 0,
            filter_queue_token: 0,
            filtering_constraints: Default::default()
        }
    }

    fn resolve_log_queue(&mut self) {
        if let Some(token) = self.get_log_queue_token() {
            self.log_queue_token = token;
        }
    }

    fn get_log_queue_token(&mut self) -> Option<u32> {
        self.resolve_shared_queue(MASTER_VM_ID, LOG_QUEUE)
    }

    fn resolve_control_queue(&mut self) {
        if let Some(token) = self.get_filter_queue_token() {
            self.filter_queue_token = token;
        }
    }

    fn get_filter_queue_token(&mut self) -> Option<u32> {
        self.resolve_shared_queue(MASTER_VM_ID, FILTER_QUEUE)
    }

    fn read_control_queue(&mut self) {
        if let Ok(optional_bytes) = self.dequeue_shared_queue(self.filter_queue_token) {
            if let Some(bytes) = optional_bytes {
                if let Ok((filter, _)) = FilteringConstraints::decode_filters(bytes) {
                    self.filtering_constraints = Rc::new(FilteringConstraints {
                        ..filter
                    });
                }
            }
        }
    }
}

impl Context for WasmRootContext {}

impl RootContext for WasmRootContext {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        self.set_tick_period(Duration::from_secs(5));
        true
    }
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        self.resolve_log_queue();
        self.resolve_control_queue();
        true
    }

    fn on_tick(&mut self) {
        self.read_control_queue();
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        let call_context =
            HttpCallContext::new(
                self.log_queue_token,
                Rc::clone(&self.filtering_constraints),
            );
        Some(Box::new(call_context))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
