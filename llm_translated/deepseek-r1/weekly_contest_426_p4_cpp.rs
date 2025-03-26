use std::collections::VecDeque;

struct Solution {
    tree1: Vec<Vec<usize>>,
    tree2: Vec<Vec<usize>>,
    color1: Vec<i32>,
    color2: Vec<i32>,
    node_color1: Vec<i32>,
    node_color2: Vec<i32>,
}

impl Solution {
    fn new() -> Self {
        Self {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: Vec::new(),
            color2: Vec::new(),
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    // Build adjacency list from edges
    fn build(&mut self, edges: &[Vec<i32>], mp: &mut Vec<Vec<usize>>) {
        let node_count = edges.len() + 1;
        mp.clear();
        mp.resize(node_count, Vec::new());
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            mp[u].push(v);
            mp[v].push(u);
        }
    }

    // BFS to color nodes and count colors
    fn bfs(&self, mp: &[Vec<usize>], color: &mut [i32], node_color: &mut [i32]) {
        let n = mp.len();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];
        queue.push_back((0, 0));

        while let Some((i, c)) = queue.pop_front() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            node_color[i] = c;
            color[c as usize] += 1;

            for &adj in &mp[i] {
                if !visited[adj] {
                    queue.push_back((adj, (c + 1) % 2));
                }
            }
        }
    }

    // Main algorithm to compute target nodes
    fn max_target_nodes(&mut self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        // Initialize data structures
        self.node_color1.resize(n, 0);
        self.node_color2.resize(m, 0);
        self.color1.resize(2, 0);
        self.color2.resize(2, 0);

        // Build adjacency lists for both trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);

        // Perform BFS coloring for both trees
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        let mx = std::cmp::max(self.color2[0], self.color2[1]);
        self.node_color1
            .iter()
            .map(|&c| self.color1[c as usize] + mx)
            .collect()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Read first tree's edges
    let n1 = tokens.next().expect("Missing node count for tree1");
    let mut edges1 = Vec::with_capacity((n1 - 1) as usize);
    for _ in 0..(n1 - 1) as usize {
        let u = tokens.next().expect("Missing edge data");
        let v = tokens.next().expect("Missing edge data");
        edges1.push(vec![u, v]);
    }

    // Read second tree's edges
    let n2 = tokens.next().expect("Missing node count for tree2");
    let mut edges2 = Vec::with_capacity((n2 - 1) as usize);
    for _ in 0..(n2 - 1) as usize {
        let u = tokens.next().expect("Missing edge data");
        let v = tokens.next().expect("Missing edge data");
        edges2.push(vec![u, v]);
    }

    // Compute and print result
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);
    for val in result {
        print!("{} ", val);
    }
    println!();
}