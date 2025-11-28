/// Type aliases matching original C types
type Bool = bool;
type UChar = u8;
type Int32 = i32;
type UInt32 = u32;

/// Constants from original C code
const BZ_MAX_ALPHA_SIZE: usize = 258;
const BZ_MAX_CODE_LEN: usize = 23;

/// Helper macros translated to Rust functions
#[inline]
fn weight_of(zz0: UInt32) -> UInt32 {
    zz0 & 0xffffff00
}

#[inline]
fn depth_of(zz1: UInt32) -> UInt32 {
    zz1 & 0x000000ff
}

#[inline]
fn my_max(zz2: UInt32, zz3: UInt32) -> UInt32 {
    if zz2 > zz3 { zz2 } else { zz3 }
}

#[inline]
fn add_weights(zw1: UInt32, zw2: UInt32) -> UInt32 {
    (weight_of(zw1) + weight_of(zw2)) | (1 + my_max(depth_of(zw1), depth_of(zw2)))
}

/// Heap operations for Huffman tree construction
struct HuffmanHeap {
    heap: Vec<Int32>,
    weight: Vec<UInt32>,
    n_heap: usize,
}

impl HuffmanHeap {
    fn new(weight: &[UInt32]) -> Self {
        Self {
            heap: vec![0; BZ_MAX_ALPHA_SIZE + 2],
            weight: weight.to_vec(),
            n_heap: 0,
        }
    }

    fn up_heap(&mut self, mut zz: usize) {
        let tmp = self.heap[zz];
        while zz > 1 && self.weight[tmp as usize] < self.weight[self.heap[zz >> 1] as usize] {
            self.heap[zz] = self.heap[zz >> 1];
            zz >>= 1;
        }
        self.heap[zz] = tmp;
    }

    fn down_heap(&mut self, mut zz: usize) {
        let tmp = self.heap[zz];
        loop {
            let mut yy = zz << 1;
            if yy > self.n_heap {
                break;
            }
            if yy < self.n_heap && self.weight[self.heap[yy + 1] as usize] < self.weight[self.heap[yy] as usize] {
                yy += 1;
            }
            if self.weight[tmp as usize] < self.weight[self.heap[yy] as usize] {
                break;
            }
            self.heap[zz] = self.heap[yy];
            zz = yy;
        }
        self.heap[zz] = tmp;
    }

    fn push(&mut self, val: Int32) {
        self.n_heap += 1;
        self.heap[self.n_heap] = val;
        self.up_heap(self.n_heap);
    }

    fn pop(&mut self) -> Option<Int32> {
        if self.n_heap == 0 {
            return None;
        }
        let result = self.heap[1];
        self.heap[1] = self.heap[self.n_heap];
        self.n_heap -= 1;
        if self.n_heap > 0 {
            self.down_heap(1);
        }
        Some(result)
    }

    fn len(&self) -> usize {
        self.n_heap
    }
}

/// Generates Huffman code lengths for given frequencies
///
/// # Arguments
/// * `len` - Output array for code lengths
/// * `freq` - Input frequency array
/// * `alpha_size` - Alphabet size
/// * `max_len` - Maximum code length allowed
///
/// # Errors
/// Returns an error if memory allocation fails or parameters are invalid
pub fn bz2_hb_make_code_lengths(
    len: &mut [UChar],
    freq: &[Int32],
    alpha_size: Int32,
    max_len: Int32,
) -> Result<(), &'static str> {
    let alpha_size = alpha_size as usize;
    let max_len = max_len as usize;

    if alpha_size == 0 || alpha_size > BZ_MAX_ALPHA_SIZE {
        return Err("Invalid alpha_size");
    }
    if len.len() < alpha_size {
        return Err("len array too small");
    }
    if freq.len() < alpha_size {
        return Err("freq array too small");
    }

    let mut _n_nodes = alpha_size;
    let mut too_long;

    // Allocate arrays on stack with reasonable bounds
    let mut weight = [0; BZ_MAX_ALPHA_SIZE * 2];
    let mut parent = [-2; BZ_MAX_ALPHA_SIZE * 2];

    // Initialize weights
    for i in 0..alpha_size {
        weight[i + 1] = if freq[i] == 0 { 1 } else { freq[i] as UInt32 } << 8;
    }

    loop {
        let mut n_nodes = alpha_size;
        
        // Initialize parent array for leaves
        for i in 1..=alpha_size {
            parent[i] = -1;
        }

        // Create HuffmanHeap with current weights
        let mut hheap = HuffmanHeap::new(&weight);

        // Initialize heap with leaves
        for i in 1..=alpha_size {
            hheap.push(i as Int32);
        }

        // Build Huffman tree
        while hheap.len() > 1 {
            let n1 = hheap.pop().ok_or("Heap underflow")?;
            let n2 = hheap.pop().ok_or("Heap underflow")?;
            
            n_nodes += 1;
            if n_nodes >= BZ_MAX_ALPHA_SIZE * 2 {
                return Err("Too many nodes in Huffman tree");
            }

            parent[n1 as usize] = n_nodes as Int32;
            parent[n2 as usize] = n_nodes as Int32;
            
            // Calculate weight for new node
            weight[n_nodes] = add_weights(weight[n1 as usize], weight[n2 as usize]);
            parent[n_nodes] = -1;
            
            // Push new node back to heap
            hheap.push(n_nodes as Int32);
        }

        // Calculate code lengths
        too_long = false;
        for i in 1..=alpha_size {
            let mut j = 0;
            let mut k = i;
            while parent[k] >= 0 {
                k = parent[k] as usize;
                j += 1;
            }
            len[i - 1] = j as UChar;
            if j > max_len {
                too_long = true;
            }
        }

        if !too_long {
            break;
        }

        // Scale frequencies and try again if codes are too long
        for i in 1..=alpha_size {
            let j = (weight[i] >> 8) as Int32;
            let j = 1 + (j / 2);
            weight[i] = (j as UInt32) << 8;
        }
    }

    Ok(())
}

/// Assigns canonical Huffman codes based on code lengths
///
/// # Arguments
/// * `code` - Output array for Huffman codes
/// * `length` - Input array of code lengths
/// * `min_len` - Minimum code length
/// * `max_len` - Maximum code length
/// * `alpha_size` - Alphabet size
pub fn bz2_hb_assign_codes(
    code: &mut [Int32],
    length: &[UChar],
    min_len: Int32,
    max_len: Int32,
    alpha_size: Int32,
) -> Result<(), &'static str> {
    let alpha_size = alpha_size as usize;

    if code.len() < alpha_size || length.len() < alpha_size {
        return Err("Array size mismatch");
    }

    let mut vec = 0;
    for n in min_len..=max_len {
        for i in 0..alpha_size {
            if length[i] == n as UChar {
                code[i] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }

    Ok(())
}

/// Creates Huffman decoding tables
///
/// # Arguments
/// * `limit` - Output limit table
/// * `base` - Output base table
/// * `perm` - Output permutation table
/// * `length` - Input code lengths
/// * `min_len` - Minimum code length
/// * `max_len` - Maximum code length
/// * `alpha_size` - Alphabet size
pub fn bz2_hb_create_decode_tables(
    limit: &mut [Int32],
    base: &mut [Int32],
    perm: &mut [Int32],
    length: &[UChar],
    min_len: Int32,
    max_len: Int32,
    alpha_size: Int32,
) -> Result<(), &'static str> {
    let alpha_size = alpha_size as usize;
    let min_len = min_len as usize;
    let max_len = max_len as usize;

    if limit.len() <= max_len || base.len() <= max_len || perm.len() < alpha_size {
        return Err("Array size mismatch");
    }

    // Generate permutation of symbols sorted by code length
    let mut pp = 0;
    for i in min_len..=max_len {
        for j in 0..alpha_size {
            if length[j] == i as UChar {
                perm[pp] = j as Int32;
                pp += 1;
            }
        }
    }

    // Initialize base table
    for i in 0..BZ_MAX_CODE_LEN {
        base[i] = 0;
    }
    for i in 0..alpha_size {
        base[length[i] as usize + 1] += 1;
    }

    // Compute cumulative base values
    for i in 1..BZ_MAX_CODE_LEN {
        base[i] += base[i - 1];
    }

    // Initialize limit table
    for i in 0..BZ_MAX_CODE_LEN {
        limit[i] = 0;
    }

    // Compute limit and base values for decoding
    let mut vec = 0;
    for i in min_len..=max_len {
        vec += base[i + 1] - base[i];
        limit[i] = vec - 1;
        vec <<= 1;
    }

    for i in min_len + 1..=max_len {
        base[i] = ((limit[i - 1] + 1) << 1) - base[i];
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_code_lengths() {
        let mut len = [0u8; 10];
        let freq = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        assert!(bz2_hb_make_code_lengths(&mut len, &freq, 10, 20).is_ok());
        
        // Verify all lengths are positive
        for &l in &len {
            assert!(l > 0);
        }
    }

    #[test]
    fn test_assign_codes() {
        let length = [2, 2, 3, 3, 3, 3];
        let mut code = [0; 6];
        
        assert!(bz2_hb_assign_codes(&mut code, &length, 2, 3, 6).is_ok());
        
        // Verify codes are assigned in canonical order
        assert_eq!(code[0], 0); // 00
        assert_eq!(code[1], 1); // 01
        // Next codes should be 100, 101, 110, 111
    }

    #[test]
    fn test_decode_tables() {
        let length = [2, 2, 3, 3, 3, 3];
        let mut limit = [0; BZ_MAX_CODE_LEN];
        let mut base = [0; BZ_MAX_CODE_LEN];
        let mut perm = [0; 6];
        
        assert!(bz2_hb_create_decode_tables(
            &mut limit, &mut base, &mut perm, &length, 2, 3, 6
        ).is_ok());
        
        // Verify permutation is correct
        assert_eq!(perm[0], 0);
        assert_eq!(perm[1], 1);
    }

    #[test]
    fn test_heap_operations() {
        let weight = [0, 10, 5, 15, 3, 8];
        let mut hheap = HuffmanHeap::new(&weight);
        
        hheap.push(1);
        hheap.push(2);
        hheap.push(3);
        hheap.push(4);
        hheap.push(5);
        
        // Should pop in order of increasing weight
        assert_eq!(hheap.pop(), Some(4)); // weight 3
        assert_eq!(hheap.pop(), Some(2)); // weight 5
        assert_eq!(hheap.pop(), Some(5)); // weight 8
        assert_eq!(hheap.pop(), Some(1)); // weight 10
        assert_eq!(hheap.pop(), Some(3)); // weight 15
        assert_eq!(hheap.pop(), None);
    }
}