// Problem: Weekly Contest 413 Problem 2
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

// Function to calculate the absolute distance
fn absolute_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for query in queries {
        let distance = absolute_distance(query[0], query[1]);
        heap.push(Reverse(distance));

        if heap.len() > k as usize {
            heap.pop();
        }

        if heap.len() == k as usize {
            result.push(heap.peek().unwrap().0.0);
        } else {
            result.push(-1);
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let queries_size: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..queries_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.trim().split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    let result = results_array(&queries, k);

    for &val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}