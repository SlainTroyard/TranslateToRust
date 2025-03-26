use std::io;

pub struct Solution;

impl Solution {
    fn build_graph(&self, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let size = edges.len() + 1;
        let mut graph = vec![Vec::new(); size];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }

    fn dfs(&self, g: &Vec<Vec<usize>>, root: usize, par: Option<usize>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut total = 1;
        for &neighbor in &g[root] {
            if Some(neighbor) != par {
                total += self.dfs(g, neighbor, Some(root), k - 1);
            }
        }
        total
    }

    pub fn max_target_nodes(&self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = self.build_graph(edges1);
        let g2 = self.build_graph(edges2);
        let m = edges2.len() + 1;
        let mut max_count = 0;
        for i in 0..m {
            let current = self.dfs(&g2, i, None, k - 1);
            if current > max_count {
                max_count = current;
            }
        }
        let n = edges1.len() + 1;
        let mut ans = Vec::with_capacity(n);
        for i in 0..n {
            let val = max_count + self.dfs(&g1, i, None, k);
            ans.push(val);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|l| l.expect("Failed to read line"));

    let n1: usize = lines.next().expect("n1 missing").parse().expect("Invalid n1");
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().expect("Edge1 missing");
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid edge1"))
            .collect();
        edges1.push(vec![parts[0], parts[1]]);
    }

    let n2: usize = lines.next().expect("n2 missing").parse().expect("Invalid n2");
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().expect("Edge2 missing");
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid edge2"))
            .collect();
        edges2.push(vec![parts[0], parts[1]]);
    }

    let k: i32 = lines.next().expect("k missing")
        .parse()
        .expect("Invalid k");

    let solution = Solution;
    let ans = solution.max_target_nodes(edges1, edges2, k);

    for val in ans {
        print!("{} ", val);
    }
    println!();
}