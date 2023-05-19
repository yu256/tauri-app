// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

use std::sync::RwLock;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde_json::json;


// グローバル変数 あとでなんとかする
static URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));
static TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));

fn extract_noteid(note_id: String) -> String {
    let temp_id: Vec<&str> = note_id.split('/').collect();
    temp_id[4].to_string()
}

#[tauri::command]
async fn get_note(note_id: String) -> (String, models::User, String, HashMap<String, usize>, Vec<HashMap<String, String>>) {
    let extracted_note_id: String = extract_noteid(note_id);
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let res: models::Note = client
        .post(&format!("{}api/notes/show", url))
        .json(&json!({ "noteId": extracted_note_id }))
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
    (res.createdAt, res.user, res.text, res.reactions, res.emojis)
}

#[tauri::command]
async fn post(text: String) {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let _response: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("{}api/notes/create", url))
        .json(&json!({ "i": access_token, "text": text }))
        .send()
        .await;
}

#[tauri::command]
fn set_token(token: String) {
    *TOKEN.write().unwrap() = token;
}

#[tauri::command]
fn set_url(instanceurl: String) {
    let temp_url: String = match instanceurl.as_str() {
        s if s.starts_with("https://") => {
            if instanceurl.ends_with('/') {
                instanceurl
            } else {
                instanceurl + "/"
            }
        }
        _ => {
            if instanceurl.ends_with('/') {
                "https://".to_string() + &instanceurl
            } else {
                "https://".to_string() + &instanceurl + "/"
            }
        }
    };
    *URL.write().unwrap() = temp_url;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_note, set_url, set_token, post])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
