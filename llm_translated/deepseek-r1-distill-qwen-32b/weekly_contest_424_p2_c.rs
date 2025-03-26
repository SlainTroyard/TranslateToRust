use std::io;
use std::collections::VecDeque;

fn is_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> bool {
    let n = nums.len();
    let mut diff = vec![0; n];
    
    for query in queries {
        let l = query[0] as usize;
        diff[l] += 1;
        let r = query[1] as usize;
        if r + 1 < n {
            diff[r + 1] -= 1;
        }
    }
    
    let mut count = 0;
    for i in 0..n {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().collect::<VecDeque<_>>();
    
    let nums_size = tokens.pop_front().unwrap().parse::<usize>().unwrap();
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.pop_front().unwrap().parse().unwrap())
        .collect();
    
    let queries_size = tokens.pop_front().unwrap().parse::<usize>().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let l = tokens.pop_front().unwrap().parse::<i32>().unwrap();
        let r = tokens.pop_front().unwrap().parse::<i32>().unwrap();
        queries.push(vec![l, r]);
    }
    
    let result = is_zero_array(&nums, &queries);
    println!("{}", if result { "true" } else { "false" });
}