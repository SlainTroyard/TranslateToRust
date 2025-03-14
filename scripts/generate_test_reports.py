#!/usr/bin/env python3
"""
Test Report Generation Script for TranslateToRust

This script generates comprehensive reports based on test results.
It scans individual test result files in the test_results directory,
extracts information, and correlates with problem information from FuzzForLeetcode.
"""

import os
import sys
import json
import re
import argparse
import glob
from datetime import datetime
from pathlib import Path
from collections import defaultdict, Counter

# 定义常量
TRANSLATE_TO_RUST_PATH = str(Path(os.path.dirname(os.path.abspath(__file__))).parent)
TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_results")
REPORTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "test_reports")

# 辅助函数：打印消息并刷新缓冲区
def log_with_flush(message):
    """打印消息并立即刷新输出缓冲区，防止长时间运行时输出卡住"""
    print(message)
    sys.stdout.flush()

def extract_test_info_from_file(result_file):
    """从测试结果文件中提取测试信息"""
    try:
        with open(result_file, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # 提取文件名
        filename_match = re.search(r"测试文件: (.+?)[\r\n]", content)
        if not filename_match:
            return None
        
        filename = filename_match.group(1).strip()
        
        # 从文件名提取比赛、题目和语言信息
        file_info_match = re.match(r"weekly_contest_(\d+)_p(\d+)_([a-z]+)\.rs", filename)
        if not file_info_match:
            return None
            
        contest = int(file_info_match.group(1))
        problem = int(file_info_match.group(2))
        language = file_info_match.group(3).upper()
        
        # 提取编译状态
        compilation_match = re.search(r"编译状态: (\w+)", content)
        compilation_success = compilation_match and compilation_match.group(1) == "成功"
        
        # 尝试提取其他测试信息
        result = {
            'filename': filename,
            'contest': contest,
            'problem': problem,
            'language': language,
            'results_file': result_file,
            'compilation_success': compilation_success
        }
        
        # 如果编译成功，提取测试结果
        if compilation_success:
            total_match = re.search(r"总测试用例: (\d+)", content)
            if not total_match:
                total_match = re.search(r"Total Test Cases: (\d+)", content)
                
            passed_match = re.search(r"通过: (\d+)", content)
            if not passed_match:
                passed_match = re.search(r"Passed: (\d+)", content)
                
            failed_match = re.search(r"失败: (\d+)", content)
            if not failed_match:
                failed_match = re.search(r"Failed: (\d+)", content)
                
            # 提取超时测试用例数量
            timeout_match = re.search(r"超时: (\d+)", content)
            
            success_rate_match = re.search(r"成功率: ([\d\.]+)%", content)
            if not success_rate_match:
                success_rate_match = re.search(r"Success Rate: ([\d\.]+)%", content)
                
            runtime_match = re.search(r"平均运行时间: ([\d\.]+) ms", content)
            
            result['total_cases'] = int(total_match.group(1)) if total_match else 0
            result['passed_cases'] = int(passed_match.group(1)) if passed_match else 0
            result['failed_cases'] = int(failed_match.group(1)) if failed_match else 0
            result['timeout_cases'] = int(timeout_match.group(1)) if timeout_match else 0
            result['success_rate'] = float(success_rate_match.group(1)) if success_rate_match else 0.0
            result['average_runtime'] = float(runtime_match.group(1)) if runtime_match else 0.0
            
            # 确定测试是否成功 - 如果通过率为100%则认为成功
            result['success'] = result.get('passed_cases', 0) == result.get('total_cases', 0) and result.get('total_cases', 0) > 0
        else:
            # 如果编译失败，设置默认值
            compilation_error_match = re.search(r"编译错误: (.+?)[\r\n]", content)
            result['compilation_error'] = compilation_error_match.group(1) if compilation_error_match else "未知编译错误"
            result['success'] = False
        
        # 检查是否有超时标记
        result['timed_out'] = result.get('timeout_cases', 0) > 0 or bool(re.search(r"(超时|timed out|TIMEOUT)", content))
        
        return result
    except Exception as e:
        log_with_flush(f"处理文件时出错 {result_file}: {e}")
        return None

def find_test_result_files(directory=None, days=7):
    """查找测试结果文件"""
    if directory is None:
        directory = TEST_RESULTS_DIR
    
    # 查找所有测试结果文件（过去days天内创建的）
    result_files = []
    
    # 文件名模式: test_results_YYYYMMDD_HHMMSS.txt
    pattern = os.path.join(directory, "test_results_*.txt")
    
    # 获取当前时间戳，用于过滤文件
    now = datetime.now().timestamp()
    days_in_seconds = days * 24 * 60 * 60
    
    for file_path in glob.glob(pattern):
        # 检查文件修改时间是否在指定天数内
        file_time = os.path.getmtime(file_path)
        if now - file_time <= days_in_seconds:
            result_files.append(file_path)
    
    # 查找details目录
    details_pattern = os.path.join(directory, "details_*")
    for details_dir in glob.glob(details_pattern):
        # 检查目录修改时间是否在指定天数内
        dir_time = os.path.getmtime(details_dir)
        if now - dir_time <= days_in_seconds:
            # 将该目录下的all_cases_summary.txt文件加入列表
            summary_file = os.path.join(details_dir, "all_cases_summary.txt")
            if os.path.exists(summary_file):
                result_files.append(summary_file)
    
    return result_files

def extract_problem_info(results):
    """从测试结果中提取题目信息，并关联FuzzForLeetcode的信息"""
    problem_info = {}
    
    # 尝试从FuzzForLeetcode中获取更多题目信息
    fuzz_path = os.path.join(TRANSLATE_TO_RUST_PATH, "..", "FuzzForLeetcode")
    
    for result in results:
        contest = result.get('contest')
        problem = result.get('problem')
        language = result.get('language')
        
        if contest and problem and language:
            key = f"{contest}_{problem}_{language}"
            if key not in problem_info:
                problem_info[key] = {
                    'contest': contest,
                    'problem': problem,
                    'language': language,
                    'difficulty': 'Unknown',
                    'title': f"Weekly Contest {contest} Problem {problem}",
                    'tags': []
                }
                
                # 尝试读取问题的更多信息
                constraint_path = os.path.join(
                    fuzz_path, "C_CPP", language, "constraints", 
                    f"weekly_contest_{contest}_p{problem}.json"
                )
                
                try:
                    if os.path.exists(constraint_path):
                        with open(constraint_path, 'r', encoding='utf-8') as f:
                            data = json.load(f)
                            problem_info[key]['difficulty'] = data.get('problem_info', {}).get('difficulty', 'Unknown')
                            problem_info[key]['title'] = data.get('problem_info', {}).get('title', problem_info[key]['title'])
                            problem_info[key]['tags'] = data.get('problem_info', {}).get('tags', [])
                except Exception as e:
                    log_with_flush(f"读取题目信息时出错 ({constraint_path}): {e}")
    
    return problem_info

def generate_markdown_report(results, problem_info, output_path=None):
    """生成Markdown格式的测试报告"""
    if output_path is None:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        os.makedirs(REPORTS_DIR, exist_ok=True)
        output_path = os.path.join(REPORTS_DIR, f"test_report_{timestamp}.md")
    
    # 计算统计信息
    total_files = len(results)
    successful_tests = sum(1 for r in results if r.get('success', False))
    compilation_successful = sum(1 for r in results if r.get('compilation_success', False))
    
    # 超时统计 - 使用timeout_cases字段
    total_timeout_tests = sum(1 for r in results if r.get('timed_out', False))
    total_timeout_cases = sum(r.get('timeout_cases', 0) for r in results)
    
    # 语言统计
    languages = Counter(r.get('language', 'Unknown') for r in results)
    
    # 按难度统计
    difficulty_stats = defaultdict(lambda: {'total': 0, 'success': 0, 'compilation': 0, 'timeout': 0})
    for result in results:
        contest = result.get('contest')
        problem = result.get('problem')
        language = result.get('language')
        
        if contest and problem and language:
            key = f"{contest}_{problem}_{language}"
            difficulty = problem_info.get(key, {}).get('difficulty', 'Unknown')
            
            difficulty_stats[difficulty]['total'] += 1
            if result.get('success', False):
                difficulty_stats[difficulty]['success'] += 1
            if result.get('compilation_success', False):
                difficulty_stats[difficulty]['compilation'] += 1
            if result.get('timed_out', False):
                difficulty_stats[difficulty]['timeout'] += 1
    
    # 生成报告
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("# Rust 测试报告\n\n")
        f.write(f"生成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        
        # 总体统计
        f.write("## 总体统计\n\n")
        f.write(f"- 总测试文件数: {total_files}\n")
        f.write(f"- 测试成功数: {successful_tests} ({successful_tests/total_files*100:.2f}%)\n")
        f.write(f"- 编译成功数: {compilation_successful} ({compilation_successful/total_files*100:.2f}%)\n")
        f.write(f"- 超时测试文件数: {total_timeout_tests} ({total_timeout_tests/total_files*100:.2f}%)\n")
        if total_timeout_cases > 0:
            f.write(f"- 超时测试用例数: {total_timeout_cases}\n")
        f.write("\n")
        
        # 按语言统计
        f.write("## 按语言统计\n\n")
        f.write("| 语言 | 测试文件数 | 比例 |\n")
        f.write("|------|------------|------|\n")
        for language, count in languages.items():
            f.write(f"| {language} | {count} | {count/total_files*100:.2f}% |\n")
        f.write("\n")
        
        # 按难度统计
        f.write("## 按难度统计\n\n")
        f.write("| 难度 | 测试文件数 | 测试成功 | 编译成功 | 超时 |\n")
        f.write("|------|------------|----------|----------|------|\n")
        for difficulty, stats in difficulty_stats.items():
            total = stats['total']
            success = stats['success']
            compilation = stats['compilation']
            timeout = stats['timeout']
            f.write(f"| {difficulty} | {total} | {success} ({success/total*100:.2f}%) | {compilation} ({compilation/total*100:.2f}%) | {timeout} ({timeout/total*100:.2f}%) |\n")
        f.write("\n")
        
        # 测试详情
        f.write("## 测试详情\n\n")
        f.write("| 文件名 | 比赛 | 题目 | 难度 | 编译 | 测试用例 | 通过率 | 运行时间 | 状态 |\n")
        f.write("|--------|------|------|------|------|----------|--------|----------|------|\n")
        
        for result in results:
            filename = result.get('filename', 'Unknown')
            contest = result.get('contest', 'Unknown')
            problem = result.get('problem', 'Unknown')
            
            key = f"{contest}_{problem}_{result.get('language', 'Unknown')}"
            difficulty = problem_info.get(key, {}).get('difficulty', 'Unknown')
            
            compilation = "✅" if result.get('compilation_success', False) else "❌"
            
            total_cases = result.get('total_cases', 0)
            passed_cases = result.get('passed_cases', 0)
            success_rate = result.get('success_rate', 0)
            
            if total_cases > 0:
                test_cases_str = f"{passed_cases}/{total_cases}"
            else:
                test_cases_str = "N/A"
            
            runtime = result.get('average_runtime', 0)
            
            timed_out = result.get('timed_out', False)
            timeout_cases = result.get('timeout_cases', 0)
            
            status = "成功" if result.get('success', False) else "失败"
            if timed_out:
                if timeout_cases > 0:
                    status = f"超时 ({timeout_cases})"
                else:
                    status = "超时"
                compilation = "⏱️"
            elif not result.get('compilation_success', False):
                status = "编译失败"
            
            f.write(f"| {filename} | {contest} | {problem} | {difficulty} | {compilation} | {test_cases_str} | {success_rate:.2f}% | {runtime:.2f} ms | {status} |\n")
        
        f.write("\n")
        
        # 失败的测试
        failed_tests = [r for r in results if not r.get('success', False)]
        if failed_tests:
            f.write("## 失败的测试\n\n")
            for i, result in enumerate(failed_tests):
                filename = result.get('filename', 'Unknown')
                f.write(f"### {i+1}. {filename}\n\n")
                
                contest = result.get('contest', 'Unknown')
                problem = result.get('problem', 'Unknown')
                language = result.get('language', 'Unknown')
                
                key = f"{contest}_{problem}_{language}"
                title = problem_info.get(key, {}).get('title', f"Weekly Contest {contest} Problem {problem}")
                
                f.write(f"- 题目: {title}\n")
                f.write(f"- 比赛: Weekly Contest {contest}\n")
                f.write(f"- 题号: Problem {problem}\n")
                f.write(f"- 原语言: {language}\n")
                
                timed_out = result.get('timed_out', False)
                timeout_cases = result.get('timeout_cases', 0)
                if timed_out:
                    if timeout_cases > 0:
                        f.write(f"- 状态: **测试超时** ({timeout_cases} 个用例超时)\n")
                    else:
                        f.write(f"- 状态: **测试超时**\n")
                else:
                    compilation_success = result.get('compilation_success', False)
                    if not compilation_success:
                        f.write(f"- 状态: **编译失败**\n")
                
                # 包含失败的详细信息链接
                results_file = result.get('results_file')
                if results_file and os.path.exists(results_file):
                    rel_path = os.path.relpath(results_file, TRANSLATE_TO_RUST_PATH)
                    f.write(f"- 详细结果: [{rel_path}]({rel_path})\n")
                
                f.write("\n")

    log_with_flush(f"Markdown报告已生成: {output_path}")
    return output_path

def generate_json_report(results, problem_info, output_path=None):
    """生成JSON格式的测试报告，包含更多详细信息"""
    if output_path is None:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        os.makedirs(REPORTS_DIR, exist_ok=True)
        output_path = os.path.join(REPORTS_DIR, f"test_report_{timestamp}.json")
    
    # 准备报告数据
    report_data = {
        'generated_at': datetime.now().isoformat(),
        'summary': {
            'total_files': len(results),
            'successful_tests': sum(1 for r in results if r.get('success', False)),
            'compilation_successful': sum(1 for r in results if r.get('compilation_success', False)),
            'timed_out_tests': sum(1 for r in results if r.get('timed_out', False)),
        },
        'language_stats': {},
        'difficulty_stats': {},
        'results': results,
        'problem_info': problem_info
    }
    
    # 语言统计
    for language, count in Counter(r.get('language', 'Unknown') for r in results).items():
        report_data['language_stats'][language] = {
            'count': count,
            'percentage': count / len(results) * 100
        }
    
    # 按难度统计
    difficulty_stats = defaultdict(lambda: {'total': 0, 'success': 0, 'compilation': 0})
    for result in results:
        contest = result.get('contest')
        problem = result.get('problem')
        language = result.get('language')
        
        if contest and problem and language:
            key = f"{contest}_{problem}_{language}"
            difficulty = problem_info.get(key, {}).get('difficulty', 'Unknown')
            
            difficulty_stats[difficulty]['total'] += 1
            if result.get('success', False):
                difficulty_stats[difficulty]['success'] += 1
            if result.get('compilation_success', False):
                difficulty_stats[difficulty]['compilation'] += 1
    
    for difficulty, stats in difficulty_stats.items():
        total = stats['total']
        report_data['difficulty_stats'][difficulty] = {
            'total': total,
            'success': stats['success'],
            'success_rate': stats['success'] / total * 100 if total > 0 else 0,
            'compilation': stats['compilation'],
            'compilation_rate': stats['compilation'] / total * 100 if total > 0 else 0
        }
    
    # 写入JSON文件
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(report_data, f, ensure_ascii=False, indent=2)
    
    log_with_flush(f"JSON报告已生成: {output_path}")
    return output_path

def main():
    parser = argparse.ArgumentParser(description="生成测试报告")
    parser.add_argument("--output-dir", type=str, default=REPORTS_DIR, help="报告输出目录")
    parser.add_argument("--format", type=str, choices=["markdown", "json", "both"], default="both", 
                       help="报告格式，可选markdown, json或both（默认）")
    parser.add_argument("--days", type=int, default=7, help="处理最近几天的测试结果，默认7天")
    
    args = parser.parse_args()
    
    # 确保输出目录存在
    os.makedirs(args.output_dir, exist_ok=True)
    
    # 查找测试结果文件
    result_files = find_test_result_files(days=args.days)
    
    if not result_files:
        log_with_flush("找不到测试结果文件")
        return
    
    log_with_flush(f"找到 {len(result_files)} 个测试结果文件")
    
    # 处理测试结果文件
    results = []
    for file_path in result_files:
        log_with_flush(f"处理测试结果文件: {file_path}")
        result = extract_test_info_from_file(file_path)
        if result:
            results.append(result)
    
    if not results:
        log_with_flush("没有找到有效的测试结果")
        return
    
    log_with_flush(f"成功处理了 {len(results)} 个测试结果")
    
    # 提取题目信息
    problem_info = extract_problem_info(results)
    
    # 生成报告
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    
    if args.format in ["markdown", "both"]:
        md_path = os.path.join(args.output_dir, f"test_report_{timestamp}.md")
        generate_markdown_report(results, problem_info, md_path)
    
    if args.format in ["json", "both"]:
        json_path = os.path.join(args.output_dir, f"test_report_{timestamp}.json")
        generate_json_report(results, problem_info, json_path)
    
    log_with_flush("报告生成完成")

if __name__ == "__main__":
    main() 