use std::io::{self, Write};

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid integer");

    // Read the elements of the array
    let mut A = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please enter a valid integer");
        A.push(num);
    }

    // Call the construct_transformed_array function
    let transformed_array = construct_transformed_array(&A);

    // Output the transformed array
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}

fn construct_transformed_array(A: &[i32]) -> Vec<i32> {
    let n = A.len();
    let mut res = vec![0; n];
    for i in 0..n {
        let index = (i as i32 + A[i] % n as i32 + n as i32) % n as i32;
        res[i] = A[index as usize];
    }
    res
}