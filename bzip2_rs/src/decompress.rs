// Constants matching original bzip2
const BZ_MAX_ALPHA_SIZE: usize = 258;
const BZ_N_GROUPS: usize = 6;
const BZ_G_SIZE: usize = 50;
const MTFA_SIZE: usize = 4096;
const MTFL_SIZE: usize = 16;
const BZ_N_OVERSHOOT: usize = 4; // Properly defined for BWT safety margin

// Error types matching original bzip2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BzError {
    Ok,
    DataError,
    DataErrorMagic,
    MemError,
    StreamEnd,
}

impl std::fmt::Display for BzError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BzError::Ok => write!(f, "OK"),
            BzError::DataError => write!(f, "DATA_ERROR"),
            BzError::DataErrorMagic => write!(f, "DATA_ERROR_MAGIC"),
            BzError::MemError => write!(f, "MEM_ERROR"),
            BzError::StreamEnd => write!(f, "STREAM_END"),
        }
    }
}

impl std::error::Error for BzError {}

// Enable the ? operator for BzError using stable features
impl From<BzError> for Result<(), BzError> {
    fn from(err: BzError) -> Self {
        match err {
            BzError::Ok => Ok(()),
            other => Err(other),
        }
    }
}

// Type aliases matching original C types
pub type Bool = bool;
pub type UChar = u8;
pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;

// Decompression state structure
pub struct DState {
    // Bit stream state
    bs_buff: UInt32,
    bs_live: Int32,
    
    // Block information
    block_size_100k: Int32,
    small_decompress: Bool,
    curr_block_no: Int32,
    verbosity: Int32,
    
    // Burrows-Wheeler transform state
    orig_ptr: Int32,
    t_pos: UInt32,
    k0: Int32,
    unzftab: [Int32; 256],
    nblock_used: Int32,
    cftab: [Int32; 257],
    cftab_copy: [Int32; 257],
    
    // Storage for BWT data
    tt: Option<Vec<UInt32>>,
    ll16: Option<Vec<UInt16>>,
    ll4: Option<Vec<UChar>>,
    
    // CRCs
    stored_block_crc: UInt32,
    stored_combined_crc: UInt32,
    calculated_block_crc: UInt32,
    calculated_combined_crc: UInt32,
    
    // Byte usage maps
    n_in_use: Int32,
    in_use: [Bool; 256],
    in_use16: [Bool; 16],
    seq_to_unseq: [UChar; 256],
    
    // MTF (Move-To-Front) state
    mtfa: [UChar; MTFA_SIZE],
    mtfbase: [Int32; 256 / MTFL_SIZE],
    selector: [UChar; BZ_MAX_ALPHA_SIZE],
    selector_mtf: [UChar; BZ_MAX_ALPHA_SIZE],
    len: [[UChar; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    
    // Huffman decoding tables
    limit: [[Int32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    base: [[Int32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    perm: [[Int32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    min_lens: [Int32; BZ_N_GROUPS],
    
    // Save area for decompression state
    save_i: Int32,
    save_j: Int32,
    save_t: Int32,
    save_alpha_size: Int32,
    save_n_groups: Int32,
    save_n_selectors: Int32,
    save_eob: Int32,
    save_group_no: Int32,
    save_group_pos: Int32,
    save_next_sym: Int32,
    save_nblock_max: Int32,
    save_nblock: Int32,
    save_es: Int32,
    save_n: Int32,
    save_curr: Int32,
    save_zt: Int32,
    save_zn: Int32,
    save_zvec: Int32,
    save_zj: Int32,
    save_g_sel: Int32,
    save_g_minlen: Int32,
}

impl Default for DState {
    fn default() -> Self {
        Self {
            bs_buff: 0,
            bs_live: 0,
            block_size_100k: 0,
            small_decompress: false,
            curr_block_no: 0,
            verbosity: 0,
            orig_ptr: 0,
            t_pos: 0,
            k0: 0,
            unzftab: [0; 256],
            nblock_used: 0,
            cftab: [0; 257],
            cftab_copy: [0; 257],
            tt: None,
            ll16: None,
            ll4: None,
            stored_block_crc: 0,
            stored_combined_crc: 0,
            calculated_block_crc: 0,
            calculated_combined_crc: 0,
            n_in_use: 0,
            in_use: [false; 256],
            in_use16: [false; 16],
            seq_to_unseq: [0; 256],
            mtfa: [0; MTFA_SIZE],
            mtfbase: [0; 256 / MTFL_SIZE],
            selector: [0; BZ_MAX_ALPHA_SIZE],
            selector_mtf: [0; BZ_MAX_ALPHA_SIZE],
            len: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
            limit: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
            base: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
            perm: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
            min_lens: [0; BZ_N_GROUPS],
            save_i: 0,
            save_j: 0,
            save_t: 0,
            save_alpha_size: 0,
            save_n_groups: 0,
            save_n_selectors: 0,
            save_eob: 0,
            save_group_no: 0,
            save_group_pos: 0,
            save_next_sym: 0,
            save_nblock_max: 0,
            save_nblock: 0,
            save_es: 0,
            save_n: 0,
            save_curr: 0,
            save_zt: 0,
            save_zn: 0,
            save_zvec: 0,
            save_zj: 0,
            save_g_sel: 0,
            save_g_minlen: 0,
        }
    }
}

impl DState {
    /// Creates maps from byte usage information
    pub fn make_maps_d(&mut self) {
        self.n_in_use = 0;
        for i in 0..256 {
            if self.in_use[i] {
                self.seq_to_unseq[self.n_in_use as usize] = i as UChar;
                self.n_in_use += 1;
            }
        }
    }

    /// Gets bits from the bit stream
    pub fn get_bits(&mut self, n: usize, input: &mut impl Iterator<Item=u8>) -> Result<UInt32, BzError> {
        if n == 0 {
            return Ok(0);
        }
        if n > 32 {
            return Err(BzError::DataError);
        }
        
        let v;
        
        while self.bs_live < n as Int32 {
            let byte = match input.next() {
                Some(b) => b,
                None => return Err(BzError::DataError), // Unexpected end of input
            };
            
            self.bs_buff = (self.bs_buff << 8) | (byte as UInt32);
            self.bs_live += 8;
        }
        
        v = (self.bs_buff >> (self.bs_live - n as Int32)) & ((1 << n) - 1);
        self.bs_live -= n as Int32;
        
        Ok(v)
    }

    /// Gets a single bit from the bit stream
    pub fn get_bit(&mut self, input: &mut impl Iterator<Item=u8>) -> Result<UInt32, BzError> {
        self.get_bits(1, input)
    }

    /// Gets a byte from the bit stream
    pub fn get_uchar(&mut self, input: &mut impl Iterator<Item=u8>) -> Result<UChar, BzError> {
        self.get_bits(8, input).map(|v| v as UChar)
    }

    /// Gets MTF (Move-To-Front) value
    pub fn get_mtf_val(
        &mut self, 
        input: &mut impl Iterator<Item=u8>,
        group_no: &mut Int32,
        group_pos: &mut Int32,
        _n_selectors: Int32,
    ) -> Result<Int32, BzError> {
        if *group_pos == 0 {
            *group_no += 1;
            if *group_no >= _n_selectors {
                return Err(BzError::DataError);
            }
            *group_pos = BZ_G_SIZE as Int32;
            
            let _g_sel = self.selector[*group_no as usize] as Int32;
            let _g_minlen = self.min_lens[_g_sel as usize];
            
            // These would be set to appropriate slices in full implementation
            let _g_limit = &self.limit[_g_sel as usize];
            let _g_perm = &self.perm[_g_sel as usize];
            let _g_base = &self.base[_g_sel as usize];
        }
        
        *group_pos -= 1;
        
        // Simplified MTF value decoding
        // In full implementation, this would decode Huffman codes
        let zn = 8; // Example minimum code length
        let zvec = self.get_bits(zn, input)?;
        
        // Placeholder for full Huffman decoding logic
        let lval = zvec as Int32;
        
        Ok(lval)
    }
}

/// Main decompression function
pub fn bz2_decompress(
    s: &mut DState, 
    input: &[u8],
    _output: &mut Vec<u8>,
) -> Result<(), BzError> {
    let mut input_iter = input.iter().copied();
    let mut ret_val = BzError::Ok;

    // Restore state from save area
    let i = s.save_i;
    let j = s.save_j;
    let t = s.save_t;
    let alpha_size = s.save_alpha_size;
    let n_groups = s.save_n_groups;
    let n_selectors = s.save_n_selectors;
    let eob = s.save_eob;
    let group_no = s.save_group_no;
    let group_pos = s.save_group_pos;
    let next_sym = s.save_next_sym;
    let nblock_max = s.save_nblock_max;
    let nblock = s.save_nblock;
    let es = s.save_es;
    let n = s.save_n;
    let curr = s.save_curr;
    let zt = s.save_zt;
    let zn = s.save_zn;
    let zvec = s.save_zvec;
    let zj = s.save_zj;
    let _g_sel = s.save_g_sel;
    let _g_minlen = s.save_g_minlen;

    // Main decompression loop
    'decompress: loop {
        // Check magic header
        let uc = s.get_uchar(&mut input_iter)?;
        if uc != 0x42 { // 'B'
            ret_val = BzError::DataErrorMagic;
            break 'decompress;
        }

        let uc = s.get_uchar(&mut input_iter)?;
        if uc != 0x5a { // 'Z'
            ret_val = BzError::DataErrorMagic;
            break 'decompress;
        }

        let uc = s.get_uchar(&mut input_iter)?;
        if uc != 0x68 { // 'h'
            ret_val = BzError::DataErrorMagic;
            break 'decompress;
        }

        // Get block size
        let block_size_100k = s.get_bits(8, &mut input_iter)? as Int32;
        if block_size_100k < 1 || block_size_100k > 9 {
            ret_val = BzError::DataErrorMagic;
            break 'decompress;
        }
        s.block_size_100k = block_size_100k;

        // Allocate memory for BWT data
        if s.small_decompress {
            let size = (s.block_size_100k * 100000) as usize;
            s.ll16 = Some(vec![0; size]);
            s.ll4 = Some(vec![0; (size + 1) >> 1]);
        } else {
            let size = (s.block_size_100k * 100000) as usize;
            s.tt = Some(vec![0; size]);
        }

        // Read block header
        let uc = s.get_uchar(&mut input_iter)?;
        if uc == 0x17 {
            // End of stream marker
            break 'decompress;
        }
        if uc != 0x31 {
            ret_val = BzError::DataError;
            break 'decompress;
        }

        // Continue with block processing...
        // This is a simplified version - full implementation would continue
        // decoding the block structure, Huffman tables, and MTF data
        
        // For now, we'll break after basic header validation
        break 'decompress;
    }

    // Save state before returning
    s.save_i = i;
    s.save_j = j;
    s.save_t = t;
    s.save_alpha_size = alpha_size;
    s.save_n_groups = n_groups;
    s.save_n_selectors = n_selectors;
    s.save_eob = eob;
    s.save_group_no = group_no;
    s.save_group_pos = group_pos;
    s.save_next_sym = next_sym;
    s.save_nblock_max = nblock_max;
    s.save_nblock = nblock;
    s.save_es = es;
    s.save_n = n;
    s.save_curr = curr;
    s.save_zt = zt;
    s.save_zn = zn;
    s.save_zvec = zvec;
    s.save_zj = zj;
    s.save_g_sel = _g_sel;
    s.save_g_minlen = _g_minlen;

    if ret_val != BzError::Ok {
        return Err(ret_val);
    }
    Ok(())
}

/// Simple decompression interface
pub fn decompress(input: &[u8]) -> Result<Vec<u8>, BzError> {
    let mut state = DState::default();
    let mut output = Vec::new();
    
    bz2_decompress(&mut state, input, &mut output)?;
    
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_maps() {
        let mut state = DState::default();
        state.in_use[65] = true; // 'A'
        state.in_use[66] = true; // 'B'
        
        state.make_maps_d();
        
        assert_eq!(state.n_in_use, 2);
        assert_eq!(state.seq_to_unseq[0], 65);
        assert_eq!(state.seq_to_unseq[1], 66);
    }

    #[test]
    fn test_get_bits() {
        let mut state = DState::default();
        let input = [0b10101010, 0b11001100];
        let mut iter = input.iter().copied();
        
        // Get first 4 bits
        let result = state.get_bits(4, &mut iter).unwrap();
        assert_eq!(result, 0b1010);
        assert_eq!(state.bs_live, 4);
        
        // Get next 8 bits (4 remaining from first byte + 4 from second)
        let result = state.get_bits(8, &mut iter).unwrap();
        assert_eq!(result, 0b10101100);
    }
}