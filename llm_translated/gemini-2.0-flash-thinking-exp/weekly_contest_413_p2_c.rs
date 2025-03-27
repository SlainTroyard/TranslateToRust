// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};
use std::vec;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 交换两个整数 (Rust version uses mutable references and std::mem::swap for efficiency, though manual swap would also work)
fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

// 向下调整堆 (Rust version using slice and usize for indexing)
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

// 向上调整堆 (Rust version using slice and usize for indexing)
fn heapify_up(heap: &mut [i32], idx: usize) {
    let mut current_idx = idx;
    while current_idx > 0 {
        let parent = (current_idx - 1) / 2;
        if heap[parent] < heap[current_idx] {
            swap(&mut heap[parent], &mut heap[current_idx]);
            current_idx = parent;
        } else {
            break;
        }
    }
}

// 插入元素到堆中 (Rust version using Vec and slice, size is implicitly managed by Vec)
fn heap_insert(heap: &mut Vec<i32>, val: i32) {
    heap.push(val);
    let size = heap.len();
    if size > 0 {
        heapify_up(heap, size - 1);
    }
}

// 移除堆顶元素 (Rust version using Vec and slice, size is implicitly managed by Vec)
fn heap_remove_top(heap: &mut Vec<i32>) {
    let size = heap.len();
    if size <= 1 {
        heap.clear(); // Or heap.truncate(0) to clear in place
        return;
    }
    heap.swap(0, size - 1);
    heap.pop(); // Remove last element (which was the top)
    heapify_down(heap, heap.len(), 0);
}

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size]; // Initialize result vector with -1

    // 使用 Vec 实现大顶堆，直接存储正距离
    let mut heap: Vec<i32> = Vec::with_capacity((k + 1) as usize);

    for i in 0..queries_size {
        // 计算正距离
        let distance = queries[i][0].abs() + queries[i][1].abs();

        // 将当前距离加入大顶堆
        heap_insert(&mut heap, distance);

        // 如果堆的大小超过k，移除最大的元素
        if heap.len() > k as usize {
            heap_remove_top(&mut heap);
        }

        // 如果堆的大小等于k，返回堆顶元素（第k大的距离）
        if heap.len() == k as usize {
            result[i] = heap[0];
        } else {
            result[i] = -1; // 堆中元素少于k个
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter.next().unwrap().parse().expect("Failed to parse queriesSize");
    let k: i32 = first_line_iter.next().unwrap().parse().expect("Failed to parse k");

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().expect("Failed to read line");
        let mut query_iter = line.split_whitespace();
        let qx: i32 = query_iter.next().unwrap().parse().expect("Failed to parse query x");
        let qy: i32 = query_iter.next().unwrap().parse().expect("Failed to parse query y");
        queries.push(vec![qx, qy]);
    }

    let result = results_array(&queries, k);

    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}