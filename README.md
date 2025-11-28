# Rustify

C/C++ to Rust 自动翻译工具。基于多智能体架构，使用 LLM 进行代码翻译，支持依赖分析、增量翻译和交互式修复。

## 安装

```bash
pip install -e .

# 可选：安装 Clang AST 支持（更精确的依赖分析）
pip install -e ".[clang]"

# 可选：安装 Web UI
pip install -e ".[web]"
```

## 配置

创建 `rustify.toml`:

```toml
[llm]
model = "deepseek-chat"
api_key = "your-api-key"
base_url = "https://api.deepseek.com"

[reasoner]
model = "deepseek-reasoner"
api_key = "your-api-key"
```

或使用环境变量：

```bash
export RUSTIFY_LLM_MODEL=deepseek-chat
export RUSTIFY_LLM_API_KEY=your-key
```

## 使用

### 命令行

```bash
# 翻译项目
rustify translate ./c-project ./rust-project

# 增量翻译（只翻译变更文件）
rustify translate ./c-project ./rust-project --incremental

# 交互式修复
rustify translate ./c-project ./rust-project --interactive

# 启动可视化面板
rustify dashboard ./c-project ./rust-project

# 分析项目依赖
rustify analyze ./c-project

# 从断点恢复
rustify resume ./rust-project/states.json
```

### Python API

```python
from rustify import Rustify

rustify = Rustify(
    llm_config={"model": "deepseek-chat", "api_key": "your-key"}
)

# 全量翻译
rustify.translate("./c-project", "./rust-project")

# 增量翻译
rustify.translate("./c-project", "./rust-project", files_filter=["modified.c"])

# 分析依赖
analysis = rustify.analyze_only("./c-project")
```

## 架构

```
Source (C/C++)
     |
     v
ProjectManager    -- 加载源项目，构建依赖图，创建目标项目
     |
     v
TechLeader        -- 按拓扑序分配翻译任务
     |
     v
CodeMonkey        -- 调用 LLM 翻译代码，编译检查，错误修复
     |
     v
TestEngineer      -- 翻译测试用例
     |
     v
Target (Rust)
```

## 项目结构

```
src/rustify/
├── __init__.py           # 主入口
├── cli.py                # 命令行
├── config.py             # 配置管理
├── agents/               # 智能体
│   ├── project_manager.py
│   ├── tech_leader.py
│   ├── code_monkey.py
│   ├── test_engineer.py
│   └── reasoner.py
├── graph/                # 依赖分析
│   ├── dep_graph.py
│   ├── parser.py         # 正则解析（fallback）
│   └── clang_parser.py   # Clang AST 解析
├── schema/               # 数据模型
├── state/                # 状态持久化
└── tools/                # 工具函数
```

## 功能

- 依赖图分析：Clang AST 或正则解析，拓扑排序确定翻译顺序
- 增量翻译：基于文件哈希检测变更，只翻译修改的文件
- 交互式修复：编译失败时暂停，展示错误和建议，用户决定如何修复
- 状态持久化：支持断点续传
- 头文件上下文：自动收集依赖的头文件定义，提供给 LLM

## 许可证

MIT
