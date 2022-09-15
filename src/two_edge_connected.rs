pub fn enumerate_k_sized_two_ege_connected_induced_subgraphs(
    k: usize,
    vertices: usize,
    edges: &[(usize, usize)],
) {
    println!(
        "This graphs is 2-edge-connected: {}",
        is_two_edge_connected(&convert_to_adjacent(vertices, edges))
    );
    let mut ans = Vec::new();
    for bit in 1..1 << vertices {
        let mut members = Vec::with_capacity(vertices);
        for t in 0..vertices {
            if bit >> t & 1 == 1 {
                members.push(t)
            }
        }

        if members.len() < std::cmp::max(2, k) {
            continue;
        }

        let compressed = members
            .iter()
            .enumerate()
            .map(|(i, e)| (e + 1, i + 1))
            .collect::<std::collections::HashMap<_, _>>();

        let u = members.len();
        let mut es = Vec::new();
        for &(u, v) in edges {
            if members.contains(&(u - 1)) && members.contains(&(v - 1)) {
                es.push((*compressed.get(&u).unwrap(), *compressed.get(&v).unwrap()))
            }
        }
        let ads = convert_to_adjacent(u, &es);
        if is_two_edge_connected(&ads) {
            print_members(&members);
            ans.push(members)
        }
    }
    println!("size = {}.", ans.len());
}

pub fn print_members(members: &[usize]) {
    println!("{:?}", members.iter().map(|e| e + 1).collect::<Vec<_>>())
}

pub fn convert_to_adjacent(vertices: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adjacent = vec![Vec::new(); vertices];
    for (u, v) in edges {
        adjacent[u - 1].push(v - 1);
        adjacent[v - 1].push(u - 1);
    }
    adjacent
}

pub fn is_two_edge_connected(adjacent: &[Vec<usize>]) -> bool {
    let mut ord = vec![None; adjacent.len()];
    let mut low = vec![std::usize::MAX; adjacent.len()];
    let mut rank = 0;
    dfs(std::usize::MAX, 0, &mut rank, &mut ord, &mut low, adjacent);

    if ord.iter().any(|e| e.is_none()) {
        return false;
    }
    let ord = ord.iter().map(|e| e.unwrap()).collect::<Vec<usize>>();
    let mut visited = vec![false; adjacent.len()];

    !have_bridge(0, &ord, &low, adjacent, &mut visited)
}

fn dfs(
    before: usize,
    start: usize,
    rank: &mut usize,
    ord: &mut [Option<usize>],
    low: &mut [usize],
    adjacent: &[Vec<usize>],
) {
    ord[start] = Some(*rank);
    low[start] = *rank;
    *rank += 1;
    for &neighbor in &adjacent[start] {
        if ord[neighbor].is_none() {
            dfs(start, neighbor, rank, ord, low, adjacent);
            low[start] = std::cmp::min(low[start], low[neighbor])
        } else {
            if neighbor == before {
                continue;
            }
            low[start] = std::cmp::min(low[start], ord[neighbor].unwrap())
        }
    }
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
            if ord[start] < low[neighbor] || have_bridge(neighbor, ord, low, adjacent, visited) {
                return true;
            }
        }
    }
    false
}
