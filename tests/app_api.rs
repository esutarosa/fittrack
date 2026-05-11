use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{LazyLock, Mutex};
use std::thread;

use fittrack::app::auth::AuthClient;
use fittrack::shared::contracts::{ApiErrorResponse, AuthResponse, UserDto};

static ENV_GUARD: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[test]
fn login_sends_http_request_and_parses_response() {
    let _guard = ENV_GUARD.lock().expect("env guard");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let port = listener.local_addr().expect("addr").port();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        let (request_line, headers, body) = read_request(&mut stream);

        assert!(request_line.contains("POST /api/auth/login HTTP/1.1"));
        assert!(headers.contains("Host: 127.0.0.1"));
        assert!(body.contains("\"username\":\"alice\""));
        assert!(body.contains("\"password\":\"password123\""));

        let response = AuthResponse {
            token: "token-123".to_owned(),
            user: UserDto {
                id: 1,
                username: "alice".to_owned(),
                created_at: "2026-05-11T00:00:00".to_owned(),
            },
        };
        let response_body = serde_json::to_string(&response).expect("json");
        write_response(&mut stream, 200, "OK", &response_body);
    });

    unsafe {
        // tests are serialized with a mutex, so this scoped env mutation is controlled
        std::env::set_var("FITTRACK_API_URL", format!("http://127.0.0.1:{port}/api"));
    }

    let client = AuthClient::from_env().expect("client");
    let auth = client.login("alice", "password123").expect("login");

    assert_eq!(auth.token, "token-123");
    assert_eq!(auth.user.username, "alice");
}

#[test]
fn login_surfaces_server_error() {
    let _guard = ENV_GUARD.lock().expect("env guard");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let port = listener.local_addr().expect("addr").port();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        let _ = read_request(&mut stream);

        let response_body = serde_json::to_string(&ApiErrorResponse {
            error: "Invalid username or password".to_owned(),
        })
        .expect("json");
        write_response(&mut stream, 401, "Unauthorized", &response_body);
    });

    unsafe {
        std::env::set_var("FITTRACK_API_URL", format!("http://127.0.0.1:{port}/api"));
    }

    let client = AuthClient::from_env().expect("client");
    let error = client.login("alice", "wrong").expect_err("error");

    assert_eq!(error.to_string(), "Invalid username or password");
}

fn read_request(stream: &mut std::net::TcpStream) -> (String, String, String) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone"));
    let mut request_line = String::new();
    reader.read_line(&mut request_line).expect("request line");

    let mut headers = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("header line");
        if line == "\r\n" {
            break;
        }
        headers.push_str(&line);
    }

    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .expect("content length");
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).expect("body");

    (request_line.trim().to_owned(), headers, String::from_utf8(body).expect("utf8"))
}

fn write_response(stream: &mut std::net::TcpStream, status: u16, reason: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );

    stream.write_all(response.as_bytes()).expect("response");
    stream.flush().expect("flush");
}
