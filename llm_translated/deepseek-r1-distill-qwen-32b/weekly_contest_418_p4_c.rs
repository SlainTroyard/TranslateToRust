use std::io;

fn gcd_values(nums: &[i32], nums_size: usize, queries: &[i64], queries_size: usize) -> Vec<i32> {
    if nums_size == 0 || queries_size == 0 {
        return Vec::new();
    }

    let mx = *nums.iter().max().unwrap();
    let mut cnt_x = vec![0; (mx + 1) as usize];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    let mut cnt_gcd = vec![0; (mx + 1) as usize];
    for i in (1..=mx).rev() {
        let i_usize = i as usize;
        let mut c = 0;
        let mut sum = 0;
        for j in (i..=mx).step_by(i as usize) {
            let j_usize = j as usize;
            c += cnt_x[j_usize];
            sum += cnt_gcd[j_usize];
        }
        cnt_gcd[i_usize] = (c * (c - 1) / 2) as i64 - sum;
    }

    for i in 1..=mx {
        let i_usize = i as usize;
        cnt_gcd[i_usize] += cnt_gcd[i_usize - 1];
    }

    let mut ans = Vec::with_capacity(queries_size);
    for &query in queries {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid as usize] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left);
    }

    ans
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    
    let nums_size = tokens.next().unwrap() as usize;
    let nums: Vec<i32> = tokens.take(nums_size).map(|x| x as i32).collect();
    let queries_size = tokens.next().unwrap() as usize;
    let queries: Vec<i64> = tokens.take(queries_size).collect();
    
    let ans = gcd_values(&nums, nums_size, &queries, queries_size);
    
    for num in ans {
        print!("{} ", num);
    }
    println!();
}