use std::collections::BinaryHeap;
use std::io::{self, Read};

fn results_array(queries: &[Vec<i32>], k: usize) -> Vec<i32> {
    let mut ans = vec![-1; queries.len()];
    // Use a max-heap to keep track of the k smallest sums. When the heap size exceeds k,
    // remove the largest element to maintain the k smallest elements.
    let mut heap = BinaryHeap::new();
    
    for (i, q) in queries.iter().enumerate() {
        let sum = q[0].abs() + q[1].abs();
        heap.push(sum);
        
        // Maintain heap size <= k by removing the largest element if necessary
        if heap.len() > k {
            heap.pop();
        }
        
        // If we have exactly k elements, the top is the k-th smallest
        if heap.len() == k {
            ans[i] = *heap.peek().unwrap();
        }
    }
    
    ans
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Parse initial parameters: number of queries and k
    let queries_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let k = tokens.next().unwrap().parse::<i32>().unwrap();
    
    // Parse queries as pairs of integers
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let x = tokens.next().unwrap().parse::<i32>().unwrap();
        let y = tokens.next().unwrap().parse::<i32>().unwrap();
        queries.push(vec![x, y]);
    }
    
    // Compute results and print with space-separated format
    let ans = results_array(&queries, k as usize);
    for num in ans {
        print!("{} ", num);
    }
    println!();
}