// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Swaps two integers in place.
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

/// Maintains the max-heap property by moving the element at `idx` down the heap.
fn heapify_down(heap: &mut [i32], size: usize, mut idx: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;

        if left < size && heap[left] > heap[largest] {
            largest = left;
        }
        if right < size && heap[right] > heap[largest] {
            largest = right;
        }

        if largest != idx {
            swap(&mut heap[idx], &mut heap[largest]);
            idx = largest;
        } else {
            break;
        }
    }
}

/// Maintains the max-heap property by moving the element at `idx` up the heap.
fn heapify_up(heap: &mut [i32], mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if heap[parent] < heap[idx] {
            swap(&mut heap[parent], &mut heap[idx]);
            idx = parent;
        } else {
            break;
        }
    }
}

/// Inserts a value into the heap and maintains the max-heap property.
fn heap_insert(heap: &mut Vec<i32>, val: i32) {
    heap.push(val);
    heapify_up(heap, heap.len() - 1);
}

/// Removes the top element of the heap and maintains the max-heap property.
fn heap_remove_top(heap: &mut Vec<i32>) {
    if heap.len() <= 1 {
        heap.clear();
        return;
    }
    heap[0] = heap.pop().unwrap();
    heapify_down(heap, heap.len(), 0);
}

/// Computes the results array based on the queries and the value of `k`.
fn results_array(queries: &[Vec<i32>], k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut heap = Vec::with_capacity(k + 1);
    let mut heap_size = 0;

    for (i, query) in queries.iter().enumerate() {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();

        // Insert the distance into the heap
        heap_insert(&mut heap, distance);
        heap_size += 1;

        // If the heap exceeds size `k`, remove the largest element
        if heap_size > k {
            heap_remove_top(&mut heap);
            heap_size -= 1;
        }

        // If the heap size equals `k`, store the top element as the result
        if heap_size == k {
            result[i] = heap[0];
        } else {
            result[i] = -1; // Heap has fewer than `k` elements
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of queries and the value of `k`
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read each query
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    // Compute the results array
    let result = results_array(&queries, k);

    // Print the results
    for value in result {
        print!("{} ", value);
    }
}