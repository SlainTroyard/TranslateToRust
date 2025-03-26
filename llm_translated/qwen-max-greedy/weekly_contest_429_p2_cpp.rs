use std::io;
use std::collections::BTreeSet;

fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
    let mut prev = i32::MIN;
    let mut c = BTreeSet::new();
    arr.sort_unstable();

    for &x in arr.iter() {
        let x_max = prev + 1;
        let x_min = x - diff;
        let candidate = x_max.max(x_min);

        if candidate <= x + diff {
            c.insert(candidate);
            prev = candidate;
        }
    }

    c.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let diff: i32 = input.trim().parse().expect("Please type a number!");

    let mut arr = vec![0; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please type a number!");
    }

    let result = max_distinct_elements(&mut arr, diff);

    println!("{}", result);
}