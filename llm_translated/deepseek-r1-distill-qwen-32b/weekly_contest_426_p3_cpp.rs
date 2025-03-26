use std::io;

struct Solution {}

impl Solution {
    fn build_graph(&self, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            graph[u as usize].push(v);
            graph[v as usize].push(u);
        }
        graph
    }

    fn dfs(&self, g: &Vec<Vec<i32>>, root: i32, par: i32, k: i32, count: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        for &node in g[root as usize].iter() {
            if node != par {
                let result = self.dfs(g, node, root, k - 1, count);
                count += result;
            }
        }
        count
    }

    fn max_target_nodes(&self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = self.build_graph(edges1);
        let g2 = self.build_graph(edges2);
        let m = g2.len();
        let n = g1.len();
        let mut max_count = 0;
        
        for i in 0..m {
            let current = self.dfs(&g2, i as i32, -1, k - 1, 1);
            if current > max_count {
                max_count = current;
            }
        }
        
        let mut ans = Vec::with_capacity(n);
        for i in 0..n {
            let current = self.dfs(&g1, i as i32, -1, k, 1);
            ans.push(max_count + current);
        }
        
        ans
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_i32() -> i32 {
    read_line().parse().expect("Invalid integer input")
}

fn main() {
    let solution = Solution {};

    let n1 = read_i32();
    let mut edges1 = Vec::with_capacity(n1 as usize);
    for _ in 0..n1 {
        let line = read_line();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let u = parts[0].parse().unwrap();
        let v = parts[1].parse().unwrap();
        edges1.push(vec![u, v]);
    }

    let n2 = read_i32();
    let mut edges2 = Vec::with_capacity(n2 as usize);
    for _ in 0..n2 {
        let line = read_line();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let u = parts[0].parse().unwrap();
        let v = parts[1].parse().unwrap();
        edges2.push(vec![u, v]);
    }

    let k = read_i32();

    let result = solution.max_target_nodes(edges1, edges2, k);

    for val in result {
        print!("{} ", val);
    }
    println!();
}