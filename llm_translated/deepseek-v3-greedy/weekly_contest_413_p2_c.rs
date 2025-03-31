// Problem: Weekly Contest 413 Problem 2
use std::cmp;

// Max-heap implementation
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn insert(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.size() - 1);
    }

    fn remove_top(&mut self) {
        if self.size() <= 1 {
            self.data.clear();
            return;
        }
        self.data[0] = self.data.pop().unwrap();
        self.heapify_down(0);
    }

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

    fn heapify_down(&mut self, mut idx: usize) {
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < self.size() && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < self.size() && self.data[right] > self.data[largest] {
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

    fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }
}

fn results_array(queries: &[Vec<i32>], k: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    let mut heap = MaxHeap::new();

    for query in queries {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();

        // Insert into max heap
        heap.insert(distance);

        // If heap size exceeds k, remove the largest element
        if heap.size() > k {
            heap.remove_top();
        }

        // If we have exactly k elements, return the largest (k-th smallest)
        if heap.size() == k {
            result.push(heap.peek().unwrap());
        } else {
            result.push(-1); // Not enough elements yet
        }
    }

    result
}

fn main() {
    // Read input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let queries_size = parts.next().unwrap().parse::<usize>().unwrap();
    let k = parts.next().unwrap().parse::<usize>().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        queries.push(vec![x, y]);
    }

    // Process queries
    let result = results_array(&queries, k);

    // Print output
    for num in result {
        print!("{} ", num);
    }
    println!();
}