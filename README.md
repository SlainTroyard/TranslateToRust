# TranslateToRust

A project to evaluate the quality of C/C++ to Rust translations produced by large language models (LLMs).

This project works with the [FuzzForLeetcode](../FuzzForLeetcode) project, which provides LeetCode problem implementations in C/C++ along with fuzz-generated test cases.

## Features

- Translate C/C++ solutions to Rust using large language models (LLMs)
- Test the translated Rust code against the original fuzz-generated test cases
- Compare the outputs to evaluate translation quality
- Generate detailed test reports with execution time metrics
- Save individual test case results for in-depth analysis
- Generate reports on translation accuracy

## Prerequisites

- Rust toolchain (rustc, cargo)
- Access to the FuzzForLeetcode project and its outputs
- (Optional) API access to a language model service (for actual LLM-based translation)

## Project Structure

```
TranslateToRust/
├── src/                    # Source code
│   ├── main.rs             # CLI entry point
│   ├── translation.rs      # Translation logic
│   ├── testing.rs          # Testing logic
│   └── utils.rs            # Utility functions
├── tests/                  # Test cases
├── scripts/                # Helper scripts
├── translated/             # Generated Rust translations
├── test_results/           # Saved test results and detailed output
│   ├── test_results_*.txt  # Summary of test results
│   └── details_*/          # Detailed test case outputs
├── Cargo.toml              # Rust project configuration
└── README.md               # Project documentation
```

## 使用方法

### 配置LLM API

在使用前，需要先配置LLM API。目前支持OpenAI、Anthropic和GoogleAI三种API提供商。

1. 复制示例配置文件为自己的配置文件:
   - 对于OpenAI: 复制`llm_config.json.example`为`llm_config.json`
   - 对于Claude: 复制`llm_configs/claude_config.json.example`为`llm_config.json`
   - 对于Google AI: 复制`llm_configs/google_ai_config.json.example`为`llm_config.json`

2. 根据您使用的API提供商，修改配置文件中的相关参数

#### OpenAI配置示例

```json
{
  "provider": "OpenAI",
  "api_key": "your-api-key-here",
  "api_url": "https://api.openai.com/v1/chat/completions",
  "default_model": "gpt-4o",
  
  "model_params": {
    "temperature": 0.2,
    "max_tokens": 4000,
    "top_p": 1.0,
    "frequency_penalty": 0.0,
    "presence_penalty": 0.0
  },
  
  "headers": {
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert. Translate the code accurately while using idiomatic Rust patterns."
}
```

#### Claude/Anthropic配置示例

```json
{
  "provider": "Anthropic",
  "api_key": "your-anthropic-api-key-here",
  "api_url": "https://api.anthropic.com/v1/messages",
  "default_model": "claude-3-opus-20240229",
  
  "model_params": {
    "temperature": 0.2,
    "max_tokens": 4000,
    "top_p": 1.0,
    "top_k": 5,
    "timeout": 300,
    "stream": true
  },
  
  "headers": {
    "anthropic-version": "2023-06-01",
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert. Translate the code accurately while using idiomatic Rust patterns."
}
```

#### Google AI配置示例

```json
{
  "provider": "GoogleAI",
  "api_key": "your-google-ai-api-key-here",
  "api_url": "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent",
  "default_model": "gemini-pro",
  
  "model_params": {
    "temperature": 0.2,
    "maxOutputTokens": 4000,
    "top_p": 0.95,
    "top_k": 40,
    "timeout": 300,
    "stream": true
  },
  
  "headers": {
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert. Translate the code accurately while using idiomatic Rust patterns."
}
```

### 运行翻译

#### 基本使用

```bash
# 运行翻译
cargo run -- run --contest <比赛编号> --problem <问题编号> --language <语言>

# 例如，翻译第413场周赛的第1个问题（C++）
cargo run -- run --contest 413 --problem 1 --language CPP
```

#### 简化的翻译流程

本工具现在支持简化的翻译流程，将整个C++文件直接发送给LLM API进行翻译，而不是使用分步流式处理。这种方式更简单，但要求LLM模型能够一次性处理较长的输入并生成完整的输出。

注意：对于某些API提供商（如Alibaba Cloud的DashScope），模型可能只支持流式模式（stream mode），配置文件中需要设置`"stream": true`。

```json
{
  "model_params": {
    "max_tokens": 8192,
    "timeout": 1800,  // 超时时间（秒）
    "stream": true    // 启用流式模式
  }
}
```

### 测试翻译结果

```bash
# 测试翻译结果
cargo run -- test --contest <比赛编号> --problem <问题编号> --language <语言> --rust-solution <Rust文件路径>

# 例如，测试第413场周赛的第1个问题的翻译结果
cargo run -- test --contest 413 --problem 1 --language CPP --rust-solution ./translated/weekly_contest_413_p1.rs
```

#### 测试结果输出

测试完成后，程序会输出测试结果摘要，包括：

- 被测试的Rust文件名
- 编译状态（成功或失败，如果失败会显示原因）
- 测试用例总数
- 通过用例数
- 失败用例数
- 成功率
- 平均运行时间（毫秒）

此外，所有测试结果都会保存到`test_results`目录中，格式为：

```
test_results/
├── test_results_YYYYMMDD_HHMMSS.txt    # 测试结果摘要
└── details_YYYYMMDD_HHMMSS/            # 详细测试信息目录
    ├── all_cases_summary.txt           # 所有测试用例的摘要
    ├── case_passed_1.txt               # 通过的测试用例详情
    ├── case_passed_2.txt               # 通过的测试用例详情
    ├── ...                             
    ├── case_failed_1.txt               # 失败的测试用例详情
    └── ...
```

每个测试用例文件包含：
- 测试用例的输入
- 期望的输出
- 实际的输出
- 运行时间（毫秒）

当编译失败时，测试结果会包含编译错误信息，而不是测试用例结果。

更多关于测试功能的详细说明，请参考[测试功能使用指南](docs/testing_guide.md)。

## 批处理工具

本项目提供多种批处理工具，用于高效处理大量文件：

### 批量翻译和测试

使用`batch_translate_improved.py`脚本可以批量翻译并测试C/C++代码：

```bash
# 翻译和测试所有C/C++文件
python scripts/batch_translate_improved.py

# 翻译和测试特定比赛的文件
python scripts/batch_translate_improved.py --contest 413
```

### 仅批量翻译

使用`batch_transonly.py`脚本可以批量翻译C/C++代码（不测试）：

```bash
# 翻译所有C/C++文件（不测试）
python scripts/batch_transonly.py

# 翻译特定比赛的文件
python scripts/batch_transonly.py --contest 413
```

### 批量测试

使用`batch_test.py`脚本可以批量测试已翻译的Rust文件：

```bash
# 测试所有已翻译的文件
python scripts/batch_test.py

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_413_p1_cpp.rs
```

### 生成测试报告

使用`generate_test_reports.py`脚本可以生成测试报告：

```bash
# 生成最近测试结果的报告
python scripts/generate_test_reports.py
```

详细的批处理工具说明请参考[批处理工具文档](README_BATCH.md)和[脚本使用文档](scripts/README.md)。

## Translation Method

The project supports using large language models (LLMs) to translate C/C++ code to Rust. There are two ways to configure the LLM API access:

### 1. Using Environment Variables

You can set the following environment variables to configure LLM access:

```bash
# Choose provider: OpenAI, Anthropic, GoogleAI, or Local
export LLM_PROVIDER="OpenAI"  

# Your API key for the selected provider
export LLM_API_KEY="your-api-key-here"  

# Model to use (provider-specific)
export LLM_MODEL="gpt-4"  

# Temperature setting (0.0 to 1.0)
export LLM_TEMPERATURE="0.2"  

# Maximum tokens in response
export LLM_MAX_TOKENS="4000"  
```

For convenience, you can copy `scripts/setup_env.sh.example` to `scripts/setup_env.sh`, update the values, and then run:

```bash
source scripts/setup_env.sh
```

### 2. Using a Configuration File

Alternatively, you can create a `llm_config.json` file in the project root with the following structure:

```json
{
  "provider": "OpenAI",
  "api_key": "your-api-key-here",
  "model": "gpt-4",
  "temperature": 0.2,
  "max_tokens": 4000
}
```

You can copy `llm_config.json.example` and update it with your credentials:

```bash
cp llm_config.json.example llm_config.json
# Now edit llm_config.json with your credentials
```

### Supported Models

The following LLM providers and models are supported:

#### OpenAI
- `gpt-4o`
- `o1`

#### Anthropic
- `claude-3-opus-20240229`
- `claude-3-sonnet-20240229`
- `claude-3-haiku-20240307`

#### Google AI
- `gemini-1.5-pro`
- `gemini-1.5-flash`

#### Local Mode
The `Local` provider doesn't use an actual LLM API but creates a placeholder translation. This is useful for testing the pipeline without using API credits.

## 配置参数说明

在配置模型参数时，所有参数都是可选的。如果模型不支持某个参数，可以安全地从配置中删除该参数。下面是主要参数的说明：

### 核心配置

| 参数 | 必选 | 描述 |
|-----|------|------|
| `provider` | 是 | LLM提供商 (OpenAI, Anthropic, GoogleAI, Local) |
| `api_key` | 是 | API密钥 |
| `api_url` | 是 | API端点URL |
| `default_model` | 是 | 默认使用的模型名称 |

### 模型参数 (model_params)

不同模型支持不同的参数集，以下参数都是可选的：

| 参数 | 支持的提供商 | 描述 |
|-----|------------|------|
| `temperature` | OpenAI, Anthropic, GoogleAI | 生成随机性 (0.0-1.0) |
| `max_tokens` | OpenAI, Anthropic | 最大输出标记数 |
| `maxOutputTokens` | GoogleAI | Google AI的最大输出标记数 |
| `top_p` | OpenAI, Anthropic | 核采样概率阈值 |
| `top_k` | GoogleAI | 只考虑top_k个标记 |
| `frequency_penalty` | OpenAI | 频率惩罚系数 |
| `presence_penalty` | OpenAI | 存在惩罚系数 |

您可以根据使用的模型添加或删除相应的参数。项目代码已设计为优雅处理缺失的参数。

### 详细配置示例

有关不同LLM提供商的详细配置示例和参数兼容性，请查看 [配置示例文档](docs/config_examples.md)。

## Future Enhancements

- Support for batch processing multiple problems
- Integration with specific LLM APIs
- Automated analysis of common translation errors
- Fine-tuning of prompts for better translation quality
- Support for additional programming languages
- Enhanced test result visualization with graphs and charts
- Benchmark comparisons between different LLM providers
- Memory usage tracking during test execution
- Performance optimization suggestions based on test results
- Integration with continuous integration systems

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE). 