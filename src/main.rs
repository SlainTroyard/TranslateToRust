use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{info, warn};
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;

mod translation;
mod testing;
mod utils;
mod llm_api;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Translate a C/C++ solution to Rust
    Translate {
        /// The contest number (e.g., 413)
        #[arg(short, long)]
        contest: u32,
        
        /// The problem number (e.g., 1)
        #[arg(short, long)]
        problem: u32,
        
        /// The language (C or CPP)
        #[arg(short, long, default_value = "CPP")]
        language: String,
        
        /// The method to use for translation [llm]
        #[arg(short, long, default_value = "llm")]
        method: String,

        /// Optional output directory path
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Test a Rust solution against the original test cases
    Test {
        /// The contest number (e.g., 413)
        #[arg(short, long)]
        contest: u32,
        
        /// The problem number (e.g., 1)
        #[arg(short, long)]
        problem: u32,
        
        /// The language of the original solution (C or CPP)
        #[arg(short, long, default_value = "CPP")]
        language: String,
        
        /// The path to the Rust solution file
        #[arg(short, long)]
        rust_solution: PathBuf,
    },
    
    /// Run the entire pipeline: translate and test
    Run {
        /// The contest number (e.g., 413)
        #[arg(short, long)]
        contest: u32,
        
        /// The problem number (e.g., 1)
        #[arg(short, long)]
        problem: u32,
        
        /// The language of the original solution (C or CPP)
        #[arg(short, long, default_value = "CPP")]
        language: String,
        
        /// The method to use for translation [llm]
        #[arg(short, long, default_value = "llm")]
        method: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Translate { contest, problem, language, method, output_dir } => {
            info!("Translating Weekly Contest {} Problem {} ({}) using method: {}", contest, problem, language, method);
            let result = translation::translate_solution(*contest, *problem, language, method, output_dir.clone())?;
            println!("Translation completed. Rust solution saved to: {}", result.display());
            Ok(())
        }
        
        Commands::Test { contest, problem, language, rust_solution } => {
            info!("Testing Rust solution for Weekly Contest {} Problem {} against original {} test cases", contest, problem, language);
            let test_result = testing::test_solution(*contest, *problem, language, rust_solution)?;
            report_test_results(&test_result);
            Ok(())
        }
        
        Commands::Run { contest, problem, language, method } => {
            info!("Running full pipeline for Weekly Contest {} Problem {} ({})", contest, problem, language);
            
            // Step 1: Translate the solution
            info!("Step 1: Translating solution using method: {}", method);
            let rust_solution = translation::translate_solution(*contest, *problem, language, method, None)
                .context("Failed to translate solution")?;
            
            // Step 2: Test the solution
            info!("Step 2: Testing translated solution");
            let test_result = testing::test_solution(*contest, *problem, language, &rust_solution)
                .context("Failed to test solution")?;
            
            // Report results
            report_test_results(&test_result);
            
            Ok(())
        }
    }
}

fn report_test_results(results: &testing::TestResults) {
    println!("\n==== Test Results ====");
    println!("测试文件: {}", results.file_name);
    
    // 报告编译状态
    if !results.compilation_success {
        println!("编译状态: 失败");
        if let Some(err) = &results.compilation_error {
            println!("编译错误: {}", err);
        }
        
        // 保存测试结果到文件
        if let Err(e) = save_test_results(results) {
            warn!("Failed to save test results: {}", e);
        }
        return;
    }
    
    println!("编译状态: 成功");
    println!("Total Test Cases: {}", results.total);
    println!("Passed: {}", results.passed);
    println!("Failed: {}", results.failed);
    println!("Success Rate: {:.2}%", 100.0 * results.passed as f64 / results.total as f64);
    println!("平均运行时间: {:.2} ms", results.average_runtime);
    
    // 添加有关大型输出的特别说明
    let has_large_outputs = results.all_cases.iter()
        .any(|case| case.actual_output.contains("[输出被截断") || 
                    case.actual_output.contains("[输出未完全读取"));
    
    if has_large_outputs {
        println!("\n注意: 部分测试用例的输出很大，已被截断或未完全读取。");
        println!("详细的测试结果已保存到文件中供后续分析。");
    }
    
    if !results.failed_cases.is_empty() {
        println!("\nFailed Test Cases:");
        for (i, case) in results.failed_cases.iter().enumerate().take(5) { // 只显示前5个失败案例
            println!("Case {}:", i + 1);
            
            // 显示输入，可能需要截断
            let truncated_input = truncate_for_display(&case.input, 500);
            println!("Input:\n{}", truncated_input);
            
            // 显示预期输出，可能需要截断
            let truncated_expected = truncate_for_display(&case.expected_output, 500);
            println!("Expected:\n{}", truncated_expected);
            
            // 显示实际输出，可能需要截断
            let truncated_actual = truncate_for_display(&case.actual_output, 500);
            println!("Actual:\n{}", truncated_actual);
            
            println!();
        }
        
        if results.failed_cases.len() > 5 {
            println!("... 还有 {} 个失败的测试用例未显示。查看保存的测试结果文件获取完整信息。", 
                     results.failed_cases.len() - 5);
        }
    }
    
    // 保存测试结果到文件
    if let Err(e) = save_test_results(results) {
        warn!("Failed to save test results: {}", e);
    }
}

/// 截断用于显示的字符串
fn truncate_for_display(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    
    let truncated: String = text.chars().take(max_chars).collect();
    format!("{}...\n[显示被截断，完整内容请查看测试结果文件]", truncated)
}

/// 将测试结果保存到文件
fn save_test_results(results: &testing::TestResults) -> Result<()> {
    // 创建results目录（如果不存在）
    let results_dir = PathBuf::from("./test_results");
    fs::create_dir_all(&results_dir)?;
    
    // 创建详细测试信息的目录
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let details_dir = results_dir.join(format!("details_{}", timestamp));
    fs::create_dir_all(&details_dir)?;
    
    // 生成摘要文件名
    let summary_file = results_dir.join(format!("test_results_{}.txt", timestamp));
    
    // 写入摘要文件
    let mut summary = File::create(&summary_file)?;
    writeln!(summary, "==== Test Results ====\n")?;
    writeln!(summary, "测试文件: {}", results.file_name)?;
    
    if !results.compilation_success {
        writeln!(summary, "编译状态: 失败")?;
        if let Some(err) = &results.compilation_error {
            writeln!(summary, "编译错误: {}", err)?;
        }
        writeln!(summary, "\n结果保存时间: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        println!("测试结果已保存到: {}", summary_file.display());
        return Ok(());
    }
    
    writeln!(summary, "编译状态: 成功")?;
    writeln!(summary, "总测试用例: {}", results.total)?;
    writeln!(summary, "通过: {}", results.passed)?;
    writeln!(summary, "失败: {}", results.failed)?;
    writeln!(summary, "成功率: {:.2}%", 100.0 * results.passed as f64 / results.total as f64)?;
    writeln!(summary, "平均运行时间: {:.2} ms", results.average_runtime)?;
    
    // 检查是否有大型输出
    let has_large_outputs = results.all_cases.iter()
        .any(|case| case.actual_output.contains("[输出被截断") || 
                   case.actual_output.contains("[输出未完全读取"));
    
    if has_large_outputs {
        writeln!(summary, "\n注意: 部分测试用例的输出很大，已被截断或未完全读取。")?;
        writeln!(summary, "这可能是由于程序生成了过多的输出数据，超出了管道缓冲区的容量。")?;
        writeln!(summary, "如有必要，请修改程序减少输出量，或考虑将输出写入文件而不是标准输出。")?;
    }
    
    // 写入失败案例的摘要
    if !results.failed_cases.is_empty() {
        writeln!(summary, "\n失败的测试用例:")?;
        for (i, _case) in results.failed_cases.iter().enumerate() {
            writeln!(summary, "用例 {}: 详情见 {}", i + 1, details_dir.join(format!("case_failed_{}.txt", i + 1)).display())?;
        }
    }
    
    writeln!(summary, "\n详细测试信息目录: {}", details_dir.display())?;
    writeln!(summary, "结果保存时间: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    
    // 创建所有测试用例的摘要文件
    let all_cases_file = details_dir.join("all_cases_summary.txt");
    let mut all_cases_summary = File::create(all_cases_file)?;
    
    writeln!(all_cases_summary, "==== 所有测试用例摘要 ====\n")?;
    writeln!(all_cases_summary, "总计: {} 个测试用例", results.total)?;
    writeln!(all_cases_summary, "通过: {} 个", results.passed)?;
    writeln!(all_cases_summary, "失败: {} 个\n", results.failed)?;
    
    // 分批写入所有测试用例的简要信息
    for (i, case) in results.all_cases.iter().enumerate() {
        writeln!(all_cases_summary, "用例 {}: {} - 运行时间: {:.2} ms", 
                i + 1, 
                if case.is_passed { "通过" } else { "失败" },
                case.runtime)?;
    }
    
    // 分别保存每个测试用例的详细信息
    for (i, case) in results.all_cases.iter().enumerate() {
        let case_status = if case.is_passed { "passed" } else { "failed" };
        let case_file = details_dir.join(format!("case_{}_{}.txt", case_status, i + 1));
        
        // 分块写入测试用例，避免一次性写入过多数据
        let mut case_details = File::create(case_file)?;
        writeln!(case_details, "==== 测试用例 {} ({}) ====\n", i + 1, case_status)?;
        
        // 写入输入
        writeln!(case_details, "输入:")?;
        write_chunked(&mut case_details, &case.input)?;
        writeln!(case_details)?;
        
        // 写入期望输出
        writeln!(case_details, "期望输出:")?;
        write_chunked(&mut case_details, &case.expected_output)?;
        writeln!(case_details)?;
        
        // 写入实际输出
        writeln!(case_details, "实际输出:")?;
        write_chunked(&mut case_details, &case.actual_output)?;
        writeln!(case_details)?;
        
        // 写入运行时间
        writeln!(case_details, "运行时间: {:.2} ms", case.runtime)?;
    }
    
    println!("测试结果已保存到: {}", summary_file.display());
    
    Ok(())
}

/// 分块写入大文本，避免内存问题
fn write_chunked<W: Write>(writer: &mut W, text: &str) -> Result<()> {
    const CHUNK_SIZE: usize = 8192;  // 8KB 块大小
    
    // 检查文本是否特别大
    if text.len() > 1_000_000 {  // 超过1MB的文本
        // 只写入前10KB和统计信息
        let preview: String = text.chars().take(10_000).collect();
        writeln!(writer, "{}", preview)?;
        writeln!(writer, "... [文本过长，已截断。总大小: {} 字节]", text.len())?;
        return Ok(());
    }
    
    // 对于中等大小的文本，分块写入
    if text.len() > CHUNK_SIZE {
        for chunk in text.chars().collect::<Vec<_>>().chunks(CHUNK_SIZE) {
            let chunk_str: String = chunk.iter().collect();
            write!(writer, "{}", chunk_str)?;
        }
    } else {
        // 对于小文本，直接写入
        write!(writer, "{}", text)?;
    }
    
    Ok(())
} 