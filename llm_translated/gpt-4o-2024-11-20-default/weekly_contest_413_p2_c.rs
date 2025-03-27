// Problem: Weekly Contest 413 Problem 2
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

/// Swap two integers in a vector
fn swap(vec: &mut Vec<i32>, a: usize, b: usize) {
    vec.swap(a, b);
}

/// Adjust the heap downward to maintain max-heap property
fn heapify_down(heap: &mut Vec<i32>, idx: usize, size: usize) {
    let mut largest = idx;
    let left = 2 * idx + 1;
    let right = 2 * idx + 2;

    if left < size && heap[left] > heap[largest] {
        largest = left;
    }
    if right < size && heap[right] > heap[largest] {
        largest = right;
    }

    if largest != idx {
        swap(heap, idx, largest);
        heapify_down(heap, largest, size);
    }
}

/// Adjust the heap upward to maintain max-heap property
fn heapify_up(heap: &mut Vec<i32>, mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if heap[parent] < heap[idx] {
            swap(heap, parent, idx);
            idx = parent;
        } else {
            break;
        }
    }
}

/// Insert a value into the heap
fn heap_insert(heap: &mut Vec<i32>, size: &mut usize, val: i32) {
    if *size < heap.len() {
        heap[*size] = val;
    } else {
        heap.push(val);
    }
    *size += 1;
    heapify_up(heap, *size - 1);
}

/// Remove the top value from the heap
fn heap_remove_top(heap: &mut Vec<i32>, size: &mut usize) {
    if *size <= 1 {
        *size = 0;
        return;
    }
    heap[0] = heap[*size - 1];
    *size -= 1;
    heapify_down(heap, 0, *size);
}

/// Compute the results array based on the queries and k
fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut heap = Vec::with_capacity(k + 1);
    let mut heap_size = 0;

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();
        heap_insert(&mut heap, &mut heap_size, distance);

        if heap_size > k {
            heap_remove_top(&mut heap, &mut heap_size);
        }

        if heap_size == k {
            result[i] = heap[0];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read queriesSize and k
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(nums);
    }

    // Compute the results
    let result = results_array(&queries, k);

    // Print the results
    for (i, res) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res);
    }
    println!();
}