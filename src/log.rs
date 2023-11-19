use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{Value, json};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{
    context::Context,
    error::{ClientError, Error, Result},
};

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    user_id: Option<u64>,
    uuid: String,
    timestamp: String,
    request_path: String,
    request_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    request_method: Method,
    uri: Uri,
    context: Option<Context>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));
    let log_line = RequestLogLine {
        user_id: context.map(|c| c.user_id()),
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        request_path: uri.to_string(),
        request_method: request_method.to_string(),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data
    };
    println!("-->> log_request \n{}", json!(log_line));

    Ok(())
}
