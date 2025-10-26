use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::env;
use std::io;
use log::{info, warn, error, debug};

async fn proxy(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let socket_path = "/var/run/docker.sock";
    let method = req.method().as_str();
    let uri = req.uri();

    // Determine client IP (prefer X-Forwarded-For if present)
    let client_ip = req.headers().get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| req.peer_addr().map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string()));

    info!("INFO: Proxying request: {} {} from {}", method, uri, client_ip);

    // Connect to Docker Unix socket
    let mut stream = match UnixStream::connect(socket_path).await {
        Ok(stream) => {
            debug!("Successfully connected to Docker socket at {}", socket_path);
            stream
        },
        Err(e) => {
            error!("Failed to connect to Docker socket at {}: {}", socket_path, e);
            return HttpResponse::InternalServerError().body("Failed to connect to Docker socket");
        }
    };

    // Build HTTP request string
    let mut http_request = format!("{} {} HTTP/1.1\r\n", method, uri);
    debug!("Building HTTP request: {} {}", method, uri);

    // Add headers
    for (name, value) in req.headers() {
        if let Ok(value_str) = value.to_str() {
            http_request.push_str(&format!("{}: {}\r\n", name, value_str));
            debug!("Added header: {}: {}", name, value_str);
        } else {
            warn!("Skipping header {} with invalid UTF-8 value", name);
        }
    }

    // Add API key if configured
    if let Ok(api_key) = env::var("API_KEY") {
        http_request.push_str(&format!("x-api-key: {}\r\n", api_key));
        info!("Added API key authentication header");
    }

    // Add Host header for Docker API
    http_request.push_str("Host: localhost\r\n");

    // Add Content-Length if we have a body
    if !body.is_empty() {
        http_request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        debug!("Request body length: {} bytes", body.len());
    }

    http_request.push_str("\r\n");

    // Write request to socket
    if let Err(e) = stream.write_all(http_request.as_bytes()).await {
        error!("Failed to write HTTP request to socket: {}", e);
        return HttpResponse::InternalServerError().body("Failed to write to socket");
    }
    debug!("Successfully wrote HTTP request headers to socket");

    // Write body if present
    if !body.is_empty() {
        if let Err(e) = stream.write_all(&body).await {
            error!("Failed to write request body to socket: {}", e);
            return HttpResponse::InternalServerError().body("Failed to write body to socket");
        }
        debug!("Successfully wrote request body to socket");
    }

    // Read response
    let mut response_buffer = Vec::new();
    if let Err(e) = stream.read_to_end(&mut response_buffer).await {
        error!("Failed to read response from socket: {}", e);
        return HttpResponse::InternalServerError().body("Failed to read response");
    }
    debug!("Read {} bytes from Docker socket", response_buffer.len());

    // Parse HTTP response
    let response_str = match String::from_utf8(response_buffer) {
        Ok(s) => s,
        Err(e) => {
            error!("Invalid UTF-8 response from Docker socket: {}", e);
            return HttpResponse::InternalServerError().body("Invalid response encoding");
        }
    };

    // Split headers and body
    let parts: Vec<String> = response_str.splitn(2, "\r\n\r\n").map(|s| s.to_string()).collect();
    if parts.len() != 2 {
        error!("Invalid HTTP response format - missing header/body separator");
        return HttpResponse::InternalServerError().body("Invalid HTTP response format");
    }

    let headers_part = &parts[0];
    let body_part = &parts[1];

    // Parse status line
    let lines: Vec<&str> = headers_part.lines().collect();
    if lines.is_empty() {
        error!("Empty response from Docker socket");
        return HttpResponse::InternalServerError().body("Empty response");
    }

    let status_line = lines[0];
    debug!("Response status line: {}", status_line);
    let status_code = if let Some(code_str) = status_line.split_whitespace().nth(1) {
        match code_str.parse::<u16>() {
            Ok(code) => {
                info!("Docker API responded with status code: {}", code);
                code
            },
            Err(e) => {
                error!("Failed to parse status code '{}': {}", code_str, e);
                500
            }
        }
    } else {
        error!("No status code found in response line: {}", status_line);
        500
    };

    // Build response
    let mut response_builder = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    );

    // Add response headers (skip status line)
    for line in &lines[1..] {
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();
            response_builder.insert_header((name, value));
            debug!("Added response header: {}: {}", name, value);
        }
    }

    info!("Proxy request completed successfully for {} {}", method, uri);
    response_builder.body(body_part.clone())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logging first
    env::set_var("RUST_LOG", "actix_web=info,docker_socket_proxy=info");
    env_logger::init();

    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");
    let port = env::var("PORT").unwrap_or_else(|_| "3277".to_string());

    println!("🚀 Starting {} v{}", app_name, app_version);
    println!("📡 Binding to 0.0.0.0:{}", port);
    info!("🚀 Starting {} v{}", app_name, app_version);
    info!("📡 Binding to 0.0.0.0:{}", port);

    let server = HttpServer::new(|| App::new().default_service(web::route().to(proxy)))
        .bind(format!("0.0.0.0:{}", port))?;

    println!("✅ {} v{} is ready and listening on port {}", app_name, app_version, port);
    println!("🔗 Docker socket proxy ready to accept connections");
    info!("✅ {} v{} is ready and listening on port {}", app_name, app_version, port);
    info!("🔗 Docker socket proxy ready to accept connections");

    server.run().await
}