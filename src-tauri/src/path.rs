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
) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

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
            return Some(cost);
        }

        if cost > *distances.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for (&neighbor, &weight) in neighbors.iter() {
                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };

                if next.cost < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                    heap.push(next);
                    distances.insert(neighbor, next.cost);
                }
            }
        }
    }

    None
}
