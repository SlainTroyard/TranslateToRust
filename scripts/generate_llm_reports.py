#!/usr/bin/env python3
"""
Generate Test Reports for LLM Translated Code

This script generates test reports for all LLM models or specified models.
"""

import os
import sys
import subprocess
import argparse
from pathlib import Path

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
LLM_TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_test_results")
LLM_TEST_REPORTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_test_reports")

def log_with_flush(message):
    """打印消息并立即刷新输出缓冲区"""
    print(message)
    sys.stdout.flush()

def ensure_directories():
    """确保所需的目录存在"""
    os.makedirs(LLM_TEST_REPORTS_DIR, exist_ok=True)
    log_with_flush(f"确保目录存在: {LLM_TEST_REPORTS_DIR}")

def get_llm_models():
    """获取llm_test_results目录下的所有LLM模型目录"""
    models = []
    for item in os.listdir(LLM_TEST_RESULTS_DIR):
        item_path = os.path.join(LLM_TEST_RESULTS_DIR, item)
        if os.path.isdir(item_path) and not item.startswith("."):
            models.append(item)
    return models

def generate_model_report(model_name):
    """为特定LLM模型生成测试报告"""
    log_with_flush(f"开始生成 {model_name} 的测试报告...")
    
    # 模型特定的路径
    model_results_dir = os.path.join(LLM_TEST_RESULTS_DIR, model_name)
    model_reports_dir = os.path.join(LLM_TEST_REPORTS_DIR, model_name)
    
    # 确保目录存在
    os.makedirs(model_reports_dir, exist_ok=True)
    
    # 检查是否有测试结果
    if not os.path.exists(model_results_dir) or not os.listdir(model_results_dir):
        log_with_flush(f"错误: {model_name} 没有测试结果，无法生成报告")
        return False
    
    # 构建generate_test_reports.py的命令
    cmd = [
        "python3", os.path.join(TRANSLATE_TO_RUST_PATH, "scripts", "generate_test_reports.py"),
        "--format", "all",
        "--output-dir", model_reports_dir,
        "--days", "30",
        "--llm",
        "--include-details"
    ]
    
    # 设置环境变量以指向模型特定的测试结果目录
    env = os.environ.copy()
    env["TEST_RESULTS_DIR"] = model_results_dir
    
    try:
        # 切换到项目根目录执行命令
        original_dir = os.getcwd()
        os.chdir(TRANSLATE_TO_RUST_PATH)
        
        log_with_flush(f"执行命令: {' '.join(cmd)}")
        process = subprocess.run(cmd, env=env)
        if process.returncode == 0:
            log_with_flush(f"{model_name} 测试报告生成成功")
            return True
        else:
            log_with_flush(f"{model_name} 生成测试报告失败，返回码: {process.returncode}")
            return False
    except Exception as e:
        log_with_flush(f"为 {model_name} 生成测试报告时出错: {e}")
        return False
    finally:
        # 恢复原始工作目录
        os.chdir(original_dir)

def main():
    """主函数"""
    parser = argparse.ArgumentParser(description='为LLM翻译的代码生成测试报告')
    parser.add_argument('--models', nargs='+',
                      help='指定要生成报告的LLM模型，如果不指定则处理所有模型')
    parser.add_argument('--days', type=int, default=30,
                      help='处理最近多少天的测试结果（默认：30）')
    
    args = parser.parse_args()
    
    # 确保目录存在
    ensure_directories()
    
    # 获取LLM模型列表
    all_models = get_llm_models()
    log_with_flush(f"在 llm_test_results 目录下发现 {len(all_models)} 个LLM模型: {', '.join(all_models)}")
    
    # 确定要处理的模型
    target_models = args.models if args.models else all_models
    
    # 过滤不存在的模型
    models_to_process = [model for model in target_models if model in all_models]
    if not models_to_process:
        log_with_flush("错误: 没有找到可处理的LLM模型")
        return 1
    
    log_with_flush(f"将处理以下 {len(models_to_process)} 个LLM模型: {', '.join(models_to_process)}")
    
    all_success = True
    
    # 逐个处理每个模型
    for model in models_to_process:
        log_with_flush(f"\n===== 开始处理 {model} 模型 =====")
        
        if not generate_model_report(model):
            log_with_flush(f"{model} 模型报告生成失败")
            all_success = False
            continue
        
        log_with_flush(f"===== {model} 模型处理完成 =====\n")
    
    if all_success:
        log_with_flush("\n所有LLM模型报告生成成功")
    else:
        log_with_flush("\n警告: 某些LLM模型报告生成失败")
    
    return 0 if all_success else 1

if __name__ == '__main__':
    sys.exit(main()) 