import os
import sys
import argparse
from pathlib import Path

# 确保可以导入项目模块
current_dir = os.path.dirname(os.path.abspath(__file__))
parent_dir = os.path.dirname(current_dir)
sys.path.insert(0, parent_dir)

from src.data_loader import load_test_results
from src.visualization import (
    plot_model_success_rate,
    plot_model_detail_comparison,
    plot_detailed_language_difficulty,
    plot_all_models_metrics,
    plot_model_series_comparison,
    plot_best_model_by_series
)
from src.report_generator import generate_summary_report

def parse_args():
    """解析命令行参数"""
    parser = argparse.ArgumentParser(description='LLM Test Results Visualization & Analysis')
    
    parser.add_argument('--data_dir', type=str, default='../../llm_test_reports',
                        help='Directory containing test result data')
    parser.add_argument('--output_dir', type=str, default='../output',
                        help='Directory to save visualizations and reports')
    parser.add_argument('--include_values', action='store_true',
                        help='Include numeric values on charts')
    parser.add_argument('--model_details', action='store_true', default=True,
                        help='Generate detailed analysis for each model')
    
    return parser.parse_args()

def main():
    """主函数"""
    # 解析命令行参数
    args = parse_args()
    
    # 确保输出目录存在
    output_dir = args.output_dir
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # 加载测试结果数据
    data_dir = args.data_dir
    print(f"Loading data from {data_dir}...")
    df = load_test_results(data_dir)
    print(f"Loaded data for {len(df)} models.")
    
    # 生成可视化图表
    print("Generating visualizations...")
    
    # 1. 主要模型对比图 - 所有模型的直接对比
    plot_model_success_rate(df, output_dir, args.include_values)
    
    # 2. 多指标综合对比图 - 所有模型在不同指标上的对比
    plot_all_models_metrics(df, output_dir, args.include_values)
    
    # 3. 语言和难度级别的模型对比
    plot_detailed_language_difficulty(df, output_dir)
    
    # 4. 为每个模型生成详细分析
    if args.model_details:
        print("Generating detailed analysis for each model...")
        plot_model_detail_comparison(df, output_dir)
    
    # 5. 模型系列分析
    print("Generating model series analysis...")
    plot_model_series_comparison(df, output_dir)
    plot_best_model_by_series(df, output_dir)
    
    # 生成分析报告
    print("Generating summary report...")
    generate_summary_report(df, output_dir)
    
    print(f"Analysis complete. Results saved to {output_dir}")

if __name__ == '__main__':
    main() 