use std::collections::VecDeque;
use std::io;

struct Solution {}

impl Solution {
    fn build(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let n = edges.len() + 1;
        let mut tree = vec![Vec::new(); n];
        for e in edges {
            let u = e[0];
            let v = e[1];
            tree[u].push(v);
            tree[v].push(u);
        }
        tree
    }

    fn bfs(tree: &Vec<Vec<usize>>) -> (Vec<i32>, Vec<i32>) {
        let n = tree.len();
        let mut color = vec![0, 0];
        let mut node_color = vec![0; n];
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        while !queue.is_empty() {
            let (node, c) = queue.pop_front().unwrap();
            if visited[node] {
                continue;
            }
            visited[node] = true;
            node_color[node] = c as i32;
            color[c] += 1;
            for neighbor in &tree[node] {
                if !visited[*neighbor] {
                    queue.push_back((*neighbor, (c + 1) % 2));
                }
            }
        }
        (color, node_color)
    }

    fn max_target_nodes(edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let tree1 = Solution::build(&edges1);
        let tree2 = Solution::build(&edges2);
        let (color1, node_color1) = Solution::bfs(&tree1);
        let (color2, _) = Solution::bfs(&tree2);
        let mx = color2[0].max(color2[1]);
        (0..n).map(|i| color1[node_color1[i] as usize] + mx).collect()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    // Read edges1
    let n1: usize = lines.next().unwrap()?.parse().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 - 1 {
        let line = lines.next().unwrap()?;
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        edges1.push(vec![parts[0], parts[1]]);
    }

    // Read edges2
    let n2: usize = lines.next().unwrap()?.parse().unwrap();
    let mut edges2 = Vec::new();
    for _ in 0..n2 - 1 {
        let line = lines.next().unwrap()?;
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        edges2.push(vec![parts[0], parts[1]]);
    }

    let solution = Solution {};
    let result = solution.max_target_nodes(edges1, edges2);

    for val in result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}