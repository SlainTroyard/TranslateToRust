use std::io::{self, BufRead, Write};

fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = a[((i + (a[i] % n as i32) + n as i32) % n as i32) as usize];
    }
    res
}

fn main() {
    // Read the size of the array from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the array from stdin
    let mut a = vec![0; n];
    for i in 0..n {
        input.clear(); // Clear the buffer before reading the next line
        io::stdin().read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Please type a number!");
    }

    // Call the construct_transformed_array function
    let transformed_array = construct_transformed_array(&a);

    // Output the transformed array
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}