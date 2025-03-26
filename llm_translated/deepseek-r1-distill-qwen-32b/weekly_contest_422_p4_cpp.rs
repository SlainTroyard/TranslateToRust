const M: i64 = 1_000_000_007;

struct Solution {
    comb: Vec<Vec<i64>>,
    cnt: [i32; 10],
    left_s: [i32; 10],
    left_c: [i32; 10],
    r1: [i64; 11],
    dp: Vec<Vec<Vec<i64>>>,
    n: i32,
}

impl Solution {
    fn pascal(&mut self) {
        let m = 80;
        self.comb = vec![vec![0; m + 1]; m + 1];
        self.comb[0][0] = 1;
        for i in 1..=m {
            self.comb[i][0] = 1;
            self.comb[i][i] = 1;
            for j in 1..i {
                self.comb[i][j] = (self.comb[i - 1][j - 1] + self.comb[i - 1][j]) % M;
            }
        }
    }

    fn dfs(&mut self, i: i32, s: i32, c: i32) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i as usize];
        }
        if i == 10 {
            return 0;
        }
        if s > self.left_s[i as usize] || c > self.left_c[i as usize] {
            return 0;
        }
        let idx_i = i as usize;
        let idx_s = s as usize;
        let idx_c = c as usize;
        if self.dp[idx_i][idx_s][idx_c] != -1 {
            return self.dp[idx_i][idx_s][idx_c];
        }
        let mut res = 0;
        let i_val = i as i32;
        let cnt_i = self.cnt[idx_i];
        for x in 0..=cnt_i {
            let a = s - x * i_val;
            if a < 0 || x > c {
                break;
            }
            let y = cnt_i - x;
            if y > (self.left_c[idx_i] - c) {
                continue;
            }
            let b = (self.dfs(i + 1, a, c - x) * self.comb[c as usize][x as usize]) % M;
            res = (res + b * self.comb[(self.left_c[idx_i] - c) as usize][y as usize]) % M;
        }
        self.dp[idx_i][idx_s][idx_c] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut cnt = [0; 10];
        let mut s = 0;
        for c in num.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            cnt[d as usize] += 1;
            s += d;
        }
        self.cnt = cnt;

        if s % 2 != 0 {
            return 0;
        }

        self.pascal();

        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..10).rev() {
            ls += i as i32 * self.cnt[i as usize];
            lc += self.cnt[i as usize];
            self.left_s[i as usize] = ls;
            self.left_c[i as usize] = lc;
            self.r1[i as usize] = (self.r1[i as usize + 1] * self.comb[lc as usize][self.cnt[i as usize] as usize]) % M;
        }

        self.dp = vec![vec![vec![-1; 81]; 721]; 10];

        let target_s = s / 2;
        let target_c = num.len() as i32 / 2;
        self.dfs(0, target_s, target_c) as i32
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim();
    if num.len() > 80 {
        println!("Input too long, maximum allowed length is 80");
        return;
    }
    let mut sol = Solution {
        comb: Vec::new(),
        cnt: [0; 10],
        left_s: [0; 10],
        left_c: [0; 10],
        r1: [0; 11],
        dp: Vec::new(),
        n: 0,
    };
    let result = sol.count_balanced_permutations(num);
    println!("{}", result);
}