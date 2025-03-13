use anyhow::{bail, Context, Result};
use log::{debug, error, info, warn};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use regex;

// Define the paths to the FuzzForLeetcode project
const FUZZ_PROJECT_ROOT: &str = "../FuzzForLeetcode";
const CPP_SRC_DIR: &str = "../FuzzForLeetcode/C_CPP/CPP/src";
const C_SRC_DIR: &str = "../FuzzForLeetcode/C_CPP/C/src";

// LLM provider configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    GoogleAI,
    Local,
}

// 定义模型参数作为一个单独的结构体，所有字段都是Option类型
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelParams {
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    
    // 允许任意其他字段，支持不同模型的专有参数
    #[serde(flatten)]
    pub extra_params: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: String,
    pub api_url: String,
    pub default_model: String,
    
    // 所有模型特定参数都集中在这里，使它们成为可选的
    #[serde(default)]
    pub model_params: ModelParams,
    
    // HTTP头
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    
    // 系统消息
    #[serde(default)]
    pub system_message: Option<String>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: LlmProvider::OpenAI,
            api_key: String::new(),
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            default_model: "gpt-4".to_string(),
            model_params: ModelParams::default(),
            headers: {
                let mut headers = std::collections::HashMap::new();
                headers.insert("content-type".to_string(), "application/json".to_string());
                headers
            },
            system_message: Some("You are a C/C++ to Rust code translator expert.".to_string()),
        }
    }
}

/// Translate a C/C++ solution to Rust using the specified method
pub fn translate_solution(
    contest: u32,
    problem: u32,
    language: &str,
    method: &str,
    output_dir: Option<PathBuf>,
) -> Result<PathBuf> {
    // 1. First, read the source code of the original solution
    let source_file = format!(
        "weekly_contest_{}_p{}.{}",
        contest,
        problem,
        if language == "CPP" { "cpp" } else { "c" }
    );

    let source_dir = if language == "CPP" { CPP_SRC_DIR } else { C_SRC_DIR };
    let source_path = Path::new(source_dir).join(&source_file);
    
    let source_code = fs::read_to_string(&source_path)
        .with_context(|| format!("Failed to read source file: {}", source_path.display()))?;
    
    debug!("Read source code from {}", source_path.display());
    info!("Source code is {} bytes", source_code.len());
    
    // 2. Translate the source code to Rust using the specified method
    let rust_code = match method {
        "llm" => {
            // Load LLM configuration from environment or config file
            let llm_config = load_llm_config()?;
            translate_with_llm(&source_code, language, contest, problem, &llm_config)?
        },
        _ => bail!("Unsupported translation method: {}", method),
    };
    
    // 3. Save the translated Rust code to a file
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from("./translated"));
    fs::create_dir_all(&output_dir)?;
    
    let rust_file = format!("weekly_contest_{}_p{}.rs", contest, problem);
    let rust_path = output_dir.join(&rust_file);
    
    let mut file = File::create(&rust_path)
        .with_context(|| format!("Failed to create Rust file: {}", rust_path.display()))?;
    
    file.write_all(rust_code.as_bytes())
        .with_context(|| format!("Failed to write to Rust file: {}", rust_path.display()))?;
    
    info!("Translated Rust code saved to {}", rust_path.display());
    
    Ok(rust_path)
}

/// Load LLM configuration from environment variables or config file
fn load_llm_config() -> Result<LlmConfig> {
    info!("开始尝试加载LLM配置...");
    
    // First try to read from environment variables
    if let Ok(provider) = env::var("LLM_PROVIDER") {
        info!("从环境变量加载配置，提供商: {}", provider);
        let provider = match provider.to_lowercase().as_str() {
            "openai" => LlmProvider::OpenAI,
            "anthropic" => LlmProvider::Anthropic,
            "google" => LlmProvider::GoogleAI,
            "local" => {
                info!("使用本地模式，不需要API密钥");
                // For Local provider, we don't need an API key
                return Ok(LlmConfig {
                    provider: LlmProvider::Local,
                    api_key: String::new(),
                    api_url: "https://api.openai.com/v1/chat/completions".to_string(),
                    default_model: "local-dummy".to_string(),
                    model_params: ModelParams::default(),
                    headers: {
                        let mut headers = std::collections::HashMap::new();
                        headers.insert("content-type".to_string(), "application/json".to_string());
                        headers
                    },
                    system_message: Some("You are a C/C++ to Rust code translator expert.".to_string()),
                });
            },
            _ => bail!("Unsupported LLM provider: {}", provider),
        };
        
        let api_key = env::var("LLM_API_KEY")
            .with_context(|| "LLM_API_KEY environment variable not set")?;
            
        let api_url = env::var("LLM_API_URL")
            .unwrap_or_else(|_| default_api_url_for_provider(&provider));
            
        let model_params = ModelParams {
            temperature: env::var("LLM_TEMPERATURE")
                .ok()
                .and_then(|v| v.parse::<f32>().ok()),
            max_tokens: env::var("LLM_MAX_TOKENS")
                .ok()
                .and_then(|v| v.parse::<u32>().ok()),
            top_p: None,
            top_k: None,
            frequency_penalty: None,
            presence_penalty: None,
            extra_params: HashMap::new(),
        };
            
        let headers = {
            let mut headers = std::collections::HashMap::new();
            headers.insert("content-type".to_string(), "application/json".to_string());
            headers
        };
        
        let system_message = env::var("LLM_SYSTEM_MESSAGE")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| default_system_message_for_provider(&provider));
            
        return Ok(LlmConfig {
            provider,
            api_key,
            api_url,
            default_model: "gpt-4".to_string(),
            model_params,
            headers,
            system_message: Some(system_message),
        });
    }
    
    // If environment variables not set, try to read from config file
    let config_path = PathBuf::from("./llm_config.json");
    info!("尝试从文件加载配置: {}", config_path.display());
    
    if config_path.exists() {
        info!("配置文件存在，开始读取");
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        
        info!("配置文件内容: {}", config_str);
            
        match serde_json::from_str::<LlmConfig>(&config_str) {
            Ok(config) => {
                info!("成功解析配置文件。提供商: {:?}, 模型: {}", config.provider, config.default_model);
                return Ok(config);
            },
            Err(e) => {
                error!("解析配置文件失败: {}", e);
                bail!("Failed to parse LLM config JSON: {}", e);
            }
        }
    } else {
        warn!("配置文件不存在: {}", config_path.display());
    }
    
    // Try absolute path
    let config_path = PathBuf::from("/home/xiaofan/dev_25/transproj/TranslateToRust/llm_config.json");
    info!("尝试从绝对路径加载配置: {}", config_path.display());
    
    if config_path.exists() {
        info!("绝对路径配置文件存在，开始读取");
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        
        info!("配置文件内容: {}", config_str);
            
        match serde_json::from_str::<LlmConfig>(&config_str) {
            Ok(config) => {
                info!("成功解析配置文件。提供商: {:?}, 模型: {}", config.provider, config.default_model);
                return Ok(config);
            },
            Err(e) => {
                error!("解析配置文件失败: {}", e);
                bail!("Failed to parse LLM config JSON: {}", e);
            }
        }
    } else {
        warn!("绝对路径配置文件不存在: {}", config_path.display());
    }
    
    // If neither environment variables nor config file is available, use local dummy implementation
    warn!("No LLM configuration found. Using local dummy implementation.");
    Ok(LlmConfig {
        provider: LlmProvider::Local,
        api_key: String::new(),
        api_url: "https://api.openai.com/v1/chat/completions".to_string(),
        default_model: "local-dummy".to_string(),
        model_params: ModelParams::default(),
        headers: {
            let mut headers = std::collections::HashMap::new();
            headers.insert("content-type".to_string(), "application/json".to_string());
            headers
        },
        system_message: Some("You are a C/C++ to Rust code translator expert.".to_string()),
    })
}

/// Get the default API URL for a given provider
fn default_api_url_for_provider(provider: &LlmProvider) -> String {
    match provider {
        LlmProvider::OpenAI => "https://api.openai.com/v1/chat/completions".to_string(),
        LlmProvider::Anthropic => "https://api.anthropic.com/v1/messages".to_string(),
        LlmProvider::GoogleAI => "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
        LlmProvider::Local => "https://api.openai.com/v1/chat/completions".to_string(),
    }
}

/// Get the default system message for a given provider
fn default_system_message_for_provider(provider: &LlmProvider) -> String {
    match provider {
        LlmProvider::OpenAI => "You are a C/C++ to Rust code translator expert.".to_string(),
        LlmProvider::Anthropic => "You are a C/C++ to Rust code translator expert.".to_string(),
        LlmProvider::GoogleAI => "You are a C/C++ to Rust code translator expert.".to_string(),
        LlmProvider::Local => "You are a C/C++ to Rust code translator expert.".to_string(),
    }
}

/// Translate C/C++ code to Rust using a large language model
fn translate_with_llm(
    source_code: &str,
    language: &str,
    contest: u32,
    problem: u32,
    config: &LlmConfig,
) -> Result<String> {
    info!("Translating with LLM provider: {:?}, model: {}", config.provider, config.default_model);
    debug!("API URL: {}", config.api_url);
    debug!("API Key length: {}", config.api_key.len());
    
    // Create the prompt for the LLM
    let prompt = create_translation_prompt(source_code, language, contest, problem);
    debug!("Prompt created, length: {} characters", prompt.len());
    
    // Call the LLM API through our llm_api module
    match crate::llm_api::translate_code_with_llm(&prompt, config) {
        Ok(translated_code) => {
            // Check if we got a placeholder response (starts with a comment about being a placeholder)
            if translated_code.starts_with("// This is a placeholder") {
                warn!("Received placeholder translation from LLM API");
                // Fall back to our local implementation
                let placeholder = create_placeholder_translation(source_code, language, contest, problem);
                debug!("Created placeholder translation, length: {} characters", placeholder.len());
                Ok(placeholder)
            } else {
                // We got a real translation, now extract the rust code from the response if needed
                debug!("Received translation from LLM API, length: {} characters", translated_code.len());
                
                let extracted_code = extract_rust_code_block(&translated_code)
                    .unwrap_or_else(|| {
                        warn!("No Rust code block found in LLM response. Using raw response.");
                        translated_code
                    });
                
                debug!("Final code length: {} characters", extracted_code.len());
                
                // We got a real translation
                Ok(extracted_code)
            }
        },
        Err(e) => {
            error!("Failed to call LLM API: {}", e);
            warn!("Falling back to placeholder translation");
            let placeholder = create_placeholder_translation(source_code, language, contest, problem);
            debug!("Created placeholder translation after error, length: {} characters", placeholder.len());
            Ok(placeholder)
        }
    }
}

/// Create a prompt for the LLM to translate C/C++ code to Rust
fn create_translation_prompt(
    source_code: &str,
    language: &str,
    contest: u32,
    problem: u32,
) -> String {
    format!(
        r#"You are a programming expert specialized in translating {language} code to Rust.

Translate this {language} code from LeetCode Weekly Contest {contest} Problem {problem} to idiomatic Rust:

```{language}
{source_code}
```

REQUIREMENTS:
1. Translate the ENTIRE file as a complete program, including the main function and I/O handling
2. Preserve the algorithm logic exactly
3. Use idiomatic Rust with proper error handling
4. Maintain the same stdin/stdout format as the original code
5. Add helpful comments where needed

IMPORTANT FOR I/O HANDLING:
- For this specific program, the test will provide coordinates as TWO SEPARATE LINES in stdin
- The first line contains the first coordinate (e.g., "a1")
- The second line contains the second coordinate (e.g., "c3")
- Do NOT expect both coordinates on the same line
- Ensure your code reads each coordinate from a separate line

Your response MUST contain ONLY the Rust code wrapped in a ```rust code block.
"#
    )
}

/// Create a placeholder translation for testing purposes
fn create_placeholder_translation(
    source_code: &str,
    language: &str,
    contest: u32,
    problem: u32,
) -> String {
    // 分析原始C++代码
    let class_regex = regex::Regex::new(r"class\s+(\w+)\s*\{").unwrap();
    let method_regex = regex::Regex::new(r"(\w+)\s+(\w+)\s*\(([^)]*)\)").unwrap();
    
    // 提取类名、方法名和参数
    let struct_name = class_regex
        .captures(source_code)
        .map(|caps| caps.get(1).unwrap().as_str())
        .unwrap_or("Solution");
    
    // 尝试提取所有方法
    let mut methods = Vec::new();
    for method_cap in method_regex.captures_iter(source_code) {
        let return_type = method_cap.get(1).unwrap().as_str();
        let method_name = method_cap.get(2).unwrap().as_str();
        let params = method_cap.get(3).unwrap().as_str();
        
        // 跳过构造函数和析构函数
        if method_name == struct_name || method_name == format!("~{}", struct_name) {
            continue;
        }
        
        methods.push((return_type, method_name, params));
    }
    
    // 提取main函数中的输入输出模式
    let main_regex = regex::Regex::new(r"int\s+main\s*\(\s*\)\s*\{([\s\S]*)\}").unwrap();
    let main_body = main_regex
        .captures(source_code)
        .map(|caps| caps.get(1).unwrap().as_str())
        .unwrap_or("");
    
    // 生成Rust代码的头部
    let header = format!(
        r#"// Translated from {} to Rust using LLM
// Original: Weekly Contest {} Problem {}

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

"#,
        language, contest, problem
    );
    
    // 构建一个通用的Solution结构体实现
    let mut solution_impl = format!("struct {};\n\n", struct_name);
    solution_impl.push_str(&format!("impl {} {{\n", struct_name));
    
    if !methods.is_empty() {
        // 为每个C++方法创建对应的Rust方法
        for (return_type, method_name, params) in &methods {
            // 转换返回类型
            let rust_return_type = match *return_type {
                "bool" => "bool",
                "int" => "i32",
                "long" | "long long" => "i64",
                "string" => "&str",
                "void" => "()",
                "char" => "char",
                "double" => "f64",
                "float" => "f32",
                _ => "i32", // 默认类型
            };
            
            // 参数列表转换（简化版）
            let rust_params = params
                .split(',')
                .map(|p| {
                    let parts: Vec<&str> = p.trim().split_whitespace().collect();
                    if parts.len() >= 2 {
                        // 基本的类型转换
                        let param_name = parts.last().unwrap().trim();
                        let param_type = match parts[0] {
                            "int" => "i32",
                            "bool" => "bool",
                            "string" => "&str",
                            "long" | "long long" => "i64",
                            "char" => "char",
                            "double" => "f64",
                            "float" => "f32",
                            _ => "&str", // 默认类型
                        };
                        format!("{}: {}", param_name, param_type)
                    } else {
                        "".to_string()
                    }
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join(", ");
            
            // 添加方法实现
            solution_impl.push_str(&format!(
                "    // Placeholder for C++ method: {} {}({})\n",
                return_type, method_name, params
            ));
            solution_impl.push_str(&format!(
                "    fn {}({}) -> {} {{\n",
                method_name, rust_params, rust_return_type
            ));
            solution_impl.push_str("        // Placeholder implementation\n");
            
            // 根据返回类型提供默认值
            match rust_return_type {
                "bool" => solution_impl.push_str("        false\n"),
                "i32" | "i64" => solution_impl.push_str("        0\n"),
                "f32" | "f64" => solution_impl.push_str("        0.0\n"),
                "char" => solution_impl.push_str("        'a'\n"),
                "&str" => solution_impl.push_str("        \"\"\n"),
                "()" => solution_impl.push_str("        ()\n"),
                _ => solution_impl.push_str("        unimplemented!()\n"),
            }
            
            solution_impl.push_str("    }\n\n");
        }
    } else {
        // 如果没有找到方法，添加一个通用方法
        solution_impl.push_str("    // No methods found in C++ code, adding placeholder\n");
        solution_impl.push_str("    fn solve() -> i32 {\n");
        solution_impl.push_str("        // Placeholder implementation\n");
        solution_impl.push_str("        0\n");
        solution_impl.push_str("    }\n");
    }
    
    solution_impl.push_str("}\n");
    
    // 创建一个通用的main函数实现
    let main_function = r#"
fn main() {
    // This is a placeholder implementation
    // In a real scenario, the LLM would translate the C++ I/O to Rust
    
    println!("Placeholder implementation. To get a proper translation, configure LLM API.");
}
"#;
    
    // 添加原始代码作为注释
    let source_comment = format!(
        r#"
/*
Original {} code:
{}
*/
"#,
        language, source_code
    );
    
    format!("{}{}{}{}", header, solution_impl, main_function, source_comment)
}

/// Extract class and method names from C/C++ source code
fn extract_class_and_method_names(source_code: &str) -> (String, String) {
    // Default names
    let mut class_name = "Solution".to_string();
    let mut method_name = "solve".to_string();
    
    // Simple regex-like parsing (in a real implementation, we'd use a proper parser)
    for line in source_code.lines() {
        if line.contains("class ") && !line.contains("//") {
            if let Some(name) = line.split_whitespace().skip(1).next() {
                class_name = name.trim_end_matches(" {").to_string();
            }
        }
        
        // Look for a method definition within the class
        if (line.contains("bool ") || line.contains("int ") || line.contains("void ") || 
            line.contains("string ") || line.contains("vector<")) && 
           line.contains("(") && !line.contains("//") {
            
            let parts: Vec<&str> = line.split('(').collect();
            if parts.len() > 1 {
                let method_part = parts[0].trim();
                let method_parts: Vec<&str> = method_part.split_whitespace().collect();
                if method_parts.len() > 1 {
                    method_name = method_parts.last().unwrap().to_string();
                }
            }
        }
    }
    
    // Convert to snake_case for Rust
    let method_name = to_snake_case(&method_name);
    
    (class_name, method_name)
}

/// Convert a camelCase or PascalCase string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in s.char_indices() {
        if i > 0 && c.is_uppercase() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    
    result
}

/// Extract Rust code block from LLM response
fn extract_rust_code_block(text: &str) -> Option<String> {
    // Look for ```rust or ``` rust patterns
    let rust_markers = [
        "```rust", 
        "``` rust", 
        "```Rust", 
        "``` Rust"
    ];
    
    for start_marker in &rust_markers {
        if let Some(start_idx) = text.find(start_marker) {
            let code_start = start_idx + start_marker.len();
            
            // Look for the closing ```
            if let Some(end_idx) = text[code_start..].find("```") {
                return Some(text[code_start..code_start + end_idx].trim().to_string());
            }
        }
    }
    
    // If we can't find Rust markers, try to find any code block
    if let Some(start_idx) = text.find("```") {
        let code_start = start_idx + 3;
        
        // Skip language identifier if present
        let code_start = if let Some(newline_idx) = text[code_start..].find('\n') {
            code_start + newline_idx + 1
        } else {
            code_start
        };
        
        // Look for the closing ```
        if let Some(end_idx) = text[code_start..].find("```") {
            return Some(text[code_start..code_start + end_idx].trim().to_string());
        }
    }
    
    None
} 