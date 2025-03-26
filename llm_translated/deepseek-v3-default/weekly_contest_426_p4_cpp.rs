use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution {
    tree1: Vec<Vec<usize>>,
    tree2: Vec<Vec<usize>>,
    color1: Vec<usize>,
    color2: Vec<usize>,
    node_color1: Vec<usize>,
    node_color2: Vec<usize>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: vec![0, 0],
            color2: vec![0, 0],
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    fn build(&mut self, edges: &Vec<Vec<usize>>, mp: &mut Vec<Vec<usize>>) {
        let n = edges.len() + 1;
        mp.resize(n, Vec::new());
        for e in edges {
            mp[e[0]].push(e[1]);
            mp[e[1]].push(e[0]);
        }
    }

    fn bfs(&mut self, mp: &Vec<Vec<usize>>, color: &mut Vec<usize>, node_color: &mut Vec<usize>) {
        let n = mp.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        q.push_back((0, 0));
        while let Some((i, c)) = q.pop_front() {
            vis[i] = true;
            node_color[i] = c;
            color[c] += 1;
            for &adj in &mp[i] {
                if !vis[adj] {
                    q.push_back((adj, (c + 1) % 2));
                }
            }
        }
    }

    fn max_target_nodes(&mut self, edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<usize> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        self.node_color1.resize(n, 0);
        self.node_color2.resize(m, 0);
        self.color1 = vec![0, 0];
        self.color2 = vec![0, 0];

        // Build trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);

        // Color trees using BFS
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        let mx = self.color2[0].max(self.color2[1]);
        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = self.color1[self.node_color1[i]] + mx;
        }
        arr
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1 - 1);
    for _ in 0..n1 - 1 {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        edges1.push(vec![u, v]);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2 - 1);
    for _ in 0..n2 - 1 {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        edges2.push(vec![u, v]);
    }

    // Call the solution method
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}