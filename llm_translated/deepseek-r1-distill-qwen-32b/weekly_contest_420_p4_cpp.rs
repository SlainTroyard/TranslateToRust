use std::io;

fn main() {
    let n = read_input_line().parse().unwrap();
    let parent: Vec<i32> = read_input_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s = read_input_line().trim().to_string();

    let solution = Solution {};
    let ans = solution.find_answer(parent, s);

    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}

fn read_input_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

struct Solution {}

impl Solution {
    fn find_answer(&self, parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut tree = vec![vec![]; n];
        for i in 1..n {
            let p = parent[i] as usize;
            tree[p].push(i);
        }

        let mut dfs_str = vec![0 as char; n];
        let mut nodes = vec![(0, 0); n];
        let mut time = 0;
        let mut stack = vec![(0, false)];

        while let Some((x, visited)) = stack.pop() {
            if visited {
                dfs_str[time] = s.chars().nth(x).unwrap();
                time += 1;
                nodes[x] = (nodes[x].0, time);
            } else {
                nodes[x].0 = time;
                stack.push((x, true));
                for &child in tree[x].iter().rev() {
                    stack.push((child, false));
                }
            }
        }

        let t: String = {
            let mut res = String::from("^");
            for c in dfs_str {
                res.push('#');
                res.push(c);
            }
            res.push_str("#$");
            res
        };

        let m = t.len();
        let mut half_len = vec![0; m];
        half_len[1] = 1;
        let (mut box_m, mut box_r) = (0, 0);

        for i in 2..m {
            let mut hl = 1;
            if i < box_r {
                hl = half_len[2 * box_m - i].min(box_r - i);
            }
            while i as i32 - hl as i32 >= 1 && i + hl < m && t.chars().nth(i - hl).unwrap() == t.chars().nth(i + hl).unwrap() {
                hl += 1;
            }
            hl -= 1;
            half_len[i] = hl;
            if i + hl > box_r {
                box_m = i;
                box_r = i + hl;
            }
        }

        let is_palindrome = |l: usize, r: usize| -> bool {
            let center = l + r + 1;
            let radius = (r - l) + 1;
            half_len[center] >= radius
        };

        let mut ans = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1 - 1);
        }
        ans
    }
}