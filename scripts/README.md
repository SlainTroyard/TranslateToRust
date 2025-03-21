# TranslateToRust 脚本使用文档

本文档说明如何使用 TranslateToRust 项目中的各类脚本。

## 目录
1. [批量翻译脚本](#批量翻译脚本)
2. [批量仅翻译脚本](#批量仅翻译脚本)
3. [批量测试脚本](#批量测试脚本)
4. [测试报告生成脚本](#测试报告生成脚本)
5. [超时处理脚本](#超时处理脚本)
6. [常见问题](#常见问题)

## 批量翻译脚本

批量翻译脚本可以自动将 FuzzForLeetcode 中的 C/C++ 解决方案翻译为 Rust。

### 使用方法

```bash
# 翻译所有解决方案
python scripts/batch_translate_improved.py

# 指定线程数
python scripts/batch_translate_improved.py --max-workers 2

# 翻译特定解决方案
python scripts/batch_translate_improved.py --contest 413 --problem 1 --language CPP

# 指定输出目录
python scripts/batch_translate_improved.py --output ./custom_reports
```

## 批量仅翻译脚本

批量仅翻译脚本(`batch_transonly.py`)可以将C/C++代码翻译为Rust，但不进行测试，适用于快速翻译大量文件的场景。

### 使用方法

```bash
# 翻译所有C/C++解决方案（不测试）
python scripts/batch_transonly.py

# 指定线程数
python scripts/batch_transonly.py --max-workers 2

# 翻译特定解决方案
python scripts/batch_transonly.py --contest 413 --problem 1 --language CPP

# 指定超时时间
python scripts/batch_transonly.py --timeout 600  # 设置超时为600秒

# 指定输出目录
python scripts/batch_transonly.py --output-dir ./custom_translated
```

### 参数说明

- `--max-workers N`: 最大并行工作线程数（默认：4）
- `--timeout N`: 每个翻译任务的超时时间，单位秒（默认： 3600）
- `--output-dir PATH`: 保存翻译后Rust文件的目录（默认：./translated）
- `--method METHOD`: 使用的翻译方法（默认：llm）
- `--file FILE`: 翻译特定C/C++文件，而不是搜索文件
- `--language {C,CPP}`: 指定要翻译的语言（C或CPP），省略则翻译两种语言
- `--contest N`: 翻译特定比赛的文件
- `--problem N`: 翻译特定问题的文件

## 批量测试脚本

批量测试脚本可以自动测试 `translated` 目录中的 Rust 文件。

### 使用方法

```bash
# 测试所有翻译的文件
python scripts/batch_test.py

# 指定超时时间
python scripts/batch_test.py --timeout  3600  # 设置超时为 3600秒

# 测试特定文件
python scripts/batch_test.py --file ./translated/weekly_contest_413_p1_cpp.rs

# 测试特定目录
python scripts/batch_test.py --dir ./custom_translated_dir

# 指定输出目录
python scripts/batch_test.py --output ./custom_results
```

## 测试报告生成脚本

测试报告生成脚本可以根据批量测试的结果生成综合报告。

### 使用方法

```bash
# 生成最新测试结果的报告
python scripts/generate_test_reports.py

# 指定测试结果文件
python scripts/generate_test_reports.py --input ./test_results/batch_test_results_20230101_120000.json

# 指定输出目录
python scripts/generate_test_reports.py --output-dir ./custom_reports

# 仅生成 Markdown 报告
python scripts/generate_test_reports.py --format markdown

# 仅生成 JSON 报告
python scripts/generate_test_reports.py --format json
```

## 超时处理脚本

超时处理脚本可以单独用于监控长时间运行的进程。

### 使用方法

```bash
# 监控特定进程，如果超过指定时间（秒）则终止
python scripts/timeout_handler.py --pid 12345 --timeout 3600

# 启用详细日志
python scripts/timeout_handler.py --pid 12345 --timeout  3600 --verbose
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
- 设置较小的超时时间，比如 `--timeout 3600`

### 2. 批量测试失败怎么办？

- 检查测试结果目录中的具体错误信息
- 查看日志中是否有编译错误
- 尝试手动运行单个测试看看详细错误

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
python scripts/batch_translate_improved.py

# 2. 批量测试
python scripts/batch_test.py

# 3. 生成报告
python scripts/generate_test_reports.py 
```

### 仅翻译工作流示例

如果只需要翻译而不需要测试（例如为了快速获取大量翻译结果）：

```bash
# 1. 只执行翻译（不测试）
python scripts/batch_transonly.py --language CPP --max-workers 8

# 2. 之后可以选择性地对翻译结果进行测试
python scripts/batch_test.py --dir ./translated
``` 