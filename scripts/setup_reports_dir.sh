#!/bin/bash

# 创建翻译报告目录
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
REPORTS_DIR="${PROJECT_ROOT}/translation_reports"

echo "Creating translation reports directory at: ${REPORTS_DIR}"
mkdir -p "${REPORTS_DIR}"
echo "Directory created successfully!"
echo "Use this command to run the batch translation:"
echo "  cd ${SCRIPT_DIR} && ./batch_translate.py --output ${REPORTS_DIR}" 