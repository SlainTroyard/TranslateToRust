use std::io::{self, BufRead, Write};

/// Constructs the transformed array based on the problem logic.
/// 
/// For each index i:
/// - If nums[i] is 0, the result is 0.
/// - Otherwise, compute the target index as (i + nums[i]) modulo nums.len(),
///   handling negative wrapping appropriately, and assign result[i] = nums[target_index].
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        let current = nums[i];
        if current == 0 {
            // If the value is 0, result[i] remains 0
            result[i] = current;
        } else {
            // Compute the effective target index with wrapping
            // Using isize arithmetic to cover negative steps
            let steps = current as isize;
            let index = i as isize;
            let n = nums_size as isize;
            // Compute modulo properly for possibly negative numbers
            let mut target_index = (index + steps) % n;
            if target_index < 0 {
                target_index += n;
            }
            result[i] = nums[target_index as usize];
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up stdin reader and buffered output writer
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Read the complete input into a string.
    // This is to handle inputs that may span multiple lines.
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // Split by whitespace into tokens
    let mut tokens = input.split_whitespace();

    // Read the size of the array from the first token
    let nums_size: usize = tokens
        .next()
        .ok_or("Missing numsSize")?
        .parse()
        .map_err(|_| "Failed to parse numsSize")?;

    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = tokens
            .next()
            .ok_or("Missing an array element")?
            .parse()
            .map_err(|_| "Failed to parse an array element")?;
        nums.push(num);
    }

    // Compute the transformed array
    let transformed_array = construct_transformed_array(&nums);

    // Print each element of the transformed array followed by a space.
    // The final output also prints a newline at the end.
    for num in transformed_array {
        write!(writer, "{} ", num)?;
    }
    writeln!(writer)?;

    // Flush to ensure all output is written.
    writer.flush()?;
    Ok(())
}