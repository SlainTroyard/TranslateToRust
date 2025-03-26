use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut dfs_str: Vec<char> = vec!['\0'; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;

        fn dfs(
            x: usize,
            g: &Vec<Vec<usize>>,
            s: &String,
            dfs_str: &mut Vec<char>,
            nodes: &mut Vec<(usize, usize)>,
            time: &mut usize,
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs(y, g, s, dfs_str, nodes, time);
            }
            dfs_str[*time] = s.chars().nth(x).unwrap();
            *time += 1;
            nodes[x].1 = *time;
        }
        
        dfs(0, &g, &s, &mut dfs_str, &mut nodes, &mut time);


        let mut t = String::from("^");
        for c in dfs_str.iter() {
            t.push('#');
            t.push(*c);
        }
        t.push_str("#$");

        let mut half_len: Vec<usize> = vec![0; t.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;

        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = std::cmp::min(half_len[2 * box_m - i], box_r - i);
            }
            while t.chars().nth(i - hl).unwrap() == t.chars().nth(i + hl).unwrap() {
                hl += 1;
                if i + hl > box_r {
                  box_m = i;
                  box_r = i + hl;
                }
            }
            half_len[i] = hl;
        }

        let is_palindrome = |l: usize, r: usize| -> bool { half_len[l + r + 1] > r - l };

        let mut ans: Vec<bool> = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse()?;
    let parent: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s: String = lines.next().unwrap().to_string();

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

    Ok(())
}