# TranslateToRust 脚本使用文档

本文档说明如何使用 TranslateToRust 项目中的各类脚本。

## 目录
1. [批量翻译脚本](#批量翻译脚本)
2. [批量测试脚本](#批量测试脚本)
3. [测试报告生成脚本](#测试报告生成脚本)
4. [超时处理脚本](#超时处理脚本)
5. [常见问题](#常见问题)

## 批量翻译脚本

批量翻译脚本可以自动将 FuzzForLeetcode 中的 C/C++ 解决方案翻译为 Rust。

### 使用方法

```bash
# 翻译所有解决方案
python scripts/batch_translate.py

# 指定线程数
python scripts/batch_translate.py --max-workers 2

# 翻译特定解决方案
python scripts/batch_translate.py --contest 413 --problem 1 --language CPP

# 指定输出目录
python scripts/batch_translate.py --output-dir ./custom_reports

# 指定超时时间
python scripts/batch_translate.py --timeout 1800  # 设置超时为1800秒
```

### 参数说明

- `--max-workers N`: 最大并行工作线程数（默认：1）
- `--timeout N`: 每个翻译任务的超时时间，单位秒（默认：600）
- `--output-dir PATH`: 保存翻译后Rust文件的目录（默认：./translated）
- `--method METHOD`: 使用的翻译方法（默认：llm）
- `--file FILE`: 翻译特定C/C++文件，而不是搜索文件
- `--language {C,CPP}`: 指定要翻译的语言（C或CPP），省略则翻译两种语言
- `--contest N`: 翻译特定比赛的文件
- `--problem N`: 翻译特定问题的文件

## 批量测试脚本

批量测试脚本可以自动测试翻译后的 Rust 文件。

### 使用方法

```bash
# 测试所有翻译的文件
python scripts/batch_test.py

# 指定超时时间
python scripts/batch_test.py --timeout 1200  # 设置超时为 1200秒

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_413_p1_cpp.rs

# 测试特定目录
python scripts/batch_test.py --directory ./custom_translated_dir

# 指定输出目录
python scripts/batch_test.py --output-dir ./custom_results

# 指定重试次数
python scripts/batch_test.py --retry-count 2
```

### 参数说明

- `--directory PATH`: 要测试的目录（默认：./translated）
- `--file PATH`: 测试特定文件
- `--timeout N`: 每个测试任务的超时时间，单位秒（默认：1800）
- `--max-workers N`: 最大并行工作线程数（默认：2）
- `--retry-count N`: 失败测试的重试次数（默认：1）
- `--output-dir PATH`: 测试结果输出目录（默认：./test_results）

## 测试报告生成脚本

测试报告生成脚本可以根据批量测试的结果生成综合报告。

### 使用方法

```bash
# 生成最新测试结果的报告
python scripts/generate_test_reports.py

# 指定天数范围
python scripts/generate_test_reports.py --days 7

# 指定输出格式
python scripts/generate_test_reports.py --format json

# 包含详细信息
python scripts/generate_test_reports.py --include-details

# 指定输出目录
python scripts/generate_test_reports.py --output-dir ./custom_reports
```

### 参数说明

- `--days N`: 包含最近N天的测试结果（默认：7）
- `--format FORMAT`: 输出格式：markdown 或 json（默认：markdown）
- `--output-dir PATH`: 报告输出目录（默认：./test_reports）
- `--include-details`: 包含详细的测试用例信息

## 超时处理脚本

超时处理脚本可以单独用于监控长时间运行的进程。

### 使用方法

```bash
# 监控特定进程，如果超过指定时间（秒）则终止
python scripts/timeout_handler.py --pid 12345 --timeout 1200

# 启用详细日志
python scripts/timeout_handler.py --pid 12345 --timeout 1200 --verbose
```

也可以在自己的 Python 脚本中导入：

```python
import timeout_handler
timeout_handler.monitor_process(pid, timeout_seconds)
```

## 常见问题

### 1. 进程执行后长时间没有反应怎么办？

如果发现进程长时间没有输出，可能是因为输出缓冲区未刷新或者进程卡住。解决方法：

- 按回车键可能会刷新输出
- 使用超时处理脚本监控进程
- 设置较小的超时时间，比如 `--timeout 1200`

### 2. 批量测试失败怎么办？

- 检查测试结果目录中的具体错误信息
- 查看日志中是否有编译错误
- 尝试手动运行单个测试看看详细错误
- 使用 `--retry-count` 参数增加重试次数

### 3. 如何自定义测试超时时间？

在 `testing.rs` 文件中，修改 `TIMEOUT_SECONDS` 常量的值（默认为 30 秒）。

### 4. 如何在测试报告中添加更多信息？

可以修改 `generate_test_reports.py` 脚本，在 `generate_markdown_report` 或 `generate_json_report` 函数中添加更多字段或统计信息。

### 5. 如何并行执行更多任务？

可以增加并行度，但要注意系统资源限制：

```bash
python scripts/batch_test.py --max-workers 8
```

## 完整工作流示例

执行完整的翻译、测试、报告生成工作流：

```bash
# 1. 批量翻译
python scripts/batch_translate.py --max-workers 4

# 2. 批量测试
python scripts/batch_test.py --max-workers 2 --retry-count 2

# 3. 生成报告
python scripts/generate_test_reports.py --days 7 --include-details
```

## 资源管理

脚本包含以下资源管理功能：

1. 系统负载监控
   - 自动监控 CPU 和内存使用情况
   - 系统过载时自动延迟任务
   - 可配置的资源使用阈值

2. 进程管理
   - 优雅的进程终止
   - 进程树清理
   - 超时处理与清理

3. 并发控制
   - 基于文件的 cargo 操作锁定
   - 可配置的工作线程池
   - 资源感知的任务调度

## 最佳实践

1. 资源管理
   - 从较少的工作线程开始（2-4）
   - 执行过程中监控系统资源
   - 根据问题复杂度调整超时时间

2. 测试策略
   - 按相似复杂度分批运行测试
   - 使用适当的超时值
   - 对可能不稳定的测试启用重试

3. 报告生成
   - 定期生成报告
   - 保留历史数据用于趋势分析
   - 包含失败案例的详细信息 