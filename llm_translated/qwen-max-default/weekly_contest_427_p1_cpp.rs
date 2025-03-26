// Problem: Weekly Contest 427 Problem 1
use std::io;

fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = a[((i + (a[i] as usize) % n + n) % n)];
    }
    res
}

fn main() {
    // Input the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Input the elements of the array
    let mut a = vec![0; n];
    for i in 0..n {
        input.clear();
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