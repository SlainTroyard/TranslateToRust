# TranslateToRust 批量处理功能

本文档详细说明如何使用 TranslateToRust 的批量处理功能进行代码翻译、测试和报告生成。

## 功能概述

- 批量翻译 C/C++ 代码为 Rust（支持多种 LLM 模型）
- 批量测试翻译后的代码，收集详细的性能和兼容性数据
- 为不同 LLM 模型生成对比报告
- 支持智能资源管理和任务调度
- 高级超时处理和进程管理，防止系统资源耗尽
- 详细的测试统计和失败分析

## 批量翻译

批量翻译脚本可以自动将 FuzzForLeetcode 中的 C/C++ 解决方案翻译为 Rust。

### 基本用法

```bash
# 翻译所有解决方案
python scripts/batch_translate.py

# 指定语言
python scripts/batch_translate.py --language CPP

# 指定比赛和问题
python scripts/batch_translate.py --contest 413 --problem 1
```

### 高级参数

```bash
# 设置翻译方法、超时和并行数
python scripts/batch_translate.py --method llm --timeout 1800 --max-workers 4

# 指定输出目录
python scripts/batch_translate.py --output-dir ./custom_translated
```

### 所有可用参数

- `--max-workers N`: 最大并行工作线程数（默认：1）
- `--timeout N`: 每个翻译任务的超时时间，单位秒（默认：1800）
- `--output-dir PATH`: 保存翻译后Rust文件的目录（默认：./translated）
- `--method METHOD`: 使用的翻译方法（默认：llm）
- `--language {C,CPP}`: 指定要翻译的语言（C或CPP），省略则翻译两种语言
- `--contest N`: 翻译特定比赛的文件
- `--problem N`: 翻译特定问题的文件

## 批量测试

批量测试脚本可以自动测试翻译后的 Rust 文件。

### 基本用法

```bash
# 测试所有翻译的文件
python scripts/batch_test.py

# 测试特定目录
python scripts/batch_test.py --directory ./translated

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_123_p1_cpp.rs
```

### 高级参数

```bash
# 设置超时和重试
python scripts/batch_test.py --timeout 1200 --retry-count 2

# 指定输出目录和并行数
python scripts/batch_test.py --output-dir ./custom_results --max-workers 2
```

### 所有可用参数

- `--directory PATH`: 要测试的目录（默认：./translated）
- `--file PATH`: 测试特定文件
- `--timeout N`: 每个测试任务的超时时间，单位秒（默认：1800）
- `--max-workers N`: 最大并行工作线程数（默认：2）
- `--retry-count N`: 失败测试的重试次数（默认：1）
- `--output-dir PATH`: 测试结果输出目录（默认：./test_results）

## LLM 特定测试

LLM 特定测试脚本用于测试和比较各种 LLM 模型翻译的 Rust 代码质量。

### 基本用法

```bash
# 测试所有 LLM 翻译的文件
python scripts/batch_test_llm.py

# 只生成报告，不进行测试
python scripts/batch_test_llm.py --skip-tests

# 只进行测试，不生成报告
python scripts/batch_test_llm.py --skip-report
```

### 高级用法

```bash
# 指定特定的 LLM 模型进行测试
python scripts/batch_test_llm.py --models claude-3-7-sonnet-20250219-default gpt-4o-2024-11-20-default

# 设置超时和并行数
python scripts/batch_test_llm.py --timeout 1800 --max-workers 2
```

### 所有可用参数

- `--skip-tests`: 跳过测试，只生成报告
- `--skip-report`: 跳过报告生成，只运行测试
- `--models MODEL1 MODEL2 ...`: 指定要测试的模型名称
- `--timeout N`: 每个测试任务的超时时间，单位秒（默认：1800）
- `--max-workers N`: 最大并行工作线程数（默认：2）
- `--output-dir PATH`: 指定测试结果输出目录

## 报告生成

报告生成脚本可以基于测试结果创建综合报告。

### 基本用法

```bash
# 生成标准测试报告
python scripts/generate_test_reports.py

# 生成 LLM 测试报告
python scripts/generate_test_reports.py --llm

# 指定输出格式
python scripts/generate_test_reports.py --format all
```

### 高级用法

```bash
# 指定天数范围和是否包含详细信息
python scripts/generate_test_reports.py --days 7 --include-details

# 生成特定模型的报告
python scripts/generate_test_reports.py --llm --models claude-3-7-sonnet-20250219-default
```

### 所有可用参数

- `--days N`: 包含最近 N 天的测试结果（默认：7）
- `--format FORMAT`: 输出格式：markdown、json、csv 或 all（默认：markdown）
- `--output-dir PATH`: 报告输出目录（默认：./test_reports 或 ./llm_test_reports）
- `--include-details`: 包含详细的测试用例信息
- `--llm`: 使用 LLM 测试结果目录
- `--models MODEL1 MODEL2 ...`: 指定要报告的模型

## 资源管理策略

该工具使用智能资源管理来确保系统稳定运行：

### 系统负载监控

- 自动监控 CPU 和内存使用情况
- 当系统负载过高时，自动延迟任务执行
- 可配置的资源使用阈值（默认 CPU 和内存阈值为 80%）

### 进程管理

- Cargo 进程的并发控制（文件锁）
- 随机任务启动延迟，避免资源尖峰
- 优雅的进程终止和资源释放

### 超时处理

- 可配置的超时时间
- 宽限期处理，确保测试结果被保存
- 智能进程树终止，防止僵尸进程

## 最佳实践

1. **翻译策略**
   - 从少量文件开始测试翻译质量
   - 对于复杂问题，适当增加超时时间
   - 为不同 LLM 模型保存单独的翻译结果

2. **测试策略**
   - 限制并行测试数量（2-4 个）避免资源竞争
   - 对大型测试用例适当增加超时时间
   - 为不稳定测试启用重试功能

3. **报告生成**
   - 定期生成报告以跟踪进度
   - 使用 `--include-details` 获取深入的失败分析
   - 保留历史报告以便比较不同版本的性能

## 常见问题解决

### 1. 翻译失败

- 检查 `llm_config.json` 的配置是否正确
- 验证 API 密钥是否有效
- 检查网络连接
- 尝试增加超时时间

### 2. 测试超时

- 增加 `--timeout` 参数的值
- 减少 `--max-workers` 以减轻系统负载
- 检查测试代码中是否有无限循环
- 使用 `--retry-count` 重试不稳定的测试

### 3. 资源耗尽

- 减少 `--max-workers` 参数值
- 在高负载时段避免运行批量任务
- 监控系统资源使用情况
- 考虑清理已完成的测试结果

## 完整执行流程示例

执行完整的翻译、测试、报告生成工作流：

```bash
# 1. 配置 LLM API
cp llm_config.json.example llm_config.json
# 编辑 llm_config.json 配置文件

# 2. 批量翻译 C++ 解决方案
python scripts/batch_translate.py --language CPP --max-workers 2

# 3. 批量测试翻译结果
python scripts/batch_test.py --max-workers 2 --retry-count 1

# 4. 生成综合报告
python scripts/generate_test_reports.py --format all --include-details

# 5. LLM 特定测试（如果使用多个 LLM 模型）
python scripts/batch_test_llm.py

# 6. 生成 LLM 对比报告
python scripts/generate_test_reports.py --llm --format all
``` 