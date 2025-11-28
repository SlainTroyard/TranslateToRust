use std::alloc::{alloc, Layout};
use std::fmt;
use std::ptr;

// Error types matching original bzip2 error codes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BzError {
    Ok = 0,
    SequenceError = -1,
    ParamError = -2,
    MemError = -3,
    DataError = -4,
    DataErrorMagic = -5,
    IoError = -6,
    UnexpectedEof = -7,
    OutbuffFull = -8,
    ConfigError = -9,
}

impl fmt::Display for BzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BzError::Ok => write!(f, "OK"),
            BzError::SequenceError => write!(f, "SEQUENCE_ERROR"),
            BzError::ParamError => write!(f, "PARAM_ERROR"),
            BzError::MemError => write!(f, "MEM_ERROR"),
            BzError::DataError => write!(f, "DATA_ERROR"),
            BzError::DataErrorMagic => write!(f, "DATA_ERROR_MAGIC"),
            BzError::IoError => write!(f, "IO_ERROR"),
            BzError::UnexpectedEof => write!(f, "UNEXPECTED_EOF"),
            BzError::OutbuffFull => write!(f, "OUTBUFF_FULL"),
            BzError::ConfigError => write!(f, "CONFIG_ERROR"),
        }
    }
}

impl std::error::Error for BzError {}

// Type aliases matching original C types
type Char = i8;
type Bool = bool;
type UChar = u8;
type Int32 = i32;
type UInt32 = u32;
type Int16 = i16;
type UInt16 = u16;

// Constants from original C code
const BZ_MAX_ALPHA_SIZE: usize = 258;
const BZ_MAX_CODE_LEN: usize = 23;
const BZ_N_GROUPS: usize = 6;
const BZ_G_SIZE: usize = 50;
const BZ_N_ITERS: usize = 4;
const BZ_MAX_UNUSED: usize = 5000;

// Stream states
#[derive(Debug, Clone, Copy, PartialEq)]
enum BzMode {
    Idle = 1,
    Running = 2,
    Flushing = 3,
    Finishing = 4,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BzState {
    Output = 1,
    Input = 2,
}

// Action types for compression
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BzAction {
    Run = 0,
    Flush = 1,
    Finish = 2,
}

// Main compression stream structure
pub struct BzStream {
    next_in: *const u8,
    avail_in: usize,
    total_in_lo32: u32,
    total_in_hi32: u32,

    next_out: *mut u8,
    avail_out: usize,
    total_out_lo32: u32,
    total_out_hi32: u32,

    state: Option<Box<EState>>,
    
    bzalloc: Option<fn(usize, usize) -> *mut u8>,
    bzfree: Option<fn(*mut u8)>,
    opaque: *mut std::ffi::c_void,
}

impl Default for BzStream {
    fn default() -> Self {
        Self {
            next_in: ptr::null(),
            avail_in: 0,
            total_in_lo32: 0,
            total_in_hi32: 0,
            next_out: ptr::null_mut(),
            avail_out: 0,
            total_out_lo32: 0,
            total_out_hi32: 0,
            state: None,
            bzalloc: None,
            bzfree: None,
            opaque: ptr::null_mut(),
        }
    }
}

// Compression state (EState equivalent)
struct EState {
    strm: *mut BzStream,
    mode: BzMode,
    state: BzState,
    avail_in_expect: u32,
    
    // Memory blocks
    arr1: Option<Box<[UInt32]>>,
    arr2: Option<Box<[UInt32]>>,
    ftab: Option<Box<[UInt32]>>,
    
    work_factor: Int32,
    
    // Run-length encoding state
    state_in_ch: UInt32,
    state_in_len: Int32,
    
    // Input/output limits
    nblock: Int32,
    nblock_max: Int32,
    num_z: Int32,
    state_out_pos: Int32,
    
    // Byte usage map
    in_use: [Bool; 256],
    
    // Bit stream
    bs_buff: UInt32,
    bs_live: Int32,
    
    // CRCs
    block_crc: UInt32,
    combined_crc: UInt32,
    
    // Administrative
    verbosity: Int32,
    block_no: Int32,
    block_size_100k: Int32,
}

impl EState {
    fn new() -> Result<Self, BzError> {
        Ok(Self {
            strm: ptr::null_mut(),
            mode: BzMode::Idle,
            state: BzState::Input,
            avail_in_expect: 0,
            arr1: None,
            arr2: None,
            ftab: None,
            work_factor: 0,
            state_in_ch: 256,
            state_in_len: 0,
            nblock: 0,
            nblock_max: 0,
            num_z: 0,
            state_out_pos: 0,
            in_use: [false; 256],
            bs_buff: 0,
            bs_live: 0,
            block_crc: 0,
            combined_crc: 0,
            verbosity: 0,
            block_no: 0,
            block_size_100k: 0,
        })
    }
}

// Default allocation functions
fn default_bzalloc(items: usize, size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(items * size).unwrap();
    unsafe { alloc(layout) }
}

fn default_bzfree(addr: *mut u8) {
    if !addr.is_null() {
        // In a real implementation, we'd need to track the layout
        // For simplicity, we'll just leak memory in this example
        // A proper implementation would need to track allocation sizes
    }
}

// Configuration check
fn bz_config_ok() -> bool {
    std::mem::size_of::<Int32>() == 4 &&
    std::mem::size_of::<UInt16>() == 2 &&
    std::mem::size_of::<UChar>() == 1
}

// Initialize compression
pub fn bz_compress_init(
    strm: &mut BzStream,
    block_size_100k: i32,
    verbosity: i32,
    work_factor: i32,
) -> Result<(), BzError> {
    if !bz_config_ok() {
        return Err(BzError::ConfigError);
    }

    if strm.state.is_some() 
        || block_size_100k < 1 
        || block_size_100k > 9 
        || work_factor < 0 
        || work_factor > 250 
    {
        return Err(BzError::ParamError);
    }

    let work_factor = if work_factor == 0 { 30 } else { work_factor };
    
    let mut state = EState::new()?;
    
    let n = 100000 * block_size_100k as usize;
    
    // Allocate memory blocks
    state.arr1 = Some(vec![0u32; n].into_boxed_slice());
    state.arr2 = Some(vec![0u32; n + 256].into_boxed_slice()); // BZ_N_OVERSHOOT = 256
    state.ftab = Some(vec![0u32; 65537].into_boxed_slice());
    
    // Check allocations
    if state.arr1.is_none() || state.arr2.is_none() || state.ftab.is_none() {
        return Err(BzError::MemError);
    }

    state.strm = strm as *mut BzStream;
    state.block_no = 0;
    state.state = BzState::Input;
    state.mode = BzMode::Running;
    state.combined_crc = 0;
    state.block_size_100k = block_size_100k;
    state.nblock_max = 100000 * block_size_100k - 19;
    state.verbosity = verbosity;
    state.work_factor = work_factor;

    init_rl(&mut state);
    prepare_new_block(&mut state);
    
    strm.state = Some(Box::new(state));
    Ok(())
}

// Initialize run-length encoding state
fn init_rl(s: &mut EState) {
    s.state_in_ch = 256;
    s.state_in_len = 0;
}

fn isempty_rl(s: &EState) -> bool {
    if s.state_in_ch < 256 && s.state_in_len > 0 {
        false
    } else {
        true
    }
}

// Prepare new block
fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.num_z = 0;
    s.state_out_pos = 0;
    s.block_crc = 0; // BZ_INITIALISE_CRC equivalent
    s.in_use = [false; 256];
    s.block_no += 1;
}

// Main compression function
pub fn bz_compress(strm: &mut BzStream, action: BzAction) -> Result<(), BzError> {
    // Extract mode first to avoid holding state reference
    let mode = {
        let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
        state.mode
    };
    
    match mode {
        BzMode::Idle => Err(BzError::SequenceError),
        
        BzMode::Running => match action {
            BzAction::Run => {
                handle_compress(strm)?;
                Ok(())
            }
            BzAction::Flush => {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    state.avail_in_expect = strm.avail_in as u32;
                    state.mode = BzMode::Flushing;
                }
                bz_compress(strm, action)
            }
            BzAction::Finish => {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    state.avail_in_expect = strm.avail_in as u32;
                    state.mode = BzMode::Finishing;
                }
                bz_compress(strm, action)
            }
        },
        
        BzMode::Flushing => {
            if action != BzAction::Flush {
                return Err(BzError::SequenceError);
            }
            
            let (avail_in_expect, is_empty) = {
                let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
                if state.avail_in_expect != strm.avail_in as u32 {
                    return Err(BzError::SequenceError);
                }
                (state.avail_in_expect, isempty_rl(state))
            };
            
            handle_compress(strm)?;
            
            if avail_in_expect > 0 || !is_empty {
                Ok(())
            } else {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    state.mode = BzMode::Running;
                }
                Ok(())
            }
        }
        
        BzMode::Finishing => {
            if action != BzAction::Finish {
                return Err(BzError::SequenceError);
            }
            
            let (avail_in_expect, is_empty) = {
                let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
                if state.avail_in_expect != strm.avail_in as u32 {
                    return Err(BzError::SequenceError);
                }
                (state.avail_in_expect, isempty_rl(state))
            };
            
            handle_compress(strm)?;
            
            if avail_in_expect > 0 || !is_empty {
                Ok(())
            } else {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    state.mode = BzMode::Idle;
                }
                Ok(())
            }
        }
    }
}

// Handle compression work
fn handle_compress(strm: &mut BzStream) -> Result<(), BzError> {
    loop {
        let (state_val, mode) = {
            let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
            (state.state, state.mode)
        };
        
        if state_val == BzState::Output {
            copy_output_until_stop(strm)?;
            
            let (state_out_pos, num_z, avail_in_expect, is_empty) = {
                let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
                (state.state_out_pos, state.num_z, state.avail_in_expect, isempty_rl(state))
            };
            
            if state_out_pos < num_z {
                break;
            }
            if mode == BzMode::Finishing 
                && avail_in_expect == 0 
                && is_empty 
            {
                break;
            }
            
            {
                let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                prepare_new_block(state);
                state.state = BzState::Input;
            }
            
            if mode == BzMode::Flushing 
                && avail_in_expect == 0 
                && is_empty 
            {
                break;
            }
        }

        if state_val == BzState::Input {
            copy_input_until_stop(strm)?;
            
            let (mode, avail_in_expect, nblock, nblock_max) = {
                let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
                (state.mode, state.avail_in_expect, state.nblock, state.nblock_max)
            };
            
            if mode != BzMode::Running && avail_in_expect == 0 {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    flush_rl(state);
                    // BZ2_compressBlock would be called here
                    state.state = BzState::Output;
                }
            } else if nblock >= nblock_max {
                {
                    let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                    // BZ2_compressBlock would be called here  
                    state.state = BzState::Output;
                }
            } else if strm.avail_in == 0 {
                break;
            }
        }
    }
    
    Ok(())
}

// Copy input data to block
fn copy_input_until_stop(strm: &mut BzStream) -> Result<bool, BzError> {
    let (mode, nblock_max) = {
        let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
        (state.mode, state.nblock_max)
    };
    
    let mut progress_in = false;

    if mode == BzMode::Running {
        // Fast track for common case
        while {
            let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
            state.nblock < nblock_max && strm.avail_in > 0
        } {
            progress_in = true;
            {
                let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                add_char_to_block(state, strm.next_in as u32);
            }
            unsafe {
                strm.next_in = strm.next_in.add(1);
            }
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
            }
        }
    } else {
        // General case
        while {
            let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
            state.nblock < nblock_max 
                && strm.avail_in > 0 
                && state.avail_in_expect > 0 
        } {
            progress_in = true;
            {
                let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
                add_char_to_block(state, strm.next_in as u32);
                state.avail_in_expect -= 1;
            }
            unsafe {
                strm.next_in = strm.next_in.add(1);
            }
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
            }
        }
    }

    Ok(progress_in)
}

// Copy output data from block
fn copy_output_until_stop(strm: &mut BzStream) -> Result<bool, BzError> {
    let (state_out_pos, num_z) = {
        let state = strm.state.as_ref().ok_or(BzError::ParamError)?;
        (state.state_out_pos, state.num_z)
    };
    
    let mut progress_out = false;

    while strm.avail_out > 0 && state_out_pos < num_z {
        progress_out = true;
        
        // In real implementation, this would copy from zbits array
        unsafe {
            *strm.next_out = 0; // Placeholder - would be s.zbits[s.state_out_pos]
            strm.next_out = strm.next_out.add(1);
        }
        
        strm.avail_out -= 1;
        {
            let state = strm.state.as_mut().ok_or(BzError::ParamError)?;
            state.state_out_pos += 1;
        }
        strm.total_out_lo32 += 1;
        if strm.total_out_lo32 == 0 {
            strm.total_out_hi32 += 1;
        }
    }

    Ok(progress_out)
}

// Add character to block with run-length encoding
fn add_char_to_block(s: &mut EState, zchh: UInt32) {
    if zchh != s.state_in_ch && s.state_in_len == 1 {
        let _ch = s.state_in_ch as u8;
        // BZ_UPDATE_CRC would update CRC here
        s.in_use[s.state_in_ch as usize] = true;
        
        // In real implementation, this would write to s.block array
        s.nblock += 1;
        s.state_in_ch = zchh;
    } else if zchh != s.state_in_ch || s.state_in_len == 255 {
        if s.state_in_ch < 256 {
            add_pair_to_block(s);
        }
        s.state_in_ch = zchh;
        s.state_in_len = 1;
    } else {
        s.state_in_len += 1;
    }
}

// Add run-length pair to block
fn add_pair_to_block(s: &mut EState) {
    let _ch = s.state_in_ch as u8;
    
    // Update CRC for each character in the run
    for _ in 0..s.state_in_len {
        // BZ_UPDATE_CRC would update CRC here
    }
    
    s.in_use[s.state_in_ch as usize] = true;
    
    match s.state_in_len {
        1 => {
            // Write single character
            s.nblock += 1;
        }
        2 => {
            // Write two characters
            s.nblock += 2;
        }
        3 => {
            // Write three characters  
            s.nblock += 3;
        }
        _ => {
            // Write run-length encoded sequence
            s.in_use[(s.state_in_len - 4) as usize] = true;
            s.nblock += 5; // 4 chars + length byte
        }
    }
}

// Flush run-length encoding state
fn flush_rl(s: &mut EState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_rl(s);
}

// End compression
pub fn bz_compress_end(strm: &mut BzStream) -> Result<(), BzError> {
    if strm.state.is_none() {
        return Err(BzError::ParamError);
    }
    
    strm.state = None;
    Ok(())
}

// Version information
pub fn bz_lib_version() -> &'static str {
    "1.0.8, 13-Jul-2019"
}

// Simple buffer-to-buffer compression interface
pub fn bz_buff_to_buff_compress(
    dest: &mut [u8],
    source: &[u8],
    block_size_100k: i32,
    verbosity: i32,
    work_factor: i32,
) -> Result<usize, BzError> {
    if dest.is_empty() || source.is_empty() {
        return Err(BzError::ParamError);
    }
    
    if block_size_100k < 1 || block_size_100k > 9 {
        return Err(BzError::ParamError);
    }
    
    let work_factor = if work_factor == 0 { 30 } else { work_factor };
    
    let mut strm = BzStream::default();
    
    bz_compress_init(&mut strm, block_size_100k, verbosity, work_factor)?;
    
    // Set up input/output buffers
    strm.next_in = source.as_ptr();
    strm.avail_in = source.len();
    strm.next_out = dest.as_mut_ptr();
    strm.avail_out = dest.len();
    
    // Compress the data
    let mut result = BzError::Ok;
    
    while strm.avail_in > 0 && result == BzError::Ok {
        result = match bz_compress(&mut strm, BzAction::Run) {
            Ok(()) => BzError::Ok,
            Err(e) => e,
        };
    }
    
    if result == BzError::Ok {
        result = match bz_compress(&mut strm, BzAction::Finish) {
            Ok(()) => BzError::Ok,
            Err(e) => e,
        };
    }
    
    let compressed_size = dest.len() - strm.avail_out;
    bz_compress_end(&mut strm)?;
    
    if result == BzError::Ok {
        Ok(compressed_size)
    } else {
        Err(result)
    }
}