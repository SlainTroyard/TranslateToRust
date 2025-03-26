struct Solution;

impl Solution {
    fn min_zero_array(&self, nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut d = vec![0; n + 1];
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut cur = -1;
        while acc <= 0 && cur < n as i32 {
            cur += 1;
            if cur >= n as i32 {
                break;
            }
            acc += d[cur as usize];
        }

        if cur == n as i32 {
            return 0;
        }

        for (i, query) in queries.iter().enumerate() {
            let l = query[0];
            let r = query[1];
            let val = query[2];

            d[r as usize + 1] += val;
            d[l as usize] -= val;

            if cur >= l && cur <= r {
                acc -= val;
                while acc <= 0 && cur < n as i32 {
                    cur += 1;
                    if cur >= n as i32 {
                        break;
                    }
                    acc += d[cur as usize];
                }
                if cur == n as i32 {
                    return (i + 1) as i32;
                }
            }
        }

        -1
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut ptr = 0;

    // Read n
    let n: usize = tokens[ptr].parse().expect("Failed to parse n");
    ptr += 1;

    // Read nums
    let nums: Vec<i32> = tokens[ptr..ptr + n]
        .iter()
        .map(|s| s.parse().expect("Failed to parse nums"))
        .collect();
    ptr += n;

    // Read m
    let m: usize = tokens[ptr].parse().expect("Failed to parse m");
    ptr += 1;

    // Read queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let l: i32 = tokens[ptr].parse().expect("Failed to parse query l");
        let r: i32 = tokens[ptr + 1].parse().expect("Failed to parse query r");
        let val: i32 = tokens[ptr + 2].parse().expect("Failed to parse query val");
        queries.push(vec![l, r, val]);
        ptr += 3;
    }

    // Call the solution function
    let solution = Solution;
    let result = solution.min_zero_array(nums, queries);
    println!("{}", result);
}