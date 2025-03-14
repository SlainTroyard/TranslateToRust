#!/usr/bin/env python3
"""
Batch Test Script for TranslateToRust

This script automates testing of all Rust files in the 'translated' directory.
It extracts contest, problem, and language information from filenames and runs tests.
"""

import os
import re
import sys
import time
import json
import argparse
from datetime import datetime
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import timeout_handler

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translated")
TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_results")
MAX_WORKERS = 4  # 并行处理的最大线程数
DEFAULT_PROCESS_TIMEOUT = 10 * 60  # 默认进程超时时间（秒）：10分钟

# 辅助函数：打印消息并刷新缓冲区
def log_with_flush(message):
    """打印消息并立即刷新输出缓冲区，防止长时间运行时输出卡住"""
    print(message)
    sys.stdout.flush()

def run_process_with_timeout(cmd, cwd, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """运行子进程，带有超时控制"""
    import subprocess
    
    log_with_flush(f"执行命令: {' '.join(cmd)}, 超时时间: {timeout_seconds}秒")
    
    # 启动子进程
    process = subprocess.Popen(
        cmd,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,  # 行缓冲
    )
    
    # 记录开始时间
    start_time = time.time()
    
    try:
        # 创建输出缓冲区
        stdout_buffer = []
        stderr_buffer = []
        
        # 非阻塞读取输出
        def read_output():
            while process.poll() is None:
                stdout_line = process.stdout.readline()
                if stdout_line:
                    stdout_buffer.append(stdout_line)
                    print(stdout_line, end='')
                    sys.stdout.flush()
                
                stderr_line = process.stderr.readline()
                if stderr_line:
                    stderr_buffer.append(stderr_line)
                    # Don't prefix with ERROR as these can be warnings too
                    print(stderr_line, end='')
                    sys.stdout.flush()
                
                # 短暂休眠，避免CPU资源占用过高
                time.sleep(0.1)
        
        # 在单独的线程中读取输出
        import threading
        output_thread = threading.Thread(target=read_output)
        output_thread.daemon = True
        output_thread.start()
        
        # 等待进程完成或超时
        process.wait(timeout=timeout_seconds)
        
        # 进程完成，等待输出线程结束
        output_thread.join(timeout=2)
        
        # 读取剩余的输出
        remaining_stdout, remaining_stderr = process.communicate()
        if remaining_stdout:
            stdout_buffer.append(remaining_stdout)
        if remaining_stderr:
            stderr_buffer.append(remaining_stderr)
        
        elapsed_time = time.time() - start_time
        log_with_flush(f"进程完成，耗时: {elapsed_time:.2f}秒")
        
        return {
            'returncode': process.returncode,
            'stdout': ''.join(stdout_buffer),
            'stderr': ''.join(stderr_buffer),
            'elapsed_time': elapsed_time
        }
    
    except subprocess.TimeoutExpired:
        log_with_flush(f"进程超时（超过{timeout_seconds}秒）")
        
        # 终止进程
        try:
            process.terminate()
            process.wait(timeout=5)  # 给进程一些时间来清理
        except:
            # 如果进程没有及时终止，强制杀死
            try:
                process.kill()
            except:
                log_with_flush("警告: 无法终止进程")
        
        elapsed_time = time.time() - start_time
        
        return {
            'returncode': -1,
            'stdout': ''.join(stdout_buffer),
            'stderr': ''.join(stderr_buffer) + f"\n进程超时（超过{timeout_seconds}秒）",
            'elapsed_time': elapsed_time,
            'timed_out': True
        }
    
    except Exception as e:
        log_with_flush(f"执行过程中发生错误: {e}")
        
        # 确保进程被终止
        try:
            process.terminate()
        except:
            pass
        
        elapsed_time = time.time() - start_time
        
        return {
            'returncode': -1,
            'stdout': ''.join(stdout_buffer),
            'stderr': ''.join(stderr_buffer) + f"\n执行错误: {e}",
            'elapsed_time': elapsed_time,
            'error': str(e)
        }

def extract_file_info(filename):
    """从文件名中提取比赛、题目和语言信息"""
    pattern = r"weekly_contest_(\d+)_p(\d+)_([a-z]+)\.rs"
    match = re.match(pattern, filename)
    
    if match:
        contest = int(match.group(1))
        problem = int(match.group(2))
        language = match.group(3).upper()  # 转为大写
        return contest, problem, language
    
    return None, None, None

def test_single_file(rust_file_path, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """测试单个Rust文件"""
    filename = os.path.basename(rust_file_path)
    log_with_flush(f"Testing Rust file: {filename}")
    
    # 从文件名中提取信息
    contest, problem, language = extract_file_info(filename)
    if contest is None:
        log_with_flush(f"无法从文件名 {filename} 中提取信息")
        return None
    
    # 准备测试命令
    cmd = [
        "cargo", "run", "--", "test",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language,
        "--rust-solution", str(rust_file_path)
    ]
    
    # 执行测试命令
    result = run_process_with_timeout(
        cmd=cmd,
        cwd=TRANSLATE_TO_RUST_PATH,
        timeout_seconds=timeout_seconds
    )
    
    # 检查是否超时
    timed_out = result.get('timed_out', False)
    if timed_out:
        log_with_flush(f"测试超时：{filename}")
        return {
            'filename': filename,
            'contest': contest,
            'problem': problem,
            'language': language,
            'success': False,
            'timed_out': True,
            'elapsed_time': result['elapsed_time'],
            'stdout': result['stdout'],
            'stderr': result['stderr']
        }
    
    # 分析测试结果
    output = result['stdout']
    
    # 提取测试结果数据
    test_results = {
        'filename': filename,
        'contest': contest,
        'problem': problem,
        'language': language,
        'success': result['returncode'] == 0,
        'elapsed_time': result['elapsed_time']
    }
    
    # 尝试从输出中提取测试统计信息
    compilation_match = re.search(r"编译状态: (\w+)", output)
    if compilation_match:
        test_results['compilation_success'] = compilation_match.group(1) == "成功"
    
    total_match = re.search(r"Total Test Cases: (\d+)", output)
    if total_match:
        test_results['total_cases'] = int(total_match.group(1))
    
    passed_match = re.search(r"Passed: (\d+)", output)
    if passed_match:
        test_results['passed_cases'] = int(passed_match.group(1))
    
    failed_match = re.search(r"Failed: (\d+)", output)
    if failed_match:
        test_results['failed_cases'] = int(failed_match.group(1))
    
    rate_match = re.search(r"Success Rate: ([\d\.]+)%", output)
    if rate_match:
        test_results['success_rate'] = float(rate_match.group(1))
    
    runtime_match = re.search(r"平均运行时间: ([\d\.]+) ms", output)
    if runtime_match:
        test_results['average_runtime'] = float(runtime_match.group(1))
    
    test_results['stdout'] = output
    test_results['stderr'] = result['stderr']
    
    # 定位并记录测试结果文件的位置
    results_dir_match = re.search(r"测试结果已保存到: (.+\.txt)", output)
    if results_dir_match:
        test_results['results_file'] = results_dir_match.group(1)
    
    log_with_flush(f"测试完成: {filename}")
    return test_results

def find_rust_files(directory=None):
    """在指定目录中查找所有Rust文件"""
    if directory is None:
        directory = TRANSLATED_DIR
    
    rust_files = []
    for file in os.listdir(directory):
        if file.endswith(".rs") and file.startswith("weekly_contest_"):
            rust_files.append(os.path.join(directory, file))
    
    return rust_files

def main():
    parser = argparse.ArgumentParser(description="批量测试translated目录中的Rust文件")
    parser.add_argument("--max-workers", type=int, default=MAX_WORKERS, 
                        help=f"最大并行处理线程数，默认为{MAX_WORKERS}")
    parser.add_argument("--timeout", type=int, default=DEFAULT_PROCESS_TIMEOUT, 
                        help=f"每个测试的超时时间（秒），默认为{DEFAULT_PROCESS_TIMEOUT}秒")
    parser.add_argument("--dir", type=str, default=TRANSLATED_DIR, 
                        help="要测试的Rust文件目录")
    parser.add_argument("--file", type=str, 
                        help="指定单个Rust文件进行测试，而不是整个目录")
    
    args = parser.parse_args()
    
    # 查找要测试的文件
    if args.file:
        # 测试单个文件
        rust_files = [args.file]
        log_with_flush(f"将测试单个文件: {args.file}")
    else:
        # 测试目录中的所有文件
        rust_files = find_rust_files(args.dir)
        log_with_flush(f"找到 {len(rust_files)} 个Rust文件待测试")
    
    if not rust_files:
        log_with_flush("没有找到可测试的Rust文件")
        return
    
    # 并行执行测试
    successful_tests = 0
    total_tests = len(rust_files)
    with ThreadPoolExecutor(max_workers=args.max_workers) as executor:
        future_to_file = {
            executor.submit(test_single_file, file, args.timeout): file
            for file in rust_files
        }
        
        for future in future_to_file:
            file = future_to_file[future]
            try:
                result = future.result()
                if result:
                    filename = os.path.basename(file)
                    log_with_flush(f"完成文件测试: {filename}")
                    if result.get('success', False):
                        successful_tests += 1
            except Exception as e:
                print(f"测试文件 {os.path.basename(file)} 时出错: {e}")
    
    # 简单总结
    log_with_flush(f"批量测试完成。共测试了 {total_tests} 个文件，其中 {successful_tests} 个测试成功。")
    log_with_flush(f"测试结果已保存在各个测试的结果文件中，可在 {TEST_RESULTS_DIR} 目录查看。")

if __name__ == "__main__":
    main() 