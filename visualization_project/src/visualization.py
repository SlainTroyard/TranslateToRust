import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns
from pathlib import Path
from typing import Dict, List, Any
import matplotlib.gridspec as gridspec

# 设置学术风格的颜色方案 - 使用ColorBrewer方案，更加专业
COLORS = {
    'thinking': '#2c7bb6',  # 蓝色
    'greedy': '#fdae61',    # 橙色
    'default': '#abd9e9',   # 浅蓝色
    'bar1': '#d7191c',      # 红色
    'bar2': '#fdae61',      # 黄色
    'line': '#1a9641',      # 绿色
    'grid': '#e0e0e0',      # 浅灰色
    'text': '#333333',      # 深灰色
    'background': '#f8f8f8' # 背景色
}

# 设置绘图样式
def set_style():
    """设置学术风格的绘图样式"""
    plt.style.use('seaborn-v0_8-paper')
    # 使用系统默认字体，避免找不到Times New Roman的问题
    plt.rcParams['font.family'] = 'DejaVu Sans'
    plt.rcParams['font.size'] = 10
    plt.rcParams['axes.labelsize'] = 11
    plt.rcParams['axes.titlesize'] = 12
    plt.rcParams['xtick.labelsize'] = 9
    plt.rcParams['ytick.labelsize'] = 9
    plt.rcParams['legend.fontsize'] = 9
    plt.rcParams['figure.titlesize'] = 13
    plt.rcParams['axes.grid'] = True
    plt.rcParams['grid.alpha'] = 0.3
    plt.rcParams['axes.axisbelow'] = True
    plt.rcParams['figure.facecolor'] = COLORS['background']
    plt.rcParams['axes.facecolor'] = COLORS['background']

def plot_all_models_metrics(df: pd.DataFrame, output_dir: str, include_values: bool = True):
    """绘制所有模型的多指标对比图"""
    set_style()
    metrics = ['success_rate', 'compilation_rate', 'timeout_rate']
    titles = ['Success Rate', 'Compilation Rate', 'Timeout Rate']
    ylabels = ['Success Rate', 'Compilation Rate', 'Timeout Rate']
    
    # 创建模型名称映射，使用更短的名称
    model_name_map = {}
    for model in df['model']:
        # 提取模型名称的关键部分
        parts = model.split('-')
        if len(parts) > 1:
            # 保留第一个部分和最后一个部分
            short_name = f"{parts[0]}-{parts[-1]}"
        else:
            short_name = model
        model_name_map[model] = short_name
    
    # 排序数据 - 按成功率排序
    df_sorted = df.sort_values('success_rate', ascending=False)
    
    # 创建模型类型标记的颜色映射
    model_type_colors = []
    for _, row in df_sorted.iterrows():
        if row['is_thinking']:
            model_type_colors.append(COLORS['thinking'])
        elif row['is_greedy']:
            model_type_colors.append(COLORS['greedy'])
        else:
            model_type_colors.append(COLORS['default'])
    
    # 创建一个大图表来包含所有三个指标
    fig = plt.figure(figsize=(16, 14))  # 增加图表大小
    gs = gridspec.GridSpec(3, 1, height_ratios=[1, 1, 1], hspace=0.5)  # 增加子图间距
    
    # 绘制每个指标的条形图
    for i, (metric, title, ylabel) in enumerate(zip(metrics, titles, ylabels)):
        ax = plt.subplot(gs[i])
        
        # 对每个指标按其自身值排序
        metric_df = df.sort_values(metric, ascending=False)
        
        # 获取排序后的颜色
        colors = []
        for _, row in metric_df.iterrows():
            if row['is_thinking']:
                colors.append(COLORS['thinking'])
            elif row['is_greedy']:
                colors.append(COLORS['greedy'])
            else:
                colors.append(COLORS['default'])
        
        # 绘制条形图
        bars = ax.bar(range(len(metric_df)), metric_df[metric], color=colors, alpha=0.8)
        
        # 添加数值标签
        if include_values:
            for bar in bars:
                height = bar.get_height()
                ax.text(bar.get_x() + bar.get_width()/2., height + 0.01,
                        f'{height:.3f}', ha='center', va='bottom', fontsize=8)
        
        # 添加模型类型标记
        for j, (bar, model) in enumerate(zip(bars, metric_df['model'])):
            # 在条形图顶部添加T/G/D标记
            model_type = 'T' if metric_df.iloc[j]['is_thinking'] else 'G' if metric_df.iloc[j]['is_greedy'] else 'D'
            ax.text(bar.get_x() + bar.get_width()/2., 0.01, 
                   model_type, ha='center', va='bottom', 
                   fontsize=7, color='black', fontweight='bold')
        
        # 设置标题和标签
        ax.set_title(title, pad=15)
        ax.set_xlabel('Model')
        ax.set_ylabel(ylabel)
        
        # 设置x轴标签
        ax.set_xticks(range(len(metric_df)))
        ax.set_xticklabels([model_name_map[model] for model in metric_df['model']], 
                          rotation=30, ha='right')  # 减小旋转角度
        
        # 设置网格线
        ax.grid(True, linestyle='--', alpha=0.3)
        
        # 为奇数行添加条带背景
        for j in range(0, len(metric_df), 2):
            ax.axvspan(j-0.4, j+0.4, color='gray', alpha=0.1)
    
    # 添加图例
    from matplotlib.patches import Patch
    legend_elements = [
        Patch(facecolor=COLORS['thinking'], label='Thinking (T)'),
        Patch(facecolor=COLORS['greedy'], label='Greedy (G)'),
        Patch(facecolor=COLORS['default'], label='Default (D)')
    ]
    fig.legend(handles=legend_elements, loc='upper center', bbox_to_anchor=(0.5, 0.98), ncol=3)
    
    # 调整布局
    plt.tight_layout(rect=[0, 0, 1, 0.96])  # 为顶部的图例留出空间
    plt.suptitle('Model Performance Metrics Comparison', fontsize=14, y=0.99)
    
    # 保存图表
    plt.savefig(f'{output_dir}/all_models_metrics.pdf', bbox_inches='tight')
    plt.savefig(f'{output_dir}/all_models_metrics.png', dpi=300, bbox_inches='tight')
    plt.close()

def plot_model_detail_comparison(df: pd.DataFrame, output_dir: str):
    """为每个模型创建详细性能分析图表"""
    set_style()
    
    # 选取关键指标
    metrics = [
        'success_rate', 'compilation_rate', 'timeout_rate',
        'c_easy_success_rate', 'c_medium_success_rate', 'c_hard_success_rate',
        'cpp_easy_success_rate', 'cpp_medium_success_rate', 'cpp_hard_success_rate'
    ]
    
    metric_labels = [
        'Success Rate', 'Compilation Rate', 'Timeout Rate',
        'C-Easy', 'C-Medium', 'C-Hard',
        'C++-Easy', 'C++-Medium', 'C++-Hard'
    ]
    
    # 创建输出目录
    detail_dir = f"{output_dir}/model_details"
    Path(detail_dir).mkdir(parents=True, exist_ok=True)
    
    # 准备所有模型的平均值作为基准
    avg_values = {}
    for metric in metrics:
        if metric in df.columns:
            avg_values[metric] = df[metric].mean()
    
    # 为每个模型创建详细对比图
    for _, row in df.iterrows():
        model_name = row['model']
        model_type = "Thinking" if row['is_thinking'] else "Greedy" if row['is_greedy'] else "Default"
        
        # 从原始指标中筛选出有效的指标
        valid_metrics = []
        valid_labels = []
        model_values = []
        avg_vals = []
        
        for metric, label in zip(metrics, metric_labels):
            if metric in row and pd.notna(row[metric]) and metric in avg_values:
                valid_metrics.append(metric)
                valid_labels.append(label)
                model_values.append(row[metric])
                avg_vals.append(avg_values[metric])
        
        if not valid_metrics:
            continue
        
        # 创建模型详情图表
        plt.figure(figsize=(10, 6))
        
        x = np.arange(len(valid_metrics))
        width = 0.35
        
        # 绘制模型值和平均值的对比
        bar1 = plt.bar(x - width/2, model_values, width, label=model_name, color=COLORS['thinking'] if row['is_thinking'] else COLORS['greedy'] if row['is_greedy'] else COLORS['default'])
        bar2 = plt.bar(x + width/2, avg_vals, width, label='Average', color='grey', alpha=0.6)
        
        # 添加数值标签
        for bar in bar1:
            height = bar.get_height()
            plt.text(bar.get_x() + bar.get_width()/2., height + 0.01,
                    f'{height:.3f}', ha='center', va='bottom', fontsize=8)
        
        # 设置标题和标签
        plt.title(f'Performance Analysis: {model_name} ({model_type})')
        plt.xlabel('Metrics')
        plt.ylabel('Value')
        plt.xticks(x, valid_labels, rotation=45, ha='right')
        plt.legend()
        
        # 突出显示该模型的优势和劣势
        for i, (model_val, avg_val) in enumerate(zip(model_values, avg_vals)):
            if model_val > avg_val * 1.1:  # 优于平均值10%或更多
                plt.axvspan(i-0.4, i+0.4, color='green', alpha=0.1)
            elif model_val < avg_val * 0.9:  # 劣于平均值10%或更多
                plt.axvspan(i-0.4, i+0.4, color='red', alpha=0.1)
        
        plt.tight_layout()
        plt.savefig(f'{detail_dir}/{model_name}_detail.pdf', bbox_inches='tight')
        plt.savefig(f'{detail_dir}/{model_name}_detail.png', dpi=300, bbox_inches='tight')
        plt.close()

def plot_detailed_language_difficulty(df: pd.DataFrame, output_dir: str):
    """绘制所有模型在不同语言和难度级别下的对比图"""
    set_style()
    
    # 创建输出目录
    lang_diff_dir = f"{output_dir}/language_difficulty"
    Path(lang_diff_dir).mkdir(parents=True, exist_ok=True)
    
    languages = ['c', 'cpp']
    difficulties = ['easy', 'medium', 'hard']
    
    # 为每个语言和难度组合创建单独的图表
    for lang in languages:
        for diff in difficulties:
            col = f"{lang}_{diff}_success_rate"
            if col in df.columns:
                # 按此指标排序
                df_sorted = df.sort_values(col, ascending=False)
                
                plt.figure(figsize=(12, 6))
                
                # 设置不同模型类型的颜色
                bar_colors = []
                for _, row in df_sorted.iterrows():
                    if row['is_thinking']:
                        bar_colors.append(COLORS['thinking'])
                    elif row['is_greedy']:
                        bar_colors.append(COLORS['greedy'])
                    else:
                        bar_colors.append(COLORS['default'])
                
                # 绘制条形图
                bars = plt.bar(range(len(df_sorted)), df_sorted[col], color=bar_colors, alpha=0.8)
                
                # 添加数值标签
                for bar in bars:
                    height = bar.get_height()
                    plt.text(bar.get_x() + bar.get_width()/2., height + 0.01,
                            f'{height:.3f}', ha='center', va='bottom', fontsize=8)
                
                # 添加标题和标签
                plt.title(f'{lang.upper()} {diff.capitalize()} Success Rate by Model')
                plt.xlabel('Model')
                plt.ylabel('Success Rate')
                
                # 设置x轴标签
                plt.xticks(range(len(df_sorted)), df_sorted['model'], rotation=45, ha='right')
                
                # 添加图例
                from matplotlib.patches import Patch
                legend_elements = [
                    Patch(facecolor=COLORS['thinking'], label='Thinking'),
                    Patch(facecolor=COLORS['greedy'], label='Greedy'),
                    Patch(facecolor=COLORS['default'], label='Default')
                ]
                plt.legend(handles=legend_elements, loc='upper right')
                
                # 添加平均线
                avg = df_sorted[col].mean()
                plt.axhline(y=avg, color='r', linestyle='--', alpha=0.7)
                plt.text(len(df_sorted)-1, avg+0.01, f'Avg: {avg:.3f}', color='r')
                
                plt.tight_layout()
                plt.savefig(f'{lang_diff_dir}/{lang}_{diff}_comparison.pdf', bbox_inches='tight')
                plt.savefig(f'{lang_diff_dir}/{lang}_{diff}_comparison.png', dpi=300, bbox_inches='tight')
                plt.close()

def plot_model_success_rate(df: pd.DataFrame, output_dir: str, include_values: bool = True):
    """绘制模型成功率对比图"""
    set_style()
    plt.figure(figsize=(12, 7))
    
    # 排序数据
    df_sorted = df.sort_values('success_rate', ascending=False)
    
    # 设置不同类型模型的颜色
    bar_colors = []
    for _, row in df_sorted.iterrows():
        if row['is_thinking']:
            bar_colors.append(COLORS['thinking'])
        elif row['is_greedy']:
            bar_colors.append(COLORS['greedy'])
        else:
            bar_colors.append(COLORS['default'])
    
    # 绘制条形图
    bars = plt.bar(range(len(df_sorted)), df_sorted['success_rate'], color=bar_colors, alpha=0.8)
    
    # 添加成功率数值标签
    if include_values:
        for idx, bar in enumerate(bars):
            height = bar.get_height()
            plt.text(bar.get_x() + bar.get_width()/2., height + 0.01,
                    f'{height:.3f}', ha='center', va='bottom', fontsize=8)
            
            # 在每个条形图上添加模型类型标识
            model_type = "T" if df_sorted.iloc[idx]['is_thinking'] else "G" if df_sorted.iloc[idx]['is_greedy'] else "D"
            plt.text(bar.get_x() + bar.get_width()/2., 0.02,
                    model_type, ha='center', va='bottom', fontsize=8, fontweight='bold')
    
    # 添加标签和标题
    plt.title('Model Success Rate Comparison', pad=15)
    plt.xlabel('Model')
    plt.ylabel('Success Rate')
    
    # 设置x轴标签
    plt.xticks(range(len(df_sorted)), df_sorted['model'], rotation=45, ha='right')
    
    # 添加平均线
    avg = df['success_rate'].mean()
    plt.axhline(y=avg, color='r', linestyle='--', alpha=0.7)
    plt.text(len(df_sorted)-1, avg+0.01, f'Avg: {avg:.3f}', color='r')
    
    # 添加图例
    from matplotlib.patches import Patch
    legend_elements = [
        Patch(facecolor=COLORS['thinking'], label='Thinking (T)'),
        Patch(facecolor=COLORS['greedy'], label='Greedy (G)'),
        Patch(facecolor=COLORS['default'], label='Default (D)')
    ]
    plt.legend(handles=legend_elements, loc='upper right')
    
    # 为奇数行添加条带背景
    for i in range(0, len(df_sorted), 2):
        plt.axvspan(i-0.4, i+0.4, color='gray', alpha=0.1)
    
    plt.tight_layout()
    plt.savefig(f'{output_dir}/success_rate_comparison.pdf', bbox_inches='tight')
    plt.savefig(f'{output_dir}/success_rate_comparison.png', dpi=300, bbox_inches='tight')
    plt.close()

def plot_difficulty_comparison_detailed(df: pd.DataFrame, output_dir: str):
    """为每个难度级别的每种模型类型分别创建对比图"""
    set_style()
    difficulties = ['easy', 'medium', 'hard']
    languages = ['c', 'cpp']
    model_types = ['thinking', 'greedy', 'default']
    
    # 创建输出目录
    difficulty_dir = f"{output_dir}/difficulty_analysis"
    Path(difficulty_dir).mkdir(parents=True, exist_ok=True)
    
    # 为每个难度和语言创建一个图表
    for lang in languages:
        for diff in difficulties:
            plt.figure(figsize=(12, 7))
            col = f"{lang}_{diff}_success_rate"
            
            if col in df.columns:
                # 为每种模型类型创建单独的条形图
                for i, model_type in enumerate(model_types):
                    if model_type == 'thinking':
                        type_df = df[df['is_thinking']]
                    elif model_type == 'greedy':
                        type_df = df[df['is_greedy']]
                    else:
                        type_df = df[df['is_default']]
                    
                    if not type_df.empty:
                        # 排序
                        type_df_sorted = type_df.sort_values(col, ascending=False)
                        
                        # 绘制条形图
                        x_positions = np.arange(len(type_df_sorted))
                        bars = plt.bar(x_positions, 
                                      type_df_sorted[col], 
                                      width=0.8,
                                      label=f"{model_type.capitalize()} Models",
                                      color=COLORS[model_type],
                                      alpha=0.7)
                        
                        # 添加标签
                        for bar in bars:
                            height = bar.get_height()
                            plt.text(bar.get_x() + bar.get_width()/2., height + 0.01,
                                    f'{height:.3f}', ha='center', va='bottom', fontsize=8)
                        
                        # 添加x轴标签
                        plt.xticks(x_positions, type_df_sorted['model'], rotation=45, ha='right')
                        
                plt.title(f'{lang.upper()} {diff.capitalize()} Difficulty Success Rate by Model Types')
                plt.xlabel('Model')
                plt.ylabel('Success Rate')
                plt.legend()
                plt.tight_layout()
                plt.savefig(f'{difficulty_dir}/{lang}_{diff}_detailed.pdf', bbox_inches='tight')
                plt.savefig(f'{difficulty_dir}/{lang}_{diff}_detailed.png', dpi=300, bbox_inches='tight')
                plt.close()

def plot_language_difficulty_heatmap(df: pd.DataFrame, output_dir: str):
    """创建语言和难度级别的热力图"""
    set_style()
    
    # 创建数据
    heatmap_data = []
    languages = ['c', 'cpp']
    difficulties = ['easy', 'medium', 'hard']
    
    for lang in languages:
        for diff in difficulties:
            col = f"{lang}_{diff}_success_rate"
            if col in df.columns:
                # 按模型类型分组计算平均值
                thinking_avg = df[df['is_thinking']][col].mean()
                greedy_avg = df[df['is_greedy']][col].mean()
                default_avg = df[df['is_default']][col].mean()
                
                heatmap_data.append({
                    'Language': lang.upper(),
                    'Difficulty': diff.capitalize(),
                    'Thinking': thinking_avg,
                    'Greedy': greedy_avg,
                    'Default': default_avg
                })
    
    # 转换为DataFrame并准备热力图数据
    heatmap_df = pd.DataFrame(heatmap_data)
    
    # 为每种模型类型创建热力图
    for model_type in ['Thinking', 'Greedy', 'Default']:
        plt.figure(figsize=(8, 6))
        
        # 将数据重新组织为热力图格式
        pivot_df = heatmap_df.pivot(index='Difficulty', columns='Language', values=model_type)
        
        # 绘制热力图
        sns.heatmap(pivot_df, annot=True, cmap='YlGnBu', fmt='.3f', cbar_kws={'label': 'Success Rate'})
        
        plt.title(f'{model_type} Models Success Rate by Language and Difficulty')
        plt.tight_layout()
        plt.savefig(f'{output_dir}/heatmap_{model_type.lower()}.pdf', bbox_inches='tight')
        plt.savefig(f'{output_dir}/heatmap_{model_type.lower()}.png', dpi=300, bbox_inches='tight')
        plt.close()

def plot_model_groups_radar(df: pd.DataFrame, output_dir: str):
    """为每个模型类型创建雷达图"""
    set_style()
    
    # 定义评估指标
    metrics = ['success_rate', 'compilation_rate', 'success_to_compilation_ratio']
    metrics_labels = ['Success Rate', 'Compilation Rate', 'Success/Compilation Ratio']
    
    # 按模型类型分组
    model_groups = {}
    for model_type in ['thinking', 'greedy', 'default']:
        if model_type == 'thinking':
            group = df[df['is_thinking']]
        elif model_type == 'greedy':
            group = df[df['is_greedy']]
        else:
            group = df[df['is_default']]
        
        if not group.empty:
            model_groups[model_type] = group
    
    # 为每个模型类型计算平均指标
    avg_metrics = {}
    for model_type, group in model_groups.items():
        avg_metrics[model_type] = [group[metric].mean() for metric in metrics]
    
    # 创建雷达图
    plt.figure(figsize=(8, 6))
    
    # 设置角度
    angles = np.linspace(0, 2*np.pi, len(metrics), endpoint=False).tolist()
    angles += angles[:1]  # 闭合图形
    
    # 设置轴
    ax = plt.subplot(111, polar=True)
    
    # 添加外环线
    plt.xticks(angles[:-1], metrics_labels)
    
    # 绘制每个模型类型的雷达图
    for model_type, values in avg_metrics.items():
        values = values + [values[0]]  # 闭合图形
        ax.plot(angles, values, 'o-', linewidth=2, label=model_type.capitalize(), color=COLORS[model_type])
        ax.fill(angles, values, alpha=0.1, color=COLORS[model_type])
    
    plt.legend(loc='upper right')
    plt.title('Model Type Performance Comparison')
    
    plt.savefig(f'{output_dir}/model_type_radar.pdf', bbox_inches='tight')
    plt.savefig(f'{output_dir}/model_type_radar.png', dpi=300, bbox_inches='tight')
    plt.close()

def plot_model_series_comparison(df: pd.DataFrame, output_dir: str):
    """为表现最好的前5个模型创建雷达图，包含更多详细指标"""
    set_style()
    
    # 定义评估指标
    metrics = [
        'success_rate',  # 总体成功率
        'compilation_rate',  # 编译率
        'success_to_compilation_ratio',  # 成功率/编译率
        'c_easy_success_rate',  # C语言简单题成功率
        'c_medium_success_rate',  # C语言中等题成功率
        'c_hard_success_rate',  # C语言困难题成功率
        'cpp_easy_success_rate',  # C++简单题成功率
        'cpp_medium_success_rate',  # C++中等题成功率
        'cpp_hard_success_rate'  # C++困难题成功率
    ]
    
    metrics_labels = [
        'Overall Success',  # 总体成功率
        'Compilation',  # 编译率
        'Success/Comp',  # 成功率/编译率
        'C-Easy',  # C语言简单题
        'C-Medium',  # C语言中等题
        'C-Hard',  # C语言困难题
        'C++-Easy',  # C++简单题
        'C++-Medium',  # C++中等题
        'C++-Hard'  # C++困难题
    ]
    
    # 获取前5名模型
    top_5_df = df.sort_values('success_rate', ascending=False).head(5)
    
    # 创建雷达图
    plt.figure(figsize=(15, 10))  # 增加图表大小以适应更多指标
    
    # 设置角度
    angles = np.linspace(0, 2*np.pi, len(metrics), endpoint=False).tolist()
    angles += angles[:1]  # 闭合图形
    
    # 设置轴
    ax = plt.subplot(111, polar=True)
    
    # 添加外环线
    plt.xticks(angles[:-1], metrics_labels, fontsize=8)
    
    # 为每个模型生成独特的颜色
    colors = plt.cm.Set3(np.linspace(0, 1, len(top_5_df)))
    
    # 绘制每个模型的雷达图
    for idx, (_, row) in enumerate(top_5_df.iterrows()):
        values = [row[metric] for metric in metrics]
        values = values + [values[0]]  # 闭合图形
        
        # 绘制雷达图，图例中只显示模型名称
        ax.plot(angles, values, 'o-', linewidth=2, 
                label=row['model'], 
                color=colors[idx])
        ax.fill(angles, values, alpha=0.1, color=colors[idx])
        
        # 在关键点上添加数值标签
        for angle, value in zip(angles[:-1], values[:-1]):
            if value > 0:  # 只显示非零值
                ax.text(angle, value + 0.05, f'{value:.2f}', 
                       ha='center', va='center', fontsize=6)
    
    # 将图例放在图表外部
    plt.legend(bbox_to_anchor=(1.3, 0.5), loc='center left', borderaxespad=0)
    plt.title('Top 5 Models Performance Comparison\nAcross Different Languages and Difficulty Levels')
    
    # 添加网格线
    ax.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig(f'{output_dir}/top_5_models_radar.pdf', bbox_inches='tight')
    plt.savefig(f'{output_dir}/top_5_models_radar.png', dpi=300, bbox_inches='tight')
    plt.close()

def plot_best_model_by_series(df: pd.DataFrame, output_dir: str):
    """创建两个热力图：一个包含所有模型，另一个只包含前10名模型"""
    set_style()
    
    # 准备热力图数据
    languages = ['c', 'cpp']
    difficulties = ['easy', 'medium', 'hard']
    
    # 创建行标签
    row_labels = []
    for diff in difficulties:
        for lang in languages:
            row_labels.append(f"{diff.capitalize()} ({lang.upper()})")
    
    def create_heatmap(models_df, suffix):
        # 创建数据矩阵
        data_matrix = []
        for diff in difficulties:
            for lang in languages:
                col = f"{lang}_{diff}_success_rate"
                if col in df.columns:
                    row_data = []
                    for _, model in models_df.iterrows():
                        row_data.append(model[col])
                    data_matrix.append(row_data)
        
        # 转换为DataFrame
        heatmap_df = pd.DataFrame(data_matrix, 
                                index=row_labels,
                                columns=[f"{model} ({rate:.3f})" for model, rate in 
                                        zip(models_df['model'], models_df['success_rate'])])
        
        # 创建热力图
        plt.figure(figsize=(15, 8))  # 增加宽度以适应更多的标签
        
        # 绘制热力图
        sns.heatmap(heatmap_df, annot=True, cmap='YlGnBu', fmt='.3f', 
                    cbar_kws={'label': 'Success Rate'})
        
        plt.title(f'Model Performance by Language and Difficulty ({suffix})')
        plt.xticks(rotation=45, ha='right')  # 调整标签角度
        plt.tight_layout()
        plt.savefig(f'{output_dir}/models_heatmap_{suffix}.pdf', bbox_inches='tight')
        plt.savefig(f'{output_dir}/models_heatmap_{suffix}.png', dpi=300, bbox_inches='tight')
        plt.close()
    
    # 创建所有模型的热力图
    df_sorted = df.sort_values('success_rate', ascending=False)
    create_heatmap(df_sorted, 'all')
    
    # 创建前10名模型的热力图
    top_10_df = df_sorted.head(10)
    create_heatmap(top_10_df, 'top10') 