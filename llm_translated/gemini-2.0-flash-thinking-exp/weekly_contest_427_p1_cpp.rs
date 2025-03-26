fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
    for i in 0..n {
        let index = (i as i32 + a[i] % n as i32 + n as i32) % n as i32;
        res[i] = a[index as usize];
    }
    res
}

fn main() {
    use std::io;

    // Input the size of the array
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    // Input the elements of the array
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).unwrap();
    let a: Vec<i32> = a_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the constructTransformedArray function
    let transformed_array = construct_transformed_array(&a);

    // Output the transformed array
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}