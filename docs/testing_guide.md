# 测试功能使用指南

本文档详细介绍了TranslateToRust项目的测试功能，包括如何运行测试、理解测试结果，以及如何利用测试结果改进Rust代码翻译。

## 运行测试

### 基本测试命令

```bash
cargo run -- test --contest <比赛编号> --problem <问题编号> --language <语言> --rust-solution <Rust文件路径>
```

参数说明：
- `contest`：比赛编号，如413表示第413场周赛
- `problem`：问题编号，从1开始计数
- `language`：原始代码语言，如CPP或C
- `rust-solution`：要测试的Rust文件路径

### 示例

测试第413场周赛的第1个问题的翻译结果：

```bash
cargo run -- test --contest 413 --problem 1 --language CPP --rust-solution ./translated/weekly_contest_413_p1.rs
```

添加调试日志，查看更多执行细节：

```bash
RUST_LOG=debug cargo run -- test --contest 413 --problem 1 --language CPP --rust-solution ./translated/weekly_contest_413_p1.rs
```

## 测试流程

测试执行过程包括以下步骤：

1. **编译阶段**：将指定的Rust文件编译成可执行文件
   - 如果编译失败，会记录错误信息而不是崩溃
   - 编译状态会显示在输出结果中

2. **加载测试用例**：从FuzzForLeetcode项目中加载相应问题的测试用例

3. **运行测试**：针对每个测试用例：
   - 向Rust程序提供测试输入
   - 记录程序输出和运行时间
   - 比较输出与预期结果
   - 汇总测试结果

4. **保存结果**：将所有测试结果保存到`test_results`目录

## 理解测试结果

### 控制台输出

测试完成后，控制台会显示摘要信息：

```
==== Test Results ====
测试文件: weekly_contest_413_p1.rs
编译状态: 成功
Total Test Cases: 102
Passed: 102
Failed: 0
Success Rate: 100.00%
平均运行时间: 0.69 ms
```

如果存在失败的测试用例，还会显示失败的原因。

### 结果文件

测试结果保存在以下位置：

- **摘要文件**：`test_results/test_results_YYYYMMDD_HHMMSS.txt`
- **详细结果**：`test_results/details_YYYYMMDD_HHMMSS/`

#### 详细结果目录包含

1. **所有测试用例摘要**：`all_cases_summary.txt`
2. **通过的测试用例**：`case_passed_N.txt`
3. **失败的测试用例**：`case_failed_N.txt`

每个测试用例文件包含：
- 测试输入
- 期望输出
- 实际输出
- 执行时间

## 编译失败处理

当编译失败时，测试结果将包含以下信息：

```
==== Test Results ====
测试文件: weekly_contest_413_p1.rs
编译状态: 失败
编译错误: [错误信息]
```

编译错误信息会帮助您诊断和修复Rust代码中的问题。

## 性能分析

测试结果包含每个测试用例的执行时间和平均运行时间，可以帮助您：

1. 识别性能瓶颈
2. 比较不同翻译方法的效率
3. 优化Rust代码实现

## 常见问题

### Q: 如何解决编译错误？

A: 查看测试结果中的编译错误信息，修复Rust代码中的语法或类型错误。通常，C++到Rust的自动翻译可能需要手动调整类型定义或标准库的使用方式。

### Q: 测试用例运行缓慢怎么办？

A: 检查Rust代码中的算法实现，尤其是循环和内存分配部分。如Rust版本的平均运行时间明显高于C++版本，可能需要重新思考实现方式。

### Q: 如何查看详细的测试执行过程？

A: 使用`RUST_LOG=debug`环境变量运行测试命令，查看详细的执行日志。

## 未来改进

我们计划为测试框架添加更多功能：

- 图形化的测试结果可视化
- 内存使用统计
- 代码覆盖率分析
- 与持续集成系统的集成

如果您有其他建议，请随时提出！ 