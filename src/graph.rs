//! Explicit graph algorithms replacing blind link traversal.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub nodes: Vec<usize>,
    pub total_cost: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tour {
    pub nodes: Vec<usize>,
    pub total_cost: f64,
    pub proven_optimal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
    InvalidNode,
    InvalidCost,
    InvalidMatrix,
    NoPath,
    NoTour,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    destination: usize,
    cost: f64,
}

#[derive(Debug, Clone)]
pub struct WeightedGraph {
    adjacency: Vec<Vec<Edge>>,
}

impl WeightedGraph {
    pub fn new(node_count: usize) -> Self {
        Self {
            adjacency: vec![Vec::new(); node_count],
        }
    }

    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn add_directed_edge(
        &mut self,
        source: usize,
        destination: usize,
        cost: f64,
    ) -> Result<(), GraphError> {
        if source >= self.node_count() || destination >= self.node_count() {
            return Err(GraphError::InvalidNode);
        }
        if !cost.is_finite() || cost < 0.0 {
            return Err(GraphError::InvalidCost);
        }
        self.adjacency[source].push(Edge { destination, cost });
        Ok(())
    }

    pub fn add_undirected_edge(
        &mut self,
        first: usize,
        second: usize,
        cost: f64,
    ) -> Result<(), GraphError> {
        self.add_directed_edge(first, second, cost)?;
        self.add_directed_edge(second, first, cost)
    }

    /// Dijkstra's algorithm for non-negative link costs.
    pub fn shortest_path(&self, start: usize, goal: usize) -> Result<Path, GraphError> {
        if start >= self.node_count() || goal >= self.node_count() {
            return Err(GraphError::InvalidNode);
        }
        if start == goal {
            return Ok(Path {
                nodes: vec![start],
                total_cost: 0.0,
            });
        }

        let mut distances = vec![f64::INFINITY; self.node_count()];
        let mut previous = vec![None; self.node_count()];
        let mut queue = BinaryHeap::new();
        distances[start] = 0.0;
        queue.push(State {
            cost: 0.0,
            node: start,
        });

        while let Some(State { cost, node }) = queue.pop() {
            if node == goal {
                break;
            }
            if cost > distances[node] {
                continue;
            }
            for edge in &self.adjacency[node] {
                let candidate = cost + edge.cost;
                if candidate < distances[edge.destination] {
                    distances[edge.destination] = candidate;
                    previous[edge.destination] = Some(node);
                    queue.push(State {
                        cost: candidate,
                        node: edge.destination,
                    });
                }
            }
        }

        if !distances[goal].is_finite() {
            return Err(GraphError::NoPath);
        }
        let mut nodes = vec![goal];
        let mut cursor = goal;
        while cursor != start {
            cursor = previous[cursor].ok_or(GraphError::NoPath)?;
            nodes.push(cursor);
        }
        nodes.reverse();
        Ok(Path {
            nodes,
            total_cost: distances[goal],
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    cost: f64,
    node: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.cost.total_cmp(&other.cost) == Ordering::Equal
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| other.node.cmp(&self.node))
    }
}

fn validate_matrix(matrix: &[Vec<f64>], start: usize) -> Result<(), GraphError> {
    let count = matrix.len();
    if count == 0 || start >= count || matrix.iter().any(|row| row.len() != count) {
        return Err(GraphError::InvalidMatrix);
    }
    for (row_index, row) in matrix.iter().enumerate() {
        for (column_index, cost) in row.iter().enumerate() {
            if row_index == column_index {
                if !cost.is_finite() || cost.abs() > f64::EPSILON {
                    return Err(GraphError::InvalidMatrix);
                }
            } else if !cost.is_finite() || *cost < 0.0 {
                return Err(GraphError::InvalidCost);
            }
        }
    }
    Ok(())
}

fn tour_cost(matrix: &[Vec<f64>], route: &[usize]) -> f64 {
    route.windows(2).map(|pair| matrix[pair[0]][pair[1]]).sum()
}

/// Solves the travelling-salesperson tour exactly with Held-Karp up to
/// `exact_limit` nodes. Larger instances use nearest-neighbour plus 2-opt and
/// are explicitly marked as not proven optimal.
pub fn travelling_salesperson(
    matrix: &[Vec<f64>],
    start: usize,
    exact_limit: usize,
) -> Result<Tour, GraphError> {
    validate_matrix(matrix, start)?;
    if matrix.len() <= exact_limit.min(18) {
        held_karp(matrix, start)
    } else {
        two_opt_tour(matrix, start)
    }
}

fn held_karp(matrix: &[Vec<f64>], start: usize) -> Result<Tour, GraphError> {
    let count = matrix.len();
    if count == 1 {
        return Ok(Tour {
            nodes: vec![start, start],
            total_cost: 0.0,
            proven_optimal: true,
        });
    }
    let state_count = 1usize << count;
    let mut costs = vec![f64::INFINITY; state_count * count];
    let mut parents = vec![usize::MAX; state_count * count];
    let start_mask = 1usize << start;
    costs[start_mask * count + start] = 0.0;

    for mask in 0..state_count {
        if mask & start_mask == 0 {
            continue;
        }
        for last in 0..count {
            let current = costs[mask * count + last];
            if !current.is_finite() || mask & (1usize << last) == 0 {
                continue;
            }
            for (next, _) in matrix.iter().enumerate() {
                let bit = 1usize << next;
                if mask & bit != 0 {
                    continue;
                }
                let next_mask = mask | bit;
                let candidate = current + matrix[last][next];
                let index = next_mask * count + next;
                if candidate < costs[index] {
                    costs[index] = candidate;
                    parents[index] = last;
                }
            }
        }
    }

    let all = state_count - 1;
    let mut best_end = usize::MAX;
    let mut best_cost = f64::INFINITY;
    for end in 0..count {
        if end == start {
            continue;
        }
        let candidate = costs[all * count + end] + matrix[end][start];
        if candidate < best_cost {
            best_cost = candidate;
            best_end = end;
        }
    }
    if best_end == usize::MAX {
        return Err(GraphError::NoTour);
    }

    let mut reverse = Vec::with_capacity(count);
    let mut mask = all;
    let mut cursor = best_end;
    while cursor != start {
        reverse.push(cursor);
        let parent = parents[mask * count + cursor];
        if parent == usize::MAX {
            return Err(GraphError::NoTour);
        }
        mask ^= 1usize << cursor;
        cursor = parent;
    }
    reverse.push(start);
    reverse.reverse();
    reverse.push(start);
    Ok(Tour {
        nodes: reverse,
        total_cost: best_cost,
        proven_optimal: true,
    })
}

fn two_opt_tour(matrix: &[Vec<f64>], start: usize) -> Result<Tour, GraphError> {
    let count = matrix.len();
    let mut unvisited: Vec<usize> = (0..count).filter(|node| *node != start).collect();
    let mut route = vec![start];
    let mut current = start;
    while !unvisited.is_empty() {
        let (position, next) = unvisited
            .iter()
            .enumerate()
            .min_by(|(_, left), (_, right)| {
                matrix[current][**left].total_cmp(&matrix[current][**right])
            })
            .map(|(position, node)| (position, *node))
            .ok_or(GraphError::NoTour)?;
        route.push(next);
        unvisited.swap_remove(position);
        current = next;
    }
    route.push(start);

    let mut best_cost = tour_cost(matrix, &route);
    let mut improved = true;
    while improved {
        improved = false;
        for first in 1..route.len() - 2 {
            for last in first + 1..route.len() - 1 {
                let mut candidate = route.clone();
                candidate[first..=last].reverse();
                let candidate_cost = tour_cost(matrix, &candidate);
                if candidate_cost + 1e-12 < best_cost {
                    route = candidate;
                    best_cost = candidate_cost;
                    improved = true;
                }
            }
        }
    }
    Ok(Tour {
        nodes: route,
        total_cost: best_cost,
        proven_optimal: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_uses_global_cost_not_first_link_walk() {
        let mut graph = WeightedGraph::new(4);
        graph.add_undirected_edge(0, 1, 4.0).unwrap();
        graph.add_undirected_edge(0, 2, 2.0).unwrap();
        graph.add_undirected_edge(2, 1, 1.0).unwrap();
        graph.add_undirected_edge(1, 3, 5.0).unwrap();
        graph.add_undirected_edge(2, 3, 8.0).unwrap();
        let path = graph.shortest_path(0, 3).unwrap();
        assert_eq!(path.nodes, vec![0, 2, 1, 3]);
        assert_eq!(path.total_cost, 8.0);
    }

    #[test]
    fn held_karp_proves_square_tour() {
        let matrix = vec![
            vec![0.0, 1.0, 2.0_f64.sqrt(), 1.0],
            vec![1.0, 0.0, 1.0, 2.0_f64.sqrt()],
            vec![2.0_f64.sqrt(), 1.0, 0.0, 1.0],
            vec![1.0, 2.0_f64.sqrt(), 1.0, 0.0],
        ];
        let tour = travelling_salesperson(&matrix, 0, 18).unwrap();
        assert!(tour.proven_optimal);
        assert!((tour.total_cost - 4.0).abs() < 1e-12);
        assert_eq!(tour.nodes.first(), Some(&0));
        assert_eq!(tour.nodes.last(), Some(&0));
    }

    #[test]
    fn large_tour_does_not_claim_proof() {
        let count: usize = 5;
        let matrix: Vec<Vec<f64>> = (0..count)
            .map(|row| {
                (0..count)
                    .map(|column| {
                        if row == column {
                            0.0
                        } else {
                            row.abs_diff(column) as f64
                        }
                    })
                    .collect()
            })
            .collect();
        let tour = travelling_salesperson(&matrix, 0, 3).unwrap();
        assert!(!tour.proven_optimal);
        assert_eq!(tour.nodes.first(), Some(&0));
        assert_eq!(tour.nodes.last(), Some(&0));
    }
}
