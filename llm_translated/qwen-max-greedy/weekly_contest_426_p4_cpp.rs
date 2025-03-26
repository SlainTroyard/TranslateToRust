use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

struct Solution {
    tree1: HashMap<usize, Vec<usize>>,
    tree2: HashMap<usize, Vec<usize>>,
    color1: [usize; 2],
    color2: [usize; 2],
    node_color1: Vec<usize>,
    node_color2: Vec<usize>,
}

impl Solution {
    fn build(&mut self, edges: &Vec<Vec<usize>>, mp: &mut HashMap<usize, Vec<usize>>) {
        for e in edges.iter() {
            mp.entry(e[0]).or_insert_with(Vec::new).push(e[1]);
            mp.entry(e[1]).or_insert_with(Vec::new).push(e[0]);
        }
    }

    fn bfs(&mut self, mp: &HashMap<usize, Vec<usize>>, color: &mut [usize; 2], node_color: &mut Vec<usize>) {
        let n = mp.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        q.push_back((0, 0));
        while let Some((i, c)) = q.pop_front() {
            if vis[i] { continue; }
            vis[i] = true;
            node_color[i] = c;
            color[c] += 1;
            if let Some(adj_list) = mp.get(&i) {
                for &adj in adj_list {
                    if !vis[adj] {
                        q.push_back((adj, (c + 1) % 2));
                    }
                }
            }
        }
    }

    fn max_target_nodes(&mut self, edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<usize> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        self.node_color1 = vec![0; n];
        self.node_color2 = vec![0; m];
        self.color1 = [0, 0];
        self.color2 = [0, 0];

        // Build trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);

        // Color trees using BFS
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        let mx = self.color2.iter().max().unwrap_or(&0);
        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = self.color1[self.node_color1[i]] + *mx;
        }
        arr
    }
}

fn main() {
    let mut solution = Solution {
        tree1: HashMap::new(),
        tree2: HashMap::new(),
        color1: [0, 0],
        color2: [0, 0],
        node_color1: Vec::new(),
        node_color2: Vec::new(),
    };

    // Input for edges1
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let edges1: Vec<Vec<usize>> = (0..n1-1)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let edges2: Vec<Vec<usize>> = (0..n2-1)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    // Call the solution method
    let result = solution.max_target_nodes(edges1, edges2);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}