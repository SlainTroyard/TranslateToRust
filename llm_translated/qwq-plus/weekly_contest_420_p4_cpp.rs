use std::io;

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        if n == 0 {
            return vec![];
        }

        // Build adjacency list
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        // DFS to record entry and exit times
        let mut nodes = vec![(0, 0); n];
        let mut dfs_str = String::new();
        let mut time = 0;
        let mut stack = vec![];
        stack.push((0, false));

        while let Some((x, visited)) = stack.pop() {
            if !visited {
                nodes[x].0 = time;
                stack.push((x, true));
                // Push children in reverse order to process them in order
                for &child in g[x].iter().rev() {
                    stack.push((child, false));
                }
            } else {
                let c = s.chars().nth(x).unwrap();
                dfs_str.push(c);
                time += 1;
                nodes[x].1 = time;
            }
        }

        // Build transformed string for Manacher's algorithm
        let mut t = String::from("^");
        for c in dfs_str.chars() {
            t.push('#');
            t.push(c);
        }
        t.push('#');
        t.push('$');

        // Manacher's algorithm
        let len_t = t.len();
        let half_len_size = len_t - 2;
        let mut half_len = vec![0; half_len_size];
        half_len[1] = 1;
        let (mut box_m, mut box_r) = (0, 0);

        for i in 2..half_len_size {
            let mut hl = 1;
            if i < box_r {
                let mirror = 2 * box_m - i;
                hl = std::cmp::min(half_len[mirror], box_r - i);
            }

            loop {
                let left = i - hl;
                let right = i + hl;
                if left < 0 || right >= len_t {
                    break;
                }
                let c1 = t.chars().nth(left).unwrap();
                let c2 = t.chars().nth(right).unwrap();
                if c1 != c2 {
                    break;
                }
                hl += 1;
                box_m = i;
                box_r = i + hl;
            }

            half_len[i] = hl;
        }

        // Check each node's substring
        let is_palindrome = |l: usize, r: usize| -> bool {
            let pos = l + r + 1;
            if pos >= half_len.len() {
                return false;
            }
            half_len[pos] > (r - l)
        };

        (0..n)
            .map(|i| {
                let (l, r) = nodes[i];
                is_palindrome(l, r)
            })
            .collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut parent = Vec::with_capacity(n);
    for _ in 0..n {
        let p: i32 = tokens.next().unwrap().parse().unwrap();
        parent.push(p);
    }
    let s: String = tokens.next().unwrap().to_string();

    let solution = Solution;
    let ans = solution.find_answer(parent, s);
    for b in ans {
        print!("{}{}", if b { "true" } else { "false" }, " ");
    }
    println!();
}