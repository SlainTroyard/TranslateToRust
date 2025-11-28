use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const BZ_MAX_FILENAME: usize = 2000;
const BZ_MAX_HANDLED_BLOCKS: usize = 50000;

const BZ_HDR_B: u8 = 0x42;
const BZ_HDR_Z: u8 = 0x5a;
const BZ_HDR_h: u8 = 0x68;
const BZ_HDR_0: u8 = 0x30;

const BLOCK_HEADER_HI: u32 = 0x00003141;
const BLOCK_HEADER_LO: u32 = 0x59265359;
const BLOCK_ENDMARK_HI: u32 = 0x00001772;
const BLOCK_ENDMARK_LO: u32 = 0x45385090;

#[derive(Debug)]
struct BitStream {
    handle: File,
    buffer: u32,
    buff_live: u32,
    mode: char,
}

impl BitStream {
    fn open_read(stream: File) -> Self {
        Self {
            handle: stream,
            buffer: 0,
            buff_live: 0,
            mode: 'r',
        }
    }

    fn open_write(stream: File) -> Self {
        Self {
            handle: stream,
            buffer: 0,
            buff_live: 0,
            mode: 'w',
        }
    }

    fn put_bit(&mut self, bit: u32) -> io::Result<()> {
        if self.buff_live == 8 {
            let byte = (self.buffer & 0xFF) as u8;
            self.handle.write_all(&[byte])?;
            self.buff_live = 1;
            self.buffer = bit & 0x1;
        } else {
            self.buffer = (self.buffer << 1) | (bit & 0x1);
            self.buff_live += 1;
        }
        Ok(())
    }

    fn get_bit(&mut self) -> io::Result<Option<u32>> {
        if self.buff_live > 0 {
            self.buff_live -= 1;
            Ok(Some((self.buffer >> self.buff_live) & 0x1))
        } else {
            let mut byte = [0u8; 1];
            match self.handle.read_exact(&mut byte) {
                Ok(()) => {
                    self.buff_live = 7;
                    self.buffer = byte[0] as u32;
                    Ok(Some((self.buffer >> 7) & 0x1))
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                Err(e) => Err(e),
            }
        }
    }

    fn close(mut self) -> io::Result<()> {
        if self.mode == 'w' {
            while self.buff_live < 8 {
                self.buff_live += 1;
                self.buffer <<= 1;
            }
            let byte = (self.buffer & 0xFF) as u8;
            self.handle.write_all(&[byte])?;
            self.handle.flush()?;
        }
        Ok(())
    }

    fn put_uchar(&mut self, c: u8) -> io::Result<()> {
        for i in (0..8).rev() {
            self.put_bit(((c as u32) >> i) & 0x1)?;
        }
        Ok(())
    }

    fn put_uint32(&mut self, c: u32) -> io::Result<()> {
        for i in (0..32).rev() {
            self.put_bit((c >> i) & 0x1)?;
        }
        Ok(())
    }
}

fn ends_in_bz2(name: &str) -> bool {
    name.to_lowercase().ends_with(".bz2")
}

fn get_output_filename(input_path: &Path, block_num: usize) -> PathBuf {
    let _stem = input_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("rec");
    
    let dir = input_path.parent().unwrap_or(Path::new(""));
    let mut output_name = format!("rec{:05}", block_num + 1);
    
    if !ends_in_bz2(input_path.to_string_lossy().as_ref()) {
        output_name.push_str(".bz2");
    }
    
    dir.join(output_name)
}

fn recover_blocks(input_path: &Path) -> io::Result<()> {
    let in_file = File::open(input_path)?;
    let _file_size = in_file.metadata()?.len();
    
    println!("bzip2recover 1.0.8: extracts blocks from damaged .bz2 files.");
    println!("searching for block boundaries ...");

    let mut bits_read: u64 = 0;
    let mut buff_hi: u32 = 0;
    let mut buff_lo: u32 = 0;
    let mut curr_block = 0;
    
    let mut b_start = [0u64; BZ_MAX_HANDLED_BLOCKS];
    let mut b_end = [0u64; BZ_MAX_HANDLED_BLOCKS];
    let mut rb_start = [0u64; BZ_MAX_HANDLED_BLOCKS];
    let mut rb_end = [0u64; BZ_MAX_HANDLED_BLOCKS];
    
    b_start[curr_block] = 0;
    let mut rb_ctr = 0;

    // First pass: identify block boundaries
    let temp_file = File::open(input_path)?;
    let mut bs_in = BitStream::open_read(temp_file);
    
    loop {
        match bs_in.get_bit()? {
            Some(bit) => {
                bits_read += 1;
                buff_hi = (buff_hi << 1) | (buff_lo >> 31);
                buff_lo = (buff_lo << 1) | (bit & 1);
                
                if ((buff_hi & 0x0000ffff) == BLOCK_HEADER_HI && buff_lo == BLOCK_HEADER_LO) ||
                   ((buff_hi & 0x0000ffff) == BLOCK_ENDMARK_HI && buff_lo == BLOCK_ENDMARK_LO) {
                    
                    if bits_read > 49 {
                        b_end[curr_block] = bits_read - 49;
                    } else {
                        b_end[curr_block] = 0;
                    }
                    
                    if curr_block > 0 && (b_end[curr_block] - b_start[curr_block]) >= 130 {
                        println!("   block {} runs from {} to {}", 
                               rb_ctr + 1, b_start[curr_block], b_end[curr_block]);
                        rb_start[rb_ctr] = b_start[curr_block];
                        rb_end[rb_ctr] = b_end[curr_block];
                        rb_ctr += 1;
                    }
                    
                    if curr_block >= BZ_MAX_HANDLED_BLOCKS - 1 {
                        eprintln!("Too many blocks to handle (max: {})", BZ_MAX_HANDLED_BLOCKS);
                        return Ok(());
                    }
                    
                    curr_block += 1;
                    b_start[curr_block] = bits_read;
                }
            }
            None => {
                if bits_read >= b_start[curr_block] && (bits_read - b_start[curr_block]) >= 40 {
                    b_end[curr_block] = bits_read - 1;
                    if curr_block > 0 {
                        println!("   block {} runs from {} to {} (incomplete)", 
                               curr_block, b_start[curr_block], b_end[curr_block]);
                    }
                }
                break;
            }
        }
    }
    
    bs_in.close()?;

    if rb_ctr < 1 {
        eprintln!("Sorry, I couldn't find any block boundaries.");
        return Ok(());
    }

    println!("splitting into blocks");

    // Second pass: extract identified blocks
    let in_file = File::open(input_path)?;
    let mut bs_in = BitStream::open_read(in_file);
    
    let mut bits_read: u64 = 0;
    let mut buff_hi: u32 = 0;
    let mut buff_lo: u32 = 0;
    let mut block_crc: u32 = 0;
    let mut wr_block = 0;
    
    let mut out_file: Option<File> = None;
    let mut bs_wr: Option<BitStream> = None;

    loop {
        match bs_in.get_bit()? {
            Some(bit) => {
                buff_hi = (buff_hi << 1) | (buff_lo >> 31);
                buff_lo = (buff_lo << 1) | (bit & 1);
                
                // Capture block CRC
                if bits_read == 47 + rb_start[wr_block] {
                    block_crc = (buff_hi << 16) | (buff_lo >> 16);
                }

                // Write bit to output if within current block range
                if out_file.is_some() && bits_read >= rb_start[wr_block] && bits_read <= rb_end[wr_block] {
                    if let Some(ref mut writer) = bs_wr {
                        writer.put_bit(bit)?;
                    }
                }

                bits_read += 1;

                // Handle block transitions
                if bits_read == rb_end[wr_block] + 1 {
                    if let (Some(_), Some(mut writer)) = (out_file.take(), bs_wr.take()) {
                        // Write block end marker and CRC
                        writer.put_uchar(0x17)?;
                        writer.put_uchar(0x72)?;
                        writer.put_uchar(0x45)?;
                        writer.put_uchar(0x38)?;
                        writer.put_uchar(0x50)?;
                        writer.put_uchar(0x90)?;
                        writer.put_uint32(block_crc)?;
                        writer.close()?;
                    }
                    
                    if wr_block >= rb_ctr - 1 {
                        break;
                    }
                    wr_block += 1;
                } else if bits_read == rb_start[wr_block] {
                    // Start new output file for this block
                    let output_path = get_output_filename(input_path, wr_block);
                    println!("   writing block {} to `{}` ...", wr_block + 1, output_path.display());
                    
                    let file = File::create(&output_path)?;
                    let mut writer = BitStream::open_write(file);
                    
                    // Write bzip2 header
                    writer.put_uchar(BZ_HDR_B)?;
                    writer.put_uchar(BZ_HDR_Z)?;
                    writer.put_uchar(BZ_HDR_h)?;
                    writer.put_uchar(BZ_HDR_0 + 9)?;
                    writer.put_uchar(0x31)?;
                    writer.put_uchar(0x41)?;
                    writer.put_uchar(0x59)?;
                    writer.put_uchar(0x26)?;
                    writer.put_uchar(0x53)?;
                    writer.put_uchar(0x59)?;
                    
                    out_file = Some(writer.handle.try_clone()?);
                    bs_wr = Some(writer);
                }
            }
            None => break,
        }
    }

    if let (Some(_), Some(writer)) = (out_file, bs_wr) {
        writer.close()?;
    }

    println!("finished");
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} damaged_file_name", args[0]);
        eprintln!("restrictions on size of recovered file: None");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    
    if !input_path.exists() {
        eprintln!("Cannot read `{}`: No such file", input_path.display());
        std::process::exit(1);
    }

    if let Err(e) = recover_blocks(input_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_ends_in_bz2() {
        assert!(ends_in_bz2("file.bz2"));
        assert!(ends_in_bz2("FILE.BZ2"));
        assert!(!ends_in_bz2("file.txt"));
        assert!(!ends_in_bz2("file"));
    }

    #[test]
    fn test_bit_stream_basic() -> io::Result<()> {
        let data = Cursor::new(Vec::new());
        let mut stream = BitStream::open_write(File::create("test.bin")?);
        
        stream.put_bit(1)?;
        stream.put_bit(0)?;
        stream.put_bit(1)?;
        stream.put_bit(1)?;
        stream.put_uchar(0xAB)?;
        stream.close()?;
        
        Ok(())
    }
}