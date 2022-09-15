mod two_edge_connected;
use two_edge_connected::*;

#[allow(dead_code)]
fn graph1() -> (usize, usize, [(usize, usize); 15]) {
    (
        8,
        10,
        [
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
        ],
    )
}

#[allow(dead_code)]
fn graph2() -> (usize, usize, [(usize, usize); 11]) {
    (
        1,
        8,
        [
            (1, 2),
            (1, 6),
            (1, 7),
            (2, 4),
            (3, 4),
            (3, 6),
            (5, 6),
            (5, 8),
            (6, 7),
            (6, 8),
            (7, 8),
        ],
    )
}

#[allow(dead_code)]
fn graph3() -> (usize, usize, [(usize, usize); 12]) {
    (
        1,
        9,
        [
            (1, 2),
            (1, 6),
            (1, 7),
            (2, 4),
            (3, 4),
            (3, 6),
            (5, 6),
            (5, 8),
            (6, 7),
            (6, 8),
            (7, 8),
            (8, 9),
        ],
    )
}

fn main() {
    // let (k, vertices, edges) = graph1();
    // let (k, vertices, edges) = graph2();
    let (k, vertices, edges) = graph3();
    enumerate_k_sized_two_ege_connected_induced_subgraphs(k, vertices, &edges);
}
