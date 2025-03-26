use std::io;

struct Solution {}

impl Solution {
    fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut grid = vec![Vec::new(); n];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            grid[x].push(y);
            grid[y].push(x);
        }

        // Find the node with the smallest degree
        let mut start = 0;
        for i in 0..n {
            if grid[i].len() < grid[start].len() {
                start = i;
            }
        }

        let mut row = vec![start as i32];
        let mut visited = vec![false; n];
        visited[start] = true;
        let initial_degree = grid[start].len();

        let mut current = start;
        loop {
            let mut next_node = -1;
            for neighbor in &grid[current] {
                let neighbor = *neighbor as usize;
                if !visited[neighbor] {
                    if next_node == -1 || grid[neighbor].len() < grid[next_node as usize].len() {
                        next_node = neighbor as i32;
                    }
                }
            }
            if next_node == -1 || grid[next_node as usize].len() <= initial_degree {
                break;
            }
            current = next_node as usize;
            row.push(current as i32);
            visited[current] = true;
        }

        let mut ans = Vec::new();
        ans.push(row);
        while ans.len() < n / ans[0].len() {
            let mut new_row = Vec::new();
            for node in &ans[ans.len() - 1] {
                let node = *node as usize;
                for neighbor in &grid[node] {
                    let neighbor = *neighbor as usize;
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        new_row.push(neighbor as i32);
                        break;
                    }
                }
            }
            ans.push(new_row);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split_whitespace();
    let n = lines.next().unwrap().parse::<i32>().unwrap();
    let edges_size = lines.next().unwrap().parse::<i32>().unwrap();
    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let x = lines.next().unwrap().parse::<i32>().unwrap();
        let y = lines.next().unwrap().parse::<i32>().unwrap();
        edges.push(vec![x, y]);
    }

    let sol = Solution {};
    let res = sol.construct_grid_layout(n, edges);
    for row in res {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}