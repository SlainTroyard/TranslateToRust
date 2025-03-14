#!/bin/bash

# 运行单个题目的翻译和测试示例
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# 默认运行Weekly Contest 413 Problem 1 (CPP)
CONTEST=${1:-413}
PROBLEM=${2:-1}
LANGUAGE=${3:-CPP}

echo "=== 运行单个题目翻译示例 ==="
echo "比赛: Weekly Contest $CONTEST"
echo "题目: Problem $PROBLEM"
echo "语言: $LANGUAGE"
echo ""

# 创建翻译报告目录
REPORTS_DIR="${PROJECT_ROOT}/translation_reports/example_${CONTEST}_${PROBLEM}_${LANGUAGE}"
mkdir -p "${REPORTS_DIR}"

# 运行翻译脚本
echo "开始翻译..."
cd "${SCRIPT_DIR}" && ./batch_translate.py --contest $CONTEST --problem $PROBLEM --language $LANGUAGE --output "${REPORTS_DIR}"

echo ""
echo "示例运行完成。结果保存在: ${REPORTS_DIR}" 