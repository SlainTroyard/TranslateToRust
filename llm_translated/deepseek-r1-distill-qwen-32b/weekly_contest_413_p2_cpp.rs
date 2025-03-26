use std::io;
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    let queries_size = tokens.next().unwrap();
    let k = tokens.next().unwrap();
    
    let mut queries = Vec::with_capacity(queries_size as usize);
    for _ in 0..queries_size {
        let x = tokens.next().unwrap();
        let y = tokens.next().unwrap();
        queries.push((x, y));
    }
    
    let mut heap = BinaryHeap::new();
    let mut ans = vec![-1; queries_size as usize];
    
    for i in 0..queries_size as usize {
        let (x, y) = queries[i];
        let sum = x.abs() + y.abs();
        heap.push(sum);
        
        if heap.len() > k as usize {
            heap.pop();
        }
        
        if heap.len() == k as usize {
            ans[i] = *heap.peek().unwrap();
        }
    }
    
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}