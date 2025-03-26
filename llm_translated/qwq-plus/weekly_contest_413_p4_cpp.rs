use std::io;

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                f[j] ^= f[j - 1];
                mx[i][j] = std::cmp::max(
                    std::cmp::max(f[j], mx[i + 1][j]),
                    mx[i][j - 1],
                );
            }
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            ans.push(mx[l][r]);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse input"));

    let nums_size = tokens.next().unwrap() as usize;
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(tokens.next().unwrap());
    }

    let queries_size = tokens.next().unwrap() as usize;
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let l = tokens.next().unwrap();
        let r = tokens.next().unwrap();
        queries.push(vec![l, r]);
    }

    let mut f = nums;
    let result = Solution::maximum_subarray_xor(&mut f, &queries);

    for &val in &result {
        print!("{} ", val);
    }
    println!();
}