#!/usr/bin/env python3
"""
Batch Test Script for LLM Translated Code

This script tests LLM translated code from different LLM models and
saves test results for each model.
"""

import os
import sys
import subprocess
import argparse
import shutil
from pathlib import Path

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
LLM_TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_translated")
LLM_TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_test_results")
TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_results")

def log_with_flush(message):
    """打印消息并立即刷新输出缓冲区"""
    print(message)
    sys.stdout.flush()

def ensure_directories(model_name=None):
    """确保所需的目录存在"""
    directories = [LLM_TEST_RESULTS_DIR]
    
    # 如果指定了模型名，则创建模型特定的子目录
    if model_name:
        directories = [os.path.join(LLM_TEST_RESULTS_DIR, model_name)]
    
    for directory in directories:
        os.makedirs(directory, exist_ok=True)
        log_with_flush(f"确保目录存在: {directory}")

def get_llm_models():
    """获取llm_translated目录下的所有LLM模型目录"""
    models = []
    for item in os.listdir(LLM_TRANSLATED_DIR):
        item_path = os.path.join(LLM_TRANSLATED_DIR, item)
        if os.path.isdir(item_path) and not item.startswith("."):
            models.append(item)
    return models

def patch_batch_test_script(model_name):
    """临时修改batch_test.py以测试特定LLM模型的翻译结果"""
    batch_test_path = os.path.join(TRANSLATE_TO_RUST_PATH, "scripts", "batch_test.py")
    temp_script_path = os.path.join(TRANSLATE_TO_RUST_PATH, "scripts", f"batch_test_{model_name}_temp.py")
    
    try:
        # 读取原始脚本
        with open(batch_test_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 替换TRANSLATED_DIR的定义，指向特定模型的目录
        modified_content = content.replace(
            f'TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translated")', 
            f'TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_translated", "{model_name}")'
        )
        
        # 修改find_rust_files函数以递归搜索子目录
        original_find_rust = '''def find_rust_files(directory=None):
    """在指定目录中查找所有Rust文件"""
    if directory is None:
        directory = TRANSLATED_DIR
    
    rust_files = []
    for file in os.listdir(directory):
        if file.endswith(".rs") and file.startswith("weekly_contest_"):
            rust_files.append(os.path.join(directory, file))
    
    return rust_files'''
        
        recursive_find_rust = '''def find_rust_files(directory=None):
    """在指定目录中递归查找所有Rust文件"""
    if directory is None:
        directory = TRANSLATED_DIR
    
    rust_files = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs") and file.startswith("weekly_contest_"):
                rust_files.append(os.path.join(root, file))
    
    return rust_files'''
        
        modified_content = modified_content.replace(original_find_rust, recursive_find_rust)
        
        # 写入临时脚本
        with open(temp_script_path, 'w', encoding='utf-8') as f:
            f.write(modified_content)
        
        return temp_script_path
    except Exception as e:
        log_with_flush(f"修改批处理脚本时出错: {e}")
        return None

def move_test_results(model_name):
    """将test_results目录中的测试结果移动到llm_test_results/<model_name>目录"""
    # 确保模型结果目录存在
    model_results_dir = os.path.join(LLM_TEST_RESULTS_DIR, model_name)
    os.makedirs(model_results_dir, exist_ok=True)
    
    # 获取所有测试结果和详情目录
    test_result_files = []
    test_details_dirs = []
    
    for item in os.listdir(TEST_RESULTS_DIR):
        item_path = os.path.join(TEST_RESULTS_DIR, item)
        if item.startswith("test_results_") and item.endswith(".txt"):
            test_result_files.append(item_path)
        elif item.startswith("details_"):
            test_details_dirs.append(item_path)
    
    # 复制所有测试结果和详情目录
    for result_file in test_result_files:
        result_name = os.path.basename(result_file)
        target_result = os.path.join(model_results_dir, result_name)
        shutil.copy2(result_file, target_result)
        log_with_flush(f"已复制测试结果: {result_file} -> {target_result}")
    
    for details_dir in test_details_dirs:
        details_name = os.path.basename(details_dir)
        target_details = os.path.join(model_results_dir, details_name)
        if os.path.exists(target_details):
            shutil.rmtree(target_details)
        shutil.copytree(details_dir, target_details)
        log_with_flush(f"已复制测试详情: {details_dir} -> {target_details}")
    
    # 清理原始测试结果目录
    log_with_flush("清理原始测试结果目录...")
    for item in test_result_files + test_details_dirs:
        try:
            if os.path.isdir(item):
                shutil.rmtree(item)
                log_with_flush(f"已删除目录: {item}")
            else:
                os.remove(item)
                log_with_flush(f"已删除文件: {item}")
        except Exception as e:
            log_with_flush(f"清理 {item} 时出错: {e}")
    
    return True

def run_tests(model_name, max_workers, timeout):
    """运行特定LLM模型的测试并保存结果"""
    log_with_flush(f"开始运行 {model_name} 模型的测试...")
    
    # 确保模型目录存在
    ensure_directories(model_name)
    
    # 临时修改批处理脚本
    temp_script_path = patch_batch_test_script(model_name)
    if not temp_script_path:
        return False
    
    try:
        # 构建批处理命令
        cmd = [
            "python3", temp_script_path,
            "--max-workers", str(max_workers),
            "--timeout", str(timeout)
        ]
        
        log_with_flush(f"运行命令: {' '.join(cmd)}")
        process = subprocess.run(cmd)
        
        # 即使测试失败，我们也尝试移动结果
        if not move_test_results(model_name):
            log_with_flush(f"{model_name} 移动测试结果失败")
            return False
        
        if process.returncode != 0:
            log_with_flush(f"{model_name} 测试执行失败，返回码: {process.returncode}")
            return False
        
        log_with_flush(f"{model_name} 测试执行完成")
        return True
    except Exception as e:
        log_with_flush(f"运行 {model_name} 测试时出错: {e}")
        return False
    finally:
        # 清理临时脚本
        try:
            if os.path.exists(temp_script_path):
                os.remove(temp_script_path)
        except Exception as e:
            log_with_flush(f"清理临时脚本时出错: {e}")

def main():
    """主函数"""
    parser = argparse.ArgumentParser(description='批量测试多个LLM翻译的代码')
    parser.add_argument('--max-workers', type=int, default=2,
                      help='最大并行工作线程数（默认：2）')
    parser.add_argument('--timeout', type=int, default=1500,
                      help='每个测试任务的超时时间，单位秒（默认：1500）')
    parser.add_argument('--models', nargs='+',
                      help='指定要测试的LLM模型，如果不指定则测试所有模型')
    
    args = parser.parse_args()
    
    # 确保顶级目录存在
    ensure_directories()
    
    # 获取LLM模型列表
    all_models = get_llm_models()
    log_with_flush(f"在 llm_translated 目录下发现 {len(all_models)} 个LLM模型: {', '.join(all_models)}")
    
    # 确定要测试的模型
    target_models = args.models if args.models else all_models
    
    # 过滤不存在的模型
    models_to_test = [model for model in target_models if model in all_models]
    if not models_to_test:
        log_with_flush("错误: 没有找到可测试的LLM模型")
        return 1
    
    log_with_flush(f"将测试以下 {len(models_to_test)} 个LLM模型: {', '.join(models_to_test)}")
    
    all_success = True
    
    # 逐个测试每个模型
    for model in models_to_test:
        log_with_flush(f"\n===== 开始处理 {model} 模型 =====")
        
        if not run_tests(model, args.max_workers, args.timeout):
            log_with_flush(f"{model} 模型测试失败")
            all_success = False
            continue
        
        log_with_flush(f"===== {model} 模型处理完成 =====\n")
    
    if all_success:
        log_with_flush("\n所有LLM模型测试成功")
    else:
        log_with_flush("\n警告: 某些LLM模型测试失败")
    
    return 0 if all_success else 1

if __name__ == '__main__':
    sys.exit(main()) 