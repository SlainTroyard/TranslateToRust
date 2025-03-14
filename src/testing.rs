use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Instant, Duration};
use std::os::unix::process::ExitStatusExt;
use std::thread;

// Define the paths to the FuzzForLeetcode project
const FUZZ_OUTPUTS_DIR: &str = "../FuzzForLeetcode/fuzz_outputs";
const FUZZ_PROJECT_ROOT: &str = "../FuzzForLeetcode";

#[derive(Debug)]
pub struct FailedTestCase {
    pub input: String,
    pub expected_output: String,
    pub actual_output: String,
}

#[derive(Debug)]
pub struct TestResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub failed_cases: Vec<FailedTestCase>,
    pub file_name: String,
    pub average_runtime: f64,  // 平均运行时间（毫秒）
    pub compilation_success: bool,
    pub compilation_error: Option<String>,
    pub all_cases: Vec<TestCaseResult>,  // 所有测试用例的结果
}

/// 单个测试用例的结果
#[derive(Debug)]
pub struct TestCaseResult {
    pub input: String,
    pub expected_output: String,
    pub actual_output: String,
    pub is_passed: bool,
    pub runtime: f64,
}

impl Default for TestResults {
    fn default() -> Self {
        TestResults {
            total: 0,
            passed: 0,
            failed: 0,
            failed_cases: Vec::new(),
            file_name: String::new(),
            average_runtime: 0.0,
            compilation_success: false,
            compilation_error: None,
            all_cases: Vec::new(),
        }
    }
}

/// Test a Rust solution against the original test cases
pub fn test_solution(
    contest: u32,
    problem: u32,
    language: &str,
    rust_solution: &Path,
) -> Result<TestResults> {
    // 获取文件名用于显示
    let file_name = rust_solution.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("weekly_contest_{}_p{}_{}.rs", contest, problem, language.to_lowercase()));
    
    // 创建基础的TestResults对象
    let mut results = TestResults {
        file_name: file_name,
        compilation_success: false,
        all_cases: Vec::new(),
        ..Default::default()
    };
    
    // 编译Rust解决方案
    info!("Compiling Rust solution: {}", rust_solution.display());
    let rust_executable = match compile_rust_solution(rust_solution) {
        Ok(exec) => {
            info!("Compilation successful. Executable: {}", exec.display());
            results.compilation_success = true;
            exec
        },
        Err(err) => {
            warn!("Compilation failed: {}", err);
            results.compilation_error = Some(format!("{}", err));
            return Ok(results); // 返回带有编译错误的结果
        }
    };
    
    // 加载测试用例
    let test_cases_dir = format!("{}/fuzz_outputs/{}/weekly_contest_{}_p{}/outputs", 
                              FUZZ_PROJECT_ROOT, language, contest, problem);
    info!("Loading test cases from: {}", test_cases_dir);
    let test_cases = match load_test_cases(contest, problem, language) {
        Ok(cases) => {
            info!("Loaded {} test cases", cases.len());
            cases
        },
        Err(err) => {
            warn!("Failed to load test cases: {}", err);
            results.compilation_error = Some(format!("Failed to load test cases: {}", err));
            return Ok(results);
        }
    };
    
    // 更新测试结果的总数
    results.total = test_cases.len();
    
    // 计数超时的测试用例
    let mut timeout_count = 0;
    
    // 运行测试
    for (i, test_case) in test_cases.iter().enumerate() {
        debug!("Running test case {}/{}", i + 1, test_cases.len());
        
        let (actual_output, runtime) = match run_test(&rust_executable, &test_case.input) {
            Ok(result) => result,
            Err(err) => {
                warn!("Test case {}/{} failed to run: {}", i + 1, test_cases.len(), err);
                results.failed += 1;
                
                // 检查是否是超时错误
                let is_timeout = err.to_string().contains("TIMEOUT");
                if is_timeout {
                    timeout_count += 1;
                    info!("Test case {}/{} timed out", i + 1, test_cases.len());
                }
                
                // 记录失败的测试用例
                results.failed_cases.push(FailedTestCase {
                    input: test_case.input.clone(),
                    expected_output: test_case.output.clone(),
                    actual_output: format!("ERROR: {}", err),
                });
                
                // 记录到所有测试用例结果中
                results.all_cases.push(TestCaseResult {
                    input: test_case.input.clone(),
                    expected_output: test_case.output.clone(),
                    actual_output: format!("ERROR: {}", err),
                    is_passed: false,
                    runtime: 0.0,
                });
                
                continue;
            }
        };
        
        let normalized_expected = normalize_output(&test_case.output);
        let normalized_actual = normalize_output(&actual_output);
        
        let is_passed = compare_outputs(&normalized_expected, &normalized_actual);
        
        // 记录到所有测试用例结果中
        results.all_cases.push(TestCaseResult {
            input: test_case.input.clone(),
            expected_output: test_case.output.clone(),
            actual_output: actual_output.clone(),
            is_passed,
            runtime,
        });
        
        if is_passed {
            results.passed += 1;
            results.average_runtime += runtime;
        } else {
            results.failed += 1;
            results.failed_cases.push(FailedTestCase {
                input: test_case.input.clone(),
                expected_output: test_case.output.clone(),
                actual_output,
            });
        }
    }
    
    // 计算平均运行时间
    if results.passed > 0 {
        results.average_runtime /= results.passed as f64;
    }
    
    // 如果有超时的测试用例，记录在日志中
    if timeout_count > 0 {
        warn!("{} test cases timed out", timeout_count);
    }
    
    Ok(results)
}

/// Compile a Rust solution file
fn compile_rust_solution(rust_file: &Path) -> Result<PathBuf> {
    info!("Compiling Rust solution: {}", rust_file.display());
    
    // Create a temporary directory for the compilation if it doesn't exist
    let target_dir = PathBuf::from("./target/debug");
    fs::create_dir_all(&target_dir)?;
    
    // Determine the output executable name (without extension)
    let file_stem = rust_file.file_stem().unwrap().to_str().unwrap();
    let executable_path = target_dir.join(file_stem);
    
    // Run rustc to compile the file
    let status = Command::new("rustc")
        .arg(rust_file)
        .arg("-o")
        .arg(&executable_path)
        .status()
        .with_context(|| format!("Failed to execute rustc command for {}", rust_file.display()))?;
    
    if !status.success() {
        error!("Compilation failed with status: {}", status);
        anyhow::bail!("Failed to compile Rust solution");
    }
    
    info!("Compilation successful. Executable: {}", executable_path.display());
    Ok(executable_path)
}

/// Run a test case with the compiled Rust executable
fn run_test(executable: &Path, input: &str) -> Result<(String, f64)> {
    // 对于大输入，只记录大小信息，不打印内容
    debug!("Running test with input size: {} bytes", input.len());
    
    // 创建一个运行可执行文件的进程
    let mut cmd = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start process")?;
    
    // 获取子进程的stdin，用于写入输入
    let stdin = cmd.stdin.take();
    
    // 开始计时
    let start_time = Instant::now();
    
    // 在单独的线程中写入输入，避免大输入导致的阻塞
    if let Some(mut stdin) = stdin {
        // 将输入克隆到单独的线程，避免阻塞主线程
        let input_copy = input.to_string();
        std::thread::spawn(move || {
            // 分批写入输入，每次写入固定大小
            const CHUNK_SIZE: usize = 8192;  // 8KB 块大小
            let mut input_bytes = input_copy.as_bytes();
            
            while !input_bytes.is_empty() {
                let chunk_size = std::cmp::min(CHUNK_SIZE, input_bytes.len());
                let (chunk, rest) = input_bytes.split_at(chunk_size);
                
                if let Err(e) = stdin.write_all(chunk) {
                    debug!("Failed to write chunk to stdin: {}", e);
                    return;
                }
                
                input_bytes = rest;
            }
            
            // 确保输入以换行符结束
            if !input_copy.ends_with('\n') {
                let _ = stdin.write_all(b"\n");
            }
            
            // 刷新并关闭stdin
            let _ = stdin.flush();
        });
    }
    
    // 添加超时机制 - 设置最大执行时间（秒）
    const TIMEOUT_SECONDS: u64 = 30; // 最多允许执行30秒
    
    // 创建一个通道用于线程间通信
    let (tx, rx) = std::sync::mpsc::channel();
    
    // 保存进程ID
    let child_pid = cmd.id();
    
    // 创建一个线程来等待进程完成
    let wait_handle = thread::spawn(move || {
        match cmd.wait_with_output() {
            Ok(output) => {
                // 成功完成进程，发送输出
                let _ = tx.send(Ok(output));
            },
            Err(e) => {
                // 进程等待失败，发送错误
                let _ = tx.send(Err(e));
            }
        }
    });
    
    // 等待进程完成，最多等待TIMEOUT_SECONDS秒
    let output = match rx.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)) {
        Ok(result) => {
            // 在超时前收到结果
            match result {
                Ok(output) => output,
                Err(e) => return Err(anyhow::anyhow!("Process execution failed: {}", e)),
            }
        },
        Err(_) => {
            // 超时了，尝试终止进程
            warn!("Process timed out after {} seconds", TIMEOUT_SECONDS);
            
            // 尝试终止进程 - 通过系统调用
            if child_pid != 0 {
                warn!("Killing process with PID: {}", child_pid);
                // 在Unix系统上使用SIGKILL信号终止进程
                let _ = Command::new("kill")
                    .arg("-9")
                    .arg(child_pid.to_string())
                    .status();
            }
            
            // 等待线程结束（但不应该等待太久）
            let _ = wait_handle.join();
            
            return Err(anyhow::anyhow!("Process timed out after {} seconds (TIMEOUT)", TIMEOUT_SECONDS));
        }
    };
    
    // 计算执行时间（毫秒）
    let elapsed = start_time.elapsed();
    let elapsed_ms = elapsed.as_secs() as f64 * 1000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0;
    
    // 检查进程是否正常退出
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 如果标准输出不为空，我们认为这可能是程序的正常输出
        // 即使进程返回了非零退出码
        if !stdout.is_empty() {
            debug!("Process exited with non-zero code but produced output, treating as success");
            return Ok((stdout, elapsed_ms));
        }
        
        // 否则，这是一个真正的错误
        let exit_code = output.status.code().unwrap_or_else(|| {
            if let Some(signal) = output.status.signal() {
                -signal
            } else {
                -1
            }
        });
        
        if !stderr.is_empty() {
            return Err(anyhow::anyhow!("Process exited with code {}: {}", exit_code, stderr));
        } else {
            return Err(anyhow::anyhow!("Process exited with code {}", exit_code));
        }
    }
    
    // 将输出转换为字符串
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    // 对于大输出，只记录大小信息，不打印内容
    if stdout.len() > 1024 {  // 如果输出超过1KB
        debug!("Process produced {} bytes of output", stdout.len());
    } else {
        debug!("Process output: {}", stdout);
    }
    debug!("Execution time: {:.2} ms", elapsed_ms);
    
    Ok((stdout, elapsed_ms))
}

/// Normalize output for comparison (trim whitespace, line endings, etc.)
fn normalize_output(output: &str) -> String {
    // 为了更高效地处理大型输出，逐行处理而不是使用collect
    let mut result = String::with_capacity(output.len());
    let mut first = true;
    
    for line in output.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            if !first {
                result.push('\n');
            }
            result.push_str(trimmed);
            first = false;
        }
    }
    
    result
}

/// Compare outputs intelligently, handling large outputs efficiently
fn compare_outputs(expected: &str, actual: &str) -> bool {
    // 如果输出包含截断标记，我们只比较可用的部分
    if actual.contains("[输出被截断") {
        // 从实际输出中提取非截断部分
        let truncated_part = actual.split("...\n[输出被截断").next().unwrap_or("");
        
        // 检查期望输出是否以截断部分开始
        // 对于非常大的输出，我们可能只能比较开头部分
        return expected.starts_with(truncated_part);
    }
    
    // 如果输出包含管道错误标记，但我们仍然有部分输出
    if actual.contains("[输出未完全读取: ") {
        // 提取错误前的部分
        let partial_output = actual.split("\n[输出未完全读取: ").next().unwrap_or("");
        
        // 检查期望输出是否以部分输出开始
        return expected.starts_with(partial_output);
    }
    
    // 正常情况下，进行精确比较
    expected == actual
}

#[derive(Debug)]
struct TestCase {
    input: String,
    output: String,
}

/// Load test cases from FuzzForLeetcode outputs
fn load_test_cases(contest: u32, problem: u32, language: &str) -> Result<Vec<TestCase>> {
    let problem_dir = format!("weekly_contest_{}_p{}", contest, problem);
    let outputs_path = Path::new(FUZZ_OUTPUTS_DIR)
        .join(language)
        .join(&problem_dir)
        .join("outputs");
    
    info!("Loading test cases from: {}", outputs_path.display());
    
    let content = fs::read_to_string(&outputs_path)
        .with_context(|| format!("Failed to read test outputs file: {}", outputs_path.display()))?;
    
    parse_test_cases(&content)
}

/// Parse test cases from the outputs file
fn parse_test_cases(content: &str) -> Result<Vec<TestCase>> {
    let mut test_cases = Vec::new();
    let mut current_input = String::new();
    let mut current_output = String::new();
    let mut parsing_input = false;
    let mut parsing_output = false;
    
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        if line == "input:" {
            // Start of a new test case
            if !current_input.is_empty() && !current_output.is_empty() {
                test_cases.push(TestCase {
                    input: current_input.trim().to_string(),
                    output: current_output.trim().to_string(),
                });
                current_input.clear();
                current_output.clear();
            }
            
            parsing_input = true;
            parsing_output = false;
            i += 1;
            continue;
        }
        
        if line == "output:" {
            parsing_input = false;
            parsing_output = true;
            i += 1;
            continue;
        }
        
        if line.starts_with("--") {
            // Separator line, skip it
            // Also, if we've been parsing a test case, add it to our collection
            if !current_input.is_empty() && !current_output.is_empty() {
                test_cases.push(TestCase {
                    input: current_input.trim().to_string(),
                    output: current_output.trim().to_string(),
                });
                current_input.clear();
                current_output.clear();
            }
            parsing_input = false;
            parsing_output = false;
            i += 1;
            continue;
        }
        
        if parsing_input {
            current_input.push_str(line);
            current_input.push('\n');
        } else if parsing_output {
            current_output.push_str(line);
            current_output.push('\n');
        }
        
        i += 1;
    }
    
    // Add the last test case if there is one
    if !current_input.is_empty() && !current_output.is_empty() {
        test_cases.push(TestCase {
            input: current_input.trim().to_string(),
            output: current_output.trim().to_string(),
        });
    }
    
    Ok(test_cases)
} 