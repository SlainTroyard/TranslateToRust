use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<usize> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        let tree1 = Self::build_tree(&edges1);
        let tree2 = Self::build_tree(&edges2);

        let mut color1 = vec![0; 2];
        let mut color2 = vec![0; 2];
        let mut node_color1 = vec![0; n];
        let mut node_color2 = vec![0; m];

        Self::bfs(&tree1, &mut color1, &mut node_color1);
        Self::bfs(&tree2, &mut color2, &mut node_color2);

        let mx = color2[0].max(color2[1]);

        (0..n)
            .map(|i| color1[node_color1[i]] + mx)
            .collect()
    }

    fn build_tree(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let size = edges.len() + 1;
        let mut tree = vec![vec![]; size];
        for e in edges {
            let a = e[0];
            let b = e[1];
            tree[a].push(b);
            tree[b].push(a);
        }
        tree
    }

    fn bfs(
        tree: &Vec<Vec<usize>>,
        color_count: &mut Vec<usize>,
        node_colors: &mut Vec<usize>,
    ) {
        let n = tree.len();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        queue.push_back((0, 0));
        while let Some((node, color)) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            node_colors[node] = color;
            color_count[color] += 1;
            for &neighbor in &tree[node] {
                if !visited[neighbor] {
                    let next_color = (color + 1) % 2;
                    queue.push_back((neighbor, next_color));
                }
            }
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut lines = lines.peekable();

    let n1: usize = lines.next().unwrap().parse().unwrap();
    let edges1: Vec<Vec<usize>> = (0..n1 - 1)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut parts = line.split_whitespace();
            let a: usize = parts.next().unwrap().parse().unwrap();
            let b: usize = parts.next().unwrap().parse().unwrap();
            vec![a, b]
        })
        .collect();

    let n2: usize = lines.next().unwrap().parse().unwrap();
    let edges2: Vec<Vec<usize>> = (0..n2 - 1)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut parts = line.split_whitespace();
            let a: usize = parts.next().unwrap().parse().unwrap();
            let b: usize = parts.next().unwrap().parse().unwrap();
            vec![a, b]
        })
        .collect();

    let solution = Solution {};
    let result = solution.max_target_nodes(edges1, edges2);

    for val in result {
        print!("{} ", val);
    }
    println!();
}