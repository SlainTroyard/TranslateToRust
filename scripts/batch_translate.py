#!/usr/bin/env python3
"""
Batch Translation and Testing Script for TranslateToRust

This script combines translation and testing of C/C++ code to Rust.
It's an all-in-one solution that first translates the code and then tests it.
"""

import os
import subprocess
import json
import re
import sys
import csv
import signal
import time
import glob
import argparse
import random
import threading
import fcntl
import psutil
from datetime import datetime
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import timeout_handler

# 定义常量
FUZZ_FOR_LEETCODE_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent.parent / "FuzzForLeetcode")
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translated")
TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_results")
TRANSLATION_REPORTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translation_reports")
MAX_WORKERS = 2  # 并行处理的最大线程数，降低以避免资源争用
DEFAULT_PROCESS_TIMEOUT = 60 * 60  # 默认进程超时时间（秒）：60分钟
CARGO_LOCK_FILE = os.path.join(TRANSLATE_TO_RUST_PATH, ".cargo_test_lock")
MIN_TASK_DELAY = 2  # 任务之间的最小延迟（秒）
MAX_TASK_DELAY = 5  # 任务之间的最大延迟（秒）

# 创建一个全局信号量，用于限制并发任务数量
cargo_semaphore = threading.Semaphore(1)  # 一次只允许一个cargo命令运行

# 辅助函数：打印消息并刷新缓冲区
def log_with_flush(message, is_error=False):
    """打印消息并立即刷新输出缓冲区，防止长时间运行时输出卡住"""
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    if is_error:
        print(f"[{timestamp}] [ERROR] {message}", file=sys.stderr)
        sys.stderr.flush()
    else:
        print(f"[{timestamp}] {message}")
    sys.stdout.flush()

def get_system_load():
    """获取系统负载信息"""
    return {
        'cpu_percent': psutil.cpu_percent(interval=0.1),
        'memory_percent': psutil.virtual_memory().percent,
        'available_memory_gb': round(psutil.virtual_memory().available / (1024**3), 2)
    }

def wait_if_system_overloaded():
    """如果系统负载过高，等待一段时间"""
    load = get_system_load()
    if load['cpu_percent'] > 80 or load['memory_percent'] > 80:
        delay = random.uniform(5, 15)  # 随机等待5-15秒
        log_with_flush(f"系统负载过高 (CPU: {load['cpu_percent']}%, 内存: {load['memory_percent']}%), 等待 {delay:.1f} 秒")
        time.sleep(delay)
        return True
    return False

def acquire_cargo_lock():
    """获取cargo操作的文件锁"""
    # 确保锁文件存在
    if not os.path.exists(CARGO_LOCK_FILE):
        open(CARGO_LOCK_FILE, 'w').close()
    
    # 尝试获取锁
    lock_file = open(CARGO_LOCK_FILE, 'r+')
    while True:
        try:
            fcntl.flock(lock_file, fcntl.LOCK_EX | fcntl.LOCK_NB)
            return lock_file
        except IOError:
            log_with_flush("等待其他cargo进程完成...")
            time.sleep(1)

def release_cargo_lock(lock_file):
    """释放cargo操作的文件锁"""
    fcntl.flock(lock_file, fcntl.LOCK_UN)
    lock_file.close()

def run_process_with_timeout(cmd, cwd, timeout_seconds=DEFAULT_PROCESS_TIMEOUT, acquire_lock=True):
    """运行子进程，带有超时控制"""
    if acquire_lock:
        # 获取cargo锁
        log_with_flush(f"等待获取cargo锁: {' '.join(cmd)}")
        with cargo_semaphore:
            # 检查系统负载
            while wait_if_system_overloaded():
                pass
            
            lock_file = acquire_cargo_lock()
            log_with_flush(f"获取cargo锁成功，执行命令: {' '.join(cmd)}, 超时时间: {timeout_seconds}秒")
    else:
    log_with_flush(f"执行命令: {' '.join(cmd)}, 超时时间: {timeout_seconds}秒")
    
    try:
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
    
        # 创建输出缓冲区
        stdout_buffer = []
        stderr_buffer = []
        # 标记是否生成了测试结果文件
        test_files_generated = False
        
        # 非阻塞读取输出
        def read_output():
            nonlocal test_files_generated
            while process.poll() is None:
                stdout_line = process.stdout.readline()
                if stdout_line:
                    stdout_buffer.append(stdout_line)
                    print(stdout_line, end='')
                    sys.stdout.flush()
                    # 检查是否已生成测试结果文件
                    if "测试结果已保存到:" in stdout_line or "Test results saved to:" in stdout_line:
                        test_files_generated = True
                
                stderr_line = process.stderr.readline()
                if stderr_line:
                    stderr_buffer.append(stderr_line)
                    # 检查是否包含warning
                    if "warning:" in stderr_line:
                        # 警告输出到标准输出，不作为错误处理
                        print(stderr_line, end='')
                    sys.stdout.flush()
                    else:
                        # 真正的错误输出到标准错误
                        print(stderr_line, end='', file=sys.stderr)
                        sys.stderr.flush()
                
                # 短暂休眠，避免CPU资源占用过高
                time.sleep(0.1)
        
        # 在单独的线程中读取输出
        output_thread = threading.Thread(target=read_output)
        output_thread.daemon = True
        output_thread.start()
        
        # 等待进程完成或超时
        try:
        process.wait(timeout=timeout_seconds)
        # 进程完成，等待输出线程结束
        output_thread.join(timeout=2)
        except subprocess.TimeoutExpired:
            log_with_flush(f"进程超时（超过{timeout_seconds}秒）", is_error=True)
            
            # 终止进程
            try:
                process.terminate()
                process.wait(timeout=5)  # 给进程一些时间来清理
            except:
                # 如果进程没有及时终止，强制杀死
                try:
                    process.kill()
                except:
                    log_with_flush("警告: 无法终止进程", is_error=True)
            
            elapsed_time = time.time() - start_time
            
            # 读取剩余的输出
            remaining_stdout, remaining_stderr = "", ""
            try:
                remaining_stdout, remaining_stderr = process.communicate(timeout=2)
            except:
                pass
            
            if remaining_stdout:
                stdout_buffer.append(remaining_stdout)
                print(remaining_stdout, end='')
                sys.stdout.flush()
            if remaining_stderr:
                stderr_buffer.append(remaining_stderr)
                print(remaining_stderr, end='', file=sys.stderr)
                sys.stderr.flush()
            
            result = {
                'returncode': -1,
                'stdout': ''.join(stdout_buffer),
                'stderr': ''.join(stderr_buffer) + f"\n进程超时（超过{timeout_seconds}秒）",
                'elapsed_time': elapsed_time,
                'timed_out': True
            }
            
            if acquire_lock:
                release_cargo_lock(lock_file)
            
            # 任务之间添加随机延迟，避免资源冲突
            delay = random.uniform(MIN_TASK_DELAY, MAX_TASK_DELAY)
            log_with_flush(f"任务完成，等待 {delay:.1f} 秒后处理下一个任务...")
            time.sleep(delay)
            
            return result
        
        # 进程正常完成
        # 读取剩余的输出
        remaining_stdout, remaining_stderr = process.communicate()
        if remaining_stdout:
            stdout_buffer.append(remaining_stdout)
            print(remaining_stdout, end='')
            sys.stdout.flush()
        if remaining_stderr:
            stderr_buffer.append(remaining_stderr)
            if "warning:" in remaining_stderr:
                print(remaining_stderr, end='')
                sys.stdout.flush()
            else:
                print(remaining_stderr, end='', file=sys.stderr)
                sys.stderr.flush()
        
        elapsed_time = time.time() - start_time
        log_with_flush(f"进程完成，耗时: {elapsed_time:.2f}秒")
        
        result = {
            'returncode': process.returncode,
            'stdout': ''.join(stdout_buffer),
            'stderr': ''.join(stderr_buffer),
            'elapsed_time': elapsed_time
        }
    
        if acquire_lock:
            release_cargo_lock(lock_file)
        
        # 任务之间添加随机延迟，避免资源冲突
        delay = random.uniform(MIN_TASK_DELAY, MAX_TASK_DELAY)
        log_with_flush(f"任务完成，等待 {delay:.1f} 秒后处理下一个任务...")
        time.sleep(delay)
        
        return result
    except Exception as e:
        log_with_flush(f"执行过程中发生错误: {e}", is_error=True)
        
        # 确保进程被终止
        try:
            process.terminate()
        except:
            pass
        
        elapsed_time = time.time() - start_time
        
        if acquire_lock:
            release_cargo_lock(lock_file)
        
        return {
            'returncode': -1,
            'stdout': ''.join(stdout_buffer),
            'stderr': ''.join(stderr_buffer) + f"\n执行错误: {e}",
            'elapsed_time': elapsed_time,
            'error': str(e)
        }

def get_difficulty(contest, problem, language):
    """获取题目难度"""
    constraint_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "constraints", 
                                  f"weekly_contest_{contest}_p{problem}.json")
    try:
        with open(constraint_path, 'r') as f:
            data = json.load(f)
            return data.get("problem_info", {}).get("difficulty", "Unknown")
    except Exception as e:
        log_with_flush(f"Error getting difficulty for {contest} p{problem} ({language}): {e}", is_error=True)
        return "Unknown"

def get_tags(contest, problem, language):
    """获取题目标签"""
    constraint_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "constraints", 
                                  f"weekly_contest_{contest}_p{problem}.json")
    try:
        with open(constraint_path, 'r') as f:
            data = json.load(f)
            return data.get("problem_info", {}).get("tags", [])
    except Exception as e:
        log_with_flush(f"Error getting tags for {contest} p{problem} ({language}): {e}", is_error=True)
        return []

def get_title(contest, problem, language):
    """获取题目标题"""
    constraint_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "constraints", 
                                  f"weekly_contest_{contest}_p{problem}.json")
    try:
        with open(constraint_path, 'r') as f:
            data = json.load(f)
            return data.get("problem_info", {}).get("title", "Unknown")
    except Exception as e:
        log_with_flush(f"Error getting title for {contest} p{problem} ({language}): {e}", is_error=True)
        return "Unknown"

def extract_file_info_from_path(file_path):
    """从文件路径中提取比赛、题目和语言信息"""
    filename = os.path.basename(file_path)
    
    # 对于C/C++源文件
    pattern_source = r"weekly_contest_(\d+)_p(\d+)\.([a-z]+)"
    match_source = re.match(pattern_source, filename)
    if match_source:
        contest = int(match_source.group(1))
        problem = int(match_source.group(2))
        ext = match_source.group(3).lower()
        language = "CPP" if ext == "cpp" else "C"
        return contest, problem, language
    
    # 对于Rust翻译文件
    pattern_rust = r"weekly_contest_(\d+)_p(\d+)_([a-z]+)\.rs"
    match_rust = re.match(pattern_rust, filename)
    if match_rust:
        contest = int(match_rust.group(1))
        problem = int(match_rust.group(2))
        language = match_rust.group(3).upper()
        return contest, problem, language
    
    return None, None, None

def find_source_files(language=None, contest=None, problem=None):
    """查找所有需要翻译的源文件"""
    source_files = []
    
    # 确定要搜索的目录
    if language is None or language.upper() == "CPP":
        # 添加C++文件
        cpp_src_dir = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", "CPP", "src")
        pattern = os.path.join(cpp_src_dir, "weekly_contest_*.cpp")
        for file_path in glob.glob(pattern):
            c, p, l = extract_file_info_from_path(file_path)
            if (contest is None or c == contest) and (problem is None or p == problem):
                source_files.append(file_path)
    
    if language is None or language.upper() == "C":
        # 添加C文件
        c_src_dir = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", "C", "src")
        pattern = os.path.join(c_src_dir, "weekly_contest_*.c")
        for file_path in glob.glob(pattern):
            c, p, l = extract_file_info_from_path(file_path)
            if (contest is None or c == contest) and (problem is None or p == problem):
                source_files.append(file_path)
    
    return source_files

def translate_file(contest, problem, language, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """翻译单个C/C++文件到Rust"""
    log_with_flush(f"翻译 Weekly Contest {contest} Problem {problem} ({language})...")
    
    # 准备翻译命令
    cmd = [
        "cargo", "run", "--", "translate",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language,
        "--method", "llm"
    ]
    
    # 执行翻译命令
    result = run_process_with_timeout(
        cmd=cmd,
        cwd=TRANSLATE_TO_RUST_PATH,
        timeout_seconds=timeout_seconds
    )
    
    # 检查是否超时
    timed_out = result.get('timed_out', False)
    if timed_out:
        log_with_flush(f"翻译超时：Weekly Contest {contest} Problem {problem} ({language})", is_error=True)
        return None
    
    # 检查翻译是否成功
    if result['returncode'] != 0:
        log_with_flush(f"翻译失败：Weekly Contest {contest} Problem {problem} ({language})", is_error=True)
        return None
    
    # 返回翻译后的Rust文件路径
    rust_file = os.path.join(TRANSLATED_DIR, f"weekly_contest_{contest}_p{problem}_{language.lower()}.rs")
    if os.path.exists(rust_file):
        log_with_flush(f"翻译成功：{rust_file}")
        return rust_file
    else:
        log_with_flush(f"翻译结果未找到：{rust_file}", is_error=True)
        return None

def test_rust_file(rust_file_path, contest, problem, language, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """测试翻译后的Rust文件"""
    if not rust_file_path or not os.path.exists(rust_file_path):
        log_with_flush(f"Rust文件不存在: {rust_file_path}", is_error=True)
        return None
    
    log_with_flush(f"测试 Rust 文件: {os.path.basename(rust_file_path)}")
    
    # 准备测试命令
    cmd = [
        "cargo", "run", "--", "test",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language,
        "--rust-solution", rust_file_path
    ]
    
    # 执行测试命令
    result = run_process_with_timeout(
            cmd=cmd,
            cwd=TRANSLATE_TO_RUST_PATH,
            timeout_seconds=timeout_seconds
        )
        
    # 提取测试结果
    output = result['stdout']
    
    # 提取编译状态
    compilation_success = "编译状态: 成功" in output or "Compilation Status: Success" in output
    
    # 如果编译成功，提取测试结果
    if compilation_success:
        total_match = re.search(r"总测试用例: (\d+)", output)
        if not total_match:
            total_match = re.search(r"Total Test Cases: (\d+)", output)
            
        passed_match = re.search(r"通过: (\d+)", output)
        if not passed_match:
            passed_match = re.search(r"Passed: (\d+)", output)
            
        failed_match = re.search(r"失败: (\d+)", output)
        if not failed_match:
            failed_match = re.search(r"Failed: (\d+)", output)
            
        success_rate_match = re.search(r"成功率: ([\d\.]+)%", output)
        if not success_rate_match:
            success_rate_match = re.search(r"Success Rate: ([\d\.]+)%", output)
            
        runtime_match = re.search(r"平均运行时间: ([\d\.]+) ms", output)
        
        test_results = {
            'compilation_success': True,
            'total_cases': int(total_match.group(1)) if total_match else 0,
            'passed_cases': int(passed_match.group(1)) if passed_match else 0,
            'failed_cases': int(failed_match.group(1)) if failed_match else 0,
            'success_rate': float(success_rate_match.group(1)) if success_rate_match else 0.0,
            'average_runtime': float(runtime_match.group(1)) if runtime_match else 0.0
        }
        
        # 提取测试结果文件路径
        result_file_match = re.search(r"测试结果已保存到: (.+?)[\r\n]", output)
        if not result_file_match:
            result_file_match = re.search(r"Test results saved to: (.+?)[\r\n]", output)
            
        if result_file_match:
            test_results['result_file'] = result_file_match.group(1)
        
        log_with_flush(f"测试结果: 通过 {test_results['passed_cases']}/{test_results['total_cases']} 成功率 {test_results['success_rate']}%")
        
        return test_results
    else:
        # 如果编译失败，提取编译错误
        compilation_error_match = re.search(r"编译错误: (.+?)[\r\n]", output)
        if not compilation_error_match:
            compilation_error_match = re.search(r"Compilation Error: (.+?)[\r\n]", output)
            
        error_message = compilation_error_match.group(1) if compilation_error_match else "Unknown compilation error"
        
        log_with_flush(f"编译失败: {error_message}", is_error=True)
        
        return {
            'compilation_success': False,
            'error': error_message
        }

def process_solution(contest, problem, language, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """处理单个解决方案的翻译和测试"""
    log_with_flush(f"处理 Weekly Contest {contest} Problem {problem} ({language})...")
    
    # 获取题目信息
    difficulty = get_difficulty(contest, problem, language)
    tags = get_tags(contest, problem, language)
    title = get_title(contest, problem, language)
    
    # 步骤1：翻译
    log_with_flush(f"步骤 1/2: 翻译 {language} 代码...")
    start_time = time.time()
    rust_file = translate_file(contest, problem, language, timeout_seconds)
    translation_time = time.time() - start_time
    
    if not rust_file:
        log_with_flush(f"翻译失败，跳过测试", is_error=True)
        return {
                "contest": contest,
                "problem": problem,
                "title": title,
                "language": language,
                "difficulty": difficulty,
                "tags": tags,
                "translation_success": False,
                "compilation_success": False,
                "test_cases_total": 0,
                "test_cases_passed": 0,
                "test_cases_failed": 0,
                "success_rate": 0.0,
                "average_runtime": 0.0,
            "translation_time": translation_time,
            "total_time": translation_time
        }
    
    # 步骤2：测试
    log_with_flush(f"步骤 2/2: 测试翻译后的Rust代码...")
    start_time = time.time()
    test_results = test_rust_file(rust_file, contest, problem, language, timeout_seconds)
    testing_time = time.time() - start_time
    
    if not test_results:
        log_with_flush(f"测试失败", is_error=True)
        return {
            "contest": contest,
            "problem": problem,
            "title": title,
            "language": language,
            "difficulty": difficulty,
            "tags": tags,
            "translation_success": True,
            "compilation_success": False,
            "test_cases_total": 0,
            "test_cases_passed": 0,
            "test_cases_failed": 0,
            "success_rate": 0.0,
            "average_runtime": 0.0,
            "translation_time": translation_time,
            "testing_time": testing_time,
            "total_time": translation_time + testing_time
        }
    
    # 汇总结果
    compilation_success = test_results.get('compilation_success', False)
    
    result = {
        "contest": contest,
        "problem": problem,
        "title": title,
        "language": language,
        "difficulty": difficulty,
        "tags": tags,
        "translation_success": True,
        "compilation_success": compilation_success,
        "rust_file": rust_file,
        "translation_time": translation_time,
        "testing_time": testing_time,
        "total_time": translation_time + testing_time
    }
    
    # 如果编译成功，添加测试结果
    if compilation_success:
        result.update({
            "test_cases_total": test_results.get('total_cases', 0),
            "test_cases_passed": test_results.get('passed_cases', 0),
            "test_cases_failed": test_results.get('failed_cases', 0),
            "success_rate": test_results.get('success_rate', 0.0),
            "average_runtime": test_results.get('average_runtime', 0.0),
            "result_file": test_results.get('result_file', None)
        })
    else:
        result.update({
            "test_cases_total": 0,
            "test_cases_passed": 0,
            "test_cases_failed": 0,
            "success_rate": 0.0,
            "average_runtime": 0.0,
            "error": test_results.get('error', "Unknown error")
        })
    
    log_with_flush(f"处理完成: Weekly Contest {contest} Problem {problem} ({language})")
    return result

def generate_report(results, output_dir=None):
    """生成测试报告"""
    if not results:
        log_with_flush("没有结果可以生成报告", is_error=True)
        return
    
    if output_dir is None:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_dir = os.path.join(TRANSLATION_REPORTS_DIR, f"report_{timestamp}")
    
    os.makedirs(output_dir, exist_ok=True)
    
    # 生成CSV报告
    csv_file = os.path.join(output_dir, "results.csv")
    log_with_flush(f"生成CSV报告: {csv_file}")
    
    with open(csv_file, 'w', newline='') as f:
        fieldnames = [
            "contest", "problem", "title", "language", "difficulty",
            "translation_success", "compilation_success",
            "test_cases_total", "test_cases_passed", "test_cases_failed",
            "success_rate", "average_runtime", "translation_time", "testing_time", "total_time"
        ]
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        
        for result in results:
            # 准备行数据
            row = {field: result.get(field, "") for field in fieldnames}
            writer.writerow(row)
    
    # 生成Markdown报告
    md_file = os.path.join(output_dir, "results.md")
    log_with_flush(f"生成Markdown报告: {md_file}")
    
    with open(md_file, 'w') as f:
        f.write("# 翻译与测试报告\n\n")
        f.write(f"生成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        
        # 总体统计
        total_solutions = len(results)
        translation_success = sum(1 for r in results if r.get("translation_success", False))
        compilation_success = sum(1 for r in results if r.get("compilation_success", False))
        test_success = sum(1 for r in results if r.get("test_cases_passed", 0) == r.get("test_cases_total", 0) and r.get("test_cases_total", 0) > 0)
        
        f.write("## 总体统计\n\n")
        f.write(f"- 总解决方案数: {total_solutions}\n")
        f.write(f"- 翻译成功数: {translation_success} ({translation_success/total_solutions*100:.2f}%)\n")
        f.write(f"- 编译成功数: {compilation_success} ({compilation_success/total_solutions*100:.2f}%)\n")
        f.write(f"- 测试全部通过数: {test_success} ({test_success/total_solutions*100:.2f}%)\n\n")
        
        # 按难度统计
        difficulty_stats = {}
        for result in results:
            difficulty = result.get("difficulty", "Unknown")
            if difficulty not in difficulty_stats:
                difficulty_stats[difficulty] = {
                    "total": 0,
                    "translation_success": 0,
                    "compilation_success": 0,
                    "test_success": 0
                }
            
            difficulty_stats[difficulty]["total"] += 1
            if result.get("translation_success", False):
                difficulty_stats[difficulty]["translation_success"] += 1
            if result.get("compilation_success", False):
                difficulty_stats[difficulty]["compilation_success"] += 1
            if result.get("test_cases_passed", 0) == result.get("test_cases_total", 0) and result.get("test_cases_total", 0) > 0:
                difficulty_stats[difficulty]["test_success"] += 1
        
        f.write("## 按难度统计\n\n")
        f.write("| 难度 | 总数 | 翻译成功率 | 编译成功率 | 测试通过率 |\n")
        f.write("|------|------|------------|------------|------------|\n")
        
        for difficulty, stats in difficulty_stats.items():
            f.write(f"| {difficulty} | {stats['total']} | {stats['translation_success']/stats['total']*100:.2f}% | {stats['compilation_success']/stats['total']*100:.2f}% | {stats['test_success']/stats['total']*100:.2f}% |\n")
        
        f.write("\n## 详细结果\n\n")
        f.write("| 比赛 | 题目 | 语言 | 难度 | 翻译 | 编译 | 测试用例 | 通过率 | 平均运行时间 |\n")
        f.write("|------|------|------|------|------|------|----------|--------|---------------|\n")
        
        for result in results:
            contest = result.get("contest", "")
            problem = result.get("problem", "")
            title = result.get("title", "Unknown")
            language = result.get("language", "")
            difficulty = result.get("difficulty", "Unknown")
            translation = "✅" if result.get("translation_success", False) else "❌"
            compilation = "✅" if result.get("compilation_success", False) else "❌"
            test_cases = f"{result.get('test_cases_passed', 0)}/{result.get('test_cases_total', 0)}"
            success_rate = f"{result.get('success_rate', 0.0):.2f}%"
            runtime = f"{result.get('average_runtime', 0.0):.2f} ms"
            
            f.write(f"| {contest} | {problem} | {language} | {difficulty} | {translation} | {compilation} | {test_cases} | {success_rate} | {runtime} |\n")
    
    log_with_flush(f"报告生成完成，保存在: {output_dir}")
    return output_dir

def main():
    """主函数"""
    parser = argparse.ArgumentParser(description="Batch translate and test C/C++ to Rust")
    parser.add_argument("--contest", type=int, help="Contest number to process")
    parser.add_argument("--problem", type=int, help="Problem number to process")
    parser.add_argument("--language", type=str, choices=["C", "CPP"], help="Language to process")
    parser.add_argument("--output", type=str, help="Output directory for reports")
    parser.add_argument("--timeout", type=int, default=DEFAULT_PROCESS_TIMEOUT, help="Timeout for each process in seconds")
    parser.add_argument("--max-workers", type=int, default=MAX_WORKERS, help="Maximum number of parallel workers")
    args = parser.parse_args()
    
    # 设置全局变量
    global MAX_WORKERS
    MAX_WORKERS = args.max_workers
    
    # 创建目录
    os.makedirs(TRANSLATED_DIR, exist_ok=True)
    os.makedirs(TEST_RESULTS_DIR, exist_ok=True)
    os.makedirs(TRANSLATION_REPORTS_DIR, exist_ok=True)
    
    log_with_flush(f"开始批量翻译和测试，最大工作线程数: {MAX_WORKERS}")
    
    # 找到所有需要处理的源文件
    source_files = find_source_files(args.language, args.contest, args.problem)
    
    if not source_files:
        log_with_flush("没有找到源文件", is_error=True)
        return
    
    log_with_flush(f"找到 {len(source_files)} 个源文件")
    
    results = []
    
    # 处理所有源文件
    for file_path in source_files:
        contest, problem, language = extract_file_info_from_path(file_path)
        result = process_solution(contest, problem, language, args.timeout)
                if result:
                    results.append(result)
    
    # 生成报告
    if results:
        output_dir = generate_report(results, args.output)
        log_with_flush(f"批量翻译和测试完成，报告保存在: {output_dir}")
    else:
        log_with_flush("批量翻译和测试完成，但没有结果")
    
if __name__ == "__main__":
    timeout_handler.setup_signal_handlers()
    try:
    main() 
    except KeyboardInterrupt:
        log_with_flush("用户中断，正在退出...", is_error=True)
        sys.exit(1) 