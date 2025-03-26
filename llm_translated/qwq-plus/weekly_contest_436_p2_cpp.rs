use std::io;

struct Solution;

impl Solution {
    pub fn assign_elements(mut groups: Vec<i32>, elements: &[i32]) -> Vec<i32> {
        if elements.is_empty() {
            return groups;
        }
        let mx = *elements.iter().max().unwrap();
        let mut target = vec![-1; (mx + 1) as usize];
        for (i, &x) in elements.iter().enumerate() {
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            for y in (x..=mx).step_by(x as usize) {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32;
                }
            }
        }

        for x in &mut groups {
            *x = target[*x as usize];
        }
        groups
    }
}

fn main() {
    let mut tokens = Vec::new();
    for line_result in io::stdin().lines() {
        let line = line_result.unwrap();
        tokens.extend(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );
    }
    let mut iter = tokens.into_iter();
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();
    let groups: Vec<_> = iter.by_ref().take(n as usize).collect();
    let elements: Vec<_> = iter.by_ref().take(m as usize).collect();

    let solution = Solution;
    let result = solution.assign_elements(groups, &elements);

    for num in &result {
        print!("{} ", num);
    }
    println!();
}