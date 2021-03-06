mod chess_bitboard;
use crate::chess_bitboard::ChessEngine;
use crate::chess_bitboard::FromString;
use bufstream::BufStream;
use chess::ChessMove;
use chess::Color;
use native_tls::TlsConnector;
use native_tls::TlsStream;
use reqwest::header;
use serde_json::Value;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn try_parse_json(json_string: &str) -> serde_json::Result<Value> {
    let v: Value = serde_json::from_str(json_string)?;
    return Ok(v);
}

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
    let mut bot_team = Color::White;
    let mut stream = BufStream::new(&mut stream);
    let mut buf = String::new();
    while stream.read_line(&mut buf).unwrap_or(0) > 0 {
        match try_parse_json(&buf) {
            Ok(v) => {
                // println!("{}", buf);
                let msg_type = v["type"].to_string();
                match msg_type.as_ref() {
                    r#""gameFull""# => {
                        let mut board = ChessEngine::default();
                        let white_team = v["white"]["name"].as_str().unwrap();
                        match white_team {
                            "gambinobot" => bot_team = Color::White,
                            _ => bot_team = Color::Black,
                        }
                        let game_id = v["id"].as_str().unwrap().to_owned();
                        println!("{}", v["state"]["moves"].as_str().unwrap());
                        for next_move in v["state"]["moves"].as_str().unwrap().split_whitespace() {
                            board = ChessEngine::move_piece(
                                &board,
                                ChessMove::from_string(next_move.to_string()),
                            );
                        }
                        println!("{}", board);
                        if board.next_to_move() == bot_team {
                            let bot_move = ChessEngine::find_next_move(&board, 1);
                            let auth_header_value = format!("Bearer {}", lichess_api_token);
                            let client = reqwest::Client::builder().build().unwrap();
                            let endpoint = format!(
                                "https://lichess.org/api/bot/game/{}/move/{}",
                                game_id, bot_move
                            );
                            let _res = client
                                .post(&endpoint)
                                .header(header::AUTHORIZATION, auth_header_value)
                                .send()
                                .await
                                .unwrap();
                        }
                    }
                    r#""gameState""# => {
                        let mut board = ChessEngine::default();
                        for next_move in v["moves"].as_str().unwrap().split_whitespace() {
                            // println!("{}", next_move);
                            board = ChessEngine::move_piece(
                                &board,
                                ChessMove::from_string(next_move.to_string()),
                            );
                        }
                        println!("{}", board);
                        if board.next_to_move() == bot_team {
                            let bot_move = ChessEngine::find_next_move(&board, 2);
                            thread::sleep(Duration::from_millis(100));
                            let auth_header_value = format!("Bearer {}", lichess_api_token);
                            let client = reqwest::Client::builder().build().unwrap();
                            let endpoint = format!(
                                "https://lichess.org/api/bot/game/{}/move/{}",
                                game_id, bot_move
                            );
                            let _res = client
                                .post(&endpoint)
                                .header(header::AUTHORIZATION, auth_header_value)
                                .send()
                                .await
                                .unwrap();
                        }
                    }
                    _ => {}
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
        match try_parse_json(&buf) {
            Ok(v) => {
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
                    _ => (),
                }
            }
            Err(_) => (),
        };
        buf.clear();
    }
}

#[tokio::main]
async fn main() {
    subscribe().await;
}
