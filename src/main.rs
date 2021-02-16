use native_tls::TlsConnector;
use native_tls::TlsStream;
use serde_json::Value;
use std::io::BufRead;
use bufstream::BufStream;
use std::io::Write;
use std::net::TcpStream;

fn try_parse_json(json_string: &str) -> serde_json::Result<Value> {
    let v: Value = serde_json::from_str(json_string)?;
    return Ok(v);
}

// Accept Challenge

fn connect_tls_stream() -> Result<TlsStream<TcpStream>, String> {
    println!("TLS connect starting");
    let connector = TlsConnector::new().unwrap();
    let tcp_stream = match TcpStream::connect("lichess.org:443".to_owned()) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };
    let tls_stream = match connector.connect(&"lichess.org".to_owned(), tcp_stream) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };
    return Ok(tls_stream);
}

fn send_msg(tls_stream: &mut TlsStream<TcpStream>, stream_event_msg: &String) {
    tls_stream.write_all(stream_event_msg.as_bytes()).unwrap();
}

async fn subscribe() {
    let mut stream = connect_tls_stream().unwrap();
    let lichess_api_token = std::env::var("lichess_api_token").unwrap_or("NA".to_string());
    let stream_event_msg = format!(
        "GET /api/stream/event HTTP/1.1\nHost: lichess.org\nUser-Agent: curl/7.68.0\nAccept: */*\nAuthorization: Bearer {}\n\n", 
    lichess_api_token);
    send_msg(&mut stream, &stream_event_msg);

    let mut stream = BufStream::new(&mut stream);
    let mut buf = String::new();
    while stream.read_line(&mut buf).unwrap_or(0) > 0 {
        println!("{}", &buf);
        match try_parse_json(&buf) {
            Ok(v) => {
                println!("JSON found");
                let msg_type = v["type"].to_string();
                match msg_type.as_ref() {
                    r#""challenge""# => {
                        println!("Challenge Message Type");
                        let challenge_id = v["challenge"]["id"].as_str().unwrap().to_owned();
                        let accept_challenge_msg = format!(
                            "POST /api/challenge/{}/accept HTTP/1.1\nHost: lichess.org\nUser-Agent: curl/7.68.0\nAccept: */*\nAuthorization: Bearer {}\n\n", 
                        challenge_id, lichess_api_token);
                        println!("Challenge Message Type: \n{}", accept_challenge_msg);
                        // send_msg(&stream, &accept_challenge_msg);
                    }
                    _ => {
                        println!("Unknown Message Type: {}", msg_type);
                    }
                }
            }
            Err(_) => println!("Not JSON"),
        };
        buf.clear();
    }
}

#[tokio::main]
async fn main() {
    subscribe().await;
}
