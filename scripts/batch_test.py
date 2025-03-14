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
import argparse
import threading
import fcntl
import random
import psutil
from datetime import datetime
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import timeout_handler

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
TRANSLATED_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translated")
TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_results")
MAX_WORKERS = 2  # 默认并行处理的最大线程数，降低以避免资源争用
DEFAULT_PROCESS_TIMEOUT = 10 * 60  # 默认进程超时时间（秒）：10分钟
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

def run_process_with_timeout(cmd, cwd, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """运行子进程，带有超时控制"""
    import subprocess
    
    # 获取cargo锁
    log_with_flush(f"等待获取cargo锁: {' '.join(cmd)}")
    with cargo_semaphore:
        # 检查系统负载
        while wait_if_system_overloaded():
            pass
        
        lock_file = acquire_cargo_lock()
        log_with_flush(f"获取cargo锁成功，执行命令: {' '.join(cmd)}, 超时时间: {timeout_seconds}秒")
        
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
            process.wait(timeout=timeout_seconds)
            
            # 进程完成，等待输出线程结束
            output_thread.join(timeout=2)
            
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
            
            result = {
                'returncode': -1,
                'stdout': ''.join(stdout_buffer),
                'stderr': ''.join(stderr_buffer) + f"\n进程超时（超过{timeout_seconds}秒）",
                'elapsed_time': elapsed_time,
                'timed_out': True
            }
        
        except Exception as e:
            log_with_flush(f"执行过程中发生错误: {e}", is_error=True)
            
            # 确保进程被终止
            try:
                process.terminate()
            except:
                pass
            
            elapsed_time = time.time() - start_time
            
            result = {
                'returncode': -1,
                'stdout': ''.join(stdout_buffer),
                'stderr': ''.join(stderr_buffer) + f"\n执行错误: {e}",
                'elapsed_time': elapsed_time,
                'error': str(e)
            }
        
        finally:
            # 释放cargo锁
            release_cargo_lock(lock_file)
            
            # 任务之间添加随机延迟，避免资源冲突
            delay = random.uniform(MIN_TASK_DELAY, MAX_TASK_DELAY)
            log_with_flush(f"任务完成，等待 {delay:.1f} 秒后处理下一个任务...")
            time.sleep(delay)
            
            return result

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

def test_single_file(rust_file_path, timeout_seconds=DEFAULT_PROCESS_TIMEOUT, task_id=None):
    """测试单个Rust文件"""
    filename = os.path.basename(rust_file_path)
    task_prefix = f"[任务 {task_id}]" if task_id is not None else ""
    log_with_flush(f"{task_prefix} 开始测试: {filename}")
    
    # 从文件名中提取信息
    contest, problem, language = extract_file_info(filename)
    if contest is None:
        log_with_flush(f"{task_prefix} 无法从文件名 {filename} 中提取信息", is_error=True)
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
        log_with_flush(f"{task_prefix} 测试超时: {filename}", is_error=True)
        return False
    
    # 提取编译状态和成功率
    output = result['stdout']
    
    compilation_success = False
    success_rate = 0
    
    # 尝试从输出中提取测试统计信息
    compilation_match = re.search(r"编译状态: (\w+)", output)
    if compilation_match:
        compilation_success = compilation_match.group(1) == "成功"
    
    rate_match = re.search(r"Success Rate: ([\d\.]+)%", output)
    if rate_match:
        success_rate = float(rate_match.group(1))
    
    # 精简显示测试结果
    compilation_status = "成功" if compilation_success else "失败"
    log_with_flush(f"{task_prefix} 测试完成: {filename} (编译:{compilation_status}, 成功率:{success_rate}%)")
    
    return result['returncode'] == 0

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
    
    # 确保cargo锁文件存在
    if not os.path.exists(CARGO_LOCK_FILE):
        open(CARGO_LOCK_FILE, 'w').close()
    
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
    
    # 记录开始时间
    start_time = time.time()
    
    # 限制cargo并发数为1
    global cargo_semaphore
    cargo_semaphore = threading.Semaphore(1)
    
    # 并行执行测试，但控制cargo并发
    successful_tests = 0
    total_tests = len(rust_files)
    
    with ThreadPoolExecutor(max_workers=args.max_workers) as executor:
        # 先提交所有任务，但实际执行时会受到信号量控制
        futures = []
        for i, file in enumerate(rust_files):
            future = executor.submit(
                test_single_file, 
                file, 
                args.timeout,
                i + 1  # 任务ID
            )
            futures.append((future, file))
        
        # 处理结果
        completed = 0
        for future, file in futures:
            try:
                result = future.result()
                completed += 1
                if result:
                    successful_tests += 1
                
                # 显示进度
                percent = (completed / total_tests) * 100
                log_with_flush(f"进度: {completed}/{total_tests} ({percent:.1f}%)")
            except Exception as e:
                completed += 1
                log_with_flush(f"测试文件 {os.path.basename(file)} 时出错: {e}", is_error=True)
    
    # 计算总耗时
    total_time = time.time() - start_time
    hours = int(total_time // 3600)
    minutes = int((total_time % 3600) // 60)
    seconds = int(total_time % 60)
    time_str = f"{hours}小时 {minutes}分钟 {seconds}秒"
    
    # 简单总结
    log_with_flush(f"批量测试完成。耗时: {time_str}")
    log_with_flush(f"共测试了 {total_tests} 个文件，其中 {successful_tests} 个测试成功 ({successful_tests/total_tests*100:.1f}%)。")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        log_with_flush("\n用户中断，正在退出...", is_error=True)
        sys.exit(1)
    except Exception as e:
        log_with_flush(f"执行脚本时发生错误: {e}", is_error=True)
        import traceback
        traceback.print_exc()
        sys.exit(1) 