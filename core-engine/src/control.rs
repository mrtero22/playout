use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::engine::PlayoutEngine;
use crate::playlist::Playlist;

pub fn run_http_server(bind: &str, engine: Arc<Mutex<PlayoutEngine>>) -> Result<(), String> {
    let listener = TcpListener::bind(bind).map_err(|err| format!("failed to bind {bind}: {err}"))?;
    for stream in listener.incoming() {
        let stream = stream.map_err(|err| format!("http accept failed: {err}"))?;
        let engine = Arc::clone(&engine);
        thread::spawn(move || {
            if let Err(err) = handle_client(stream, engine) {
                eprintln!("http request failed: {err}");
            }
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, engine: Arc<Mutex<PlayoutEngine>>) -> Result<(), String> {
    let mut buffer = [0_u8; 16384];
    let read = stream.read(&mut buffer).map_err(|err| err.to_string())?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or("");
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

    let response = route(request_line, body, engine);
    stream
        .write_all(response.as_bytes())
        .map_err(|err| format!("failed to write response: {err}"))
}

fn route(request_line: &str, body: &str, engine: Arc<Mutex<PlayoutEngine>>) -> String {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return response(400, "text/plain", "bad request");
    }
    let method = parts[0];
    let path = parts[1];
    let mut engine = match engine.lock() {
        Ok(engine) => engine,
        Err(_) => return response(500, "text/plain", "engine lock poisoned"),
    };

    match (method, path) {
        ("GET", "/") => response(200, "text/html", include_str!("../web/index.html")),
        ("GET", "/api/status") => response(200, "application/json", &engine.status_json()),
        ("GET", "/api/playlist") => response(200, "application/json", &engine.playlist_json()),
        ("POST", "/api/play") => match engine.start() {
            Ok(()) => response(202, "application/json", "{\"ok\":true}"),
            Err(err) => response(409, "application/json", &format!("{{\"ok\":false,\"error\":\"{}\"}}", crate::playlist::json_escape(&err))),
        },
        ("POST", "/api/stop") => match engine.stop() {
            Ok(()) => response(202, "application/json", "{\"ok\":true}"),
            Err(err) => response(500, "application/json", &format!("{{\"ok\":false,\"error\":\"{}\"}}", crate::playlist::json_escape(&err))),
        },
        ("POST", "/api/restart") => match engine.restart() {
            Ok(()) => response(202, "application/json", "{\"ok\":true}"),
            Err(err) => response(409, "application/json", &format!("{{\"ok\":false,\"error\":\"{}\"}}", crate::playlist::json_escape(&err))),
        },
        ("POST", "/api/playlist") => {
            let path = body.trim();
            match Playlist::from_file(Path::new(path)) {
                Ok(playlist) => {
                    engine.load_playlist(playlist);
                    response(202, "application/json", "{\"ok\":true}")
                }
                Err(err) => response(400, "application/json", &format!("{{\"ok\":false,\"error\":\"{}\"}}", crate::playlist::json_escape(&err))),
            }
        }
        _ => response(404, "text/plain", "not found"),
    }
}

fn response(status: u16, content_type: &str, body: &str) -> String {
    let reason = match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        500 => "Internal Server Error",
        _ => "OK",
    };
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.as_bytes().len()
    )
}

