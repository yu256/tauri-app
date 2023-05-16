// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

static URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));

fn extract_noteid(note_id: String) -> String {
    let tempid: Vec<&str> = note_id.split('/').collect();
    if tempid.len() == 5 {
        tempid[4].to_string()
    } else {
        note_id
    }
}

#[tauri::command]
async fn get_note(note_id: String) -> (String, HashMap<String, usize>) {
    let extracted_note_id: String = extract_noteid(note_id);
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let res: Note = client
        .post(&format!("{}api/notes/show", url))
        .json(&json!({ "noteId": extracted_note_id }))
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
    (res.text, res.reactions)
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Note {
    text: String,
    reactions: HashMap<String, usize>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_note, set_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
