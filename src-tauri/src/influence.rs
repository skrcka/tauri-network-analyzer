use rand::Rng;
use rayon_hash::{HashMap, HashSet};

pub fn simulate_influnce_spread(
    sparse_matrix: &HashMap<usize, HashMap<usize, usize>>,
    initial_nodes: Vec<usize>,
    steps: u32,
    probability: f64,
) -> Vec<HashSet<usize>> {
    let mut rng = rand::thread_rng();

    let mut influence_history: Vec<HashSet<usize>> = Vec::new();
    let mut influencing_nodes: HashSet<usize> = initial_nodes.into_iter().collect();
    influence_history.push(influencing_nodes.clone());
    let mut influenced_nodes: HashSet<usize> = HashSet::new();
    for _ in 0..steps {
        let mut new_influencers: HashSet<usize> = HashSet::new();
        for node in influencing_nodes.clone().iter() {
            for neighbours in sparse_matrix.get(node) {
                for (neigbour, _) in neighbours.iter() {
                    if influencing_nodes.contains(neigbour) || influenced_nodes.contains(neigbour) {
                        continue;
                    }
                    let rand = rng.gen::<f64>();
                    if rand > probability {
                        continue;
                    }
                    new_influencers.insert(*neigbour);
                }
            }
        }
        influenced_nodes.extend(influencing_nodes.iter());
        influence_history.push(influenced_nodes.clone());
        influencing_nodes = new_influencers;
    }
    println!("influence history: {:?}", influence_history);
    influence_history
}

pub fn get_best_starting_nodes(
    sparse_matrix: &HashMap<usize, HashMap<usize, usize>>,
    n: u32,
) -> Vec<usize> {
    let mut degrees: Vec<(usize, usize)> = sparse_matrix
        .iter()
        .map(|(&node, neighbors)| (node, neighbors.len()))
        .collect();

    degrees.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut selected_nodes = Vec::new();
    let mut selected_count = 0;

    for (node, _) in degrees {
        if selected_nodes
            .iter()
            .all(|&selected_node| !sparse_matrix[&selected_node].contains_key(&node))
        {
            selected_nodes.push(node);
            selected_count += 1;

            if selected_count == n {
                break;
            }
        }
    }
    selected_nodes
}
