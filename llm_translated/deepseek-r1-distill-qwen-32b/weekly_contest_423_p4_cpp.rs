use std::io;

struct Solver<'a> {
    s: &'a str,
    cnt: Vec<i32>,
    k: i32,
    mod: i32,
    dp: Vec<Vec<Vec<i32>>>,
}

impl<'a> Solver<'a> {
    fn new(s: &'a str, k: i32) -> Solver<'a> {
        let mut cnt = vec![0; 801];
        for i in 2..=800 {
            let set_bits = i.count_ones() as i32;
            cnt[i] = 1 + cnt[set_bits as usize];
        }
        let dp = vec![vec![vec![-1; 801]; 2]; 801];
        Solver {
            s,
            cnt,
            k,
            mod: 1_000_000_007,
            dp,
        }
    }

    fn solve(&mut self, i: usize, tight: bool, set_bits: i32) -> i32 {
        if i == self.s.len() {
            if tight || set_bits == 0 {
                return 0;
            } else {
                return if self.cnt[set_bits as usize] < self.k { 1 } else { 0 };
            }
        }

        let tight_idx = tight as usize;
        let set_bits_usize = set_bits as usize;
        if self.dp[i][tight_idx][set_bits_usize] != -1 {
            return self.dp[i][tight_idx][set_bits_usize];
        }

        let res;
        if tight {
            let current_char = self.s.chars().nth(i).unwrap();
            if current_char == '0' {
                res = self.solve(i + 1, true, set_bits);
            } else {
                let res1 = self.solve(i + 1, true, set_bits + 1);
                let res2 = self.solve(i + 1, false, set_bits);
                res = (res1 + res2) % self.mod;
            }
        } else {
            let res1 = self.solve(i + 1, false, set_bits + 1);
            let res2 = self.solve(i + 1, false, set_bits);
            res = (res1 + res2) % self.mod;
        }

        self.dp[i][tight as usize][set_bits as usize] = res;
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let k = parts[1].parse::<i32>().unwrap();

    let mut solver = Solver::new(s, k);
    let result = solver.solve(0, true, 0);
    println!("{}", result);
}