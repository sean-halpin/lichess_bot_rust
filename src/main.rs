use native_tls::TlsConnector;
use native_tls::TlsStream;
use serde_json::Value;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn try_parse_json(json_string: &str) -> serde_json::Result<Value> {
    let v: Value = serde_json::from_str(json_string)?;
    return Ok(v);
}

// Accept Challenge

// > POST /api/challenge/MUlGTvsX/accept HTTP/1.1
// > Host: lichess.org
// > User-Agent: curl/7.68.0
// > Accept: */*
// > Authorization: Bearer {}
// > 


fn connect_tls_stream() -> Result<TlsStream<TcpStream>, String> {
    println!("TLS connect starting");
    let lichess_api_token = std::env::var("lichess_api_token").unwrap_or("NA".to_string());
    let stream_event_msg = format!(
        "GET /api/stream/event HTTP/1.1\nHost: lichess.org\nUser-Agent: curl/7.68.0\nAccept: */*\nAuthorization: Bearer {}\n\n", 
    lichess_api_token);

    println!("{}", stream_event_msg);

    let connector = TlsConnector::new().unwrap();

    let tcp_stream = match TcpStream::connect("lichess.org:443".to_owned()) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };
    let mut tls_stream = match connector.connect(&"lichess.org".to_owned(), tcp_stream) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };

    tls_stream.write_all(stream_event_msg.as_bytes()).unwrap();
    return Ok(tls_stream);
}

pub async fn subscribe() {
    let stream = connect_tls_stream().unwrap();

    let mut stream_reader = BufReader::new(stream);
    let mut buf = String::new();
    while stream_reader.read_line(&mut buf).unwrap_or(0) > 0 {
        println!("{}", &buf);
        match try_parse_json(&buf) {
            Ok(v) => {
                println!("JSON found");
                let msg_type = v["type"].to_string();
                match msg_type.as_ref() {
                    r#""challenge""# => {
                        println!("Challenge Message Type");
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
