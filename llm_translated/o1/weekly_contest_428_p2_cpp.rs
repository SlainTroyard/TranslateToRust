use std::collections::HashMap;
use std::io::{self, BufRead};

/// A structure to replicate the logic from the C++ `Solution` class
struct Solution;

impl Solution {
    /// Performs the Bellman-like relaxation steps over the provided pairs and rates.
    /// This function updates the `best` HashMap in-place.
    fn bellman(
        &self,
        best: &mut HashMap<String, f64>,
        pairs: &[(String, String)],
        rates: &[f64],
    ) {
        // Perform as many relaxation iterations as there are pairs (just like in the C++ code)
        for _relax_iteration in 0..pairs.len() {
            // Go through all pairs and try to improve the best conversions
            for (i, (from, to)) in pairs.iter().enumerate() {
                let rate = rates[i];

                // Get the old values; default to 0.0 if not present
                let old_from_val = *best.get(from).unwrap_or(&0.0);
                let old_to_val = *best.get(to).unwrap_or(&0.0);

                // Relaxation step 1:
                // best[to] = max(best[to], best[from] * rate)
                let new_to_val = old_to_val.max(old_from_val * rate);
                best.insert(to.clone(), new_to_val);

                // Relaxation step 2:
                // best[from] = max(best[from], best[to] / rate)
                let updated_to_val = *best.get(to).unwrap_or(&0.0);
                let new_from_val = old_from_val.max(updated_to_val / rate);
                best.insert(from.clone(), new_from_val);
            }
        }
    }

    /// Computes the maximum amount of the initial currency that can be obtained
    /// after going through two sets of exchange pairs and rates.
    fn max_amount(
        &self,
        initial_currency: &str,
        pairs1: &[(String, String)],
        rates1: &[f64],
        pairs2: &[(String, String)],
        rates2: &[f64],
    ) -> f64 {
        // Initialize the best map with the initial currency set to 1.0
        let mut best: HashMap<String, f64> = HashMap::new();
        best.insert(initial_currency.to_string(), 1.0);

        // First relax using pairs1
        self.bellman(&mut best, pairs1, rates1);
        // Then relax using pairs2
        self.bellman(&mut best, pairs2, rates2);

        // Return the final value for the initial currency
        *best.get(initial_currency).unwrap_or(&0.0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the initial currency string
    let initial_currency = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing initial currency"))??;
    let initial_currency = initial_currency.trim().to_string();

    // Read the integer n1
    let n1 = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing n1"))??;
    let n1: usize = n1.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Failed to parse n1 as an integer")
    })?;

    // Read n1 lines of data: each contains two currencies and a rate
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing pair1 data line"))??;
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Insufficient data in a pairs1 line",
            ));
        }
        let from = tokens[0].to_string();
        let to = tokens[1].to_string();
        let rate: f64 = tokens[2].parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Failed to parse rate in pairs1")
        })?;
        pairs1.push((from, to));
        rates1.push(rate);
    }

    // Read the integer n2
    let n2 = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing n2"))??;
    let n2: usize = n2.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Failed to parse n2 as an integer")
    })?;

    // Read n2 lines of data: each contains two currencies and a rate
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing pair2 data line"))??;
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Insufficient data in a pairs2 line",
            ));
        }
        let from = tokens[0].to_string();
        let to = tokens[1].to_string();
        let rate: f64 = tokens[2].parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Failed to parse rate in pairs2")
        })?;
        pairs2.push((from, to));
        rates2.push(rate);
    }

    // Create a `Solution` instance and compute the result
    let solution = Solution;
    let result = solution.max_amount(
        &initial_currency,
        &pairs1,
        &rates1,
        &pairs2,
        &rates2,
    );

    // Output the result with 5 decimal places
    println!("{:.5}", result);

    Ok(())
}