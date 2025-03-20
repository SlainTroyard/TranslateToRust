# TranslateToRust

一个评估大型语言模型(LLMs)从C/C++到Rust翻译质量的项目。

本项目与[FuzzForLeetcode](../FuzzForLeetcode)项目协同工作，后者提供C/C++实现的LeetCode问题解决方案以及由模糊测试生成的测试用例。

## 功能特性

- 使用大型语言模型(LLMs)将C/C++代码翻译成Rust
- 使用原始的模糊测试生成的测试用例测试翻译后的Rust代码
- 比较输出结果以评估翻译质量
- 生成包含执行时间指标的详细测试报告
- 保存单个测试用例结果以供深入分析
- 生成翻译准确性报告

## 前提条件

- Rust工具链(rustc, cargo)
- 访问FuzzForLeetcode项目及其输出
- (可选)语言模型服务的API访问权限(用于实际基于LLM的翻译)

## 项目结构

```
TranslateToRust/
├── src/                    # 源代码
│   ├── main.rs             # CLI入口点
│   ├── translation.rs      # 翻译逻辑
│   ├── testing.rs          # 测试逻辑
│   └── utils.rs            # 工具函数
├── scripts/                # 辅助脚本
│   ├── batch_test.py       # 批量测试脚本
│   ├── batch_transonly.py  # 仅批量翻译脚本
│   ├── generate_test_reports.py # 生成测试报告脚本 
│   └── run_example.sh      # 运行示例脚本
├── translated/             # 生成的Rust翻译
├── test_results/           # 保存的测试结果和详细输出
│   ├── test_results_*.txt  # 测试结果摘要
│   └── details_*/          # 详细测试用例输出
├── test_reports/           # 生成的测试报告
├── translation_reports/    # 翻译报告
├── Cargo.toml              # Rust项目配置
└── README.md               # 项目文档
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

#### 翻译方法

本工具支持使用LLM API进行翻译，将整个C/C++文件直接发送给LLM API进行翻译。这种方式要求LLM模型能够一次性处理较长的输入并生成完整的输出。

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
cargo run -- test --contest 413 --problem 1 --language CPP --rust-solution ./translated/weekly_contest_413_p1_cpp.rs
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

## 批处理工具

本项目提供多种批处理工具，用于高效处理大量文件：

### 批量测试

使用`batch_test.py`脚本可以批量测试已翻译的Rust文件：

```bash
# 测试所有已翻译的文件
python scripts/batch_test.py

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_413_p1_cpp.rs

# 设置最大工作线程数（默认为2）
python scripts/batch_test.py --max-workers 4
```

### 批量翻译

使用`batch_transonly.py`脚本可以批量翻译C/C++代码（不测试）：

```bash
# 翻译所有C/C++文件（不测试）
python scripts/batch_transonly.py

# 翻译特定比赛的文件
python scripts/batch_transonly.py --contest 413

# 翻译特定语言的文件
python scripts/batch_transonly.py --language CPP

# 设置超时时间（秒）
python scripts/batch_transonly.py --timeout 1200
```

### 生成测试报告

使用`generate_test_reports.py`脚本可以生成测试报告：

```bash
# 生成最近测试结果的报告
python scripts/generate_test_reports.py

# 指定报告类型 (markdown 或 json)
python scripts/generate_test_reports.py --format markdown

# 指定最近几天的测试结果 (默认为7天)
python scripts/generate_test_reports.py --days 3
```

### 运行示例

使用`run_example.sh`脚本可以快速运行单个题目的翻译和测试：

```bash
# 使用默认参数 (Weekly Contest 413, Problem 1, CPP)
./scripts/run_example.sh

# 指定参数
./scripts/run_example.sh 414 2 CPP
```

详细的批处理工具说明请参考[批处理工具文档](README_BATCH.md)。

## 支持的模型

以下LLM提供商和模型已经过测试：

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

#### 本地模式
`Local`提供商不使用实际的LLM API，而是创建一个占位符翻译。这对于在不使用API额度的情况下测试管道很有用。

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

## 未来增强计划

- 支持更多编程语言的翻译
- 增强测试结果可视化，使用图表和图形
- 不同LLM提供商之间的基准比较
- 测试执行期间的内存使用跟踪
- 基于测试结果的性能优化建议
- 自动分析常见翻译错误
- 微调提示以提高翻译质量

## 贡献

欢迎贡献！请随时提交Pull Request。

## 许可证

本项目是开源的，根据[MIT许可证](LICENSE)可用。 