```rust
use std::io::{self, Write};
use std::cmp::min;


// Constants defined for modular arithmetic and the alphabet size (26 letters)
const MODULO_VAL: i64 = 1_000_000_007;
const RADIX_VAL: usize = 26;

// Matrix struct to represent a 26x26 matrix
#[derive(Clone)]
struct Matrix {
    m: [[i64; RADIX_VAL]; RADIX_VAL],
}

// Function to initialize the empty Matrix 
impl Matrix {
    fn new_empty()-> Self{
        Self { m: [[0usize64,RADIX_VALRADIX_VAL);}
