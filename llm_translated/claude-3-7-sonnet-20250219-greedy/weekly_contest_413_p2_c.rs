// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};
use std::cmp::Ordering;

/// Create a max-heap implementation for the problem
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    // Create a new empty max-heap
    fn new(capacity: usize) -> Self {
        MaxHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    // Insert a value into the heap
    fn insert(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1);
    }

    // Remove the top (maximum) element from the heap
    fn remove_top(&mut self) {
        if self.data.len() <= 1 {
            self.data.clear();
            return;
        }
        
        let last = self.data.pop().unwrap();
        self.data[0] = last;
        self.heapify_down(0);
    }

    // Get the top (maximum) element of the heap
    fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    // Get the current size of the heap
    fn size(&self) -> usize {
        self.data.len()
    }

    // Heapify up to maintain heap property after insertion
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

    // Heapify down to maintain heap property after removal
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

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut result = Vec::with_capacity(queries.len());
    
    // Initialize a max-heap to store distances
    let mut heap = MaxHeap::new(k + 1);
    
    for query in queries {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();
        
        // Add the current distance to the max-heap
        heap.insert(distance);
        
        // If heap size exceeds k, remove the maximum element
        if heap.size() > k {
            heap.remove_top();
        }
        
        // If heap size equals k, return the top element (kth largest distance)
        if heap.size() == k {
            result.push(heap.peek().unwrap());
        } else {
            result.push(-1); // Heap has fewer than k elements
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line containing queriesSize and k
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Parse the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(vec![coords[0], coords[1]]);
    }
    
    // Process the queries and get the result
    let result = results_array(&queries, k);
    
    // Print the result
    for (i, val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    
    Ok(())
}