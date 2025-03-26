use std::io;

struct Solution;

impl Solution {
    fn construct_transformed_array(a: &[i32]) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0; n];
        
        for i in 0..n {
            // Calculate the new index using the formula (i + A[i] % n + n) % n
            // rem_euclid is used to handle negative numbers consistently
            let new_index = ((i as i32 + a[i].rem_euclid(n as i32) + (n as i32)) % (n as i32)) as usize;
            res[i] = a[new_index];
        }
        
        res
    }
}

fn main() {
    // Input the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse n");
    
    // Input the elements of the array
    let mut a = Vec::with_capacity(n);
    let mut tokens_read = 0;
    
    while tokens_read < n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        for token in input.split_whitespace() {
            if tokens_read >= n {
                break;
            }
            let num: i32 = token.parse().expect("Failed to parse array element");
            a.push(num);
            tokens_read += 1;
        }
    }
    
    // Call the constructTransformedArray function
    let transformed_array = Solution::construct_transformed_array(&a);
    
    // Output the transformed array
    for (i, &num) in transformed_array.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}