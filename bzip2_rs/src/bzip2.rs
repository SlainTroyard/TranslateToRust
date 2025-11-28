use std::ffi::OsString;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, Read, BufReader, BufWriter, stdin, stdout};
use std::path::Path;
use std::process;

// Add Unix-specific file operations
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

// Type definitions for blocksort.rs
type UInt16 = u16;
type UInt32 = u32;

// Constants from original C code
const FILE_NAME_LEN: usize = 1034;
const BZ_N_SUFFIX_PAIRS: usize = 4;

// Operation modes
const OM_Z: i32 = 1;
const OM_UNZ: i32 = 2;
const OM_TEST: i32 = 3;

// Source modes
const SM_I2O: i32 = 1; // stdin to stdout
const SM_F2O: i32 = 2; // file to stdout  
const SM_F2F: i32 = 3; // file to file

// Error handling type
#[derive(Debug)]
pub enum BzipError {
    IoError(io::Error),
    BzError(&'static str),
    ConfigError,
    MemoryError,
    DataError,
    UnexpectedEof,
    InvalidInput,
}

impl From<io::Error> for BzipError {
    fn from(err: io::Error) -> Self {
        BzipError::IoError(err)
    }
}

type Result<T> = std::result::Result<T, BzipError>;

// Global configuration state
struct Config {
    verbosity: i32,
    keep_input_files: bool,
    small_mode: bool,
    delete_output_on_interrupt: bool,
    force_overwrite: bool,
    test_fails_exist: bool,
    unz_fails_exist: bool,
    noisy: bool,
    num_file_names: i32,
    num_files_processed: i32,
    block_size_100k: i32,
    exit_value: i32,
    op_mode: i32,
    src_mode: i32,
    longest_file_name: i32,
    work_factor: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbosity: 0,
            keep_input_files: false,
            small_mode: false,
            delete_output_on_interrupt: false,
            force_overwrite: false,
            test_fails_exist: false,
            unz_fails_exist: false,
            noisy: true,
            num_file_names: 0,
            num_files_processed: 0,
            block_size_100k: 9,
            exit_value: 0,
            op_mode: OM_Z,
            src_mode: SM_I2O,
            longest_file_name: 7,
            work_factor: 30,
        }
    }
}

// File name buffers
struct FileNames {
    in_name: String,
    out_name: String,
    tmp_name: String,
    prog_name: String,
    prog_name_really: String,
}

impl Default for FileNames {
    fn default() -> Self {
        Self {
            in_name: String::with_capacity(FILE_NAME_LEN),
            out_name: String::with_capacity(FILE_NAME_LEN),
            tmp_name: String::with_capacity(FILE_NAME_LEN),
            prog_name: String::new(),
            prog_name_really: String::with_capacity(FILE_NAME_LEN),
        }
    }
}

// Main bzip2 application
pub struct Bzip2App {
    config: Config,
    file_names: FileNames,
    output_handle: Option<File>,
}

impl Bzip2App {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            file_names: FileNames::default(),
            output_handle: None,
        }
    }

    // Copy file name with bounds checking
    fn copy_file_name(&mut self, dest: &mut String, src: &str) -> Result<()> {
        if src.len() > FILE_NAME_LEN - 10 {
            return Err(BzipError::InvalidInput);
        }
        dest.clear();
        dest.push_str(src);
        Ok(())
    }

    // Check if file exists
    fn file_exists(&self, name: &str) -> bool {
        Path::new(name).exists()
    }

    // Check if file has given suffix
    fn has_suffix(&self, s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    // Map suffix from old to new
    fn map_suffix(&self, name: &mut String, old_suffix: &str, new_suffix: &str) -> bool {
        if self.has_suffix(name, old_suffix) {
            let new_len = name.len() - old_suffix.len();
            name.truncate(new_len);
            name.push_str(new_suffix);
            true
        } else {
            false
        }
    }

    // Open output file safely
    fn fopen_output_safely(&self, name: &str) -> Result<File> {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        
        // Only use mode() on Unix systems
        #[cfg(unix)]
        options.mode(0o600); // rw-------
        
        options.open(name).map_err(BzipError::from)
    }

    // Compress a stream
    fn compress_stream<R: Read, W: io::Write>(
        &mut self,
        mut stream: R,
        mut z_stream: W,
    ) -> Result<()> {
        // This would interface with the bzip2 library
        // For now, just copy the data as a placeholder
        let mut buffer = [0u8; 5000];
        
        loop {
            let n_read = stream.read(&mut buffer)?;
            if n_read == 0 {
                break;
            }
            z_stream.write_all(&buffer[..n_read])?;
        }
        
        z_stream.flush()?;
        Ok(())
    }

    // Uncompress a stream  
    fn uncompress_stream<R: Read, W: io::Write>(
        &mut self,
        mut z_stream: R,
        mut stream: W,
    ) -> Result<bool> {
        // This would interface with the bzip2 library
        // For now, just copy the data as a placeholder
        let mut buffer = [0u8; 5000];
        
        loop {
            let n_read = z_stream.read(&mut buffer)?;
            if n_read == 0 {
                break;
            }
            stream.write_all(&buffer[..n_read])?;
        }
        
        stream.flush()?;
        Ok(true)
    }

    // Test a stream
    fn test_stream<R: Read>(&mut self, mut z_stream: R) -> Result<bool> {
        // This would test bzip2 file integrity
        // For now, just read through it
        let mut buffer = [0u8; 5000];
        
        while z_stream.read(&mut buffer)? > 0 {
            // Just reading through the data
        }
        
        Ok(true)
    }

    // Main compression function
    fn compress(&mut self, name: Option<&str>) -> Result<()> {
        self.config.delete_output_on_interrupt = false;

        match (name, self.config.src_mode) {
            (None, SM_I2O) => {
                self.copy_file_name(&mut self.file_names.in_name, "(stdin)")?;
                self.copy_file_name(&mut self.file_names.out_name, "(stdout)")?;
            }
            (Some(name), SM_F2F) => {
                self.copy_file_name(&mut self.file_names.in_name, name)?;
                self.copy_file_name(&mut self.file_names.out_name, name)?;
                self.file_names.out_name.push_str(".bz2");
            }
            (Some(name), SM_F2O) => {
                self.copy_file_name(&mut self.file_names.in_name, name)?;
                self.copy_file_name(&mut self.file_names.out_name, "(stdout)")?;
            }
            _ => return Err(BzipError::InvalidInput),
        }

        // Check input file exists for file modes
        if self.config.src_mode != SM_I2O {
            if !self.file_exists(&self.file_names.in_name) {
                if self.config.noisy {
                    eprintln!("{}: Can't open input file {}: No such file", 
                             self.file_names.prog_name, self.file_names.in_name);
                }
                self.config.exit_value = 1;
                return Ok(());
            }
        }

        // Check for existing output file
        if self.config.src_mode == SM_F2F && self.file_exists(&self.file_names.out_name) {
            if !self.config.force_overwrite {
                if self.config.noisy {
                    eprintln!("{}: Output file {} already exists.", 
                             self.file_names.prog_name, self.file_names.out_name);
                }
                self.config.exit_value = 1;
                return Ok(());
            }
        }

        // Open input and output streams
        match self.config.src_mode {
            SM_I2O => {
                let in_str = stdin();
                let out_str = stdout();
                
                // Removed atty check since feature is not properly configured
                self.compress_stream(in_str.lock(), out_str.lock())?;
            }
            SM_F2O => {
                let in_str = File::open(&self.file_names.in_name)?;
                let out_str = stdout();
                
                // Removed atty check since feature is not properly configured
                self.compress_stream(BufReader::new(in_str), out_str.lock())?;
            }
            SM_F2F => {
                let in_str = File::open(&self.file_names.in_name)?;
                let out_str = self.fopen_output_safely(&self.file_names.out_name)?;
                
                self.output_handle = Some(out_str.try_clone()?);
                self.config.delete_output_on_interrupt = true;
                
                self.compress_stream(BufReader::new(in_str), BufWriter::new(out_str))?;
                
                self.output_handle = None;
                self.config.delete_output_on_interrupt = false;
                
                // Delete input file if not keeping it
                if !self.config.keep_input_files {
                    remove_file(&self.file_names.in_name)?;
                }
            }
            _ => return Err(BzipError::InvalidInput),
        }

        if self.config.verbosity >= 1 {
            eprintln!("  {}: done", self.file_names.in_name);
        }

        Ok(())
    }

    // Main decompression function
    fn uncompress(&mut self, name: Option<&str>) -> Result<()> {
        self.config.delete_output_on_interrupt = false;

        let mut cant_guess = false;
        match (name, self.config.src_mode) {
            (None, SM_I2O) => {
                self.copy_file_name(&mut self.file_names.in_name, "(stdin)")?;
                self.copy_file_name(&mut self.file_names.out_name, "(stdout)")?;
            }
            (Some(name), SM_F2F) => {
                self.copy_file_name(&mut self.file_names.in_name, name)?;
                self.copy_file_name(&mut self.file_names.out_name, name)?;
                
                // Try to remove .bz2 suffix
                let suffixes = [".bz2", ".bz", ".tbz2", ".tbz"];
                let unz_suffixes = ["", "", ".tar", ".tar"];
                
                let mut found = false;
                for i in 0..BZ_N_SUFFIX_PAIRS {
                    if self.map_suffix(&mut self.file_names.out_name, suffixes[i], unz_suffixes[i]) {
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    cant_guess = true;
                    self.file_names.out_name.push_str(".out");
                }
            }
            (Some(name), SM_F2O) => {
                self.copy_file_name(&mut self.file_names.in_name, name)?;
                self.copy_file_name(&mut self.file_names.out_name, "(stdout)")?;
            }
            _ => return Err(BzipError::InvalidInput),
        }

        if cant_guess && self.config.noisy {
            eprintln!("{}: Can't guess original name for {} -- using {}", 
                     self.file_names.prog_name, self.file_names.in_name, self.file_names.out_name);
        }

        // Check input file exists for file modes
        if self.config.src_mode != SM_I2O && !self.file_exists(&self.file_names.in_name) {
            if self.config.noisy {
                eprintln!("{}: Can't open input file {}: No such file", 
                         self.file_names.prog_name, self.file_names.in_name);
            }
            self.config.exit_value = 1;
            return Ok(());
        }

        // Check for existing output file
        if self.config.src_mode == SM_F2F && self.file_exists(&self.file_names.out_name) {
            if !self.config.force_overwrite {
                if self.config.noisy {
                    eprintln!("{}: Output file {} already exists.", 
                             self.file_names.prog_name, self.file_names.out_name);
                }
                self.config.exit_value = 1;
                return Ok(());
            }
        }

        // Open input and output streams
        match self.config.src_mode {
            SM_I2O => {
                let in_str = stdin();
                let out_str = stdout();
                
                // Removed atty check since feature is not properly configured
                let magic_ok = self.uncompress_stream(in_str.lock(), out_str.lock())?;
                if !magic_ok {
                    self.config.unz_fails_exist = true;
                }
            }
            SM_F2O => {
                let in_str = File::open(&self.file_names.in_name)?;
                let out_str = stdout();
                
                let magic_ok = self.uncompress_stream(BufReader::new(in_str), out_str.lock())?;
                if !magic_ok {
                    self.config.unz_fails_exist = true;
                }
            }
            SM_F2F => {
                let in_str = File::open(&self.file_names.in_name)?;
                let out_str = self.fopen_output_safely(&self.file_names.out_name)?;
                
                self.output_handle = Some(out_str.try_clone()?);
                self.config.delete_output_on_interrupt = true;
                
                let magic_ok = self.uncompress_stream(BufReader::new(in_str), BufWriter::new(out_str))?;
                
                self.output_handle = None;
                self.config.delete_output_on_interrupt = false;
                
                if magic_ok {
                    if !self.config.keep_input_files {
                        remove_file(&self.file_names.in_name)?;
                    }
                } else {
                    self.config.unz_fails_exist = true;
                    remove_file(&self.file_names.out_name)?;
                }
            }
            _ => return Err(BzipError::InvalidInput),
        }

        if self.config.verbosity >= 1 {
            if self.config.src_mode == SM_F2F {
                eprintln!("  {}: done", self.file_names.in_name);
            }
        }

        Ok(())
    }

    // Test a file
    fn testf(&mut self, name: Option<&str>) -> Result<()> {
        self.config.delete_output_on_interrupt = false;

        match (name, self.config.src_mode) {
            (None, SM_I2O) => {
                self.copy_file_name(&mut self.file_names.in_name, "(stdin)")?;
            }
            (Some(name), _) => {
                self.copy_file_name(&mut self.file_names.in_name, name)?;
            }
            _ => return Err(BzipError::InvalidInput),
        }

        self.copy_file_name(&mut self.file_names.out_name, "(none)")?;

        // Check input file exists for file modes
        if self.config.src_mode != SM_I2O && !self.file_exists(&self.file_names.in_name) {
            if self.config.noisy {
                eprintln!("{}: Can't open input {}: No such file", 
                         self.file_names.prog_name, self.file_names.in_name);
            }
            self.config.exit_value = 1;
            return Ok(());
        }

        match self.config.src_mode {
            SM_I2O => {
                // Removed atty check since feature is not properly configured
                let in_str = stdin();
                let all_ok = self.test_stream(in_str.lock())?;
                if !all_ok {
                    self.config.test_fails_exist = true;
                }
            }
            SM_F2O | SM_F2F => {
                let in_str = File::open(&self.file_names.in_name)?;
                let all_ok = self.test_stream(BufReader::new(in_str))?;
                if !all_ok {
                    self.config.test_fails_exist = true;
                }
            }
            _ => return Err(BzipError::InvalidInput),
        }

        if self.config.verbosity >= 1 {
            eprintln!("  {}: ok", self.file_names.in_name);
        }

        Ok(())
    }

    // Show license information
    fn license(&self) {
        eprintln!(
            "bzip2, a block-sorting file compressor.  Version {}.\n\
             \n\
             Copyright (C) 1996-2019 by Julian Seward.\n\
             \n\
             This program is free software; you can redistribute it and/or modify\n\
             it under the terms set out in the LICENSE file, which is included\n\
             in the bzip2 source distribution.\n\
             \n\
             This program is distributed in the hope that it will be useful,\n\
             but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
             MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
             LICENSE file for more details.\n",
            "1.0.8" // Would use actual version from bzip2 library
        );
    }

    // Show usage information
    fn usage(&self, full_prog_name: &str) {
        eprintln!(
            "bzip2, a block-sorting file compressor.  Version {}.\n\
             \n   usage: {} [flags and input files in any order]\n\
             \n\
             -h --help           print this message\n\
             -d --decompress     force decompression\n\
             -z --compress       force compression\n\
             -k --keep           keep (don't delete) input files\n\
             -f --force          overwrite existing output files\n\
             -t --test           test compressed file integrity\n\
             -c --stdout         output to standard out\n\
             -q --quiet          suppress noncritical error messages\n\
             -v --verbose        be verbose (a 2nd -v gives more)\n\
             -L --license        display software version & license\n\
             -V --version        display software version & license\n\
             -s --small          use less memory (at most 2500k)\n\
             -1 .. -9            set block size to 100k .. 900k\n\
             --fast              alias for -1\n\
             --best              alias for -9\n\
             \n\
             If invoked as `bzip2', default action is to compress.\n\
                        as `bunzip2',  default action is to decompress.\n\
                        as `bzcat', default action is to decompress to stdout.\n\
             \n\
             If no file names are given, bzip2 compresses or decompresses\n\
             from standard input to standard output.  You can combine\n\
             short flags, so `-v -4' means the same as -v4 or -4v, &c.\n",
            "1.0.8", // Would use actual version from bzip2 library
            full_prog_name
        );
    }

    // Clean up and exit
    fn clean_up_and_fail(&mut self, ec: i32) -> ! {
        if self.config.src_mode == SM_F2F 
            && self.config.op_mode != OM_TEST
            && self.config.delete_output_on_interrupt {
            
            // Check if input file still exists before deleting output
            if self.file_exists(&self.file_names.in_name) {
                if self.config.noisy {
                    eprintln!("{}: Deleting output file {}, if it exists.",
                             self.file_names.prog_name, self.file_names.out_name);
                }
                if let Some(handle) = &self.output_handle {
                    drop(handle); // Close the file handle
                }
                let _ = remove_file(&self.file_names.out_name);
            }
        }

        if self.config.noisy 
            && self.config.num_file_names > 0 
            && self.config.num_files_processed < self.config.num_file_names {
            eprintln!(
                "{}: WARNING: some files have not been processed:\n\
                 {}:    {} specified on command line, {} not processed yet.\n",
                self.file_names.prog_name, self.file_names.prog_name,
                self.config.num_file_names, 
                self.config.num_file_names - self.config.num_files_processed
            );
        }

        self.config.exit_value = ec;
        process::exit(self.config.exit_value);
    }

    // Main entry point
    pub fn run<I, T>(&mut self, args: I) -> Result<()> 
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args: Vec<OsString> = args.into_iter().map(|a| a.into()).collect();
        
        // Set program name
        if let Some(arg0) = args.get(0).and_then(|a| a.to_str()) {
            self.file_names.prog_name_really = arg0.to_string();
            self.file_names.prog_name = Path::new(arg0)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("bzip2")
                .to_string();
        } else {
            self.file_names.prog_name_really = "bzip2".to_string();
            self.file_names.prog_name = "bzip2".to_string();
        }

        // Parse command line arguments
        let mut file_names = Vec::new();
        let mut decode = true;

        for arg in args.iter().skip(1) {
            if let Some(arg_str) = arg.to_str() {
                if decode && arg_str == "--" {
                    decode = false;
                    continue;
                }
                
                if decode && arg_str.starts_with('-') {
                    // Parse flags
                    self.parse_flag(arg_str)?;
                } else {
                    file_names.push(arg_str.to_string());
                }
            }
        }

        self.config.num_file_names = file_names.len() as i32;

        // Determine operation mode based on program name
        let prog_lower = self.file_names.prog_name.to_lowercase();
        if prog_lower.contains("unzip") {
            self.config.op_mode = OM_UNZ;
        } else if prog_lower.contains("zcat") || prog_lower.contains("z2cat") {
            self.config.op_mode = OM_UNZ;
            if self.config.num_file_names == 0 {
                self.config.src_mode = SM_I2O;
            } else {
                self.config.src_mode = SM_F2O;
            }
        }

        // Process files based on operation mode
        match self.config.op_mode {
            OM_Z => {
                if self.config.src_mode == SM_I2O {
                    self.compress(None)?;
                } else {
                    for file_name in &file_names {
                        self.config.num_files_processed += 1;
                        self.compress(Some(file_name))?;
                    }
                }
            }
            OM_UNZ => {
                if self.config.src_mode == SM_I2O {
                    self.uncompress(None)?;
                } else {
                    for file_name in &file_names {
                        self.config.num_files_processed += 1;
                        self.uncompress(Some(file_name))?;
                    }
                }
                if self.config.unz_fails_exist {
                    self.config.exit_value = 2;
                }
            }
            OM_TEST => {
                if self.config.src_mode == SM_I2O {
                    self.testf(None)?;
                } else {
                    for file_name in &file_names {
                        self.config.num_files_processed += 1;
                        self.testf(Some(file_name))?;
                    }
                }
                if self.config.test_fails_exist {
                    self.config.exit_value = 2;
                }
            }
            _ => return Err(BzipError::InvalidInput),
        }

        Ok(())
    }

    // Parse command line flags
    fn parse_flag(&mut self, flag: &str) -> Result<()> {
        match flag {
            "-h" | "--help" => {
                self.usage(&self.file_names.prog_name_really);
                process::exit(0);
            }
            "-d" | "--decompress" => {
                self.config.op_mode = OM_UNZ;
            }
            "-z" | "--compress" => {
                self.config.op_mode = OM_Z;
            }
            "-k" | "--keep" => {
                self.config.keep_input_files = true;
            }
            "-f" | "--force" => {
                self.config.force_overwrite = true;
            }
            "-t" | "--test" => {
                self.config.op_mode = OM_TEST;
            }
            "-c" | "--stdout" => {
                self.config.src_mode = SM_F2O;
            }
            "-q" | "--quiet" => {
                self.config.noisy = false;
            }
            "-v" | "--verbose" => {
                self.config.verbosity += 1;
            }
            "-L" | "--license" | "-V" | "--version" => {
                self.license();
                process::exit(0);
            }
            "-s" | "--small" => {
                self.config.small_mode = true;
            }
            "--fast" => {
                self.config.block_size_100k = 1;
            }
            "--best" => {
                self.config.block_size_100k = 9;
            }
            _ => {
                // Handle numeric compression levels
                if flag.starts_with('-') && flag.len() == 2 {
                    if let Some(c) = flag.chars().nth(1) {
                        if let Some(digit) = c.to_digit(10) {
                            if (1..=9).contains(&digit) {
                                self.config.block_size_100k = digit as i32;
                                return Ok(());
                            }
                        }
                    }
                }
                
                eprintln!("{}: Bad flag `{}`", self.file_names.prog_name, flag);
                self.usage(&self.file_names.prog_name_really);
                process::exit(1);
            }
        }
        Ok(())
    }
}

impl Default for Bzip2App {
    fn default() -> Self {
        Self::new()
    }
}

// Main function
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = Bzip2App::new();
    
    match app.run(args) {
        Ok(()) => {
            process::exit(app.config.exit_value);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name_copy() {
        let mut app = Bzip2App::new();
        let mut dest = String::new();
        assert!(app.copy_file_name(&mut dest, "test.txt").is_ok());
        assert_eq!(dest, "test.txt");
    }

    #[test]
    fn test_has_suffix() {
        let app = Bzip2App::new();
        assert!(app.has_suffix("file.bz2", ".bz2"));
        assert!(!app.has_suffix("file.txt", ".bz2"));
    }

    #[test]
    fn test_map_suffix() {
        let app = Bzip2App::new();
        let mut name = "file.bz2".to_string();
        assert!(app.map_suffix(&mut name, ".bz2", ""));
        assert_eq!(name, "file");
    }
}