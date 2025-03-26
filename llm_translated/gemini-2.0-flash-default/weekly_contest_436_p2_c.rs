use std::io;
use std::cmp::min;
use std::f64;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = min(index[element as usize], i as i32);
    }

    let mut ans = vec![0; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        ans[i] = i32::MAX;
        let sqrt_group = (group as f64).sqrt() as i32;
        for j in 1..=sqrt_group {
            if group % j == 0 {
                if index[j as usize] != i32::MAX || index[(group / j) as usize] != i32::MAX {
                    ans[i] = min(ans[i], min(index[(group / j) as usize], index[j as usize]));
                }
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut groups = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        groups.push(input.trim().parse::<i32>().unwrap());
    }

    let mut elements = Vec::with_capacity(m);
    for _ in 0..m {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        elements.push(input.trim().parse::<i32>().unwrap());
    }

    let ans = assign_elements(&groups, &elements);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();
}