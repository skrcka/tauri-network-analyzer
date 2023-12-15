use rayon::{
    iter::IntoParallelRefIterator,
    prelude::{ParallelBridge, ParallelIterator},
};
use rayon_hash::{HashMap, HashSet};

pub fn get_node_count(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    let start = std::time::Instant::now();
    let node_count = sparse_matrix.len();
    let end = std::time::Instant::now();
    println!(
        "Node count par: {} in {}",
        node_count,
        (end - start).as_millis()
    );
    node_count
}

pub fn get_edge_count(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    let start = std::time::Instant::now();
    let mut edge_count: usize = sparse_matrix.par_iter().map(|(_, v)| v.len()).sum();
    edge_count /= 2;
    let end = std::time::Instant::now();
    println!(
        "Edge count par: {} in {}",
        edge_count,
        (end - start).as_millis()
    );
    edge_count
}

pub fn get_avg_dg(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> f64 {
    let start = std::time::Instant::now();
    let len = sparse_matrix.len();
    let sum: usize = sparse_matrix.into_iter().map(|(_, v)| v.len()).sum();
    let avg_degree = sum as f64 / len as f64;
    let end = std::time::Instant::now();
    println!(
        "Average degree par: {} in {}",
        avg_degree,
        (end - start).as_millis()
    );
    avg_degree
}

pub fn get_max_dg(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    let start = std::time::Instant::now();
    let max_degree: usize = sparse_matrix
        .par_iter()
        .map(|(_, v)| v.len())
        .max()
        .unwrap();
    let end = std::time::Instant::now();
    println!(
        "Max degree par: {} in {}",
        max_degree,
        (end - start).as_millis()
    );
    max_degree
}

pub fn get_dg_dis(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> Vec<(usize, usize)> {
    let start = std::time::Instant::now();
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    let degree_distribution_arc =
        std::sync::Arc::new(std::sync::Mutex::new(&mut degree_distribution));
    sparse_matrix.par_iter().for_each(|(_, v)| {
        degree_distribution_arc
            .lock()
            .unwrap()
            .entry(v.len())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });
    let end = std::time::Instant::now();
    println!("Degree distribution par in {}", (end - start).as_millis());

    let mut degree_distribution_vec: Vec<(usize, usize)> = degree_distribution
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect();
    degree_distribution_vec.sort_by(|a, b| a.0.cmp(&b.0));
    degree_distribution_vec
}

pub fn get_cl_ef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> f64 {
    let sparse_matrix_arc = std::sync::Arc::new(sparse_matrix);
    let start = std::time::Instant::now();
    let sum: usize = sparse_matrix_arc
        .iter()
        .par_bridge()
        .map(|(_, neighbors)| {
            let matrix_ref = sparse_matrix_arc.as_ref();
            neighbors
                .keys()
                .filter_map(|&neighbor| matrix_ref.get(&neighbor))
                .flatten()
                .filter(|&(key, _)| neighbors.contains_key(key))
                .count()
        })
        .sum();

    let clustering_effect = sum as f64 / sparse_matrix_arc.len() as f64;

    let end = std::time::Instant::now();
    println!(
        "Clustering effect par: {} in {}",
        clustering_effect,
        (end - start).as_millis()
    );
    clustering_effect
}

fn get_cl_coef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>, node: usize) -> f64 {
    let neighbors = match sparse_matrix.get(&node) {
        Some(neigh) => neigh.keys().collect::<Vec<&usize>>(),
        None => return 0.0, // node doesn't exist
    };

    if neighbors.len() < 2 {
        return 0.0; // no way to form a triangle with less than 2 neighbors
    }

    let mut triangles = 0;
    for i in 0..neighbors.len() {
        for j in i + 1..neighbors.len() {
            if let Some(inner) = sparse_matrix.get(neighbors[i]) {
                if inner.contains_key(neighbors[j]) {
                    triangles += 1;
                }
            }
        }
    }

    let triples = neighbors.len() * (neighbors.len() - 1) / 2;
    triangles as f64 / triples as f64
}

pub fn get_avg_cl_coef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> f64 {
    let start = std::time::Instant::now();
    let sum: f64 = sparse_matrix
        .par_iter()
        .map(|(&node, _)| get_cl_coef(sparse_matrix, node))
        .sum();
    let avg_cl_coef = sum / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Average clustering coefficient par: {} in {}",
        avg_cl_coef,
        (end - start).as_millis()
    );
    avg_cl_coef
}

pub fn get_all_cl_coef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> Vec<f64> {
    let start = std::time::Instant::now();
    let cl_coefs: Vec<f64> = sparse_matrix
        .par_iter()
        .map(|(&node, _)| get_cl_coef(sparse_matrix, node))
        .collect();
    let end = std::time::Instant::now();
    println!(
        "All clustering coefficients par in {}",
        (end - start).as_millis()
    );
    cl_coefs
}

pub fn get_cl_coef_dis(
    sparse_matrix: &HashMap<usize, HashMap<usize, usize>>,
    bins: u32,
) -> Vec<(usize, usize)> {
    let start = std::time::Instant::now();
    let mut coeficients: Vec<f64> = sparse_matrix
        .par_iter()
        .map(|(&node, _)| get_cl_coef(sparse_matrix, node))
        .collect();
    coeficients.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let max = coeficients[coeficients.len() - 1];
    let min = coeficients[0];
    println!("Max: {}, Min: {}", max, min);
    let bin_size = (max - min) / bins as f64;
    let mut cl_coef_dis: HashMap<usize, usize> = HashMap::new();
    for coef in coeficients {
        let bin = ((coef - min) / bin_size) as usize;
        cl_coef_dis.entry(bin).and_modify(|e| *e += 1).or_insert(1);
    }
    let end = std::time::Instant::now();
    println!(
        "Clustering coefficient distribution par in {}",
        (end - start).as_millis()
    );
    cl_coef_dis.into_iter().collect()
}

pub fn get_cl_ef_dis(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> Vec<(usize, f64)> {
    let start = std::time::Instant::now();

    let coefficients: HashMap<usize, f64> = sparse_matrix
        .par_iter()
        .map(|(&node, _)| (node, get_cl_coef(sparse_matrix, node)))
        .collect();

    let mut degree_to_coefficients: HashMap<usize, Vec<f64>> = HashMap::new();

    for (node, coeff) in coefficients.iter() {
        let degree = sparse_matrix[node].len();
        degree_to_coefficients
            .entry(degree)
            .or_default()
            .push(*coeff);
    }

    let mut distribution_vec: Vec<(usize, f64)> = degree_to_coefficients
        .into_iter()
        .map(|(degree, coeffs)| {
            let avg_coeff = coeffs.iter().sum::<f64>() / coeffs.len() as f64;
            (degree, avg_coeff)
        })
        .collect();

    let end = std::time::Instant::now();
    println!(
        "Clustering effect distribution par in {}",
        (end - start).as_millis()
    );

    distribution_vec.sort_by(|a, b| a.0.cmp(&b.0));
    distribution_vec
}

#[allow(dead_code)]
pub fn get_cl_ds(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> HashMap<usize, usize> {
    let start = std::time::Instant::now();
    let mut clustering_distribution: HashMap<usize, usize> = HashMap::new();
    let sparse_matrix_copy = sparse_matrix.clone();
    let results: Vec<HashMap<usize, usize>> = sparse_matrix_copy
        .par_iter()
        .map(|(_, neighbors)| {
            let mut local_distribution: HashMap<usize, usize> = HashMap::new();
            let mut count = 0;
            for &neighbor in neighbors.keys() {
                if let Some(neighbor_neighbors) = sparse_matrix.get(&neighbor) {
                    for &neighbor_neighbor in neighbor_neighbors.keys() {
                        if neighbors.contains_key(&neighbor_neighbor) {
                            count += 1;
                        }
                    }
                }
            }
            local_distribution
                .entry(count)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            local_distribution
        })
        .collect();

    for local_dist in results.iter() {
        for (&k, &v) in local_dist.iter() {
            clustering_distribution
                .entry(k)
                .and_modify(|e| *e += v)
                .or_insert(v);
        }
    }
    let end = std::time::Instant::now();
    println!(
        "Clustering distribution par in {}",
        (end - start).as_millis()
    );
    clustering_distribution
}

pub fn get_avg_cm_nb(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> f64 {
    let start = std::time::Instant::now();
    let (total_common, total_pairs) = sparse_matrix
        .par_iter()
        .map(|(_, neighbors1)| {
            let mut common_neighbors_count = 0;
            let mut total_pairs_count = 0;

            for (_, neighbors2) in sparse_matrix.iter() {
                let common_neighbors: HashSet<_> = neighbors1
                    .keys()
                    .filter(|&&x| neighbors2.contains_key(&x))
                    .collect();
                common_neighbors_count += common_neighbors.len();
                total_pairs_count += 1;
            }

            (common_neighbors_count, total_pairs_count)
        })
        .reduce(
            || (0, 0),
            |acc, (common, count)| (acc.0 + common, acc.1 + count),
        );

    let mut avg_common_neighbors = 0.0;
    if total_pairs != 0 {
        avg_common_neighbors = total_common as f64 / total_pairs as f64;
    }
    let end = std::time::Instant::now();
    println!(
        "Average common neighbors par: {} in {}",
        avg_common_neighbors,
        (end - start).as_millis()
    );
    avg_common_neighbors
}

pub fn get_max_cm_ng(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    let start = std::time::Instant::now();
    let max_common = sparse_matrix
        .par_iter()
        .map(|(node1, neighbors1)| {
            sparse_matrix
                .iter()
                .map(|(node2, neighbors2)| {
                    if node1 > node2 {
                        return 0;
                    }
                    let common_neighbors: HashSet<_> = neighbors1
                        .keys()
                        .filter(|&&x| neighbors2.contains_key(&x))
                        .collect();
                    common_neighbors.len()
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0);

    let end = std::time::Instant::now();
    println!(
        "Maximum common neighbors par: {} in {}",
        max_common,
        (end - start).as_millis()
    );
    max_common
}
