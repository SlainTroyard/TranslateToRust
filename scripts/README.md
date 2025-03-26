# TranslateToRust 脚本使用文档

本文档详细说明 TranslateToRust 项目中的各类脚本的使用方法、参数和最佳实践。

## 目录
1. [批量翻译脚本 (batch_translate.py)](#批量翻译脚本)
2. [批量测试脚本 (batch_test.py)](#批量测试脚本)
3. [LLM 特定测试脚本 (batch_test_llm.py)](#llm-特定测试脚本)
4. [测试报告生成脚本 (generate_test_reports.py)](#测试报告生成脚本)
5. [LLM 报告生成脚本 (generate_llm_reports.py)](#llm-报告生成脚本)
6. [超时处理脚本 (timeout_handler.py)](#超时处理脚本)
7. [工作流示例](#工作流示例)
8. [常见问题](#常见问题)

## 批量翻译脚本

`batch_translate.py` 脚本负责自动将 FuzzForLeetcode 项目中的 C/C++ 解决方案批量翻译为 Rust 代码。

### 功能

- 扫描 FuzzForLeetcode 中的 C/C++ 源代码文件
- 使用配置的 LLM API 进行代码翻译
- 支持多种 LLM 提供商和模型
- 处理超时和错误情况
- 生成翻译报告

### 使用方法

```bash
# 基本用法 - 翻译所有解决方案
python scripts/batch_translate.py

# 指定语言
python scripts/batch_translate.py --language CPP

# 指定比赛和问题
python scripts/batch_translate.py --contest 413 --problem 1

# 设置翻译方法和超时时间
python scripts/batch_translate.py --method llm --timeout 1800

# 设置并行工作线程数
python scripts/batch_translate.py --max-workers 4

# 指定输出目录
python scripts/batch_translate.py --output-dir ./custom_translated
```

### 参数说明

- `--max-workers N`: 最大并行工作线程数（默认：1）
- `--timeout N`: 每个翻译任务的超时时间，单位秒（默认：1800）
- `--output-dir PATH`: 保存翻译后Rust文件的目录（默认：./translated）
- `--method METHOD`: 使用的翻译方法（默认：llm）
- `--language {C,CPP}`: 指定要翻译的语言（C或CPP），省略则翻译两种语言
- `--contest N`: 翻译特定比赛的文件
- `--problem N`: 翻译特定问题的文件

## 批量测试脚本

`batch_test.py` 脚本负责自动测试翻译后的 Rust 文件，收集测试结果和性能数据。

### 功能

- 批量测试翻译后的 Rust 文件
- 收集测试结果、性能指标和编译错误
- 支持资源感知的任务调度
- 重试失败的测试
- 生成详细的测试报告

### 使用方法

```bash
# 基本用法 - 测试所有翻译的文件
python scripts/batch_test.py

# 测试特定目录
python scripts/batch_test.py --directory ./translated

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_123_p1_cpp.rs

# 设置超时时间
python scripts/batch_test.py --timeout 1200

# 设置重试次数
python scripts/batch_test.py --retry-count 2

# 设置并行工作线程数
python scripts/batch_test.py --max-workers 2

# 指定输出目录
python scripts/batch_test.py --output-dir ./custom_results
```

### 参数说明

- `--directory PATH`: 要测试的目录（默认：./translated）
- `--file PATH`: 测试特定文件
- `--timeout N`: 每个测试任务的超时时间，单位秒（默认：1800）
- `--max-workers N`: 最大并行工作线程数（默认：2）
- `--retry-count N`: 失败测试的重试次数（默认：1）
- `--output-dir PATH`: 测试结果输出目录（默认：./test_results）

## LLM 特定测试脚本

`batch_test_llm.py` 脚本专为测试不同 LLM 模型生成的 Rust 代码而设计，支持模型对比功能。

### 功能

- 测试特定 LLM 模型翻译的 Rust 文件
- 创建模型特定的测试结果目录结构
- 生成各 LLM 模型的性能对比报告
- 提供灵活的测试和报告选项

### 使用方法

```bash
# 基本用法 - 测试所有 LLM 翻译的文件
python scripts/batch_test_llm.py

# 跳过测试，只生成报告
python scripts/batch_test_llm.py --skip-tests

# 跳过报告生成，只运行测试
python scripts/batch_test_llm.py --skip-report

# 测试特定 LLM 模型
python scripts/batch_test_llm.py --models claude-3-7-sonnet-20250219-default gpt-4o-2024-11-20-default

# 设置超时和并行数
python scripts/batch_test_llm.py --timeout 1800 --max-workers 2
```

### 参数说明

- `--skip-tests`: 跳过测试，只生成报告
- `--skip-report`: 跳过报告生成，只运行测试
- `--models MODEL1 MODEL2 ...`: 指定要测试的模型名称
- `--timeout N`: 每个测试任务的超时时间，单位秒（默认：1800）
- `--max-workers N`: 最大并行工作线程数（默认：2）
- `--output-dir PATH`: 测试结果输出目录（默认：./llm_test_results）

## 测试报告生成脚本

`generate_test_reports.py` 脚本可以根据测试结果生成全面的测试报告，包括成功率、编译状态、运行时间等信息。

### 功能

- 分析测试结果文件
- 生成多种格式的报告（Markdown、JSON、CSV）
- 提供综合统计和详细测试用例信息
- 按比赛、问题、语言等组织报告

### 使用方法

```bash
# 基本用法 - 生成最新测试结果的报告
python scripts/generate_test_reports.py

# 指定天数范围
python scripts/generate_test_reports.py --days 7

# 指定输出格式
python scripts/generate_test_reports.py --format markdown
python scripts/generate_test_reports.py --format json
python scripts/generate_test_reports.py --format csv
python scripts/generate_test_reports.py --format all

# 包含详细信息
python scripts/generate_test_reports.py --include-details

# 指定输出目录
python scripts/generate_test_reports.py --output-dir ./custom_reports
```

### 参数说明

- `--days N`: 包含最近N天的测试结果（默认：7）
- `--format FORMAT`: 输出格式：markdown、json、csv 或 all（默认：markdown）
- `--output-dir PATH`: 报告输出目录（默认：./test_reports）
- `--include-details`: 包含详细的测试用例信息
- `--llm`: 使用 LLM 测试结果目录（直接调用 generate_llm_reports.py）

## LLM 报告生成脚本

`generate_llm_reports.py` 脚本专门用于生成 LLM 翻译结果的对比报告，帮助分析不同 LLM 模型的性能差异。

### 功能

- 分析不同 LLM 模型的测试结果
- 生成模型对比报告
- 提供各模型的成功率、编译率等指标
- 识别模型间的优势和劣势

### 使用方法

```bash
# 基本用法 - 生成所有 LLM 模型的报告
python scripts/generate_llm_reports.py

# 指定模型
python scripts/generate_llm_reports.py --models claude-3-7-sonnet-20250219-default gpt-4o-2024-11-20-default

# 指定输出格式
python scripts/generate_llm_reports.py --format all

# 指定输出目录
python scripts/generate_llm_reports.py --output-dir ./custom_llm_reports
```

### 参数说明

- `--models MODEL1 MODEL2 ...`: 指定要包含在报告中的模型
- `--format FORMAT`: 输出格式：markdown、json、csv 或 all（默认：markdown）
- `--output-dir PATH`: 报告输出目录（默认：./llm_test_reports）
- `--include-details`: 包含详细的测试用例信息

## 超时处理脚本

`timeout_handler.py` 是一个工具模块，用于处理长时间运行的进程，防止系统资源被耗尽。

### 功能

- 监控进程运行时间
- 安全终止超时进程
- 提供进程树终止功能
- 记录进程资源使用情况

### 使用方法

作为独立脚本使用：

```bash
# 监控特定进程，超时时间为 1200 秒
python scripts/timeout_handler.py --pid 12345 --timeout 1200

# 启用详细日志输出
python scripts/timeout_handler.py --pid 12345 --timeout 1200 --verbose
```

作为模块导入：

```python
import timeout_handler

# 监控进程
timeout_handler.monitor_process(pid, timeout_seconds)

# 终止进程树
timeout_handler.terminate_process_tree(pid)

# 在执行命令时使用超时控制
result = timeout_handler.run_with_timeout(cmd, cwd, timeout_seconds)
```

## 工作流示例

以下是完整的工作流示例，展示了如何使用这些脚本进行端到端的翻译、测试和报告生成：

### 标准工作流

```bash
# 1. 配置 LLM API（首次运行）
cp llm_config.json.example llm_config.json
# 编辑 llm_config.json 文件，添加 API 密钥和设置

# 2. 批量翻译 C++ 解决方案
python scripts/batch_translate.py --language CPP --max-workers 2

# 3. 批量测试翻译结果
python scripts/batch_test.py --max-workers 2 --retry-count 1

# 4. 生成综合报告
python scripts/generate_test_reports.py --format all --include-details
```

### LLM 对比工作流

```bash
# 配置多个 LLM 模型
cp llm_configs/claude_config.json.example llm_configs/claude_config.json
cp llm_configs/gpt4_config.json.example llm_configs/gpt4_config.json
# 编辑配置文件，添加相应的 API 密钥

# 运行 LLM 特定测试（会自动使用所有配置的模型）
python scripts/batch_test_llm.py --max-workers 2

# 生成 LLM 对比报告
python scripts/generate_llm_reports.py --format all
# 或者使用
python scripts/generate_test_reports.py --llm --format all
```

## 常见问题

### 资源管理

1. **系统资源不足**
   - 减少 `--max-workers` 值
   - 增加任务间隔时间
   - 清理临时文件和旧测试结果

2. **进程超时**
   - 增加 `--timeout` 值
   - 检查测试代码中是否有无限循环
   - 针对复杂问题单独设置更长的超时时间

### 配置问题

1. **LLM API 配置**
   - 确保 API 密钥正确
   - 检查网络连接
   - 验证 API URL 格式
   - 确认模型名称正确

2. **路径配置**
   - 确保 FuzzForLeetcode 项目路径正确
   - 验证输出目录存在并可写
   - 检查文件权限

### 测试问题

1. **编译错误**
   - 检查翻译后的 Rust 代码语法
   - 确认 Rust 编译器版本
   - 查看编译错误日志

2. **测试失败**
   - 检查测试用例格式
   - 验证输入/输出匹配
   - 使用详细日志模式运行测试

3. **报告生成问题**
   - 确认测试结果文件存在
   - 检查文件格式是否正确
   - 尝试指定具体的输出格式

### 执行环境

1. **并发问题**
   - 使用文件锁控制并发（默认已实现）
   - 错开批量任务执行时间
   - 监控进程状态

2. **内存泄漏**
   - 设置合理的超时时间
   - 使用 timeout_handler 监控资源
   - 定期重启长时间运行的脚本 