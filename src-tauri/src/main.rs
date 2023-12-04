// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;

use lazy_static::lazy_static;
use rayon_hash::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<HashMap<usize, HashMap<usize, usize>>> = Mutex::new(HashMap::new());
}

#[tauri::command]
async fn load_dataset(path: String) {
    println!("Loading dataset");
    println!("Path: {}", path);
    let file = File::open(path).unwrap();
    let mut sparse_matrix = STATE.lock().unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        if let (Some(from_str), Some(to_str)) = (iter.next(), iter.next()) {
            if let (Ok(from), Ok(to)) = (from_str.parse::<usize>(), to_str.parse::<usize>()) {
                sparse_matrix
                    .entry(from)
                    .or_insert_with(HashMap::new)
                    .entry(to)
                    .or_insert(1);
                sparse_matrix
                    .entry(to)
                    .or_insert_with(HashMap::new)
                    .entry(from)
                    .or_insert(1);
            }
        }
    }
    println!("Dataset loaded");
}

#[tauri::command]
async fn get_avg_dg() -> f64 {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_avg_dg(&sparse_matrix)
}

#[tauri::command]
async fn get_max_dg() -> usize {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_max_dg(&sparse_matrix)
}

#[tauri::command]
async fn get_cl_ef() -> f64 {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_cl_ef(&sparse_matrix)
}

#[tauri::command]
async fn get_avg_cm_nb() -> f64 {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_avg_cm_nb(&sparse_matrix)
}

#[tauri::command]
async fn get_max_cm_ng() -> usize {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_max_cm_ng(&sparse_matrix)
}

#[tauri::command]
async fn get_dg_dis() -> Vec<(usize, usize)> {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_dg_dis(&sparse_matrix)
}

#[tauri::command]
async fn get_cl_ef_dis() -> Vec<(usize, f64)> {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_cl_ef_dis(&sparse_matrix)
}

#[tauri::command]
async fn get_node_count() -> usize {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_node_count(&sparse_matrix)
}

#[tauri::command]
async fn get_edge_count() -> usize {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_edge_count(&sparse_matrix)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_dataset,
            get_avg_dg,
            get_max_dg,
            get_cl_ef,
            get_avg_cm_nb,
            get_max_cm_ng,
            get_dg_dis,
            get_cl_ef_dis,
            get_node_count,
            get_edge_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
