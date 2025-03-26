use std::io;
use std::io::Read;
use std::cmp::max;

struct Solution {}

impl Solution {
    fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        g
    }

    fn dfs(g: &Vec<Vec<usize>>, root: usize, par: i32, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut count = 1;
        for &node in &g[root] {
            if node as i32 != par {
                count += Self::dfs(g, node, root as i32, k - 1);
            }
        }
        count
    }

    fn max_target_nodes(edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = Self::build_graph(edges1);
        let g2 = Self::build_graph(edges2);
        let mut count = 0;
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..m {
            count = max(count, Self::dfs(&g2, i, -1, k - 1));
        }

        for i in 0..n {
            ans.push(count + Self::dfs(&g1, i, -1, k));
        }

        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut iter = s.split_whitespace();

    let n1: usize = iter.next().unwrap().parse().unwrap();
    let mut edges1: Vec<Vec<i32>> = vec![vec![0; 2]; n1];
    for i in 0..n1 {
        edges1[i][0] = iter.next().unwrap().parse().unwrap();
        edges1[i][1] = iter.next().unwrap().parse().unwrap();
    }

    let n2: usize = iter.next().unwrap().parse().unwrap();
    let mut edges2: Vec<Vec<i32>> = vec![vec![0; 2]; n2];
    for i in 0..n2 {
        edges2[i][0] = iter.next().unwrap().parse().unwrap();
        edges2[i][1] = iter.next().unwrap().parse().unwrap();
    }

    let k: i32 = iter.next().unwrap().parse().unwrap();

    let solution = Solution {};
    let result: Vec<i32> = solution.max_target_nodes(&edges1, &edges2, k);

    for &val in &result {
        print!("{} ", val);
    }
    println!();
}