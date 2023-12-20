// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod influence;
mod path;

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon_hash::HashMap;
use std::collections::HashMap as HashMapSTD;
use std::collections::HashSet as HashSetSTD;
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
async fn get_avg_cl_coef() -> f64 {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_avg_cl_coef(&sparse_matrix)
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
async fn get_all_cl_coef() -> Vec<f64> {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_all_cl_coef(&sparse_matrix)
}

#[tauri::command]
async fn get_cl_coef_dis(bins: u32) -> Vec<(usize, usize)> {
    let sparse_matrix = STATE.lock().unwrap();
    functions::get_cl_coef_dis(&sparse_matrix, bins)
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

#[tauri::command]
async fn djikstra(start: usize, end: usize) -> Option<usize> {
    println!("Djikstra");
    let sparse_matrix = STATE.lock().unwrap();
    let path = path::dijkstra(&sparse_matrix, start, end);
    match path {
        Some(path) => {
            println!("Path: {:?}", path);
            Some(path.len())
        }
        None => None,
    }
}

#[tauri::command]
async fn djikstra_path(
    start: usize,
    end: usize,
) -> Option<(HashMapSTD<usize, HashMapSTD<usize, usize>>, Vec<usize>)> {
    println!("Djikstra path");
    let sparse_matrix = STATE.lock().unwrap();
    let path = path::dijkstra(&sparse_matrix, start, end);
    match path {
        Some(path) => {
            let mut nodes_to_send: HashMapSTD<usize, HashMapSTD<usize, usize>> = HashMapSTD::new();
            for &node in &path {
                if let Some(neighbors) = sparse_matrix.get(&node) {
                    for &neighbor in neighbors.keys() {
                        nodes_to_send
                            .entry(node)
                            .or_insert_with(HashMapSTD::new)
                            .entry(neighbor)
                            .or_insert(1);
                        nodes_to_send
                            .entry(neighbor)
                            .or_insert_with(HashMapSTD::new)
                            .entry(node)
                            .or_insert(1);
                    }
                }
            }
            Some((nodes_to_send, path))
        }
        None => None,
    }
}

#[tauri::command]
async fn simulate_influnce_spread(
    initial_nodes: Option<Vec<usize>>,
    steps: Option<u32>,
    probability: Option<f64>,
) -> (
    HashMapSTD<usize, HashMapSTD<usize, usize>>,
    HashSetSTD<usize>,
) {
    let sparse_matrix = STATE.lock().unwrap();
    let initial_nodes = match initial_nodes {
        Some(v) => v,
        _ => {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            let keys: Vec<_> = sparse_matrix.keys().collect();
            if let Some(&random_key) = keys.choose(&mut rng) {
                v.push(*random_key)
            }
            v
        }
    };
    let steps = steps.unwrap_or(500);
    let probability = probability.unwrap_or(0.5);
    let simulations =
        influence::simulate_influnce_spread(&sparse_matrix, initial_nodes, steps, probability);
    let influnced_nodes: HashSetSTD<usize> = simulations.into_iter().flat_map(|hs| hs).collect();
    let mut nodes_to_send: HashMapSTD<usize, HashMapSTD<usize, usize>> = HashMapSTD::new();
    for &node in &influnced_nodes {
        if let Some(neighbors) = sparse_matrix.get(&node) {
            for &neighbor in neighbors.keys() {
                nodes_to_send
                    .entry(node)
                    .or_insert_with(HashMapSTD::new)
                    .entry(neighbor)
                    .or_insert(1);
                nodes_to_send
                    .entry(neighbor)
                    .or_insert_with(HashMapSTD::new)
                    .entry(node)
                    .or_insert(1);
            }
        }
    }
    (nodes_to_send, influnced_nodes)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_dataset,
            get_avg_dg,
            get_max_dg,
            get_cl_ef,
            get_avg_cl_coef,
            get_avg_cm_nb,
            get_max_cm_ng,
            get_dg_dis,
            get_cl_ef_dis,
            get_all_cl_coef,
            get_cl_coef_dis,
            get_node_count,
            get_edge_count,
            djikstra,
            djikstra_path,
            simulate_influnce_spread,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
