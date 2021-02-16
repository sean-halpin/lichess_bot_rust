use bufstream::BufStream;
use native_tls::TlsConnector;
use native_tls::TlsStream;
use reqwest::header;
use serde_json::Value;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;
mod chess;

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

async fn play_game(game_id: String) {
    let mut stream = connect_tls_stream().unwrap();
    let lichess_api_token = std::env::var("lichess_api_token").unwrap_or("NA".to_string());
    let stream_event_msg = format!(
        "GET /api/bot/game/stream/{} HTTP/1.1\nHost: lichess.org\nUser-Agent: curl/7.68.0\nAccept: */*\nAuthorization: Bearer {}\n\n", 
    game_id, lichess_api_token);
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
                    // {"id":"z3gxFMhD","variant":{"key":"standard","name":"Standard","short":"Std"},"clock":{"initial":600000,"increment":30000},"speed":"classical","perf":{"name":"Classical"},"rated":false,"createdAt":1613505004393,"white":{"id":"ajedrez_87","name":"ajedrez_87","title":null,"rating":1500,"provisional":true},"black":{"id":"gambinobot","name":"gambinobot","title":"BOT","rating":1500,"provisional":true},"initialFen":"startpos","type":"gameFull","state":{"type":"gameState","moves":"","wtime":600000,"btime":600000,"winc":30000,"binc":30000,"wdraw":false,"bdraw":false,"status":"started"}}
                    r#""gameFull""# => {}
                    r#""gameState""# => {}
                    _ => {
                        println!("Game Stream - Unknown Message Type: {}", msg_type);
                    }
                }
            }
            Err(_) => (),
        };
        buf.clear();
    }
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
                // println!("JSON found");
                let msg_type = v["type"].to_string();
                match msg_type.as_ref() {
                    r#""challenge""# => {
                        let challenge_id = v["challenge"]["id"].as_str().unwrap().to_owned();
                        let auth_header_value = format!("Bearer {}", lichess_api_token);
                        let client = reqwest::Client::builder().build().unwrap();
                        let endpoint =
                            format!("https://lichess.org/api/challenge/{}/accept", challenge_id);
                        let _res = client
                            .post(&endpoint)
                            .header(header::AUTHORIZATION, auth_header_value)
                            .send()
                            .await
                            .unwrap();
                    }
                    r#""gameStart""# => {
                        let game_id = v["game"]["id"].as_str().unwrap().to_owned();
                        tokio::spawn(async move { play_game(game_id).await });
                    }
                    _ => {
                        () // println!("Event Stream - Unknown Message Type: {}", msg_type);
                    }
                }
            }
            Err(_) => (),
        };
        buf.clear();
    }
}

#[tokio::main]
async fn main() {
    Board();
    subscribe().await;
}
