use std::collections::VecDeque;

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<usize>> = (0..n).map(|_| Vec::with_capacity(10)).collect();
    let mut pos_r: Vec<usize> = vec![n; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        let x = nums[i];
        while let Some(&top) = stack.last() {
            if x >= nums[top] {
                pos_r[top] = i;
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stack.last() {
            g[top].push(i);
        }
        stack.push(i);
    }

    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    let mut l = 0;
    let mut q: VecDeque<usize> = VecDeque::new();

    for r in 0..n {
        let x = nums[r];
        while let Some(&back) = q.back() {
            if nums[back] <= x {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(r);
        let front = q.front().unwrap();
        cnt += (nums[*front] as i64) - (x as i64);

        while cnt > k as i64 {
            let out = nums[l];
            for &i in &g[l] {
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < r + 1 {
                    pos_r[i]
                } else {
                    r + 1
                };
                cnt -= (out as i64 - nums[i] as i64) * (min_pos as i64 - i as i64);
            }
            l += 1;
            if let Some(front) = q.front() {
                if *front < l {
                    q.pop_front();
                }
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let mut tokens = input.split_whitespace();

    let nums_size: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => {
            eprintln!("Error reading input for numsSize and k");
            std::process::exit(1);
        }
    };

    let k: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(k_val) => k_val,
        None => {
            eprintln!("Error reading input for numsSize and k");
            std::process::exit(1);
        }
    };

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for i in 0..nums_size {
        let token = tokens.next().ok_or(()).unwrap_or_else(||
            {
                eprintln!("Error reading input for nums[{}]", i);
                std::process::exit(1);
            });
        let num = match token.parse().ok() {
            Some(n) => n,
            None => {
                eprintln!("Error reading input for nums[{}]", i);
                std::process::exit(1);
            }
        };
        nums.push(num);
    }

    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
}