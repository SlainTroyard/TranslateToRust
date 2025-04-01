import os
import json
import pandas as pd
from typing import Dict, List, Any

def load_test_results(base_dir: str) -> pd.DataFrame:
    """加载所有测试结果并转换为DataFrame"""
    results = []
    language_stats = {'C': 0, 'CPP': 0}
    
    # 定义thinking模型列表
    thinking_models = {'o1', 'o3-mini', 'deepseek-r1', 'qwq-plus', 'deepseek-r1-distill-qwen-32b'}
    
    for model_dir in os.listdir(base_dir):
        if not os.path.isdir(os.path.join(base_dir, model_dir)):
            continue
            
        model_path = os.path.join(base_dir, model_dir)
        for file in os.listdir(model_path):
            if file.endswith('.json'):
                with open(os.path.join(model_path, file), 'r', encoding='utf-8') as f:
                    try:
                        data = json.load(f)
                        stats = data['overall_statistics']
                        stats['model'] = model_dir
                        
                        # 更新模型类型标记逻辑
                        base_name = model_dir.lower()
                        stats['is_thinking'] = base_name in thinking_models or 'thinking' in base_name
                        stats['is_greedy'] = 'greedy' in base_name
                        stats['is_default'] = not stats['is_thinking'] and not stats['is_greedy'] and 'default' in base_name
                        
                        # 提取模型基础名称（不包含策略后缀）
                        for suffix in ['-thinking', '-greedy', '-default']:
                            base_name = base_name.replace(suffix, '')
                        stats['base_model'] = base_name
                        
                        results.append(stats)
                        
                        # 更新语言统计
                        if 'language_statistics' in data:
                            language_stats['C'] = data['language_statistics']['C']
                            language_stats['CPP'] = data['language_statistics']['CPP']
                            
                        # 添加难度统计
                        if 'language_specific_statistics' in data:
                            c_stats = data['language_specific_statistics'].get('C', {})
                            cpp_stats = data['language_specific_statistics'].get('CPP', {})
                            
                            for lang_stats, prefix in [(c_stats, 'c_'), (cpp_stats, 'cpp_')]:
                                if 'difficulty_stats' in lang_stats:
                                    diff_stats = lang_stats['difficulty_stats']
                                    for diff in ['Easy', 'Medium', 'Hard']:
                                        if diff in diff_stats:
                                            stats[f"{prefix}{diff.lower()}_total"] = diff_stats[diff]['total']
                                            stats[f"{prefix}{diff.lower()}_success"] = diff_stats[diff]['success']
                                            stats[f"{prefix}{diff.lower()}_compilation"] = diff_stats[diff]['compilation']
                                            
                    except json.JSONDecodeError:
                        print(f"Error reading {file}")
                        continue
    
    df = pd.DataFrame(results)
    df['language_stats'] = [language_stats] * len(df)
    
    # 计算额外的统计指标
    df['success_to_compilation_ratio'] = df['success_files'] / df['compilation_success']
    df['failure_rate'] = 1 - df['success_rate']
    df['compilation_failure_rate'] = 1 - df['compilation_rate']
    df['timeout_rate'] = df['timed_out_files'] / df['total_files']
    
    # 计算难度相关的成功率
    for lang in ['c', 'cpp']:
        for diff in ['easy', 'medium', 'hard']:
            col_total = f"{lang}_{diff}_total"
            col_success = f"{lang}_{diff}_success"
            if col_total in df.columns and col_success in df.columns:
                df[f"{lang}_{diff}_success_rate"] = df[col_success] / df[col_total]
    
    return df

def get_model_groups(df: pd.DataFrame) -> dict:
    """将模型按类型分组"""
    groups = {
        'thinking': df[df['is_thinking']],
        'greedy': df[df['is_greedy']],
        'default': df[df['is_default']]
    }
    return groups 