use whoami;
use reqwest;
use std::fs::{self, File};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt,SinkExt};
use std::path::{Path, PathBuf};

async fn get_paths() -> (String, String) {
    let user = whoami::username();

    let executor_path = format!("C:/Users/{user}/AppData/Local/ui/workspace");
    let response = reqwest::get("https://clientsettings.roblox.com/v1/client-version/WindowsPlayer")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();

    let current_version = response.get("clientVersionUpload").unwrap();
    let roblox_path = format!("C:/Program Files (x86)/Roblox/Versions/{current_version}/content");

    if fs::metadata(&roblox_path).is_err() {
        panic!("Roblox installation not found, are you maybe on the wrong version?");
    }

    if fs::metadata(&executor_path).is_err() {
        panic!("Executor path not found");
    }

    (roblox_path, executor_path)
}

#[tokio::main]
async fn main() {
    let (roblox_path, executor_path) = get_paths().await;

    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    println!("Listening on ws://127.0.0.1:6969");

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        while let Some(Ok(msg)) = read.next().await {
            if msg.is_text() {
                let executor_file = format!("{executor_path}/{msg}");
                match fs::metadata(&executor_file) {
                    Ok(_) => {
                        let file_name = Path::new(&executor_file).file_name().unwrap().to_str().unwrap();
                        let destination = format!("{roblox_path}/{file_name}");
                        let asset = format!("rbxasset://{file_name}");

                        /*
                            I won't add a check to see if the destination already exists since
                            assets could have the same name
                         */

                        match fs::copy(&executor_file, &destination) {
                            Ok(_) => {

                                println!("Successfully moved: {executor_file}");
                                write.send(Message::Text(asset)).await.unwrap()
                            },
                            Err(e) => eprintln!("Failed to copy executor file: {} {executor_file}", e),
                        }
                    },
                    Err(e) => {
                        eprintln!("Executor file does not exist or cannot be accessed: {}", e);
                    }
                }
            }
        }
    }
}
