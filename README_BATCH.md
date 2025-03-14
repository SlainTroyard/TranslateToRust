# 批量翻译C/C++到Rust功能说明

本项目现已支持批量翻译FuzzForLeetcode中的C/C++代码到Rust，并生成详细的统计报告。该功能可以帮助您批量处理多个LeetCode竞赛题目的代码，并对翻译结果进行多维度分析。

## 功能概述

- **批量翻译**：自动扫描FuzzForLeetcode项目中的所有C/C++代码文件，将它们翻译为Rust代码
- **自动测试**：对翻译后的Rust代码进行编译和测试，验证其正确性
- **多维度统计**：生成详细的统计报告，包括按难度、按语言、按标签、按比赛等多个维度
- **并行处理**：支持多线程并行处理，提高批量翻译的效率
- **可定制化**：支持指定特定比赛、特定题目或特定语言进行翻译
- **仅翻译模式**：支持只进行翻译而不测试，适用于快速处理大量文件

## 如何使用

### 准备工作

在使用批量翻译功能前，请确保：

1. TranslateToRust项目已正确配置，特别是LLM API配置
2. FuzzForLeetcode项目结构完整，包含需要翻译的C/C++代码文件

### 使用步骤

1. **设置翻译报告目录**

   ```bash
   cd /home/xiaofan/dev_25/transproj/TranslateToRust/scripts
   ./setup_reports_dir.sh
   ```

2. **运行单个示例**（可选，用于测试）

   ```bash
   # 默认翻译Weekly Contest 413 Problem 1 (CPP)
   ./run_example.sh
   
   # 指定比赛、题目和语言
   ./run_example.sh 413 2 C
   ```

3. **运行批量翻译**

   ```bash
   # 翻译所有C/C++代码
    python batch_translate.py > batch_log.txt 2>&1
   
   # 调整并行线程数
   ./batch_translate.py --max-workers 8
   
   # 指定输出目录
   ./batch_translate.py --output /path/to/output/dir
   ```

## 统计报告说明

批量翻译完成后，系统会生成包含以下内容的统计报告：

### 重要说明

在统计报告中，**翻译成功**的定义为：**代码编译成功且通过所有测试用例**。这确保了翻译结果不仅在语法上正确，而且在功能上与原始C/C++代码一致。

### 1. 总体统计

- 总解决方案数量
- C和C++各自的解决方案数量
- 翻译成功率和编译成功率

### 2. 按难度统计 

按照LeetCode题目难度（Easy、Medium、Hard）分类统计：
- 每个难度级别的题目数量
- 翻译成功率和编译成功率
- 测试用例平均通过率
- 平均运行时间

### 3. 按语言统计

比较C和C++代码的：
- 翻译成功率
- 编译成功率
- 测试用例平均通过率
- 平均运行时间

### 4. 按标签统计

按照算法类型和数据结构（如树、图、动态规划等）分类统计：
- 每种类型的题目数量
- 翻译成功率和编译成功率
- 测试用例平均通过率
- 平均运行时间

### 5. 按比赛统计

按照LeetCode周赛分类统计：
- 每场比赛的题目数量
- 翻译成功率和编译成功率
- 测试用例平均通过率
- 平均运行时间

## 脚本文件说明

- `scripts/batch_translate_improved.py`：主要的批量翻译和测试脚本
- `scripts/batch_transonly.py`：仅进行翻译而不测试的脚本
- `scripts/batch_test.py`：批量测试已翻译的Rust文件的脚本
- `scripts/generate_test_reports.py`：生成测试报告的脚本
- `scripts/setup_reports_dir.sh`：创建翻译报告目录的脚本
- `scripts/run_example.sh`：运行单个示例的脚本
- `scripts/README.md`：详细的脚本使用说明

## 注意事项

- 批量翻译可能需要较长时间，特别是文件较多时
- 批量翻译过程中可能会消耗大量API调用，请注意API使用成本
- 如果遇到网络问题或API限制，可以考虑减少并行线程数或分批次运行 

# Batch Processing Tools

This document describes the batch processing tools available in the TranslateToRust project.

## Overview

The project includes several scripts for batch processing:

1. **batch_translate_improved.py**: Translates and tests C/C++ files to Rust
2. **batch_transonly.py**: Translates C/C++ files to Rust without testing them
3. **batch_test.py**: Tests all translated Rust files in the `translated` directory
4. **generate_test_reports.py**: Generates comprehensive reports from test results

## Batch Translation and Testing

The `batch_translate_improved.py` script automates both translation and testing of C/C++ files.

### Usage

```bash
python scripts/batch_translate_improved.py [options]
```

### Options

- `--max-workers N`: Maximum number of parallel workers (default: 4)
- `--output PATH`: Directory to save reports (default: ./reports)
- `--language {C,CPP}`: Specify which language to translate (C or CPP), omit to translate both
- `--contest N`: Translate files from a specific contest
- `--problem N`: Translate files for a specific problem

### Examples

```bash
# Translate and test all C/C++ files
python scripts/batch_translate_improved.py

# Translate and test only C++ files
python scripts/batch_translate_improved.py --language CPP

# Translate and test files from a specific contest
python scripts/batch_translate_improved.py --contest 413
```

## Batch Translation Only

The `batch_transonly.py` script automates translation of C/C++ files to Rust without testing them.

### Usage

```bash
python scripts/batch_transonly.py [options]
```

### Options

- `--max-workers N`: Maximum number of parallel workers (default: 4)
- `--timeout N`: Timeout in seconds for each translation (default: 600)
- `--output-dir PATH`: Directory to save translated Rust files (default: ./translated)
- `--method METHOD`: Translation method to use (default: llm)
- `--file FILE`: Translate a specific C/C++ file instead of searching for files
- `--language {C,CPP}`: Specify which language to translate (C or CPP), omit to translate both
- `--contest N`: Translate files from a specific contest
- `--problem N`: Translate files for a specific problem

### Examples

Translate all available C and C++ files:
```bash
python scripts/batch_transonly.py
```

Translate only C++ files:
```bash
python scripts/batch_transonly.py --language CPP
```

Translate files from a specific contest:
```bash
python scripts/batch_transonly.py --contest 413
```

Translate a specific file:
```bash
python scripts/batch_transonly.py --file /path/to/weekly_contest_413_p1.cpp
```

## Batch Testing

The `batch_test.py` script automates testing of Rust files.

### Usage

```bash
python scripts/batch_test.py [options]
```

### Options

- `--max-workers N`: Maximum number of parallel workers (default: 4)
- `--timeout N`: Timeout in seconds for each test (default: 600)
- `--dir PATH`: Directory containing Rust files to test (default: ./translated)
- `--file FILE`: Test a specific Rust file instead of the entire directory
- `--output PATH`: Directory to save test results (default: ./test_results)

### Examples

Test all Rust files in the translated directory:
```bash
python scripts/batch_test.py
```

Test a specific file:
```bash
python scripts/batch_test.py --file translated/weekly_contest_413_p1_cpp.rs
```

Test with a custom timeout:
```bash
python scripts/batch_test.py --timeout 300
```

## Test Reports

The `generate_test_reports.py` script generates comprehensive reports from test results.

### Usage

```bash
python scripts/generate_test_reports.py [options]
```

### Options

- `--output-dir PATH`: Directory to save reports (default: ./test_reports)
- `--format {markdown,json,both}`: Report format (default: both)
- `--days N`: Process test results from the last N days (default: 7)
- `--input FILE`: Process a specific test results file

### Examples

Generate reports from recent test results:
```bash
python scripts/generate_test_reports.py
```

Generate only markdown reports:
```bash
python scripts/generate_test_reports.py --format markdown
```

Process results from the last 30 days:
```bash
python scripts/generate_test_reports.py --days 30
```

## Complete Workflow

For a complete translation and testing workflow:

1. **Translate files with testing**: 
   ```
   python scripts/batch_translate_improved.py --language CPP
   ```

2. **Or translate files without testing**:
   ```
   python scripts/batch_transonly.py --language CPP
   ```

3. **Test translated files** (if using batch_transonly.py):
   ```
   python scripts/batch_test.py
   ```

4. **Generate reports**:
   ```
   python scripts/generate_test_reports.py
   ```

## Performance Considerations

- Translation processes can be resource-intensive and time-consuming
- Consider reducing the number of parallel workers if you encounter performance issues
- For large batches, it may be more efficient to use batch_transonly.py first, then batch_test.py
- Set appropriate timeouts based on your system's capabilities and the complexity of the code being translated/tested 