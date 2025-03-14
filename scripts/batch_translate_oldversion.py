#!/usr/bin/env python3
import os
import subprocess
import json
import re
import csv
from datetime import datetime
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import argparse

# 定义常量
FUZZ_FOR_LEETCODE_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent.parent / "FuzzForLeetcode")
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
MAX_WORKERS = 4  # 并行处理的最大线程数

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
        print(f"Error getting difficulty for {contest} p{problem} ({language}): {e}")
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
        print(f"Error getting tags for {contest} p{problem} ({language}): {e}")
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
        print(f"Error getting title for {contest} p{problem} ({language}): {e}")
        return "Unknown"

def process_solution(contest, problem, language):
    """处理单个解决方案"""
    print(f"Processing {language} solution for Weekly Contest {contest} Problem {problem}...")
    sys.stdout.flush()  # 强制刷新输出缓冲区
    
    # 检查源文件是否存在
    source_file = os.path.join(FUZZ_FOR_LEETCODE_PATH, "C_CPP", language, "src", 
                              f"weekly_contest_{contest}_p{problem}.{'cpp' if language == 'CPP' else 'c'}")
    if not os.path.exists(source_file):
        print(f"Source file not found: {source_file}")
        return None
    
    # 获取题目信息
    difficulty = get_difficulty(contest, problem, language)
    tags = get_tags(contest, problem, language)
    title = get_title(contest, problem, language)
    
    # 运行翻译命令
    cmd = [
        "cargo", "run", "--", "run",
        "--contest", str(contest),
        "--problem", str(problem),
        "--language", language
    ]
    
    try:
        start_time = datetime.now()
        process = subprocess.run(
            cmd, 
            cwd=TRANSLATE_TO_RUST_PATH,
            capture_output=True,
            text=True,
            check=False  # 不在失败时抛出异常
        )
        end_time = datetime.now()
        elapsed_time = (end_time - start_time).total_seconds()
        
        # 尝试从输出中提取测试结果
        output = process.stdout
        error = process.stderr
        
        # 检查是否成功
        process_success = process.returncode == 0
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
        
        return result
    
    except Exception as e:
        print(f"Error processing {language} solution for Weekly Contest {contest} Problem {problem}: {e}")
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
    
    print("开始批量翻译FuzzForLeetcode中的C/C++代码到Rust...")
    
    # 查找所有解决方案
    if args.contest and args.problem and args.language:
        solutions = [(args.contest, args.problem, args.language)]
    else:
        solutions = find_all_solutions()
    
    print(f"找到{len(solutions)}个解决方案需要翻译")
    
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
                    print(f"完成 {language} Weekly Contest {contest} Problem {problem}")
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