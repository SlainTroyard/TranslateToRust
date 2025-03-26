use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i32; SIZE]; SIZE];

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for k in 0..SIZE {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..SIZE {
                c[i][j] = (c[i][j] as i64 + (a[i][k] as i64 * b[k][j] as i64) % MOD) as i32;
            }
        }
    }
    c
}

fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    let mut res = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1; // Identity matrix
    }
    while n > 0 {
        if n & 1 == 1 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    res
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut m = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in i + 1..=i + nums[i] as usize {
            m[i][j % SIZE] = 1;
        }
    }
    let m = pow(m, t);

    let mut cnt = [0; SIZE];
    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..SIZE {
        // Sum of row i in matrix m is f[t][i]
        let row_sum: i64 = m[i].iter().map(|&x| x as i64).sum();
        ans += row_sum * cnt[i] as i64;
    }
    (ans % MOD) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first line: s_len, s, t
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let _s_len: usize = parts[0].parse().unwrap();
    let s = parts[1].to_string();
    let t: i32 = parts[2].parse().unwrap();
    
    // Read nums (26 integers)
    let mut nums = Vec::with_capacity(26);
    for _ in 0..26 {
        if nums.len() == 26 {
            break;
        }
        
        let line = lines.next().unwrap().unwrap();
        for num in line.split_whitespace() {
            nums.push(num.parse::<i32>().unwrap());
            if nums.len() == 26 {
                break;
            }
        }
    }
    
    let solution = length_after_transformations(&s, t, &nums);
    println!("{}", solution);
}