# LLM API 实现指南

本文档提供了如何在`TranslateToRust`项目中实现和使用大型语言模型(LLM) API的指南。

## 配置

项目支持使用环境变量或配置文件配置LLM API访问：

### 环境变量配置

设置以下环境变量：

```bash
export LLM_PROVIDER="OpenAI"  # 可选: OpenAI, Anthropic, GoogleAI
export LLM_API_KEY="your-api-key-here"
export LLM_MODEL="gpt-4"  # 根据选择的提供商而定
export LLM_TEMPERATURE="0.2"
export LLM_MAX_TOKENS="4000"
```

使用提供的脚本模板：

```bash
cp scripts/setup_env.sh.example scripts/setup_env.sh
# 编辑 setup_env.sh 更新API密钥和其他设置
source scripts/setup_env.sh
```

### 配置文件

创建和编辑`llm_config.json`：

```bash
cp llm_config.json.example llm_config.json
# 编辑 llm_config.json 添加你的凭证
```

## 启用API支持

项目使用Cargo特性(features)来有条件地启用API客户端支持。激活特定的API支持：

```bash
# 启用OpenAI API支持
cargo build --features openai

# 启用Anthropic API支持
cargo build --features anthropic

# 启用Google AI API支持
cargo build --features google

# 启用多个API
cargo build --features "openai anthropic"
```

## 实现API调用

LLM API调用的实现位于`src/llm_api.rs`文件中。每个提供商都有一个占位函数，可以按照以下步骤实现实际的API调用：

### 实现OpenAI API

编辑`call_openai_api`函数。该函数已包含一个注释掉的实现示例。取消注释并修改代码以适应你的需求。

```rust
fn call_openai_api(prompt: &str, config: &LlmConfig) -> Result<String> {
    use reqwest::blocking::Client;
    use serde_json::json;
    
    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": config.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a C/C++ to Rust code translator."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": config.temperature,
            "max_tokens": config.max_tokens
        }))
        .send()?;
    
    if !response.status().is_success() {
        let error_text = response.text()?;
        return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
    }
    
    let json_response: serde_json::Value = response.json()?;
    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .context("Failed to extract content from OpenAI API response")?;
    
    Ok(content.to_string())
}
```

### 实现Anthropic API

类似地，实现`call_anthropic_api`函数：

```rust
fn call_anthropic_api(prompt: &str, config: &LlmConfig) -> Result<String> {
    use reqwest::blocking::Client;
    use serde_json::json;
    
    let client = Client::new();
    let response = client.post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": config.model,
            "messages": [{
                "role": "user",
                "content": prompt
            }],
            "temperature": config.temperature,
            "max_tokens": config.max_tokens
        }))
        .send()?;
    
    // 错误处理和响应解析...
    
    Ok(/* 从响应中提取的代码 */)
}
```

### 实现Google AI API

同样地，实现`call_google_api`函数用于Google的Gemini模型。

## 调整提示词(Prompt)

可以在`src/translation.rs`文件中的`create_translation_prompt`函数中修改提示词模板，以获得更好的翻译效果。

## 测试API实现

实现API调用后，可以通过运行翻译命令来测试：

```bash
cargo run --features openai -- translate --contest 413 --problem 1 --language CPP
```

查看生成的Rust代码位于`./translated/weekly_contest_413_p1.rs`。 