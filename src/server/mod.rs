mod web_server;
mod engine_service;
mod protocol;
mod utils;

pub use web_server::WebServer;
pub use engine_service::EngineService;

use protocol::{
    GetPathing, put_pathing, get_pathing, del_pathing,
    Respond
};

use utils::{
    http_request_to_bytes, bytes_to_http_response, text_to_http_response, http_request_validate_cors, cors_preflight_http_response,
    bytes_to_deserialized_value, serialized_respond_to_bytes
};
