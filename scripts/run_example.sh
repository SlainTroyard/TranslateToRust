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
cd "${SCRIPT_DIR}" && python3 ./batch_transonly.py --contest $CONTEST --problem $PROBLEM --language $LANGUAGE

# 确定Rust文件路径
RUST_FILE="${PROJECT_ROOT}/translated/weekly_contest_${CONTEST}_p${PROBLEM}_${LANGUAGE,,}.rs"
if [ ! -f "$RUST_FILE" ]; then
    echo "错误：翻译后的Rust文件不存在: $RUST_FILE"
    exit 1
fi

# 运行测试脚本
echo "开始测试..."
cd "${SCRIPT_DIR}" && python3 ./batch_test.py --file "$RUST_FILE"

echo ""
echo "示例运行完成。翻译文件保存在: ${RUST_FILE}"
echo "测试结果保存在test_results目录中。" 