#!/usr/bin/env python3
import os
import subprocess
import json
import re
import sys
import csv
import signal
import time
from datetime import datetime
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import argparse
import timeout_handler

# 定义常量
FUZZ_FOR_LEETCODE_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent.parent / "FuzzForLeetcode")
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
MAX_WORKERS = 4  # 并行处理的最大线程数
DEFAULT_PROCESS_TIMEOUT = 10 * 60  # 默认进程超时时间（秒）：10分钟

# 辅助函数：打印消息并刷新缓冲区
def log_with_flush(message):
    """打印消息并立即刷新输出缓冲区，防止长时间运行时输出卡住"""
    print(message)
    sys.stdout.flush()

# 存储结果的数据结构
results = []

def get_difficulty(contest, problem, language):
    """获取题目难度"""
    constraint_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "constraints", 
                                  f"weekly_contest_{contest}_p{problem}.json")
    try:
        with open(constraint_path, 'r') as f:
            data = json.load(f)
            return data.get("problem_info", {}).get("difficulty", "Unknown")
    except Exception as e:
        log_with_flush(f"Error getting difficulty for {contest} p{problem} ({language}): {e}")
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
        log_with_flush(f"Error getting tags for {contest} p{problem} ({language}): {e}")
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
        log_with_flush(f"Error getting title for {contest} p{problem} ({language}): {e}")
        return "Unknown"

def run_process_with_timeout(cmd, cwd, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """运行子进程，带有超时控制"""
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
    
    # 启动超时监控
    monitor_thread = None
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
                    print(f"ERROR: {stderr_line}", end='')
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

def process_solution(contest, problem, language, timeout_seconds=DEFAULT_PROCESS_TIMEOUT):
    """处理单个解决方案"""
    log_with_flush(f"Processing {language} solution for Weekly Contest {contest} Problem {problem}...")
    
    # 检查源文件是否存在
    source_file = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "src", 
                              f"weekly_contest_{contest}_p{problem}.{'cpp' if language == 'CPP' else 'c'}")
    if not os.path.exists(source_file):
        log_with_flush(f"Source file not found: {source_file}")
        return None
    
    # 获取题目信息
    difficulty = get_difficulty(contest, problem, language)
    tags = get_tags(contest, problem, language)
    title = get_title(contest, problem, language)
    
    # 运行翻译命令
    log_with_flush(f"开始执行翻译命令，这可能需要一些时间...")
    cmd = [
        "cargo", "run", "--", "run",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language
    ]
    
    try:
        # 使用带超时控制的进程执行
        process_result = run_process_with_timeout(
            cmd=cmd,
            cwd=TRANSLATE_TO_RUST_PATH,
            timeout_seconds=timeout_seconds
        )
        
        output = process_result['stdout']
        error = process_result['stderr']
        elapsed_time = process_result['elapsed_time']
        
        # 检查是否超时
        timed_out = process_result.get('timed_out', False)
        
        # 检查是否成功
        process_success = process_result['returncode'] == 0
        
        if timed_out:
            log_with_flush(f"翻译命令执行超时（超过{timeout_seconds}秒）")
            
            # 创建一个失败的结果
            result = {
                "contest": contest,
                "problem": problem,
                "title": title,
                "language": language,
                "difficulty": difficulty,
                "tags": tags,
                "process_success": False,
                "translation_success": False,
                "compilation_success": False,
                "test_cases_total": 0,
                "test_cases_passed": 0,
                "test_cases_failed": 0,
                "success_rate": 0.0,
                "average_runtime": 0.0,
                "elapsed_time": elapsed_time,
                "timed_out": True
            }
            
            return result
        
        # 提取编译状态
        compilation_success = "编译状态: 成功" in output
        
        # 提取测试结果
        test_cases_total = 0
        test_cases_passed = 0
        test_cases_failed = 0
        success_rate = 0
        average_runtime = 0
        
        if compilation_success:
            # 提取测试统计信息
            total_match = re.search(r"Total Test Cases: (\d+)", output)
            passed_match = re.search(r"Passed: (\d+)", output)
            failed_match = re.search(r"Failed: (\d+)", output)
            rate_match = re.search(r"Success Rate: ([\d\.]+)%", output)
            runtime_match = re.search(r"平均运行时间: ([\d\.]+) ms", output)
            
            if total_match:
                test_cases_total = int(total_match.group(1))
            if passed_match:
                test_cases_passed = int(passed_match.group(1))
            if failed_match:
                test_cases_failed = int(failed_match.group(1))
            if rate_match:
                success_rate = float(rate_match.group(1))
            if runtime_match:
                average_runtime = float(runtime_match.group(1))
        
        # 更新翻译成功的定义：编译成功且测试用例全部通过
        # 如果没有测试用例(test_cases_total = 0)，则只要求编译成功
        translation_success = compilation_success and (test_cases_failed == 0)
        if test_cases_total == 0 and compilation_success:
            translation_success = True  # 只有编译成功，没有测试用例时也算成功
        
        # 保存结果
        result = {
            "contest": contest,
            "problem": problem,
            "title": title,
            "language": language,
            "difficulty": difficulty,
            "tags": tags,
            "process_success": process_success,  # 保留原始的进程成功状态
            "translation_success": translation_success,  # 新的定义：编译成功且测试通过
            "compilation_success": compilation_success,
            "test_cases_total": test_cases_total,
            "test_cases_passed": test_cases_passed,
            "test_cases_failed": test_cases_failed,
            "success_rate": success_rate,
            "average_runtime": average_runtime,
            "elapsed_time": elapsed_time
        }
        
        log_with_flush(f"处理完成: Weekly Contest {contest} Problem {problem} ({language})")
        
        return result
    
    except Exception as e:
        log_with_flush(f"Error processing {language} solution for Weekly Contest {contest} Problem {problem}: {e}")
        return None

def find_all_solutions():
    """查找所有需要翻译的解决方案"""
    solutions = []
    
    # 查找C++解决方案
    cpp_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", "CPP", "src")
    for file in os.listdir(cpp_path):
        if file.endswith(".cpp") and file.startswith("weekly_contest_"):
            match = re.match(r"weekly_contest_(\d+)_p(\d+)\.cpp", file)
            if match:
                contest = int(match.group(1))
                problem = int(match.group(2))
                solutions.append((contest, problem, "CPP"))
    
    # 查找C解决方案
    c_path = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", "C", "src")
    for file in os.listdir(c_path):
        if file.endswith(".c") and file.startswith("weekly_contest_"):
            match = re.match(r"weekly_contest_(\d+)_p(\d+)\.c", file)
            if match:
                contest = int(match.group(1))
                problem = int(match.group(2))
                solutions.append((contest, problem, "C"))
    
    return solutions

def generate_report(results, output_dir):
    """生成统计报告"""
    os.makedirs(output_dir, exist_ok=True)
    
    # 按多个维度统计结果
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    report_path = os.path.join(output_dir, f"translation_report_{timestamp}")
    
    # 保存原始数据
    csv_path = f"{report_path}.csv"
    with open(csv_path, 'w', newline='') as f:
        fieldnames = [
            "contest", "problem", "title", "language", "difficulty", "tags",
            "process_success", "translation_success", "compilation_success", 
            "test_cases_total", "test_cases_passed", "test_cases_failed",
            "success_rate", "average_runtime", "elapsed_time"
        ]
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        for result in results:
            # 处理tags列表，将其转换为字符串
            result_copy = result.copy()
            result_copy["tags"] = ", ".join(result_copy["tags"])
            writer.writerow(result_copy)
    
    # 生成统计报告
    with open(f"{report_path}.md", 'w') as f:
        f.write("# C/C++ to Rust Translation Report\n\n")
        f.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write("**注意:** 在本报告中，「翻译成功」定义为编译成功且通过所有测试用例\n\n")
        
        # 总体统计
        total_count = len(results)
        c_count = sum(1 for r in results if r["language"] == "C")
        cpp_count = sum(1 for r in results if r["language"] == "CPP")
        
        translation_success_count = sum(1 for r in results if r["translation_success"])
        compilation_success_count = sum(1 for r in results if r["compilation_success"])
        
        f.write("## 总体统计\n\n")
        f.write(f"- 总解决方案数量: {total_count}\n")
        f.write(f"- C语言解决方案: {c_count}\n")
        f.write(f"- C++解决方案: {cpp_count}\n")
        f.write(f"- 翻译成功率: {translation_success_count/total_count*100:.2f}% ({translation_success_count}/{total_count})\n")
        f.write(f"- 编译成功率: {compilation_success_count/total_count*100:.2f}% ({compilation_success_count}/{total_count})\n\n")
        
        # 按难度统计
        f.write("## 按难度统计\n\n")
        difficulties = ["Easy", "Medium", "Hard", "Unknown"]
        f.write("| 难度 | 总数 | 翻译成功 | 编译成功 | 测试用例通过率 | 平均运行时间(ms) |\n")
        f.write("|------|------|----------|----------|----------------|------------------|\n")
        
        for difficulty in difficulties:
            difficulty_results = [r for r in results if r["difficulty"] == difficulty]
            if not difficulty_results:
                continue
                
            count = len(difficulty_results)
            translation_success = sum(1 for r in difficulty_results if r["translation_success"])
            compilation_success = sum(1 for r in difficulty_results if r["compilation_success"])
            
            # 只考虑编译成功的案例计算平均通过率和运行时间
            compiled_results = [r for r in difficulty_results if r["compilation_success"]]
            if compiled_results:
                avg_success_rate = sum(r["success_rate"] for r in compiled_results) / len(compiled_results)
                avg_runtime = sum(r["average_runtime"] for r in compiled_results) / len(compiled_results)
            else:
                avg_success_rate = 0
                avg_runtime = 0
                
            f.write(f"| {difficulty} | {count} | {translation_success} ({translation_success/count*100:.2f}%) | "
                   f"{compilation_success} ({compilation_success/count*100:.2f}%) | {avg_success_rate:.2f}% | {avg_runtime:.2f} |\n")
        
        f.write("\n")
        
        # 按语言统计
        f.write("## 按语言统计\n\n")
        f.write("| 语言 | 总数 | 翻译成功 | 编译成功 | 测试用例通过率 | 平均运行时间(ms) |\n")
        f.write("|------|------|----------|----------|----------------|------------------|\n")
        
        for language in ["C", "CPP"]:
            language_results = [r for r in results if r["language"] == language]
            if not language_results:
                continue
                
            count = len(language_results)
            translation_success = sum(1 for r in language_results if r["translation_success"])
            compilation_success = sum(1 for r in language_results if r["compilation_success"])
            
            # 只考虑编译成功的案例计算平均通过率和运行时间
            compiled_results = [r for r in language_results if r["compilation_success"]]
            if compiled_results:
                avg_success_rate = sum(r["success_rate"] for r in compiled_results) / len(compiled_results)
                avg_runtime = sum(r["average_runtime"] for r in compiled_results) / len(compiled_results)
            else:
                avg_success_rate = 0
                avg_runtime = 0
                
            f.write(f"| {language} | {count} | {translation_success} ({translation_success/count*100:.2f}%) | "
                   f"{compilation_success} ({compilation_success/count*100:.2f}%) | {avg_success_rate:.2f}% | {avg_runtime:.2f} |\n")
        
        f.write("\n")
        
        # 按标签统计
        f.write("## 按标签统计\n\n")
        
        # 收集所有标签
        all_tags = set()
        for result in results:
            all_tags.update(result["tags"])
        
        f.write("| 标签 | 总数 | 翻译成功 | 编译成功 | 测试用例通过率 | 平均运行时间(ms) |\n")
        f.write("|------|------|----------|----------|----------------|------------------|\n")
        
        for tag in sorted(all_tags):
            # 包含该标签的结果
            tag_results = [r for r in results if tag in r["tags"]]
            if not tag_results:
                continue
                
            count = len(tag_results)
            translation_success = sum(1 for r in tag_results if r["translation_success"])
            compilation_success = sum(1 for r in tag_results if r["compilation_success"])
            
            # 只考虑编译成功的案例计算平均通过率和运行时间
            compiled_results = [r for r in tag_results if r["compilation_success"]]
            if compiled_results:
                avg_success_rate = sum(r["success_rate"] for r in compiled_results) / len(compiled_results)
                avg_runtime = sum(r["average_runtime"] for r in compiled_results) / len(compiled_results)
            else:
                avg_success_rate = 0
                avg_runtime = 0
                
            f.write(f"| {tag} | {count} | {translation_success} ({translation_success/count*100:.2f}%) | "
                   f"{compilation_success} ({compilation_success/count*100:.2f}%) | {avg_success_rate:.2f}% | {avg_runtime:.2f} |\n")
        
        f.write("\n")
        
        # 按比赛统计
        f.write("## 按比赛统计\n\n")
        
        # 收集所有比赛
        all_contests = sorted(set(r["contest"] for r in results))
        
        f.write("| 比赛 | 总数 | 翻译成功 | 编译成功 | 测试用例通过率 | 平均运行时间(ms) |\n")
        f.write("|------|------|----------|----------|----------------|------------------|\n")
        
        for contest in all_contests:
            # 该比赛的结果
            contest_results = [r for r in results if r["contest"] == contest]
            if not contest_results:
                continue
                
            count = len(contest_results)
            translation_success = sum(1 for r in contest_results if r["translation_success"])
            compilation_success = sum(1 for r in contest_results if r["compilation_success"])
            
            # 只考虑编译成功的案例计算平均通过率和运行时间
            compiled_results = [r for r in contest_results if r["compilation_success"]]
            if compiled_results:
                avg_success_rate = sum(r["success_rate"] for r in compiled_results) / len(compiled_results)
                avg_runtime = sum(r["average_runtime"] for r in compiled_results) / len(compiled_results)
            else:
                avg_success_rate = 0
                avg_runtime = 0
                
            f.write(f"| Weekly Contest {contest} | {count} | {translation_success} ({translation_success/count*100:.2f}%) | "
                   f"{compilation_success} ({compilation_success/count*100:.2f}%) | {avg_success_rate:.2f}% | {avg_runtime:.2f} |\n")
        
        print(f"Report generated: {report_path}.md")
        print(f"Raw data saved to: {csv_path}")
        
        return f"{report_path}.md"

def main():
    parser = argparse.ArgumentParser(description="批量翻译FuzzForLeetcode中的C/C++代码到Rust")
    parser.add_argument("--max-workers", type=int, default=MAX_WORKERS, help=f"最大并行处理线程数，默认为{MAX_WORKERS}")
    parser.add_argument("--output", type=str, default="./translation_reports", help="报告输出目录")
    parser.add_argument("--contest", type=int, help="指定比赛编号")
    parser.add_argument("--problem", type=int, help="指定题目编号")
    parser.add_argument("--language", type=str, choices=["C", "CPP"], help="指定语言")
    
    args = parser.parse_args()
    
    log_with_flush("开始批量翻译FuzzForLeetcode中的C/C++代码到Rust...")
    
    # 查找所有解决方案
    if args.contest and args.problem and args.language:
        solutions = [(args.contest, args.problem, args.language)]
    else:
        solutions = find_all_solutions()
    
    log_with_flush(f"找到{len(solutions)}个解决方案需要翻译")
    
    # 使用线程池并行处理
    results = []
    with ThreadPoolExecutor(max_workers=args.max_workers) as executor:
        future_to_solution = {
            executor.submit(process_solution, contest, problem, language): (contest, problem, language)
            for contest, problem, language in solutions
        }
        
        for future in future_to_solution:
            contest, problem, language = future_to_solution[future]
            try:
                result = future.result()
                if result:
                    results.append(result)
                    log_with_flush(f"完成 {language} Weekly Contest {contest} Problem {problem}")
                    sys.stdout.flush()  # 强制刷新输出缓冲区
            except Exception as e:
                print(f"处理 {language} Weekly Contest {contest} Problem {problem} 失败: {e}")
                print(f"处理失败")
    
    # 生成报告
    if results:
        report_path = generate_report(results, args.output)
        print(f"翻译完成。报告已生成: {report_path}")
    else:
        print("没有成功翻译的结果")
    
if __name__ == "__main__":
    main() 