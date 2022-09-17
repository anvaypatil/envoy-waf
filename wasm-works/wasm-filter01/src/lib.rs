use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use root_context::WasmRootContext;

mod http_context;
mod root_context;
mod http_instance;
mod expr_matcher;
mod filtering_constraints;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(WasmRootContext::new()) });
}}
