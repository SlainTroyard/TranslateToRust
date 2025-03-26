fn is_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> bool {
    let n = nums.len();
    let mut v = vec![0; n + 1];
    
    // Apply all queries to the difference array
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        v[l] += 1;
        if r + 1 <= n {
            v[r + 1] -= 1;
        }
    }
    
    // Compute prefix sums to get the count of operations for each index
    for i in 1..n {
        v[i] += v[i - 1];
    }
    
    // Check if all elements can be reduced to zero or less
    nums.iter()
        .zip(v.iter().take(n))
        .all(|(&num, &count)| num - count <= 0)
}

fn main() {
    // Read entire input and split into tokens
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    // Parse nums array
    let n = tokens.next().expect("Missing array size");
    let nums: Vec<i32> = (0..n).map(|_| tokens.next().expect("Missing num")).collect();
    
    // Parse queries
    let m = tokens.next().expect("Missing query count");
    let queries: Vec<Vec<i32>> = (0..m)
        .map(|_| {
            let l = tokens.next().expect("Missing query left");
            let r = tokens.next().expect("Missing query right");
            vec![l, r]
        })
        .collect();
    
    // Check and print result
    let result = is_zero_array(&nums, &queries);
    println!("{}", if result { "true" } else { "false" });
}