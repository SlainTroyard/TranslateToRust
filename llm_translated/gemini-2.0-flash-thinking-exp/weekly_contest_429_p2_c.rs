fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::min_value();
    let mut distinct_count = 0;

    arr.sort();

    for &val in arr.iter() {
        let x = std::cmp::max(prev + 1, val - diff);

        if x <= val + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    use std::io;

    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut diff_str = String::new();
    io::stdin().read_line(&mut diff_str).expect("Failed to read line");
    let diff: i32 = diff_str.trim().parse().expect("Invalid input for diff");

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for array element");
        arr.push(num);
    }

    let result = max_distinct_elements(&mut arr, diff);
    println!("{}", result);
}