# LLM Translation Analysis Project

这个项目用于分析和可视化大语言模型（LLM）将C/C++代码翻译成Rust的测试结果。

## 项目结构

```
visualization_project/
├── src/                      # 源代码目录
│   ├── data_loader.py        # 数据加载和预处理模块
│   ├── visualization.py      # 数据可视化模块
│   ├── report_generator.py   # 报告生成模块
│   └── main.py               # 主程序
├── output/                   # 输出目录（图表和报告）
│   └── difficulty_analysis/  # 难度分析细分图表
├── config/                   # 配置文件目录
└── README.md                 # 项目说明文档
```

## 功能特点

1. **学术风格的可视化**：采用学术论文常用的配色方案和字体样式
2. **细化的难度级别分析**：针对C/C++语言在不同难度级别下的表现进行详细分析
3. **模型类型对比**：对Thinking/Greedy/Default三种模型策略进行对比
4. **热力图分析**：使用热力图展示不同模型类型在不同语言和难度级别下的表现
5. **雷达图对比**：使用雷达图多维度对比不同模型类型的表现
6. **Markdown格式报告**：生成结构化的分析报告，包含统计数据和关键发现

## 如何使用

1. 安装依赖：

```bash
pip install -r requirements.txt
```

2. 运行分析程序：

```bash
cd src
python main.py --data_dir ../../llm_test_reports --output_dir ../output --include_values
```

参数说明：
- `--data_dir`: 测试结果数据目录
- `--output_dir`: 输出目录
- `--include_values`: 在图表上显示具体数值

## 输出内容

1. **基础可视化图表**：
   - 模型成功率对比图
   - 模型编译成功率对比图
   - 模型类型雷达图对比

2. **难度级别分析**：
   - 针对每个难度级别和语言的详细对比图
   - 每种模型类型在不同难度级别的热力图

3. **分析报告**：
   - summary_report.md：包含统计数据、对比分析和关键发现的详细报告 