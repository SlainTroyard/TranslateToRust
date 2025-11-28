use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Write, BufWriter};

/// Generates a large file with repetitive patterns for testing bzip2 compression
/// of files larger than 2GB. The output consists of alternating sequences of
/// 'a', 'b', and 'c' characters with randomized lengths to create compressible
/// but non-trivial data.
///
/// # Errors
/// Returns `io::Error` if writing to stdout fails.
pub fn generate_large_test_file() -> io::Result<()> {
    const MEGABYTES: usize = 5000;
    const N_BUF: usize = 1_000_000;
    
    // Initialize deterministic random number generator with fixed seed
    let mut rng = DeterministicRng::new(1);
    
    let stdout = io::stdout();
    let mut writer = BufWriter::with_capacity(N_BUF, stdout.lock());
    
    // Generate approximately MEGABYTES of compressible data
    // Original C code uses 515 as a scaling factor to achieve roughly the desired size
    for _ in 0..(MEGABYTES * 515) / 3 {
        // Generate random pattern length between 25 and 74
        let p = 25 + (rng.next_u32() % 50) as usize;
        
        // Write sequences of 'a', 'b', and 'c' characters
        write_repeated(&mut writer, b'a', p)?;
        write_repeated(&mut writer, b'b', p - 1)?;
        write_repeated(&mut writer, b'c', p + 1)?;
    }
    
    writer.flush()?;
    Ok(())
}

/// Writes a character repeated `count` times to the output writer.
///
/// # Arguments
/// * `writer` - Output writer
/// * `ch` - Character to repeat
/// * `count` - Number of repetitions
///
/// # Errors
/// Returns `io::Error` if writing fails.
fn write_repeated<W: Write>(writer: &mut W, ch: u8, count: usize) -> io::Result<()> {
    const CHUNK_SIZE: usize = 8192;
    let chunk = [ch; CHUNK_SIZE];
    
    let full_chunks = count / CHUNK_SIZE;
    let remainder = count % CHUNK_SIZE;
    
    for _ in 0..full_chunks {
        writer.write_all(&chunk)?;
    }
    
    if remainder > 0 {
        writer.write_all(&chunk[..remainder])?;
    }
    
    Ok(())
}

/// Simple deterministic RNG for reproducible test data generation
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next_u32(&mut self) -> u32 {
        let mut hasher = DefaultHasher::new();
        self.state.hash(&mut hasher);
        self.state = hasher.finish();
        (self.state >> 32) as u32
    }
}

fn main() -> io::Result<()> {
    generate_large_test_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_repeated() {
        let mut output = Vec::new();
        write_repeated(&mut output, b'x', 5).unwrap();
        assert_eq!(output, b"xxxxx");
    }

    #[test]
    fn test_write_repeated_large() {
        let mut output = Vec::new();
        write_repeated(&mut output, b'y', 10000).unwrap();
        assert_eq!(output.len(), 10000);
        assert!(output.iter().all(|&b| b == b'y'));
    }
}