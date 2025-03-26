use std::cmp::min;
use std::io;
use std::io::Read;

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
                let other = group / j;
                if index[j as usize] != i32::MAX || index[other as usize] != i32::MAX {
                    ans[i] = min(ans[i], min(index[other as usize], index[j as usize]));
                }
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n = first_split.next().unwrap().parse::<usize>().unwrap();
    let m = first_split.next().unwrap().parse::<usize>().unwrap();

    let mut groups = Vec::new();
    let second_line = lines.next().unwrap();
    let groups_str: Vec<&str> = second_line.split_whitespace().collect();
    for i in 0..n {
        groups.push(groups_str[i].parse::<i32>().unwrap());
    }

    let mut elements = Vec::new();
    let third_line = lines.next().unwrap();
    let elements_str: Vec<&str> = third_line.split_whitespace().collect();
    for i in 0..m {
        elements.push(elements_str[i].parse::<i32>().unwrap());
    }

    let ans = assign_elements(&groups, &elements);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}