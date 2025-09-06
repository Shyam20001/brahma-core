//! ============================================================================
//!  Server Module (Created by Shyam. M)
//!
//!  📌 Important: This module is a core part of Brahma-Core.
//!  It contains the HTTP server implementation, request handling logic,
//!  and the integration layer that bridges Rust <-> Node.js callbacks
//!  currently operates in synchronous only to achieve fire & forget low latency requests.
//!
//!  🔖 Credits: Authored by Shyam. M
//!  🔗 Repository: https://github.com/Shyam20001/brahma-core.git
//!
//!  💡 Note: Please discuss with me before making structural changes.
//!  This file is intended to evolve with future improvements in performance,
//!  security, and extensibility of the framework.
//!
//! ============================================================================

use crate::callback::JS_RESPONSE_CALLBACK;
use crate::executor::TokioExecutor;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto;
use napi_derive::napi;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct ResponseData {
    status: Option<u16>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

#[napi]
pub async fn start_server(host: Option<String>, port: Option<u16>) -> napi::Result<()> {
    let host = host.unwrap_or_else(|| "127.0.0.1".to_string());
    let port = port.unwrap_or(2025);
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| napi::Error::from_reason(format!("Failed to bind to {}: {}", addr, e)))?;

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let service = service_fn(handle_request);

        tokio::spawn(async move {
            if let Err(err) = auto::Builder::new(TokioExecutor)
                .serve_connection(io, service)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    mut req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("").to_string();

    let mut headers_map = HashMap::new();
    for (name, value) in req.headers().iter() {
        headers_map.insert(
            name.as_str().to_string(),
            value.to_str().unwrap_or("").to_string(),
        );
    }
    let headers_json = serde_json::to_string(&headers_map)?;

    let body_bytes = req.body_mut().collect().await?.to_bytes();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    if let Some(js_cb) = JS_RESPONSE_CALLBACK.get() {
        let js_response = js_cb
            .call_async(Ok((path.clone(), query, headers_json, body_str)))
            .await?;

        let resp_data: ResponseData = serde_json::from_str(&js_response).unwrap_or(ResponseData {
            status: Some(200),
            headers: None,
            body: Some(js_response.clone()),
        });

        let mut response_builder = Response::builder().status(resp_data.status.unwrap_or(200));

        if let Some(headers) = resp_data.headers {
            for (k, v) in headers {
                response_builder = response_builder.header(k.as_str(), v.as_str());
            }
        } else {
            response_builder = response_builder.header("Content-Type", "text/plain");
        }

        let body = resp_data.body.unwrap_or_default();
        Ok(response_builder.body(Full::new(Bytes::from(body)))?)
    } else {
        Ok(Response::builder()
            .status(500)
            .body(Full::new(Bytes::from("JS callback not registered")))
            .unwrap())
    }
}
