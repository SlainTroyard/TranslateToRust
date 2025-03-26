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
        # 使用上下文管理器确保文件正确关闭
        with open(result_file, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # 提取文件名
        filename_match = re.search(r"测试文件: (.+?)[\r\n]", content)
        if not filename_match:
            log_with_flush(f"警告: 无法从文件中提取测试文件名: {result_file}")
            
            # 尝试从文件路径中猜测文件名（通常用于LLM测试结果）
            file_basename = os.path.basename(result_file)
            if file_basename.startswith("test_results_"):
                # 检查文件内容中是否可能包含文件名
                lines = content.split('\n')
                for line in lines:
                    if "weekly_contest_" in line and ".rs" in line:
                        match = re.search(r"(weekly_contest_\d+_p\d+_[a-z]+\.rs)", line)
                        if match:
                            filename = match.group(1)
                            log_with_flush(f"从内容中推断文件名: {filename}")
                            break
                else:
                    log_with_flush(f"无法从文件内容推断文件名，跳过: {result_file}")
                    return None
            else:
                log_with_flush(f"无法推断文件名，跳过: {result_file}")
                return None
        else:
            filename = filename_match.group(1).strip()
        
        # 从文件名提取比赛、题目和语言信息
        file_info_match = re.match(r"weekly_contest_(\d+)_p(\d+)_([a-z]+)\.rs", filename)
        if not file_info_match:
            log_with_flush(f"警告: 文件名格式不正确: {filename}")
            return None
            
        contest = int(file_info_match.group(1))
        problem = int(file_info_match.group(2))
        language = file_info_match.group(3).upper()
        
        # 提取编译状态
        compilation_match = re.search(r"编译状态: (\w+)", content)
        if not compilation_match:
            # 尝试查找其他编译状态的指示符
            if "编译失败" in content or "compilation failed" in content.lower():
                compilation_success = False
            elif "编译成功" in content or "compilation succeeded" in content.lower():
                compilation_success = True
            else:
                # 默认假设编译成功
                compilation_success = True
                log_with_flush(f"警告: 无法确定编译状态，假设成功: {result_file}")
        else:
            compilation_success = compilation_match.group(1) == "成功"
        
        # 创建基本结果对象
        result = {
            'filename': filename,
            'contest': contest,
            'problem': problem,
            'language': language,
            'results_file': result_file,
            'compilation_success': compilation_success,
            'timed_out': False  # 默认非超时
        }
        
        # 如果编译成功，提取测试结果
        if compilation_success:
            # 检查是否包含"超时"标记
            timeout_match = re.search(r"超时: (\d+)", content)
            timeout_keyword = bool(re.search(r"(超时|timed out|TIMEOUT)", content))
            
            # 设置超时信息
            result['timed_out'] = bool(timeout_match) or timeout_keyword
            result['timeout_cases'] = int(timeout_match.group(1)) if timeout_match else 0
            
            # 提取测试用例信息
            total_match = re.search(r"总测试用例: (\d+)", content)
            if not total_match:
                total_match = re.search(r"Total Test Cases: (\d+)", content)
                
            passed_match = re.search(r"通过: (\d+)", content)
            if not passed_match:
                passed_match = re.search(r"Passed: (\d+)", content)
                
            failed_match = re.search(r"失败: (\d+)", content)
            if not failed_match:
                failed_match = re.search(r"Failed: (\d+)", content)
            
            success_rate_match = re.search(r"成功率: ([\d\.]+)%", content)
            if not success_rate_match:
                success_rate_match = re.search(r"Success Rate: ([\d\.]+)%", content)
                
            runtime_match = re.search(r"平均运行时间: ([\d\.]+) ms", content)
            
            result['total_cases'] = int(total_match.group(1)) if total_match else 0
            result['passed_cases'] = int(passed_match.group(1)) if passed_match else 0
            result['failed_cases'] = int(failed_match.group(1)) if failed_match else 0
            result['success_rate'] = float(success_rate_match.group(1)) if success_rate_match else 0.0
            result['average_runtime'] = float(runtime_match.group(1)) if runtime_match else 0.0
            
            # 超时文件特殊处理
            if "超时原因:" in content:
                log_with_flush(f"发现超时测试报告: {result_file}")
                # 确保超时标记设置正确
                result['timed_out'] = True
                
                # 如果这是一个简单的超时报告文件，设置默认值
                if result['total_cases'] == 0 and result['timeout_cases'] == 0:
                    result['timeout_cases'] = 1
            
            # 确定测试是否成功 - 如果通过率为100%则认为成功
            result['success'] = result.get('passed_cases', 0) == result.get('total_cases', 0) and result.get('total_cases', 0) > 0
        else:
            # 如果编译失败，设置默认值
            compilation_error_match = re.search(r"编译错误: (.+?)[\r\n]", content)
            result['compilation_error'] = compilation_error_match.group(1) if compilation_error_match else "未知编译错误"
            result['success'] = False
        
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
        try:
            # 检查文件修改时间是否在指定天数内
            file_time = os.path.getmtime(file_path)
            if now - file_time <= days_in_seconds:
                result_files.append(file_path)
        except Exception as e:
            log_with_flush(f"处理文件路径时出错 {file_path}: {e}")
    
    # 查找details目录
    details_pattern = os.path.join(directory, "details_*")
    for details_dir in glob.glob(details_pattern):
        try:
            # 检查目录修改时间是否在指定天数内
            dir_time = os.path.getmtime(details_dir)
            if now - dir_time <= days_in_seconds:
                # 将该目录下的all_cases_summary.txt文件加入列表
                summary_file = os.path.join(details_dir, "all_cases_summary.txt")
                if os.path.exists(summary_file):
                    result_files.append(summary_file)
        except Exception as e:
            log_with_flush(f"处理详情目录时出错 {details_dir}: {e}")
    
    # 如果这是在使用--llm模式并且没有找到结果，检查LLM测试结果目录是否有子目录结构
    if not result_files and "llm_test_results" in directory:
        log_with_flush(f"在{directory}中没有找到直接的测试结果，尝试使用子目录结构...")
        # 这里我们假设LLM测试结果按模型目录/测试结果文件组织
        
        for file_path in glob.glob(os.path.join(directory, "test_results_*.txt")):
            try:
                file_time = os.path.getmtime(file_path)
                if now - file_time <= days_in_seconds:
                    result_files.append(file_path)
                    log_with_flush(f"找到文件: {file_path}")
            except Exception as e:
                log_with_flush(f"处理文件路径时出错 {file_path}: {e}")
        
        for details_dir in glob.glob(os.path.join(directory, "details_*")):
            try:
                dir_time = os.path.getmtime(details_dir)
                if now - dir_time <= days_in_seconds:
                    summary_file = os.path.join(details_dir, "all_cases_summary.txt")
                    if os.path.exists(summary_file):
                        result_files.append(summary_file)
                        log_with_flush(f"找到摘要文件: {summary_file}")
            except Exception as e:
                log_with_flush(f"处理详情目录时出错 {details_dir}: {e}")
    
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
                            
                            # 从多个可能的位置获取标签
                            tags = []
                            if 'tags' in data:
                                tags.extend(data['tags'])
                            if 'problem_info' in data and 'tags' in data['problem_info']:
                                tags.extend(data['problem_info']['tags'])
                            if 'metadata' in data and 'tags' in data['metadata']:
                                tags.extend(data['metadata']['tags'])
                            
                            # 去重并过滤空值
                            tags = list(set(tag for tag in tags if tag and isinstance(tag, str)))
                            problem_info[key]['tags'] = tags
                            
                            log_with_flush(f"Found tags for {key}: {tags}")
                    else:
                        log_with_flush(f"Warning: Constraint file not found: {constraint_path}")
                except Exception as e:
                    log_with_flush(f"Error reading problem info ({constraint_path}): {e}")
    
    return problem_info

def generate_tag_statistics(results, problem_info):
    """生成标签统计信息"""
    tag_stats = defaultdict(lambda: {
        'total': 0,
        'success': 0,
        'compilation': 0,
        'timeout': 0,
        'avg_runtime': [],
        'success_rate': [],
        'problems': set()
    })
    
    for result in results:
        contest = result.get('contest')
        problem = result.get('problem')
        language = result.get('language')
        
        if contest and problem and language:
            key = f"{contest}_{problem}_{language}"
            tags = problem_info.get(key, {}).get('tags', [])
            
            for tag in tags:
                tag_stats[tag]['total'] += 1
                tag_stats[tag]['problems'].add(key)
                
                if result.get('success', False):
                    tag_stats[tag]['success'] += 1
                if result.get('compilation_success', False):
                    tag_stats[tag]['compilation'] += 1
                if result.get('timed_out', False):
                    tag_stats[tag]['timeout'] += 1
                    
                if result.get('average_runtime'):
                    tag_stats[tag]['avg_runtime'].append(result['average_runtime'])
                if result.get('success_rate') is not None:
                    tag_stats[tag]['success_rate'].append(result['success_rate'])
    
    # 计算平均值
    for tag_data in tag_stats.values():
        tag_data['avg_runtime'] = sum(tag_data['avg_runtime']) / len(tag_data['avg_runtime']) if tag_data['avg_runtime'] else 0
        tag_data['success_rate'] = sum(tag_data['success_rate']) / len(tag_data['success_rate']) if tag_data['success_rate'] else 0
        tag_data['unique_problems'] = len(tag_data['problems'])
        tag_data['problems'] = sorted(list(tag_data['problems']))  # 转换为排序列表
    
    return tag_stats

def generate_language_specific_statistics(results, problem_info):
    """生成按语言分类的统计信息"""
    lang_stats = defaultdict(lambda: {
        'total': 0,
        'success': 0,
        'compilation': 0,
        'timeout': 0,
        'avg_runtime': [],
        'success_rate': [],
        'difficulty_stats': defaultdict(lambda: {'total': 0, 'success': 0, 'compilation': 0, 'timeout': 0}),
        'tag_stats': defaultdict(lambda: {
            'total': 0,
            'success': 0,
            'compilation': 0,
            'timeout': 0,
            'avg_runtime': [],
            'success_rate': [],
            'problems': set()
        })
    })
    
    for result in results:
        contest = result.get('contest')
        problem = result.get('problem')
        language = result.get('language')
        
        if contest and problem and language:
            key = f"{contest}_{problem}_{language}"
            problem_data = problem_info.get(key, {})
            difficulty = problem_data.get('difficulty', 'Unknown')
            tags = problem_data.get('tags', [])
            
            # 更新语言总体统计
            lang_stats[language]['total'] += 1
            if result.get('success', False):
                lang_stats[language]['success'] += 1
            if result.get('compilation_success', False):
                lang_stats[language]['compilation'] += 1
            if result.get('timed_out', False):
                lang_stats[language]['timeout'] += 1
                
            if result.get('average_runtime'):
                lang_stats[language]['avg_runtime'].append(result['average_runtime'])
            if result.get('success_rate') is not None:
                lang_stats[language]['success_rate'].append(result['success_rate'])
            
            # 更新难度统计
            lang_stats[language]['difficulty_stats'][difficulty]['total'] += 1
            if result.get('success', False):
                lang_stats[language]['difficulty_stats'][difficulty]['success'] += 1
            if result.get('compilation_success', False):
                lang_stats[language]['difficulty_stats'][difficulty]['compilation'] += 1
            if result.get('timed_out', False):
                lang_stats[language]['difficulty_stats'][difficulty]['timeout'] += 1
            
            # 更新标签统计
            for tag in tags:
                tag_stats = lang_stats[language]['tag_stats'][tag]
                tag_stats['total'] += 1
                tag_stats['problems'].add(key)
                
                if result.get('success', False):
                    tag_stats['success'] += 1
                if result.get('compilation_success', False):
                    tag_stats['compilation'] += 1
                if result.get('timed_out', False):
                    tag_stats['timeout'] += 1
                    
                if result.get('average_runtime'):
                    tag_stats['avg_runtime'].append(result['average_runtime'])
                if result.get('success_rate') is not None:
                    tag_stats['success_rate'].append(result['success_rate'])
    
    # 计算平均值
    for lang_data in lang_stats.values():
        # 计算总体平均值
        lang_data['avg_runtime'] = sum(lang_data['avg_runtime']) / len(lang_data['avg_runtime']) if lang_data['avg_runtime'] else 0
        lang_data['success_rate'] = sum(lang_data['success_rate']) / len(lang_data['success_rate']) if lang_data['success_rate'] else 0
        
        # 计算标签统计的平均值
        for tag_data in lang_data['tag_stats'].values():
            tag_data['avg_runtime'] = sum(tag_data['avg_runtime']) / len(tag_data['avg_runtime']) if tag_data['avg_runtime'] else 0
            tag_data['success_rate'] = sum(tag_data['success_rate']) / len(tag_data['success_rate']) if tag_data['success_rate'] else 0
            tag_data['unique_problems'] = len(tag_data['problems'])
            tag_data['problems'] = sorted(list(tag_data['problems']))
    
    return lang_stats

def generate_markdown_report(results, problem_info, output_path):
    """生成Markdown格式的测试报告"""
    try:
        # 按照文件名排序结果
        results.sort(key=lambda x: x['filename'])
        
        # 统计结果
        total_files = len(results)
        success_files = sum(1 for result in results if result.get('success', False))
        compilation_success = sum(1 for result in results if result.get('compilation_success', False))
        timed_out_files = sum(1 for result in results if result.get('timed_out', False))
        timed_out_cases = sum(result.get('timeout_cases', 0) for result in results)
        
        # 语言统计
        language_stats = Counter(result['language'] for result in results)
        
        # 难度统计
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
        
        # 如果没有测试结果，返回错误
        if total_files == 0:
            log_with_flush("错误: 没有测试结果可用于生成报告")
            return None
        
        # 添加标签统计
        tag_stats = generate_tag_statistics(results, problem_info)
        
        # 添加语言特定统计
        lang_stats = generate_language_specific_statistics(results, problem_info)
        
        # 生成报告内容
        report = [
            "# TranslateToRust 测试报告",
            f"\n生成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            "\n## 总体统计",
            f"- 总文件数: {total_files}",
            f"- 测试成功: {success_files} ({success_files/total_files*100:.2f}%)",
            f"- 编译成功: {compilation_success} ({compilation_success/total_files*100:.2f}%)",
            f"- 超时文件: {timed_out_files} ({timed_out_files/total_files*100:.2f}%)",
            f"- 超时用例: {timed_out_cases}",
        ]
        
        # 添加总体难度统计
        report.append("\n### 总体难度统计")
        for difficulty, stats in difficulty_stats.items():
            total = stats['total']
            if total > 0:
                report.extend([
                    f"\n#### {difficulty}",
                    f"- 总数: {total}",
                    f"- 成功率: {stats['success']/total*100:.2f}%",
                    f"- 编译成功率: {stats['compilation']/total*100:.2f}%",
                    f"- 超时率: {stats['timeout']/total*100:.2f}%"
                ])
        
        # 添加总体标签统计
        report.append("\n### 总体标签统计")
        sorted_tags = sorted(tag_stats.items(), key=lambda x: x[1]['total'], reverse=True)
        for tag, stats in sorted_tags:
            total = stats['total']
            if total > 0:
                report.extend([
                    f"\n#### {tag}",
                    f"- 题目数量: {stats['unique_problems']}",
                    f"- 总测试数: {total}",
                    f"- 成功率: {stats['success']/total*100:.2f}%",
                    f"- 编译成功率: {stats['compilation']/total*100:.2f}%",
                    f"- 超时率: {stats['timeout']/total*100:.2f}%",
                    f"- 平均运行时间: {stats['avg_runtime']:.2f} ms",
                    f"- 平均测试通过率: {stats['success_rate']:.2f}%"
                ])
        
        # 添加语言分布
        report.append("\n### 语言分布")
        for lang, count in language_stats.items():
            report.append(f"- {lang}: {count} ({count/total_files*100:.2f}%)")
        
        # 添加按语言分类的统计
        report.append("\n## 按语言分类统计")
        for lang, stats in lang_stats.items():
            total = stats['total']
            if total > 0:
                report.extend([
                    f"\n### {lang}",
                    f"- 总数: {total}",
                    f"- 成功率: {stats['success']/total*100:.2f}%",
                    f"- 编译成功率: {stats['compilation']/total*100:.2f}%",
                    f"- 超时率: {stats['timeout']/total*100:.2f}%",
                    f"- 平均运行时间: {stats['avg_runtime']:.2f} ms",
                    f"- 平均测试通过率: {stats['success_rate']:.2f}%",
                    f"\n#### {lang} 难度统计"
                ])
                
                for difficulty, diff_stats in stats['difficulty_stats'].items():
                    diff_total = diff_stats['total']
                    if diff_total > 0:
                        report.extend([
                            f"\n##### {difficulty}",
                            f"- 总数: {diff_total}",
                            f"- 成功率: {diff_stats['success']/diff_total*100:.2f}%",
                            f"- 编译成功率: {diff_stats['compilation']/diff_total*100:.2f}%",
                            f"- 超时率: {diff_stats['timeout']/diff_total*100:.2f}%"
                        ])
                
                report.append(f"\n#### {lang} 标签统计")
                sorted_tags = sorted(stats['tag_stats'].items(), key=lambda x: x[1]['total'], reverse=True)
                for tag, tag_stats in sorted_tags:
                    tag_total = tag_stats['total']
                    if tag_total > 0:
                        report.extend([
                            f"\n##### {tag}",
                            f"- 题目数量: {tag_stats['unique_problems']}",
                            f"- 总测试数: {tag_total}",
                            f"- 成功率: {tag_stats['success']/tag_total*100:.2f}%",
                            f"- 编译成功率: {tag_stats['compilation']/tag_total*100:.2f}%",
                            f"- 超时率: {tag_stats['timeout']/tag_total*100:.2f}%",
                            f"- 平均运行时间: {tag_stats['avg_runtime']:.2f} ms",
                            f"- 平均测试通过率: {tag_stats['success_rate']:.2f}%"
                        ])
        
        report.append("\n## 详细测试结果")
        for result in results:
            filename = result['filename']
            contest = result.get('contest')
            problem = result.get('problem')
            language = result.get('language')
            
            if contest and problem and language:
                key = f"{contest}_{problem}_{language}"
                problem_data = problem_info.get(key, {})
                
                report.extend([
                    f"\n### {filename}",
                    f"- 标题: {problem_data.get('title', '未知')}",
                    f"- 难度: {problem_data.get('difficulty', '未知')}",
                    f"- 标签: {', '.join(problem_data.get('tags', ['未知']))}",
                    f"- 编译状态: {'成功' if result.get('compilation_success') else '失败'}",
                ])
                
                if result.get('compilation_success'):
                    report.extend([
                        f"- 测试用例: {result.get('total_cases', 0)}",
                        f"- 通过数量: {result.get('passed_cases', 0)}",
                        f"- 失败数量: {result.get('failed_cases', 0)}",
                        f"- 成功率: {result.get('success_rate', 0):.2f}%",
                        f"- 平均运行时间: {result.get('average_runtime', 0):.2f} ms",
                        f"- 超时: {'是' if result.get('timed_out') else '否'}"
                    ])
                else:
                    report.append(f"- 编译错误: {result.get('compilation_error', '未知错误')}")
        
        # 写入报告文件
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(report))
        
        return True
    except Exception as e:
        log_with_flush(f"生成Markdown报告时出错: {e}")
        return False

def generate_json_report(results, problem_info, output_path=None):
    """生成JSON格式的测试报告"""
    try:
        # 基本统计
        total_files = len(results)
        success_files = sum(1 for result in results if result.get('success', False))
        compilation_success = sum(1 for result in results if result.get('compilation_success', False))
        timed_out_files = sum(1 for result in results if result.get('timed_out', False))
        timed_out_cases = sum(result.get('timeout_cases', 0) for result in results)
        
        # 语言统计
        language_stats = dict(Counter(result['language'] for result in results))
        
        # 难度统计
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
        
        # 标签统计
        tag_stats = generate_tag_statistics(results, problem_info)
        
        # 添加语言特定统计
        lang_stats = generate_language_specific_statistics(results, problem_info)
        
        # 生成报告数据
        report_data = {
            'generated_at': datetime.now().isoformat(),
            'overall_statistics': {
                'total_files': total_files,
                'success_files': success_files,
                'compilation_success': compilation_success,
                'timed_out_files': timed_out_files,
                'timed_out_cases': timed_out_cases,
                'success_rate': success_files/total_files if total_files > 0 else 0,
                'compilation_rate': compilation_success/total_files if total_files > 0 else 0
            },
            'language_statistics': language_stats,
            'language_specific_statistics': lang_stats,
            'difficulty_statistics': difficulty_stats,
            'tag_statistics': tag_stats,
            'detailed_results': results
        }
        
        # 如果指定了输出路径，写入文件
        if output_path:
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(report_data, f, indent=2, ensure_ascii=False)
        
        return report_data
    except Exception as e:
        log_with_flush(f"生成JSON报告时出错: {e}")
        return None

def generate_csv_report(results, problem_info, output_path):
    """生成CSV格式的测试报告"""
    try:
        import csv
        
        # 准备CSV数据
        csv_data = []
        
        # 添加总体统计
        total_files = len(results)
        success_files = sum(1 for result in results if result.get('success', False))
        compilation_success = sum(1 for result in results if result.get('compilation_success', False))
        timed_out_files = sum(1 for result in results if result.get('timed_out', False))
        timed_out_cases = sum(result.get('timeout_cases', 0) for result in results)
        
        csv_data.append(['总体统计'])
        csv_data.append(['指标', '数值', '百分比'])
        csv_data.append(['总文件数', total_files, '100.00%'])
        csv_data.append(['测试成功', success_files, f'{success_files/total_files*100:.2f}%'])
        csv_data.append(['编译成功', compilation_success, f'{compilation_success/total_files*100:.2f}%'])
        csv_data.append(['超时文件', timed_out_files, f'{timed_out_files/total_files*100:.2f}%'])
        csv_data.append(['超时用例', timed_out_cases, ''])
        csv_data.append([])  # 空行分隔
        
        # 添加语言分布
        language_stats = Counter(result['language'] for result in results)
        csv_data.append(['语言分布'])
        csv_data.append(['语言', '数量', '百分比'])
        for lang, count in language_stats.items():
            csv_data.append([lang, count, f'{count/total_files*100:.2f}%'])
        csv_data.append([])  # 空行分隔
        
        # 添加总体难度统计
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
        
        csv_data.append(['总体难度统计'])
        csv_data.append(['难度', '总数', '成功率', '编译成功率', '超时率'])
        for difficulty, stats in difficulty_stats.items():
            total = stats['total']
            if total > 0:
                csv_data.append([
                    difficulty,
                    total,
                    f'{stats["success"]/total*100:.2f}%',
                    f'{stats["compilation"]/total*100:.2f}%',
                    f'{stats["timeout"]/total*100:.2f}%'
                ])
        csv_data.append([])  # 空行分隔
        
        # 添加总体标签统计
        tag_stats = generate_tag_statistics(results, problem_info)
        csv_data.append(['总体标签统计'])
        csv_data.append(['标签', '题目数量', '总测试数', '成功率', '编译成功率', '超时率', '平均运行时间(ms)', '平均测试通过率'])
        for tag, stats in sorted(tag_stats.items(), key=lambda x: x[1]['total'], reverse=True):
            total = stats['total']
            if total > 0:
                csv_data.append([
                    tag,
                    stats['unique_problems'],
                    total,
                    f'{stats["success"]/total*100:.2f}%',
                    f'{stats["compilation"]/total*100:.2f}%',
                    f'{stats["timeout"]/total*100:.2f}%',
                    f'{stats["avg_runtime"]:.2f}',
                    f'{stats["success_rate"]:.2f}%'
                ])
        csv_data.append([])  # 空行分隔
        
        # 添加按语言分类的统计
        lang_stats = generate_language_specific_statistics(results, problem_info)
        csv_data.append(['按语言分类统计'])
        
        for lang, stats in lang_stats.items():
            total = stats['total']
            if total > 0:
                csv_data.append([f'{lang} 总体统计'])
                csv_data.append(['指标', '数值', '百分比'])
                csv_data.append(['总数', total, '100.00%'])
                csv_data.append(['成功率', stats['success'], f'{stats["success"]/total*100:.2f}%'])
                csv_data.append(['编译成功率', stats['compilation'], f'{stats["compilation"]/total*100:.2f}%'])
                csv_data.append(['超时率', stats['timeout'], f'{stats["timeout"]/total*100:.2f}%'])
                csv_data.append(['平均运行时间(ms)', f'{stats["avg_runtime"]:.2f}', ''])
                csv_data.append(['平均测试通过率', f'{stats["success_rate"]:.2f}%', ''])
                csv_data.append([])  # 空行分隔
                
                # 添加语言特定的难度统计
                csv_data.append([f'{lang} 难度统计'])
                csv_data.append(['难度', '总数', '成功率', '编译成功率', '超时率'])
                for difficulty, diff_stats in stats['difficulty_stats'].items():
                    diff_total = diff_stats['total']
                    if diff_total > 0:
                        csv_data.append([
                            difficulty,
                            diff_total,
                            f'{diff_stats["success"]/diff_total*100:.2f}%',
                            f'{diff_stats["compilation"]/diff_total*100:.2f}%',
                            f'{diff_stats["timeout"]/diff_total*100:.2f}%'
                        ])
                csv_data.append([])  # 空行分隔
                
                # 添加语言特定的标签统计
                csv_data.append([f'{lang} 标签统计'])
                csv_data.append(['标签', '题目数量', '总测试数', '成功率', '编译成功率', '超时率', '平均运行时间(ms)', '平均测试通过率'])
                sorted_tags = sorted(stats['tag_stats'].items(), key=lambda x: x[1]['total'], reverse=True)
                for tag, tag_stats in sorted_tags:
                    tag_total = tag_stats['total']
                    if tag_total > 0:
                        csv_data.append([
                            tag,
                            tag_stats['unique_problems'],
                            tag_total,
                            f'{tag_stats["success"]/tag_total*100:.2f}%',
                            f'{tag_stats["compilation"]/tag_total*100:.2f}%',
                            f'{tag_stats["timeout"]/tag_total*100:.2f}%',
                            f'{tag_stats["avg_runtime"]:.2f}',
                            f'{tag_stats["success_rate"]:.2f}%'
                        ])
                csv_data.append([])  # 空行分隔
        
        # 添加详细测试结果
        csv_data.append(['详细测试结果'])
        csv_data.append(['文件名', '标题', '难度', '标签', '编译状态', '测试用例数', '通过数', '失败数', '成功率', '平均运行时间(ms)', '超时', '编译错误'])
        for result in results:
            filename = result['filename']
            contest = result.get('contest')
            problem = result.get('problem')
            language = result.get('language')
            
            if contest and problem and language:
                key = f"{contest}_{problem}_{language}"
                problem_data = problem_info.get(key, {})
                
                row = [
                    filename,
                    problem_data.get('title', '未知'),
                    problem_data.get('difficulty', '未知'),
                    ';'.join(problem_data.get('tags', ['未知'])),
                    '成功' if result.get('compilation_success') else '失败'
                ]
                
                if result.get('compilation_success'):
                    row.extend([
                        result.get('total_cases', 0),
                        result.get('passed_cases', 0),
                        result.get('failed_cases', 0),
                        f'{result.get("success_rate", 0):.2f}%',
                        f'{result.get("average_runtime", 0):.2f}',
                        '是' if result.get('timed_out') else '否',
                        ''
                    ])
                else:
                    row.extend([
                        '', '', '', '', '', '',
                        result.get('compilation_error', '未知错误')
                    ])
                
                csv_data.append(row)
        
        # 写入CSV文件
        with open(output_path, 'w', encoding='utf-8', newline='') as f:
            writer = csv.writer(f)
            writer.writerows(csv_data)
        
        return True
    except Exception as e:
        log_with_flush(f"生成CSV报告时出错: {e}")
        return False

def main():
    """主函数"""
    # 声明全局变量
    global TEST_RESULTS_DIR, REPORTS_DIR
    
    parser = argparse.ArgumentParser(description='生成测试报告')
    parser.add_argument('--days', type=int, default=7, help='包含最近几天的测试结果（默认：7）')
    parser.add_argument('--format', choices=['markdown', 'json', 'csv', 'all'], default='markdown',
                      help='输出格式（默认：markdown）')
    parser.add_argument('--output-dir', default=REPORTS_DIR,
                      help='报告输出目录（默认：test_reports/）')
    parser.add_argument('--include-details', action='store_true',
                      help='是否包含详细的测试用例信息')
    parser.add_argument('--llm', action='store_true',
                      help='使用LLM测试结果目录')
    
    args = parser.parse_args()
    
    # 根据是否使用LLM选择目录
    if args.llm:
        TEST_RESULTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_test_results")
        REPORTS_DIR = os.path.join(TRANSLATE_TO_RUST_PATH, "llm_test_reports")
        log_with_flush("使用LLM测试结果目录")
        
        # 检查是否已通过环境变量指定了特定模型的目录
        if "TEST_RESULTS_DIR" in os.environ and os.path.exists(os.environ["TEST_RESULTS_DIR"]):
            TEST_RESULTS_DIR = os.environ["TEST_RESULTS_DIR"]
            log_with_flush(f"使用环境变量指定的测试结果目录: {TEST_RESULTS_DIR}")
    else:
        log_with_flush("使用标准测试结果目录")
    
    # 确保输出目录存在
    os.makedirs(args.output_dir, exist_ok=True)
    
    # 查找测试结果文件
    result_files = find_test_result_files(directory=TEST_RESULTS_DIR, days=args.days)
    if not result_files:
        log_with_flush(f"错误: 在过去{args.days}天内没有找到测试结果文件")
        return 1
    
    # 提取测试结果
    results = []
    for file_path in result_files:
        result = extract_test_info_from_file(file_path)
        if result:
            results.append(result)
    
    if not results:
        log_with_flush("错误: 没有有效的测试结果可以生成报告")
        return 1
    
    # 提取题目信息
    problem_info = extract_problem_info(results)
    
    # 生成报告
    timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
    success = True
    
    if args.format in ['markdown', 'all']:
        markdown_path = os.path.join(args.output_dir, f'test_report_{timestamp}.md')
        if not generate_markdown_report(results, problem_info, markdown_path):
            success = False
        else:
            log_with_flush(f"已生成Markdown报告: {markdown_path}")
    
    if args.format in ['json', 'all']:
        json_path = os.path.join(args.output_dir, f'test_report_{timestamp}.json')
        if not generate_json_report(results, problem_info, json_path):
            success = False
        else:
            log_with_flush(f"已生成JSON报告: {json_path}")
    
    if args.format in ['csv', 'all']:
        csv_path = os.path.join(args.output_dir, f'test_report_{timestamp}.csv')
        if not generate_csv_report(results, problem_info, csv_path):
            success = False
        else:
            log_with_flush(f"已生成CSV报告: {csv_path}")
    
    return 0 if success else 1

if __name__ == '__main__':
    sys.exit(main()) 