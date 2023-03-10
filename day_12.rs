use std::{io::*, collections::VecDeque};

type Map = Vec<Vec<u8>>;

fn main() {
    let mut map: Map = stdin().lines()
        .map(|rs| rs.unwrap().into_bytes())
        .collect();

    let (si, sj) = map_find(&map, b'S')[0];
    let (ei, ej) = map_find(&map, b'E')[0];
    map[si][sj] = b'a';
    map[ei][ej] = b'z';

    println!("Answer to part one: {}", bfs(&map, (si, sj), (ei, ej)));

    let ans = map_find(&map, b'a')
        .into_iter()
        .map(|start| bfs(&map, start, (ei, ej)))
        .min()
        .unwrap();

    println!("Answer to part two: {ans}");
}

fn bfs(map: &Map, (si, sj): (usize, usize), (ei, ej): (usize, usize)) -> usize {
    let n = map.len();
    let m = map[0].len();
    assert!(map.iter().all(|row| row.len() == m));

    let mut dist = vec![vec![usize::MAX; m]; n];
    dist[si][sj] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((si, sj));

    while let Some((ci, cj)) = queue.pop_front() {
        assert_ne!(dist[ci][cj], usize::MAX);
        let max_h = map[ci][cj] + 1;

        let mut go = |i: usize, j: usize| {
            if map[i][j] <= max_h && dist[i][j] == usize::MAX {
                dist[i][j] = dist[ci][cj] + 1;
                queue.push_back((i, j));
            }
        };

        if ci > 0 { go(ci - 1, cj); }
        if cj > 0 { go(ci, cj - 1); }
        if ci + 1 < n { go(ci + 1, cj); }
        if cj + 1 < m { go(ci, cj + 1); }
    }

    dist[ei][ej]
}

fn map_find(map: &Map, needle: u8) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == needle {
                ret.push((i, j));
            }
        }
    }
    ret
}
