fn main() {
    let vertices = 10;
    let edges = [
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 6),
        (1, 7),
        (2, 3),
        (2, 7),
        (3, 7),
        (3, 8),
        (4, 5),
        (5, 8),
        (6, 7),
        (6, 9),
        (8, 10),
        (9, 10),
    ];

    let adjacent = adjacent(vertices, &edges);

    println!("{}", is_two_edge_connected(&adjacent));
}

fn adjacent(vertices: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adjacent = vec![Vec::new(); vertices];
    for (u, v) in edges {
        adjacent[u - 1].push(v - 1);
        adjacent[v - 1].push(u - 1);
    }
    adjacent
}

fn dfs(
    start: usize,
    rank: usize,
    ord: &mut [Option<usize>],
    low: &mut [usize],
    adjacent: &[Vec<usize>],
) {
    ord[start] = Some(rank);
    low[start] = rank;
    for &neighbor in &adjacent[start] {
        if ord[neighbor].is_none() {
            dfs(neighbor, rank + 1, ord, low, adjacent);
            low[start] = std::cmp::min(low[start], low[neighbor])
        } else {
            low[start] = std::cmp::min(low[start], ord[neighbor].unwrap())
        }
    }
}

fn is_two_edge_connected(adjacent: &[Vec<usize>]) -> bool {
    let mut ord = vec![None; adjacent.len()];
    let mut low = vec![std::usize::MAX; adjacent.len()];
    dfs(0, 0, &mut ord, &mut low, adjacent);

    if ord.iter().any(|e| e.is_none()) {
        return false;
    }
    let ord = ord.iter().map(|e| e.unwrap()).collect::<Vec<usize>>();
    let mut visited = vec![false; adjacent.len()];
    have_bridge(0, &ord, &low, adjacent, &mut visited)
}

fn have_bridge(
    start: usize,
    ord: &[usize],
    low: &[usize],
    adjacent: &[Vec<usize>],
    visited: &mut [bool],
) -> bool {
    visited[start] = true;
    for &neighbor in &adjacent[start] {
        if !visited[neighbor] {
            if ord[start] < low[neighbor] || !have_bridge(neighbor, ord, low, adjacent, visited) {
                return false;
            }
        }
    }
    true
}
