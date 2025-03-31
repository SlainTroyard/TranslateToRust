// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};
use std::cmp::Ordering;

/// Implements a max-heap data structure for integers
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    /// Create a new empty max-heap
    fn new(capacity: usize) -> Self {
        MaxHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Get the current size of the heap
    fn size(&self) -> usize {
        self.data.len()
    }

    /// Insert a value into the heap
    fn insert(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1);
    }

    /// Remove and return the top (maximum) element from the heap
    fn remove_top(&mut self) {
        if self.data.len() <= 1 {
            self.data.clear();
            return;
        }
        
        // Replace root with the last element and remove the last
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        self.data.pop();
        
        // Re-heapify from the root
        self.heapify_down(0);
    }

    /// Get the top (maximum) element without removing it
    fn peek(&self) -> Option<i32> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data[0])
        }
    }

    /// Heapify up - move a node up the tree to maintain heap property
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[parent] < self.data[idx] {
                self.data.swap(parent, idx);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// Heapify down - move a node down the tree to maintain heap property
    fn heapify_down(&mut self, mut idx: usize) {
        let size = self.data.len();
        loop {
            let mut largest = idx;
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            
            if left < size && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < size && self.data[right] > self.data[largest] {
                largest = right;
            }
            
            if largest != idx {
                self.data.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }
}

/// Calculate the results array based on Manhattan distances
fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    
    // Use a max-heap to keep track of the k smallest distances
    let mut heap = MaxHeap::new((k + 1) as usize);
    
    for query in queries {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();
        
        // Add the current distance to the max-heap
        heap.insert(distance);
        
        // If the heap size exceeds k, remove the largest element
        if heap.size() > k as usize {
            heap.remove_top();
        }
        
        // If the heap size equals k, return the top element (kth smallest distance)
        if heap.size() == k as usize {
            result.push(heap.peek().unwrap());
        } else {
            result.push(-1); // Heap has fewer than k elements
        }
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first line containing queriesSize and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }
    
    // Calculate the results
    let result = results_array(&queries, k);
    
    // Print the results
    for (i, val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}