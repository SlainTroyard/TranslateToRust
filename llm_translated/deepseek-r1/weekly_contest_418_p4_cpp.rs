use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Read n and nums
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(tokens.next().unwrap().parse::<i32>().unwrap());
    }
    
    // Read q and queries
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        queries.push(tokens.next().unwrap().parse::<i64>().unwrap());
    }
    
    // Process and get the answer
    let ans = gcd_values(nums, queries);
    
    // Print the answer
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    // Handle empty input edge case
    if nums.is_empty() {
        return vec![0; queries.len()];
    }
    
    let mx = *nums.iter().max().unwrap() as usize;
    let mut cnt_x = vec![0; mx + 1];
    for &x in &nums {
        cnt_x[x as usize] += 1;
    }
    
    // Compute GCD counts using inclusion-exclusion principle
    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        let mut j = i;
        while j <= mx {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += (c as i64) * (c as i64 - 1) / 2;
    }
    
    // Convert to prefix sums (cumulative counts)
    let mut sum = 0;
    for i in 0..=mx {
        sum += cnt_gcd[i];
        cnt_gcd[i] = sum;
    }
    
    // Answer queries using binary search for upper bound
    queries
        .iter()
        .map(|&k| {
            cnt_gcd
                .partition_point(|&x| x <= k) as i32
        })
        .collect()
}