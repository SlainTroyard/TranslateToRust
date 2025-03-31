// Problem: Weekly Contest 413 Problem 2
use std::io;

fn heap_insert(heap: &mut Vec<i32>, val: i32) {
    heap.push(val);
    let idx = heap.len() - 1;
    heapify_up(heap, idx);
}

fn heapify_up(heap: &mut Vec<i32>, mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if heap[parent] < heap[idx] {
            heap.swap(parent, idx);
            idx = parent;
        } else {
            break;
        }
    }
}

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
        heap.swap(idx, largest);
        idx = largest;
    }
}

fn heap_remove_top(heap: &mut Vec<i32>) {
    if heap.is_empty() {
        return;
    }
    if heap.len() == 1 {
        heap.pop();
        return;
    }
    heap.swap(0, heap.len() - 1);
    heap.pop();
    let size = heap.len();
    heapify_down(heap, size, 0);
}

pub fn results_array(queries: &[(i32, i32)], k: usize) -> Vec<i32> {
    let mut heap = Vec::new();
    let mut result = Vec::with_capacity(queries.len());
    for &(x, y) in queries {
        let distance = x.abs() + y.abs();
        heap_insert(&mut heap, distance);
        if heap.len() > k {
            heap_remove_top(&mut heap);
        }
        if heap.len() == k {
            result.push(heap[0]);
        } else {
            result.push(-1);
        }
    }
    result
}

fn main() {
    let mut numbers = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        numbers.extend(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );
    }
    let queries_size = numbers[0] as usize;
    let k = numbers[1] as usize;
    let queries: Vec<(i32, i32)> = (0..queries_size)
        .map(|i| {
            let idx = 2 + 2 * i;
            (numbers[idx], numbers[idx + 1])
        })
        .collect();
    let result = results_array(&queries, k);
    for &num in &result {
        print!("{} ", num);
    }
    println!();
}