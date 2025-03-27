// Problem: Weekly Contest 413 Problem 2
use std::cmp::Ordering;
use std::io::{self, Write};

// Function to swap two integers in-place (similar to the C version)
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

// Function to heapify down (adjust the heap from the given index downwards)
fn heapify_down(heap: &mut Vec<i32>, size: usize, idx: usize) {
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
        swap(&mut heap[idx], &mut heap[largest]);
        heapify_down(heap, size, largest);
    }
}

// Function to heapify up (adjust the heap upwards from the given index)
fn heapify_up(heap: &mut Vec<i32>, idx: usize) {
    let mut idx = idx;
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

// Function to insert a new element into the heap
fn heap_insert(heap: &mut Vec<i32>, size: &mut usize, val: i32) {
    heap.push(val); // Insert value at the end
    heapify_up(heap, *size);
    *size += 1;
}

// Function to remove the top element from the heap
fn heap_remove_top(heap: &mut Vec<i32>, size: &mut usize) {
    if *size <= 1 {
        *size = 0;
        return;
    }
    heap[0] = heap[*size - 1]; // Move the last element to the top
    *size -= 1;
    heapify_down(heap, *size, 0);
}

// Main function to process queries and return results
fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut heap = Vec::with_capacity(k + 1);
    let mut heap_size = 0;

    for (i, query) in queries.iter().enumerate() {
        // Calculate the Manhattan distance (absolute sum of coordinates)
        let distance = query[0].abs() + query[1].abs();

        // Insert the new distance into the heap
        heap_insert(&mut heap, &mut heap_size, distance);

        // If the heap size exceeds k, remove the largest element
        if heap_size > k {
            heap_remove_top(&mut heap, &mut heap_size);
        }

        // If the heap has exactly k elements, the top element is the k-th largest
        if heap_size == k {
            result[i] = heap[0];
        }
    }

    result
}

fn main() {
    // Read inputs from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let query: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Process the queries
    let result = results_array(&queries, k);

    // Print the result in the expected format
    for &value in &result {
        print!("{} ", value);
    }
    println!();
}