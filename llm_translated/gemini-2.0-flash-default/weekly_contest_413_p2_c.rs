use std::io;
use std::io::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    let mut max_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for i in 0..queries_size {
        let distance = (queries[i][0].abs() + queries[i][1].abs()) as i32;

        if max_heap.len() < k as usize {
            max_heap.push(Reverse(distance));
        } else if distance < max_heap.peek().unwrap().0 {
            max_heap.pop();
            max_heap.push(Reverse(distance));
        }

        if max_heap.len() == k as usize {
            result[i] = max_heap.peek().unwrap().0;
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    let result = results_array(&queries, k);

    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}