// Problem: Weekly Contest 413 Problem 2
use std::io;
use std::cmp::Ordering;

// Swap two elements in a vector
fn swap(arr: &mut Vec<i32>, i: usize, j: usize) {
    arr.swap(i, j);
}

// Heapify down to maintain max-heap property
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
        swap(heap, idx, largest);
        heapify_down(heap, size, largest);
    }
}

// Heapify up to maintain max-heap property
fn heapify_up(heap: &mut Vec<i32>, idx: usize) {
    let mut current = idx;

    while current > 0 {
        let parent = (current - 1) / 2;
        if heap[parent] < heap[current] {
            swap(heap, parent, current);
            current = parent;
        } else {
            break;
        }
    }
}

// Insert an element into the heap
fn heap_insert(heap: &mut Vec<i32>, size: &mut usize, val: i32) {
    heap[*size] = val;
    heapify_up(heap, *size);
    *size += 1;
}

// Remove the top element from the heap
fn heap_remove_top(heap: &mut Vec<i32>, size: &mut usize) {
    if *size <= 1 {
        *size = 0;
        return;
    }
    heap[0] = heap[*size - 1];
    *size -= 1;
    heapify_down(heap, *size, 0);
}

// Process queries and calculate results
fn results_array(queries: Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut heap = vec![0; k + 1];
    let mut heap_size = 0;

    for query in queries {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();

        // Insert current distance into the heap
        heap_insert(&mut heap, &mut heap_size, distance);

        // If heap size exceeds k, remove the largest element
        if heap_size > k {
            heap_remove_top(&mut heap, &mut heap_size);
        }

        // If heap size equals k, return the top element (k-th largest distance)
        if heap_size == k {
            result.push(heap[0]);
        } else {
            result.push(-1); // Not enough elements in the heap
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().expect("Invalid input");
    let k: usize = parts.next().unwrap().parse().expect("Invalid input");

    let mut queries = Vec::new();
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();
        queries.push(coords);
    }

    let result = results_array(queries, k);
    for r in result {
        print!("{} ", r);
    }
}