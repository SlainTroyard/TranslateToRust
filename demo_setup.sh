#!/bin/bash
# Rustify 演示环境准备脚本

set -e

echo "🦀 Rustify 演示环境准备"
echo "========================"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目路径
RUSTIFY_DIR="$(cd "$(dirname "$0")" && pwd)"
DEMO_SOURCE="/tmp/c-algorithms-demo"
DEMO_TARGET="/tmp/c-algorithms-rs"

echo ""
echo -e "${BLUE}[1/5]${NC} 清理旧的演示文件..."
rm -rf "$DEMO_SOURCE" 2>/dev/null || true
rm -rf "$DEMO_TARGET" 2>/dev/null || true
echo -e "${GREEN}✓${NC} 清理完成"

echo ""
echo -e "${BLUE}[2/5]${NC} 复制演示项目..."
if [ -d "../c-algorithms" ]; then
    cp -r ../c-algorithms "$DEMO_SOURCE"
    echo -e "${GREEN}✓${NC} 复制完成: $DEMO_SOURCE"
elif [ -d "$RUSTIFY_DIR/dataset/c-algorithms" ]; then
    cp -r "$RUSTIFY_DIR/dataset/c-algorithms" "$DEMO_SOURCE"
    echo -e "${GREEN}✓${NC} 复制完成: $DEMO_SOURCE"
else
    echo -e "${RED}✗${NC} 未找到 c-algorithms 项目"
    echo "  请确保 ../c-algorithms 或 dataset/c-algorithms 目录存在"
    exit 1
fi

echo ""
echo -e "${BLUE}[3/5]${NC} 清理不必要的文件..."
find "$DEMO_SOURCE" -name "*.o" -delete 2>/dev/null || true
find "$DEMO_SOURCE" -name "*.lo" -delete 2>/dev/null || true
find "$DEMO_SOURCE" -name "*.la" -delete 2>/dev/null || true
find "$DEMO_SOURCE" -name "Makefile*" -delete 2>/dev/null || true
echo -e "${GREEN}✓${NC} 清理完成"

echo ""
echo -e "${BLUE}[4/5]${NC} 统计项目信息..."
C_FILES=$(find "$DEMO_SOURCE" -name "*.c" 2>/dev/null | wc -l)
H_FILES=$(find "$DEMO_SOURCE" -name "*.h" 2>/dev/null | wc -l)
echo -e "${GREEN}✓${NC} 项目统计:"
echo "  - C 源文件: $C_FILES 个"
echo "  - 头文件: $H_FILES 个"

echo ""
echo -e "${BLUE}[5/5]${NC} 检查配置..."
if [ -f "$RUSTIFY_DIR/.env" ]; then
    echo -e "${GREEN}✓${NC} 找到 .env 文件"
elif [ -n "$RUSTIFY_LLM_API_KEY" ]; then
    echo -e "${GREEN}✓${NC} API Key 已通过环境变量配置"
else
    echo -e "${YELLOW}⚠${NC} 未找到 API Key 配置"
    echo "  请创建 .env 文件或设置环境变量:"
    echo "  export RUSTIFY_LLM_API_KEY=your-api-key"
fi

if [ -f "$RUSTIFY_DIR/rustify.toml" ]; then
    echo -e "${GREEN}✓${NC} 找到 rustify.toml 配置文件"
else
    echo -e "${YELLOW}⚠${NC} 未找到 rustify.toml，将使用默认配置"
fi

echo ""
echo "========================"
echo -e "${GREEN}演示环境准备完成！${NC}"
echo ""
echo "演示命令:"
echo ""
echo -e "${YELLOW}# 1. 分析项目（不翻译）${NC}"
echo "rustify analyze $DEMO_SOURCE/src"
echo ""
echo -e "${YELLOW}# 2. 完整翻译${NC}"
echo "rustify translate $DEMO_SOURCE/src $DEMO_TARGET --overwrite"
echo ""
echo -e "${YELLOW}# 3. 带监控面板翻译${NC}"
echo "rustify translate $DEMO_SOURCE/src $DEMO_TARGET --dashboard --overwrite"
echo ""
echo -e "${YELLOW}# 4. 增量翻译（修改文件后）${NC}"
echo "echo '// modified' >> $DEMO_SOURCE/src/slist.c"
echo "rustify translate $DEMO_SOURCE/src $DEMO_TARGET --incremental"
echo ""
echo -e "${YELLOW}# 5. 交互式修复${NC}"
echo "rustify fix $DEMO_TARGET"
echo ""
echo -e "${YELLOW}# 6. 启动独立监控面板${NC}"
echo "rustify dashboard $DEMO_TARGET --port 8765"
echo ""
