use std::collections::{VecDeque, HashSet};
use std::io::{self, BufRead};
use std::cmp::max;

#[derive(Debug)]
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(vertex: usize) -> Self {
        Node {
            vertex,
            next: None,
        }
    }
}

fn add_edge(adj_list: &mut Vec<Option<Box<Node>>>, src: usize, dest: usize) {
    let mut new_node = Box::new(Node::new(dest));
    new_node.next = adj_list[src].take();
    adj_list[src] = Some(new_node);
}

fn check(n: usize, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    let mut adj_list = vec![None; n];
    
    for edge in edges.iter() {
        if edge[2] <= limit {
            add_edge(&mut adj_list, edge[1] as usize, edge[0] as usize);
        }
    }
    
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    
    visited[0] = true;
    queue.push_back(0);
    
    while let Some(current_vertex) = queue.pop_front() {
        let mut temp = adj_list[current_vertex].as_ref();
        
        while let Some(node) = temp {
            let adj_vertex = node.vertex;
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.push_back(adj_vertex);
            }
            temp = node.next.as_ref();
        }
    }
    
    visited.iter().all(|&v| v)
}

fn find_max_weight(edges: &Vec<Vec<i32>>) -> i32 {
    edges.iter().map(|edge| edge[2]).max().unwrap_or(0)
}

fn min_max_weight(n: usize, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let max_weight = find_max_weight(&edges);
    
    if !check(n, &edges, max_weight) {
        return -1;
    }
    
    let mut left = 0;
    let mut right = max_weight;
    
    while left < right {
        let mid = left + (right - left) / 2;
        if check(n, &edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    left
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let threshold: i32 = first_line_iter.next().unwrap().parse().unwrap();
    
    let mut edges = Vec::new();
    
    for line in lines {
        let line = line.unwrap();
        let mut edge_iter = line.split_whitespace();
        let src: i32 = edge_iter.next().unwrap().parse().unwrap();
        let dest: i32 = edge_iter.next().unwrap().parse().unwrap();
        let weight: i32 = edge_iter.next().unwrap().parse().unwrap();
        edges.push(vec![src, dest, weight]);
    }
    
    let result = min_max_weight(n, edges, threshold);
    println!("{}", result);
}