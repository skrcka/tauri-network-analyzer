use rayon::{
    iter::IntoParallelRefIterator,
    prelude::{ParallelBridge, ParallelIterator},
};
use rayon_hash::{HashMap, HashSet};

pub fn louvain_community_detection(
    graph: &HashMap<usize, HashMap<usize, usize>>,
) -> HashMap<usize, usize> {
    let mut communities = initialize_communities(graph);
    let m: usize = graph
        .values()
        .map(|neighbors| neighbors.values().sum::<usize>())
        .sum::<usize>()
        / 2;
    let mut modularity = calculate_modularity(graph, &communities, m);

    loop {
        let mut improvement = false;

        for node in graph.keys() {
            let best_community = find_best_community(graph, *node, &mut communities, m);
            if communities[node] != best_community {
                communities.insert(*node, best_community);
                improvement = true;
            }
        }

        if !improvement {
            break;
        }

        let new_modularity = calculate_modularity(graph, &communities, m);
        if new_modularity <= modularity {
            break;
        }
        modularity = new_modularity;
    }

    communities
}

fn initialize_communities(graph: &HashMap<usize, HashMap<usize, usize>>) -> HashMap<usize, usize> {
    graph.keys().map(|&node| (node, node)).collect()
}

fn calculate_modularity(
    graph: &HashMap<usize, HashMap<usize, usize>>,
    communities: &HashMap<usize, usize>,
    m: usize,
) -> f64 {
    graph
        .par_iter()
        .map(|(&node, neighbors)| {
            let community = communities[&node];
            let k_i = neighbors.values().sum::<usize>();

            neighbors
                .iter()
                .map(|(&neighbor, &weight)| {
                    let delta = if communities[&neighbor] == community {
                        1
                    } else {
                        0
                    };
                    (weight as f64
                        - (k_i * graph[&neighbor].values().sum::<usize>()) as f64 / (2 * m) as f64)
                        * delta as f64
                })
                .sum::<f64>()
        })
        .sum::<f64>()
        / (2 * m) as f64
}
fn find_best_community(
    graph: &HashMap<usize, HashMap<usize, usize>>,
    node: usize,
    communities: &HashMap<usize, usize>,
    m: usize,
) -> usize {
    let current_community = communities[&node];
    let mut best_community = current_community;
    let mut best_gain = 0.0;

    let possible_communities: Vec<usize> = graph.get(&node).map_or(Vec::new(), |neighbors| {
        neighbors
            .keys()
            .map(|neighbor| communities[neighbor])
            .collect()
    });

    for &community in possible_communities.iter() {
        let mut new_communities = communities.clone();
        new_communities.insert(node, community);
        let new_modularity = calculate_modularity(graph, &new_communities, m);
        let gain = new_modularity - calculate_modularity(graph, &new_communities, m);
        if gain > best_gain {
            best_gain = gain;
            best_community = community;
        }
    }

    best_community
}
