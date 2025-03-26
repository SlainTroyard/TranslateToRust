use std::io;

const MOD: u32 = 1_000_000_007;
const SIZE: usize = 26;

fn multiply(a: &[[u32; 26]; 26], b: &[[u32; 26]; 26]) -> [[u32; 26]; 26] {
    let mut c = [[0; 26]; 26];
    for i in 0..SIZE {
        for k in 0..SIZE {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..SIZE {
                let val = (a[i][k] as u64) * (b[k][j] as u64);
                c[i][j] = (c[i][j] as u64 + val) % (MOD as u64) as u32;
            }
        }
    }
    c
}

fn matrix_pow(mut a: [[u32; 26]; 26], mut n: u32) -> [[u32; 26]; 26] {
    let mut res = [[0; 26]; 26];
    for i in 0..SIZE {
        res[i][i] = 1;
    }
    while n > 0 {
        if n & 1 == 1 {
            res = multiply(&res, &a);
        }
        a = multiply(&a, &a);
        n >>= 1;
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let s_len: usize = tokens.next().unwrap().parse().expect("Invalid s_len");
    let s: String = tokens.next().unwrap().to_string();
    let t: u32 = tokens.next().unwrap().parse().expect("Invalid t");

    let mut nums = Vec::new();
    for _ in 0..26 {
        let num: u32 = tokens.next().unwrap().parse().expect("Invalid nums entry");
        nums.push(num);
    }

    let mut m = [[0; 26]; 26];
    for i in 0..26 {
        let max_j = i + nums[i] as usize;
        for j in (i + 1)..=max_j {
            let col = j % 26;
            m[i][col] = 1;
        }
    }

    let m_pow = matrix_pow(m, t);

    let mut cnt = [0; 26];
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }

    let mut ans: u64 = 0;
    for i in 0..26 {
        let row_sum: u32 = m_pow[i].iter().sum();
        ans += (row_sum as u64) * (cnt[i] as u64);
    }

    let result = ans % (MOD as u64);
    println!("{}", result);
}