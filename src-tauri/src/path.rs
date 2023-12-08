use rayon_hash::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(
    graph: &HashMap<usize, HashMap<usize, usize>>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut predecessors = HashMap::new();

    for &node in graph.keys() {
        distances.insert(node, usize::MAX);
    }
    distances.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            let mut path = vec![end];
            let mut current = end;
            while let Some(&predecessor) = predecessors.get(&current) {
                path.push(predecessor);
                current = predecessor;
                if current == start {
                    break;
                }
            }
            path.reverse();
            return Some(path);
        }

        if cost > *distances.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for (&neighbor, &weight) in neighbors.iter() {
                let next_cost = cost + weight;
                if next_cost < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                    heap.push(State {
                        cost: next_cost,
                        position: neighbor,
                    });
                    distances.insert(neighbor, next_cost);
                    predecessors.insert(neighbor, position);
                }
            }
        }
    }

    None
}
