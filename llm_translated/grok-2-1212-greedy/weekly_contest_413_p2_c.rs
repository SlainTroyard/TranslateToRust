// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};

// Swap two integers
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

// Heapify down
fn heapify_down(heap: &mut [i32], size: usize, idx: usize) {
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

// Heapify up
fn heapify_up(heap: &mut [i32], idx: usize) {
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

// Insert element into heap
fn heap_insert(heap: &mut [i32], size: &mut usize, val: i32) {
    heap[*size] = val;
    heapify_up(heap, *size);
    *size += 1;
}

// Remove top element from heap
fn heap_remove_top(heap: &mut [i32], size: &mut usize) {
    if *size <= 1 {
        *size = 0;
        return;
    }
    heap[0] = heap[*size - 1];
    *size -= 1;
    heapify_down(heap, *size, 0);
}

// Process queries and return results
fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut result = vec![0; queries.len()];
    let mut heap = vec![0; k + 1];
    let mut heap_size = 0;

    for (i, query) in queries.iter().enumerate() {
        // Calculate positive distance
        let distance = query[0].abs() + query[1].abs();

        // Insert current distance into max heap
        heap_insert(&mut heap, &mut heap_size, distance);

        // If heap size exceeds k, remove the largest element
        if heap_size > k {
            heap_remove_top(&mut heap, &mut heap_size);
        }

        // If heap size equals k, return the top element (kth largest distance)
        if heap_size == k {
            result[i] = heap[0];
        } else {
            result[i] = -1; // Less than k elements in the heap
        }
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read queries size and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let query: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Process queries and get results
    let result = results_array(&queries, k);

    // Print results
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}