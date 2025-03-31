// Problem: Weekly Contest 413 Problem 2
// Weekly Contest 413 Problem 2 in Rust
// Implements a max-heap to track top k Manhattan distances

use std::io;
use std::cmp;

/// Swaps two elements in the heap
fn swap(heap: &mut Vec<i32>, a: usize, b: usize) {
    let temp = heap[a];
    heap[a] = heap[b];
    heap[b] = temp;
}

/// Adjusts the heap by moving down from the given index
fn heapify_down(heap: &mut Vec<i32>, size: usize, mut idx: usize) {
    while idx < size {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;

        if left < size && heap[left] > heap[largest] {
            largest = left;
        }
        if right < size && heap[right] > heap[largest] {
            largest = right;
        }

        if largest == idx {
            break;
        }

        swap(heap, idx, largest);
        idx = largest;
    }
}

/// Adjusts the heap by moving up from the given index
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

/// Inserts a new value into the heap
fn heap_insert(heap: &mut Vec<i32>, size: &mut usize, val: i32) {
    heap.push(val);
    heapify_up(heap, *size);
    *size += 1;
}

/// Removes the top element from the heap
fn heap_remove_top(heap: &mut Vec<i32>, size: &mut usize) {
    if *size <= 1 {
        *size = 0;
        return;
    }
    
    *size -= 1;
    heap[0] = heap[*size];
    heapify_down(heap, *size, 0);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    
    let mut queries = Vec::with_capacity(queries_size);
    let mut distance_heap = Vec::new();
    let mut heap_size = 0;
    let mut results = Vec::with_capacity(queries_size);
    
    for _ in 0..queries_size {
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        
        let distance = (x.abs() + y.abs()) as i32;
        
        heap_insert(&mut distance_heap, &mut heap_size, distance);
        
        if heap_size > k {
            heap_remove_top(&mut distance_heap, &mut heap_size);
        }
        
        if heap_size == k {
            results.push(distance_heap[0]);
        } else {
            results.push(-1);
        }
    }
    
    println!(
        "{}",
        results
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}