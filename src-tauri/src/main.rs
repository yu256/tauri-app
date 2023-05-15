// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

static URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));

#[tauri::command]
async fn get_note(note_id: String) -> String {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let res: Note = client
        .post(&format!("{}api/notes/show", url))
        .json(&json!({ "noteId": note_id }))
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
    res.text
}

#[tauri::command]
fn set_url(instanceurl: String) {
    let temp_url: String = match instanceurl.as_str() {
        "https://" =>
            if instanceurl.ends_with("/") {
                instanceurl.clone()
            } else {
                instanceurl.clone() + "/"
            },
        _ => {
            if instanceurl.ends_with('/') {
                "https://".to_owned() + &instanceurl
            } else {
                "https://".to_owned() + &instanceurl + "/"
            }
        }
    };
    *URL.write().unwrap() = temp_url;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Note {
    text: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_note, set_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
