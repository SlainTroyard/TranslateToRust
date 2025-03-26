use std::io;

struct Solution;

impl Solution {
    fn assign_elements(mut groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        if elements.is_empty() {
            return groups;
        }
        let mx = *elements.iter().max().unwrap();
        let mut target = vec![-1; (mx + 1) as usize];
        
        for (i, &x) in elements.iter().enumerate() {
            if x <= 0 || x > mx || target[x as usize] != -1 {
                continue;
            }
            let step = x as usize;
            for y in (x..=mx).step_by(step) {
                if target[y as usize] == -1 {
                    target[y as usize] = i as i32;
                }
            }
        }
        
        for x in groups.iter_mut() {
            if *x >= 0 && *x <= mx {
                *x = target[*x as usize];
            }
        }
        
        groups
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    let n = tokens[0] as usize;
    let m = tokens[1] as usize;
    
    let groups = tokens[2..2 + n].to_vec();
    let elements = tokens[2 + n .. 2 + n + m].to_vec();
    
    let solution = Solution;
    let result = solution.assign_elements(groups, elements);
    
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")); 
}