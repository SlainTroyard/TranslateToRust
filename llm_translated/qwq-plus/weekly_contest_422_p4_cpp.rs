const MOD: i64 = 1_000_000_007;

struct Solution {
    cnt: [i32; 10],
    left_s: [i32; 10],
    left_c: [i32; 10],
    dp: Vec<Vec<Vec<i64>>>,
    cb: Vec<Vec<i64>>,
    r1: Vec<i64>,
    n: usize,
    sum: i32,
}

impl Solution {
    fn new(num: &str) -> Self {
        let mut cnt = [0; 10];
        let mut s = 0;
        for c in num.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            s += d;
            cnt[d as usize] += 1;
        }
        let n = num.len();
        let mut cb = vec![vec![0; 81]; 81];
        Self::pascal(&mut cb);
        let mut r1 = vec![0; 11];
        r1[10] = 1;
        let (mut left_s, mut left_c) = ([0; 10], [0; 10]);
        for i in (0..10).rev() {
            let mut ls = if i < 9 { left_s[i + 1] } else { 0 };
            let mut lc = if i < 9 { left_c[i + 1] } else { 0 };
            ls += i as i32 * cnt[i];
            lc += cnt[i];
            left_s[i] = ls;
            left_c[i] = lc;
            let c = lc;
            let cnt_i = cnt[i];
            r1[i] = (Self::comb(&cb, c as usize, cnt_i as usize) * r1[i + 1]) % MOD;
        }
        let dp = vec![vec![vec![-1; 81]; 721]; 10];
        Self {
            cnt,
            left_s,
            left_c,
            dp,
            cb,
            r1,
            n,
            sum: s,
        }
    }

    fn pascal(cb: &mut Vec<Vec<i64>>) {
        cb[0][0] = 1;
        for i in 1..=80 {
            cb[i][0] = 1;
            cb[i][i] = 1;
            for j in 1..i {
                cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % MOD;
            }
        }
    }

    fn comb(cb: &Vec<Vec<i64>>, n: usize, k: usize) -> i64 {
        if k > n || k < 0 {
            0
        } else {
            cb[n][k]
        }
    }

    fn dfs(&mut self, i: usize, s: i32, c: i32) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == 10 {
            return 0;
        }
        if s > self.left_s[i] as i32 || c > self.left_c[i] as i32 {
            return 0;
        }
        let s_usize = s as usize;
        let c_usize = c as usize;
        if self.dp[i][s_usize][c_usize] != -1 {
            return self.dp[i][s_usize][c_usize];
        }
        let mut res = 0i64;
        let mut x = 0;
        let mut a = s;
        let mut y = self.cnt[i] as i32;
        while x <= self.cnt[i] as i32 && a >= 0 && x <= c {
            if y > (self.left_c[i] as i32 - c) {
                x += 1;
                a -= i as i32;
                y -= 1;
                continue;
            }
            let term1 = self.dfs(i + 1, a, c - x);
            let comb1 = Self::comb(&self.cb, c as usize, x as usize);
            let term_part = (term1 * comb1) % MOD;
            let remaining_cnt_i = y - x;
            let available_slots = (self.left_c[i] as i32 - c) as usize;
            let comb2 = Self::comb(&self.cb, available_slots, remaining_cnt_i as usize);
            let contribution = (term_part * comb2) % MOD;
            res = (res + contribution) % MOD;
            x += 1;
            a -= i as i32;
            y -= 1;
        }
        self.dp[i][s_usize][c_usize] = res;
        res
    }

    fn count(&mut self) -> i32 {
        if self.sum % 2 != 0 {
            return 0;
        }
        let target_s = self.sum / 2;
        let target_c = (self.n as i32) / 2;
        let result = self.dfs(0, target_s, target_c);
        result as i32
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim().to_string();
    if num.len() > 80 {
        println!("Input too long, maximum allowed length is 80");
        return;
    }
    let mut sol = Solution::new(&num);
    if sol.sum % 2 != 0 {
        println!("0");
    } else {
        let result = sol.count();
        println!("{}", result);
    }
}