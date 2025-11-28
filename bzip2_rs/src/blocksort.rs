use std::fmt;

/// Error types matching original bzip2 error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Type aliases matching original C types
type Bool = bool;
type UChar = u8;
type Int32 = i32;
type UInt32 = u32;
type UInt16 = u16;

/// Constants from original C code
const BZ_N_OVERSHOOT: usize = 256;
const FALLBACK_QSORT_SMALL_THRESH: Int32 = 10;
const FALLBACK_QSORT_STACK_SIZE: usize = 100;
const MAIN_QSORT_SMALL_THRESH: Int32 = 20;
const MAIN_QSORT_STACK_SIZE: usize = 100;

/// Helper functions for bucket header table operations
fn set_bh(bhtab: &mut [UInt32], zz: Int32) {
    let idx = (zz >> 5) as usize;
    let bit = (zz & 31) as u32;
    bhtab[idx] |= 1 << bit;
}

fn clear_bh(bhtab: &mut [UInt32], zz: Int32) {
    let idx = (zz >> 5) as usize;
    let bit = (zz & 31) as u32;
    bhtab[idx] &= !(1 << bit);
}

fn is_set_bh(bhtab: &[UInt32], zz: Int32) -> bool {
    let idx = (zz >> 5) as usize;
    let bit = (zz & 31) as u32;
    (bhtab[idx] & (1 << bit)) != 0
}

fn word_bh(bhtab: &[UInt32], zz: Int32) -> UInt32 {
    let idx = (zz >> 5) as usize;
    bhtab[idx]
}

fn unaligned_bh(zz: Int32) -> bool {
    (zz & 0x1f) != 0
}

/// Fallback sorting functions for repetitive blocks
pub mod fallback_sort {
    use super::*;

    /// Simple insertion sort for small arrays
    pub fn fallback_simple_sort(
        fmap: &mut [UInt32],
        eclass: &[UInt32],
        lo: Int32,
        hi: Int32,
    ) {
        if lo == hi {
            return;
        }

        // Optimized sorting for larger ranges
        if hi - lo > 3 {
            for i in (lo..=hi - 4).rev() {
                let tmp = fmap[i as usize];
                let ec_tmp = eclass[tmp as usize];
                let mut j = i + 4;
                
                while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                    fmap[(j - 4) as usize] = fmap[j as usize];
                    j += 4;
                }
                fmap[(j - 4) as usize] = tmp;
            }
        }

        // Final insertion sort pass
        for i in (lo..=hi - 1).rev() {
            let tmp = fmap[i as usize];
            let ec_tmp = eclass[tmp as usize];
            let mut j = i + 1;
            
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                fmap[(j - 1) as usize] = fmap[j as usize];
                j += 1;
            }
            fmap[(j - 1) as usize] = tmp;
        }
    }

    /// Quick sort implementation for fallback sorting
    pub fn fallback_qsort3(
        fmap: &mut [UInt32],
        eclass: &[UInt32],
        lo_st: Int32,
        hi_st: Int32,
    ) {
        let mut stack_lo = [0; FALLBACK_QSORT_STACK_SIZE];
        let mut stack_hi = [0; FALLBACK_QSORT_STACK_SIZE];
        let mut sp = 0;
        
        stack_lo[sp] = lo_st;
        stack_hi[sp] = hi_st;
        sp += 1;

        while sp > 0 {
            sp -= 1;
            let lo = stack_lo[sp];
            let hi = stack_hi[sp];

            if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
                fallback_simple_sort(fmap, eclass, lo, hi);
                continue;
            }

            // Random pivot selection
            static mut R: UInt32 = 0;
            let r = unsafe {
                R = R.wrapping_mul(7621).wrapping_add(1) % 32768;
                R
            };
            let r3 = r % 3;
            
            let med = match r3 {
                0 => eclass[fmap[lo as usize] as usize],
                1 => eclass[fmap[((lo + hi) >> 1) as usize] as usize],
                _ => eclass[fmap[hi as usize] as usize],
            };

            let mut un_lo = lo;
            let mut un_hi = hi;
            let mut lt_lo = lo;
            let mut gt_hi = hi;

            loop {
                while un_lo <= un_hi {
                    let n = eclass[fmap[un_lo as usize] as usize] as Int32 - med as Int32;
                    if n == 0 {
                        fmap.swap(un_lo as usize, lt_lo as usize);
                        lt_lo += 1;
                        un_lo += 1;
                        continue;
                    }
                    if n > 0 {
                        break;
                    }
                    un_lo += 1;
                }

                while un_lo <= un_hi {
                    let n = eclass[fmap[un_hi as usize] as usize] as Int32 - med as Int32;
                    if n == 0 {
                        fmap.swap(un_hi as usize, gt_hi as usize);
                        gt_hi -= 1;
                        un_hi -= 1;
                        continue;
                    }
                    if n < 0 {
                        break;
                    }
                    un_hi -= 1;
                }

                if un_lo > un_hi {
                    break;
                }

                fmap.swap(un_lo as usize, un_hi as usize);
                un_lo += 1;
                un_hi -= 1;
            }

            if gt_hi < lt_lo {
                continue;
            }

            let n = (lt_lo - lo).min(un_lo - lt_lo);
            fvswap(fmap, lo, un_lo - n, n);

            let m = (hi - gt_hi).min(gt_hi - un_hi);
            fvswap(fmap, un_lo, hi - m + 1, m);

            let n = lo + un_lo - lt_lo - 1;
            let m = hi - (gt_hi - un_hi) + 1;

            if n - lo > hi - m {
                if sp < FALLBACK_QSORT_STACK_SIZE {
                    stack_lo[sp] = lo;
                    stack_hi[sp] = n;
                    sp += 1;
                    stack_lo[sp] = m;
                    stack_hi[sp] = hi;
                    sp += 1;
                }
            } else {
                if sp < FALLBACK_QSORT_STACK_SIZE {
                    stack_lo[sp] = m;
                    stack_hi[sp] = hi;
                    sp += 1;
                    stack_lo[sp] = lo;
                    stack_hi[sp] = n;
                    sp += 1;
                }
            }
        }
    }

    /// Swap a range of elements between two positions
    fn fvswap(fmap: &mut [UInt32], yy_p1: Int32, yy_p2: Int32, yy_n: Int32) {
        let mut yyp1 = yy_p1;
        let mut yyp2 = yy_p2;
        let mut yyn = yy_n;
        
        while yyn > 0 {
            fmap.swap(yyp1 as usize, yyp2 as usize);
            yyp1 += 1;
            yyp2 += 1;
            yyn -= 1;
        }
    }

    /// Main fallback sort entry point
    pub fn fallback_sort(
        fmap: &mut [UInt32],
        eclass: &mut [UInt32],
        bhtab: &mut [UInt32],
        nblock: Int32,
        _verb: Int32,
    ) -> Result<(), BzError> {
        let mut ftab = [0; 257];
        let mut ftab_copy = [0; 256];
        let eclass8 = unsafe {
            std::slice::from_raw_parts_mut(eclass.as_mut_ptr() as *mut UChar, nblock as usize)
        };

        // Initial 1-char radix sort
        for i in 0..nblock as usize {
            ftab[eclass8[i] as usize] += 1;
        }
        
        ftab_copy[..256].copy_from_slice(&ftab[..256]);
        
        for i in 1..257 {
            ftab[i] += ftab[i - 1];
        }

        for i in 0..nblock as usize {
            let j = eclass8[i] as usize;
            ftab[j] -= 1;
            fmap[ftab[j] as usize] = i as UInt32;
        }

        // Initialize bucket header table
        let n_bhtab = 2 + (nblock / 32) as usize;
        for i in 0..n_bhtab {
            bhtab[i] = 0;
        }
        for i in 0..256 {
            set_bh(bhtab, ftab[i]);
        }

        // Set sentinel bits for block-end detection
        for i in 0..32 {
            set_bh(bhtab, nblock + 2 * i);
            clear_bh(bhtab, nblock + 2 * i + 1);
        }

        // Main refinement loop
        let mut h = 1;
        loop {
            let mut n_not_done = 0;
            
            // Update equivalence classes
            let mut j = 0;
            for i in 0..nblock as usize {
                if is_set_bh(bhtab, i as Int32) {
                    j = i as Int32;
                }
                let mut k = fmap[i] as Int32 - h;
                if k < 0 {
                    k += nblock;
                }
                eclass[k as usize] = j as UInt32;
            }

            // Process buckets
            let mut r = -1;
            loop {
                // Find next non-singleton bucket
                let mut k = r + 1;
                while k < nblock && is_set_bh(bhtab, k) && unaligned_bh(k) {
                    k += 1;
                }
                if k < nblock && is_set_bh(bhtab, k) {
                    while k < nblock && word_bh(bhtab, k) == 0xffffffff {
                        k += 32;
                    }
                    while k < nblock && is_set_bh(bhtab, k) {
                        k += 1;
                    }
                }
                let l = k - 1;
                if l >= nblock {
                    break;
                }

                while k < nblock && !is_set_bh(bhtab, k) && unaligned_bh(k) {
                    k += 1;
                }
                if k < nblock && !is_set_bh(bhtab, k) {
                    while k < nblock && word_bh(bhtab, k) == 0 {
                        k += 32;
                    }
                    while k < nblock && !is_set_bh(bhtab, k) {
                        k += 1;
                    }
                }
                r = k - 1;
                if r >= nblock {
                    break;
                }

                // Now [l, r] bracket current bucket
                if r > l {
                    n_not_done += r - l + 1;
                    fallback_qsort3(fmap, eclass, l, r);

                    // Scan bucket and generate header bits
                    let mut cc = -1;
                    for i in l..=r {
                        let cc1 = eclass[fmap[i as usize] as usize] as Int32;
                        if cc != cc1 {
                            set_bh(bhtab, i);
                            cc = cc1;
                        }
                    }
                }
            }

            h *= 2;
            if h > nblock || n_not_done == 0 {
                break;
            }
        }

        // Reconstruct the original block
        let mut j = 0;
        for i in 0..nblock as usize {
            while ftab_copy[j] == 0 {
                j += 1;
            }
            ftab_copy[j] -= 1;
            eclass8[fmap[i] as usize] = j as UChar;
        }

        Ok(())
    }
}

/// Main sorting functions for normal non-repetitive blocks
pub mod main_sort {
    use super::*;

    /// Compare two strings for main sort
    pub fn main_gt_u(
        i1: UInt32,
        i2: UInt32,
        block: &[UChar],
        quadrant: &[UInt16],
        nblock: Int32,
        _budget: &mut Int32,
    ) -> Bool {
        // Compare first 12 bytes directly
        for offset in 0..12 {
            let c1 = block[(i1 + offset) as usize];
            let c2 = block[(i2 + offset) as usize];
            if c1 != c2 {
                return c1 > c2;
            }
        }

        // Compare remaining bytes with quadrant assistance
        let mut k = nblock + 8;
        let mut ii1 = i1;
        let mut ii2 = i2;

        while k >= 0 {
            for _ in 0..8 {
                let c1 = block[ii1 as usize];
                let c2 = block[ii2 as usize];
                if c1 != c2 {
                    return c1 > c2;
                }

                let s1 = quadrant[ii1 as usize];
                let s2 = quadrant[ii2 as usize];
                if s1 != s2 {
                    return s1 > s2;
                }

                ii1 += 1;
                ii2 += 1;

                if ii1 >= nblock as UInt32 {
                    ii1 -= nblock as UInt32;
                }
                if ii2 >= nblock as UInt32 {
                    ii2 -= nblock as UInt32;
                }
            }
            k -= 8;
            *_budget -= 1;
        }

        false
    }

    /// Simple sort using shell sort increments
    pub fn main_simple_sort(
        ptr: &mut [UInt32],
        block: &[UChar],
        quadrant: &[UInt16],
        nblock: Int32,
        lo: Int32,
        hi: Int32,
        d: Int32,
        budget: &mut Int32,
    ) {
        let big_n = hi - lo + 1;
        if big_n < 2 {
            return;
        }

        // Shell sort increments
        let incs = [1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484];
        
        let mut hp = 0;
        while hp < incs.len() && incs[hp] < big_n {
            hp += 1;
        }
        if hp > 0 {
            hp -= 1;
        }

        for h in (0..=hp).rev().map(|i| incs[i]) {
            let mut i = lo + h;
            while i <= hi {
                let v = ptr[i as usize];
                let mut j = i;
                
                while j > lo + h - 1 {
                    if !main_gt_u(
                        ptr[(j - h) as usize] + d as UInt32,
                        v + d as UInt32,
                        block,
                        quadrant,
                        nblock,
                        budget,
                    ) {
                        break;
                    }
                    ptr[j as usize] = ptr[(j - h) as usize];
                    j -= h;
                }
                ptr[j as usize] = v;
                i += 1;

                if *budget < 0 {
                    return;
                }
            }
        }
    }

    /// Main quicksort implementation
    pub fn main_qsort3(
        ptr: &mut [UInt32],
        block: &[UChar],
        quadrant: &[UInt16],
        nblock: Int32,
        lo_st: Int32,
        hi_st: Int32,
        d_st: Int32,
        budget: &mut Int32,
    ) {
        let mut stack_lo = [0; MAIN_QSORT_STACK_SIZE];
        let mut stack_hi = [0; MAIN_QSORT_STACK_SIZE];
        let mut stack_d = [0; MAIN_QSORT_STACK_SIZE];
        let mut sp = 0;

        stack_lo[sp] = lo_st;
        stack_hi[sp] = hi_st;
        stack_d[sp] = d_st;
        sp += 1;

        while sp > 0 {
            sp -= 1;
            let lo = stack_lo[sp];
            let hi = stack_hi[sp];
            let d = stack_d[sp];

            if hi - lo < MAIN_QSORT_SMALL_THRESH {
                main_simple_sort(ptr, block, quadrant, nblock, lo, hi, d, budget);
                if *budget < 0 {
                    return;
                }
                continue;
            }

            // Median of three pivot selection
            let med = {
                let c1 = block[ptr[lo as usize] as usize + d as usize];
                let c2 = block[ptr[hi as usize] as usize + d as usize];
                let c3 = block[ptr[((lo + hi) >> 1) as usize] as usize + d as usize];
                
                let mut chars = [c1, c2, c3];
                chars.sort();
                chars[1]
            };

            let mut un_lo = lo;
            let mut un_hi = hi;
            let mut lt_lo = lo;
            let mut gt_hi = hi;

            loop {
                while un_lo <= un_hi {
                    let n = block[ptr[un_lo as usize] as usize + d as usize] as Int32 - med as Int32;
                    if n == 0 {
                        ptr.swap(un_lo as usize, lt_lo as usize);
                        lt_lo += 1;
                        un_lo += 1;
                        continue;
                    }
                    if n > 0 {
                        break;
                    }
                    un_lo += 1;
                }

                while un_lo <= un_hi {
                    let n = block[ptr[un_hi as usize] as usize + d as usize] as Int32 - med as Int32;
                    if n == 0 {
                        ptr.swap(un_hi as usize, gt_hi as usize);
                        gt_hi -= 1;
                        un_hi -= 1;
                        continue;
                    }
                    if n < 0 {
                        break;
                    }
                    un_hi -= 1;
                }

                if un_lo > un_hi {
                    break;
                }

                ptr.swap(un_lo as usize, un_hi as usize);
                un_lo += 1;
                un_hi -= 1;
            }

            if gt_hi < lt_lo {
                if sp < MAIN_QSORT_STACK_SIZE {
                    stack_lo[sp] = lo;
                    stack_hi[sp] = hi;
                    stack_d[sp] = d + 1;
                    sp += 1;
                }
                continue;
            }

            let n = (lt_lo - lo).min(un_lo - lt_lo);
            mvswap(ptr, lo, un_lo - n, n);

            let m = (hi - gt_hi).min(gt_hi - un_hi);
            mvswap(ptr, un_lo, hi - m + 1, m);

            let n_part = lo + un_lo - lt_lo - 1;
            let m_part = hi - (gt_hi - un_hi) + 1;

            // Push partitions onto stack in optimal order
            if n_part - lo > hi - m_part {
                if sp + 2 < MAIN_QSORT_STACK_SIZE {
                    stack_lo[sp] = lo;
                    stack_hi[sp] = n_part;
                    stack_d[sp] = d;
                    sp += 1;
                    
                    stack_lo[sp] = m_part;
                    stack_hi[sp] = hi;
                    stack_d[sp] = d;
                    sp += 1;
                    
                    stack_lo[sp] = n_part + 1;
                    stack_hi[sp] = m_part - 1;
                    stack_d[sp] = d + 1;
                    sp += 1;
                }
            } else {
                if sp + 2 < MAIN_QSORT_STACK_SIZE {
                    stack_lo[sp] = m_part;
                    stack_hi[sp] = hi;
                    stack_d[sp] = d;
                    sp += 1;
                    
                    stack_lo[sp] = lo;
                    stack_hi[sp] = n_part;
                    stack_d[sp] = d;
                    sp += 1;
                    
                    stack_lo[sp] = n_part + 1;
                    stack_hi[sp] = m_part - 1;
                    stack_d[sp] = d + 1;
                    sp += 1;
                }
            }
        }
    }

    /// Swap a range of elements between two positions
    fn mvswap(ptr: &mut [UInt32], zzp1: Int32, zzp2: Int32, zzn: Int32) {
        let mut yyp1 = zzp1;
        let mut yyp2 = zzp2;
        let mut yyn = zzn;
        
        while yyn > 0 {
            ptr.swap(yyp1 as usize, yyp2 as usize);
            yyp1 += 1;
            yyp2 += 1;
            yyn -= 1;
        }
    }
}

/// Main block sort entry point
pub fn bz2_block_sort(
    ptr: &mut [UInt32],
    block: &mut [UChar],
    ftab: &mut [UInt32],
    nblock: Int32,
    verb: Int32,
    work_factor: Int32,
) -> Result<Int32, BzError> {
    if nblock < 10000 {
        let mut bhtab = vec![0; (2 + (nblock / 32)) as usize];
        // Convert block to UInt32 slice for fallback_sort compatibility
        let block_u32 = unsafe {
            std::slice::from_raw_parts_mut(block.as_mut_ptr() as *mut UInt32, nblock as usize)
        };
        fallback_sort::fallback_sort(ptr, block_u32, &mut bhtab, nblock, verb)?;
    } else {
        // For larger blocks, use main sort with budget
        let mut budget = nblock * ((work_factor.max(1).min(100) - 1) / 3);
        
        // Create quadrant array
        let mut quadrant = vec![0 as UInt16; nblock as usize + BZ_N_OVERSHOOT];
        
        main_sort::main_qsort3(
            ptr,
            block,
            &quadrant,
            nblock,
            0,
            nblock - 1,
            0,
            &mut budget,
        );
        
        if budget < 0 {
            // Fall back to fallback sort if budget exhausted
            let mut bhtab = vec![0; (2 + (nblock / 32)) as usize];
            // Convert block to UInt32 slice for fallback_sort compatibility
            let block_u32 = unsafe {
                std::slice::from_raw_parts_mut(block.as_mut_ptr() as *mut UInt32, nblock as usize)
            };
            fallback_sort::fallback_sort(ptr, block_u32, &mut bhtab, nblock, verb)?;
        }
    }

    // Find original pointer (position of first byte in sorted order)
    let mut orig_ptr = -1;
    for i in 0..nblock as usize {
        if ptr[i] == 0 {
            orig_ptr = i as Int32;
            break;
        }
    }

    Ok(orig_ptr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bzerror_display() {
        assert_eq!(format!("{}", BzError::Ok), "OK");
        assert_eq!(format!("{}", BzError::SequenceError), "SEQUENCE_ERROR");
    }

    #[test]
    fn test_bucket_operations() {
        let mut bhtab = [0u32; 10];
        
        set_bh(&mut bhtab, 5);
        assert!(is_set_bh(&bhtab, 5));
        
        clear_bh(&mut bhtab, 5);
        assert!(!is_set_bh(&bhtab, 5));
    }
}