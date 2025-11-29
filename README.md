# Rustify

C/C++ to Rust 自动翻译工具。基于多智能体架构，使用 LLM 进行代码翻译，支持依赖分析、增量翻译、双循环错误修复和状态回滚。

## 特性

- **双循环错误修复** — 语法错误内循环 + 逻辑错误外循环
- **状态回滚机制** — 自动保存最佳状态，失败时回滚
- **行级代码修改** — 精确修改指定行，避免全文件替换
- **增量翻译** — 只翻译修改的文件，节省 80%+ 时间
- **实时监控面板** — Web UI 展示翻译进度和依赖图

## 安装

```bash
pip install -e .

# 可选：安装 Clang AST 支持（更精确的依赖分析）
pip install -e ".[clang]"

# 可选：安装 Web UI
pip install -e ".[web]"
```

## 配置

### 方式一：配置文件 (`rustify.toml`)

```toml
[llm]
model = "deepseek-chat"
base_url = "https://api.deepseek.com"
# api_key = "..."  # 推荐放在 .env 中

[analyzer]
model = "deepseek-reasoner"  # 用于深度分析

# 修复次数配置
max_fix_attempts = 10        # 编译/逻辑修复次数
max_test_fix_attempts = 20   # 语法修复次数（内循环）
```

### 方式二：环境变量 (`.env`)

```bash
# .env 文件（不提交到 Git）
RUSTIFY_LLM_API_KEY=your-api-key
RUSTIFY_ANALYZER_API_KEY=your-api-key
```

> **提示**：敏感信息（API Key）放 `.env`，其他配置放 `rustify.toml`。两者会自动合并。

## 使用

### 命令行

```bash
# 翻译项目
rustify translate ./c-project ./rust-project

# 覆盖已存在的目标
rustify translate ./c-project ./rust-project --overwrite

# 增量翻译（只翻译变更文件）
rustify translate ./c-project ./rust-project --incremental

# 带实时监控面板
rustify translate ./c-project ./rust-project --dashboard

# 分析项目依赖（不翻译）
rustify analyze ./c-project

# 从断点恢复
rustify resume ./rust-project/states.json

# 交互式修复
rustify fix ./rust-project
```

### Python API

```python
from rustify import Rustify
from rustify.config import RustifyConfig

# 使用配置文件
config = RustifyConfig.load("rustify.toml")
rustify = Rustify(config=config)
rustify.translate("./c-project", "./rust-project")

# 或直接配置
rustify = Rustify(
    llm_config={
        "model": "deepseek-chat", 
        "api_key": "your-key",
        "base_url": "https://api.deepseek.com"
    }
)
rustify.translate("./c-project", "./rust-project")
```

## 架构

```
Source (C/C++)
     │
     ▼
┌─────────────┐
│ Orchestrator │  ← 加载源项目，构建依赖图，创建目标项目
└─────────────┘
     │
     ▼
┌─────────────┐
│  Architect  │  ← 按拓扑序分配翻译任务，协调各 Agent
└─────────────┘
     │
     ├──────────────────┬──────────────────┐
     ▼                  ▼                  ▼
┌────────────┐    ┌───────────┐    ┌─────────────┐
│ Translator │    │ Validator │    │ Benchmarker │
└────────────┘    └───────────┘    └─────────────┘
     │                  │
     │                  │
     ▼                  ▼
  翻译代码         双循环测试修复
  编译检查         ├─ 内循环：语法错误 (20次)
  错误修复         └─ 外循环：逻辑错误 (10次)
     │                  │
     └────────┬─────────┘
              ▼
        Target (Rust)
```

### Agent 说明

| Agent | 职责 |
|-------|------|
| **Orchestrator** | 项目管理，加载源码，创建目标项目 |
| **Architect** | 模块管理，协调翻译流程 |
| **Translator** | 代码翻译，编译检查，错误修复 |
| **Validator** | 测试翻译，双循环错误修复，状态回滚 |
| **Benchmarker** | 性能基准测试生成 |
| **Analyzer** | 深度分析，复杂问题推理 |

## 项目结构

```
src/rustify/
├── __init__.py           # 主入口
├── cli.py                # 命令行
├── config.py             # 配置管理
├── agents/               # 智能体
│   ├── orchestrator.py   # 项目编排
│   ├── architect.py      # 模块管理
│   ├── translator.py     # 代码翻译
│   ├── validator.py      # 测试验证
│   ├── benchmarker.py    # 性能测试
│   └── analyzer.py       # 深度分析
├── graph/                # 依赖分析
│   ├── dep_graph.py
│   ├── parser.py         # 正则解析（fallback）
│   └── clang_parser.py   # Clang AST 解析
├── schema/               # 数据模型
├── state/                # 状态持久化
├── tools/                # 工具函数
│   ├── rust_utils.py     # Cargo 工具
│   ├── patch.py          # 行级代码修改
│   └── file_tools.py     # 文件操作
├── incremental/          # 增量翻译
└── web/                  # Web UI
```

## 核心功能

### 双循环错误修复

```
┌─────────────────────────────────────────────┐
│  外循环：逻辑错误修复 (max_fix_attempts)     │
│  ┌───────────────────────────────────────┐  │
│  │  内循环：语法错误修复                  │  │
│  │  (max_test_fix_attempts)              │  │
│  │                                       │  │
│  │  编译 → 失败 → 修复 → 重试...          │  │
│  │          ↓                            │  │
│  │        成功 → 运行测试                 │  │
│  └───────────────────────────────────────┘  │
│                    ↓                        │
│  测试失败 → 深度分析 → 修复 → 回到内循环     │
│                    ↓                        │
│  测试通过 → 完成!                           │
└─────────────────────────────────────────────┘
```

### 状态回滚

- 每次修复前自动保存状态
- 追踪"最佳状态"（失败测试最少）
- 达到最大尝试次数时回滚到最佳状态

### 行级代码修改

使用 TransFactor 格式进行精确修改：

```markdown
```rust:src/lib.rs:45:52
fn fixed_function() {
    // 只修改第 45-52 行
}
```
```

## 配置选项

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `max_fix_attempts` | 10 | 编译/逻辑修复最大次数 |
| `max_test_fix_attempts` | 20 | 语法修复最大次数 |
| `parallel_tasks` | 1 | 并行翻译任务数 |
| `log_level` | INFO | 日志级别 |

# 1. 安装系统 libclang
sudo apt install libclang-dev

# 2. 安装 Python 绑定
pip install -e ".[clang]"

## 许可证

MIT
