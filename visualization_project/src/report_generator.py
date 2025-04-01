import pandas as pd
import numpy as np
from pathlib import Path

def generate_summary_report(df: pd.DataFrame, output_dir: str):
    """生成详细的分析报告"""
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # 排序后的模型列表，按成功率降序
    top_models = df.sort_values('success_rate', ascending=False)
    
    with open(f'{output_dir}/summary_report.md', 'w', encoding='utf-8') as f:
        # 报告标题
        f.write('# LLM Translation Performance Analysis Report\n\n')
        
        # 1. 总体统计
        f.write('## 1. Overall Statistics\n\n')
        overall_stats = {
            'Total Models': len(df),
            'Average Success Rate': df['success_rate'].mean(),
            'Average Compilation Rate': df['compilation_rate'].mean(),
            'Average Timeout Rate': df['timeout_rate'].mean(),
            'Total Test Files': df['total_files'].sum(),
            'Total Success Files': df['success_files'].sum(),
            'Success to Compilation Ratio': df['success_to_compilation_ratio'].mean()
        }
        
        f.write('| Metric | Value |\n')
        f.write('|--------|-------|\n')
        for key, value in overall_stats.items():
            f.write(f'| {key} | {value:.4f} |\n')
        
        # 2. 模型性能排行榜
        f.write('\n## 2. Model Performance Ranking\n\n')
        
        f.write('### 2.1 Success Rate Ranking\n\n')
        f.write('| Rank | Model | Success Rate | Model Type | vs. Avg |\n')
        f.write('|------|-------|--------------|------------|--------|\n')
        
        avg_success = df['success_rate'].mean()
        for i, (_, row) in enumerate(top_models.iterrows(), 1):
            model_type = "Thinking" if row['is_thinking'] else "Greedy" if row['is_greedy'] else "Default"
            vs_avg = ((row['success_rate'] / avg_success) - 1) * 100
            vs_avg_str = f"+{vs_avg:.1f}%" if vs_avg > 0 else f"{vs_avg:.1f}%"
            
            f.write(f'| {i} | {row["model"]} | {row["success_rate"]:.4f} | {model_type} | {vs_avg_str} |\n')
        
        # 3. 每个模型的优势和劣势指标
        f.write('\n## 3. Individual Model Strengths and Weaknesses\n\n')
        
        # 定义关键指标及其描述
        key_metrics = [
            ('success_rate', 'Success Rate'),
            ('compilation_rate', 'Compilation Rate'),
            ('timeout_rate', 'Timeout Rate'),
            ('c_easy_success_rate', 'C-Easy Success'), 
            ('c_medium_success_rate', 'C-Medium Success'),
            ('c_hard_success_rate', 'C-Hard Success'),
            ('cpp_easy_success_rate', 'C++-Easy Success'),
            ('cpp_medium_success_rate', 'C++-Medium Success'),
            ('cpp_hard_success_rate', 'C++-Hard Success')
        ]
        
        # 为顶部5个模型提供详细分析
        for i, (_, row) in enumerate(top_models.head(5).iterrows()):
            model_name = row['model']
            model_type = "Thinking" if row['is_thinking'] else "Greedy" if row['is_greedy'] else "Default"
            
            f.write(f'### 3.{i+1} {model_name} ({model_type})\n\n')
            
            # 总体性能
            f.write(f'- **Overall Performance**: {row["success_rate"]:.4f} success rate (rank {i+1}/{len(df)})\n')
            
            # 优势指标 - 找出显著高于平均值的指标
            strengths = []
            weaknesses = []
            
            for metric, label in key_metrics:
                if metric in row and pd.notna(row[metric]) and metric in df.columns:
                    metric_avg = df[metric].mean()
                    metric_std = df[metric].std()
                    metric_val = row[metric]
                    
                    # Z-score表示该指标与平均值的差距（标准差单位）
                    z_score = (metric_val - metric_avg) / metric_std if metric_std > 0 else 0
                    
                    # 百分比差异
                    pct_diff = ((metric_val / metric_avg) - 1) * 100 if metric_avg > 0 else 0
                    
                    # 确定是否为优势/劣势
                    if metric == 'timeout_rate':  # 针对timeout_rate，值越低越好
                        if z_score < -0.8:  # 显著低于平均值
                            strengths.append(f'{label}: {metric_val:.4f} ({pct_diff:.1f}% below avg)')
                        elif z_score > 0.8:  # 显著高于平均值
                            weaknesses.append(f'{label}: {metric_val:.4f} ({pct_diff:.1f}% above avg)')
                    else:  # 对于其他指标，值越高越好
                        if z_score > 0.8:  # 显著高于平均值
                            strengths.append(f'{label}: {metric_val:.4f} ({pct_diff:.1f}% above avg)')
                        elif z_score < -0.8:  # 显著低于平均值
                            weaknesses.append(f'{label}: {metric_val:.4f} ({pct_diff:.1f}% below avg)')
            
            # 输出优势
            if strengths:
                f.write('- **Strengths**:\n')
                for strength in strengths:
                    f.write(f'  - {strength}\n')
            else:
                f.write('- **Strengths**: No significant strengths identified\n')
            
            # 输出劣势
            if weaknesses:
                f.write('- **Weaknesses**:\n')
                for weakness in weaknesses:
                    f.write(f'  - {weakness}\n')
            else:
                f.write('- **Weaknesses**: No significant weaknesses identified\n')
            
            f.write('\n')
        
        # 4. 各种语言和难度级别的最佳模型
        f.write('\n## 4. Best Models by Language and Difficulty\n\n')
        
        languages = ['c', 'cpp']
        difficulties = ['easy', 'medium', 'hard']
        
        for lang in languages:
            f.write(f'### 4.{1 if lang == "c" else 2} {lang.upper()} Language\n\n')
            f.write('| Difficulty | Best Model | Success Rate | Model Type |\n')
            f.write('|------------|------------|--------------|------------|\n')
            
            for diff in difficulties:
                col = f"{lang}_{diff}_success_rate"
                if col in df.columns:
                    # 找出在这个难度级别表现最好的模型
                    best_idx = df[col].idxmax()
                    best_model = df.loc[best_idx]
                    model_type = "Thinking" if best_model['is_thinking'] else "Greedy" if best_model['is_greedy'] else "Default"
                    
                    f.write(f'| {diff.capitalize()} | {best_model["model"]} | {best_model[col]:.4f} | {model_type} |\n')
        
        # 5. 模型类型对比
        f.write('\n## 5. Model Type Comparison\n\n')
        
        f.write('| Model Type | Count | Success Rate | Compilation Rate | Timeout Rate |\n')
        f.write('|------------|-------|--------------|------------------|-------------|\n')
        
        for model_type, col in [('Thinking', 'is_thinking'), 
                               ('Greedy', 'is_greedy'), 
                               ('Default', 'is_default')]:
            type_df = df[df[col]]
            if not type_df.empty:
                f.write(f'| {model_type} | {len(type_df)} | {type_df["success_rate"].mean():.4f} | ' +
                       f'{type_df["compilation_rate"].mean():.4f} | {type_df["timeout_rate"].mean():.4f} |\n')
        
        # 6. 难度级别分析
        f.write('\n## 6. Difficulty Level Analysis\n\n')
        
        # C语言
        f.write('### 6.1 C Language\n\n')
        f.write('| Difficulty | Success Rate | Thinking | Greedy | Default |\n')
        f.write('|------------|--------------|----------|--------|--------|\n')
        
        for diff in difficulties:
            col = f"c_{diff}_success_rate"
            if col in df.columns:
                thinking_rate = df[df['is_thinking']][col].mean() if not df[df['is_thinking']].empty else 0
                greedy_rate = df[df['is_greedy']][col].mean() if not df[df['is_greedy']].empty else 0
                default_rate = df[df['is_default']][col].mean() if not df[df['is_default']].empty else 0
                
                f.write(f'| {diff.capitalize()} | {df[col].mean():.4f} | {thinking_rate:.4f} | ' + 
                       f'{greedy_rate:.4f} | {default_rate:.4f} |\n')
        
        # C++语言
        f.write('\n### 6.2 C++ Language\n\n')
        f.write('| Difficulty | Success Rate | Thinking | Greedy | Default |\n')
        f.write('|------------|--------------|----------|--------|--------|\n')
        
        for diff in difficulties:
            col = f"cpp_{diff}_success_rate"
            if col in df.columns:
                thinking_rate = df[df['is_thinking']][col].mean() if not df[df['is_thinking']].empty else 0
                greedy_rate = df[df['is_greedy']][col].mean() if not df[df['is_greedy']].empty else 0
                default_rate = df[df['is_default']][col].mean() if not df[df['is_default']].empty else 0
                
                f.write(f'| {diff.capitalize()} | {df[col].mean():.4f} | {thinking_rate:.4f} | ' + 
                       f'{greedy_rate:.4f} | {default_rate:.4f} |\n')
        
        # 7. 相关性分析
        f.write('\n## 7. Correlation Analysis\n\n')
        
        corr = df[['success_rate', 'compilation_rate', 'timeout_rate']].corr()
        f.write('| Metric | Success Rate | Compilation Rate | Timeout Rate |\n')
        f.write('|--------|--------------|------------------|-------------|\n')
        
        for metric in ['success_rate', 'compilation_rate', 'timeout_rate']:
            f.write(f'| {metric} | {corr.loc[metric, "success_rate"]:.4f} | ' +
                   f'{corr.loc[metric, "compilation_rate"]:.4f} | {corr.loc[metric, "timeout_rate"]:.4f} |\n')
        
        # 8. 主要发现和建议
        f.write('\n## 8. Key Findings and Recommendations\n\n')
        
        # 计算一些关键指标
        thinking_success = df[df['is_thinking']]['success_rate'].mean()
        greedy_success = df[df['is_greedy']]['success_rate'].mean()
        default_success = df[df['is_default']]['success_rate'].mean()
        
        best_model = df.loc[df['success_rate'].idxmax()]['model']
        worst_model = df.loc[df['success_rate'].idxmin()]['model']
        
        c_avg = df[['c_easy_success_rate', 'c_medium_success_rate', 'c_hard_success_rate']].mean(axis=1).mean()
        cpp_avg = df[['cpp_easy_success_rate', 'cpp_medium_success_rate', 'cpp_hard_success_rate']].mean(axis=1).mean()
        
        f.write('### 8.1 Top Performing Models\n\n')
        
        # 按类型列出顶级模型
        for model_type, col in [('Thinking', 'is_thinking'), 
                               ('Greedy', 'is_greedy'), 
                               ('Default', 'is_default')]:
            type_df = df[df[col]]
            if not type_df.empty:
                best_in_type = type_df.loc[type_df['success_rate'].idxmax()]
                f.write(f'- Best {model_type} model: **{best_in_type["model"]}** with {best_in_type["success_rate"]:.4f} success rate\n')
        
        f.write('\n### 8.2 Model Strategy Effectiveness\n\n')
        f.write(f'- Thinking models show an average success rate of {thinking_success:.4f}, ' +
                f'{"higher" if thinking_success > greedy_success and thinking_success > default_success else "lower"} ' +
                f'than other strategies.\n')
        f.write(f'- The best performing model overall is {best_model} with a success rate of {df.loc[df["success_rate"].idxmax()]["success_rate"]:.4f}.\n')
        f.write(f'- The worst performing model overall is {worst_model} with a success rate of {df.loc[df["success_rate"].idxmin()]["success_rate"]:.4f}.\n\n')
        
        f.write('### 8.3 Language and Difficulty Patterns\n\n')
        f.write(f'- C language translation is {"more" if c_avg > cpp_avg else "less"} successful than C++ with ' +
                f'average success rates of {c_avg:.4f} vs {cpp_avg:.4f}.\n')
        
        for lang in ['c', 'cpp']:
            easy_rate = df[f'{lang}_easy_success_rate'].mean()
            medium_rate = df[f'{lang}_medium_success_rate'].mean()
            hard_rate = df[f'{lang}_hard_success_rate'].mean()
            
            f.write(f'- For {lang.upper()} language, difficulty levels show expected patterns: ' +
                    f'Easy ({easy_rate:.4f}), Medium ({medium_rate:.4f}), Hard ({hard_rate:.4f}).\n')
        
        f.write('\n### 8.4 Recommendations\n\n')
        
        # 基于最佳模型给出建议
        f.write('- Based on the analysis, we recommend:\n')
        f.write(f'  1. For best overall performance, consider using the {best_model} model or similar architecture.\n')
        f.write(f'  2. Prioritize the use of {"Thinking" if thinking_success > greedy_success and thinking_success > default_success else "Greedy" if greedy_success > thinking_success and greedy_success > default_success else "Default"} strategy for future model development.\n')
        f.write(f'  3. Focus improvement efforts on {"C++" if c_avg > cpp_avg else "C"} translation capabilities.\n')
        f.write('  4. Examine the strengths of top-performing models to identify effective techniques.\n')
        
        # 9. 附录：所有模型的详细数据
        f.write('\n## 9. Appendix: Detailed Model Data\n\n')
        
        f.write('| Model | Success Rate | Compilation Rate | Timeout Rate | Model Type |\n')
        f.write('|-------|--------------|------------------|--------------|------------|\n')
        
        for _, row in df.sort_values('success_rate', ascending=False).iterrows():
            model_type = "Thinking" if row['is_thinking'] else "Greedy" if row['is_greedy'] else "Default"
            f.write(f'| {row["model"]} | {row["success_rate"]:.4f} | {row["compilation_rate"]:.4f} | ' +
                   f'{row["timeout_rate"]:.4f} | {model_type} |\n')
    
    print(f"Summary report generated: {output_dir}/summary_report.md") 