#!/usr/bin/env python3
"""
Batch Translation Script for TranslateToRust

This script automates translation of C/C++ files to Rust.
It only performs the translation step without testing.
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
import glob

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
OUTPUT_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translated")
TRANSLATION_REPORTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "translation_reports")
FUZZ_PROJECT_ROOT = os.path.join(TRANSLATE_TO_RUST_PATH, "..", "FuzzForLeetcode")
CPP_SRC_DIR = os.path.join(FUZZ_PROJECT_ROOT, "C_CPP", "CPP", "src")
C_SRC_DIR = os.path.join(FUZZ_PROJECT_ROOT, "C_CPP", "C", "src")

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

def extract_file_info_from_path(file_path):
    """从文件路径中提取比赛、题目和语言信息"""
    filename = os.path.basename(file_path)
    pattern = r"weekly_contest_(\d+)_p(\d+)\.([a-z]+)"
    match = re.match(pattern, filename)
    
    if match:
        contest = int(match.group(1))
        problem = int(match.group(2))
        ext = match.group(3).lower()
        language = "CPP" if ext == "cpp" else "C"
        return contest, problem, language
    
    return None, None, None

def find_source_files(language=None, contest=None, problem=None):
    """查找所有需要翻译的源文件"""
    source_files = []
    
    # 确定要搜索的目录
    if language is None or language.upper() == "CPP":
        # 添加C++文件
        pattern = os.path.join(CPP_SRC_DIR, "weekly_contest_*.cpp")
        for file_path in glob.glob(pattern):
            c, p, l = extract_file_info_from_path(file_path)
            if (contest is None or c == contest) and (problem is None or p == problem):
                source_files.append(file_path)
    
    if language is None or language.upper() == "C":
        # 添加C文件
        pattern = os.path.join(C_SRC_DIR, "weekly_contest_*.c")
        for file_path in glob.glob(pattern):
            c, p, l = extract_file_info_from_path(file_path)
            if (contest is None or c == contest) and (problem is None or p == problem):
                source_files.append(file_path)
    
    return source_files

def translate_file(file_path, method="llm", timeout_seconds=DEFAULT_PROCESS_TIMEOUT, output_dir=None):
    """翻译单个C/C++文件到Rust"""
    filename = os.path.basename(file_path)
    log_with_flush(f"翻译源文件: {filename}")
    
    # 从文件路径提取信息
    contest, problem, language = extract_file_info_from_path(file_path)
    if contest is None:
        log_with_flush(f"无法从文件名 {filename} 中提取信息")
        return None
    
    # 准备翻译命令
    cmd = [
        "cargo", "run", "--", "translate",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language,
        "--method", method
    ]
    
    # 如果指定了输出目录，添加到命令中
    if output_dir:
        cmd.extend(["--output-dir", output_dir])
    
    # 执行翻译命令
    result = run_process_with_timeout(
        cmd=cmd,
        cwd=TRANSLATE_TO_RUST_PATH,
        timeout_seconds=timeout_seconds
    )
    
    # 检查是否超时
    timed_out = result.get('timed_out', False)
    if timed_out:
        log_with_flush(f"翻译超时：{filename}")
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
    
    # 分析翻译结果
    output = result['stdout']
    
    # 提取翻译结果数据
    translation_result = {
        'filename': filename,
        'contest': contest,
        'problem': problem,
        'language': language,
        'success': result['returncode'] == 0,
        'elapsed_time': result['elapsed_time'],
        'stdout': output,
        'stderr': result['stderr']
    }
    
    # 尝试从输出中提取Rust文件路径
    rust_file_match = re.search(r"Rust solution saved to: (.+\.rs)", output)
    if rust_file_match:
        translation_result['rust_file'] = rust_file_match.group(1)
    
    log_with_flush(f"翻译完成: {filename}")
    return translation_result

def main():
    parser = argparse.ArgumentParser(description="批量翻译C/C++文件到Rust")
    parser.add_argument("--max-workers", type=int, default=MAX_WORKERS, 
                        help=f"最大并行处理线程数，默认为{MAX_WORKERS}")
    parser.add_argument("--timeout", type=int, default=DEFAULT_PROCESS_TIMEOUT, 
                        help=f"每个翻译的超时时间（秒），默认为{DEFAULT_PROCESS_TIMEOUT}秒")
    parser.add_argument("--output-dir", type=str, default=OUTPUT_DIR, 
                        help="Rust文件输出目录")
    parser.add_argument("--method", type=str, default="llm", 
                        help="翻译方法，默认为llm")
    parser.add_argument("--file", type=str, 
                        help="指定单个源文件进行翻译，而不是搜索目录")
    parser.add_argument("--language", type=str, choices=["C", "CPP"], 
                        help="指定要翻译的语言（C或CPP），不指定则两者都翻译")
    parser.add_argument("--contest", type=int, 
                        help="指定要翻译的比赛编号")
    parser.add_argument("--problem", type=int, 
                        help="指定要翻译的题目编号")
    
    args = parser.parse_args()
    
    # 确保输出目录存在
    os.makedirs(args.output_dir, exist_ok=True)
    
    # 查找要翻译的文件
    if args.file:
        # 翻译单个文件
        source_files = [args.file]
        log_with_flush(f"将翻译单个文件: {args.file}")
    else:
        # 查找并翻译多个文件
        source_files = find_source_files(args.language, args.contest, args.problem)
        log_with_flush(f"找到 {len(source_files)} 个源文件待翻译")
    
    if not source_files:
        log_with_flush("没有找到可翻译的源文件")
        return
    
    # 并行执行翻译
    successful_translations = 0
    total_files = len(source_files)
    with ThreadPoolExecutor(max_workers=args.max_workers) as executor:
        future_to_file = {
            executor.submit(translate_file, file, args.method, args.timeout, args.output_dir): file
            for file in source_files
        }
        
        for future in future_to_file:
            file = future_to_file[future]
            try:
                result = future.result()
                if result:
                    filename = os.path.basename(file)
                    log_with_flush(f"完成文件翻译: {filename}")
                    if result.get('success', False):
                        successful_translations += 1
            except Exception as e:
                print(f"翻译文件 {os.path.basename(file)} 时出错: {e}")
    
    # 简单总结
    log_with_flush(f"批量翻译完成。共翻译了 {total_files} 个文件，其中 {successful_translations} 个翻译成功。")
    log_with_flush(f"Rust文件已保存在 {args.output_dir} 目录中。")

if __name__ == "__main__":
    main() 