"""
Fix Agent - Repair compilation errors in Rust code.
"""

import re
from typing import Optional
from dataclasses import dataclass, field
from loguru import logger

from rustify.llm import LLMAdapter


@dataclass
class CompileError:
    """Represents a Rust compilation error."""
    
    code: str  # Error code like E0425
    message: str
    rendered: str  # Full rendered error message
    file: Optional[str] = None
    line: Optional[int] = None
    suggestion: Optional[str] = None


@dataclass
class FixResult:
    """Result of a fix attempt."""
    
    code: str
    success: bool
    fixed_errors: list[str] = field(default_factory=list)
    remaining_errors: list[str] = field(default_factory=list)
    error: Optional[str] = None


class FixAgent:
    """
    Agent responsible for fixing Rust compilation errors.
    
    Features:
    - Analyzes cargo check output
    - Generates targeted fixes
    - Iterative repair with multiple attempts
    """
    
    SYSTEM_PROMPT = """你是一名 Rust 专家。你的任务是修复 Rust 代码中的编译错误。

# 修复策略

1. **仔细分析每个错误**：理解错误原因和建议的修复方法
2. **最小化改动**：只修复必要的部分，不要重构整个代码
3. **保持功能**：确保修复后的代码功能与原代码相同
4. **遵循 Rust 惯例**：使用正确的所有权、借用和生命周期

# 常见错误修复

- **E0425 (未找到名称)**：添加缺失的 use 语句或定义
- **E0308 (类型不匹配)**：添加类型转换或修改类型
- **E0382 (移动后使用)**：使用 clone() 或 借用
- **E0502 (借用冲突)**：调整借用范围或使用 RefCell
- **E0599 (方法未找到)**：添加 impl 或 use 语句
- **E0106 (缺少生命周期)**：添加生命周期参数

# 输出格式
只输出修复后的完整代码，使用 ```rust ... ``` 格式"""

    def __init__(self, llm_adapter: LLMAdapter):
        self.llm = llm_adapter
    
    async def fix(
        self,
        code: str,
        errors: list[CompileError],
        context: str = "",
        max_errors: int = 10,
    ) -> FixResult:
        """
        Fix compilation errors in Rust code.
        
        Args:
            code: Rust source code with errors.
            errors: List of compilation errors.
            context: Additional context.
            max_errors: Maximum errors to include in prompt.
            
        Returns:
            FixResult with fixed code.
        """
        if not errors:
            return FixResult(code=code, success=True)
        
        # Limit errors to prevent context overflow
        errors_to_fix = errors[:max_errors]
        
        # Build prompt
        prompt = self._build_fix_prompt(code, errors_to_fix, context)
        
        try:
            response = await self.llm.complete(
                prompt=prompt,
                system_prompt=self.SYSTEM_PROMPT,
                temperature=0.2,
                max_tokens=8192,
            )
            
            fixed_code = self._extract_code(response)
            
            if fixed_code and fixed_code != code:
                return FixResult(
                    code=fixed_code,
                    success=True,
                    fixed_errors=[e.code for e in errors_to_fix],
                )
            else:
                return FixResult(
                    code=code,
                    success=False,
                    error="Failed to extract fixed code or no changes made",
                )
                
        except Exception as e:
            logger.error(f"Fix failed: {e}")
            return FixResult(
                code=code,
                success=False,
                error=str(e),
            )
    
    def _build_fix_prompt(
        self,
        code: str,
        errors: list[CompileError],
        context: str
    ) -> str:
        """Build the fix prompt."""
        error_texts = []
        for i, err in enumerate(errors, 1):
            error_text = f"### 错误 {i}"
            if err.code:
                error_text += f" ({err.code})"
            error_text += f"\n{err.message}"
            if err.rendered:
                error_text += f"\n```\n{err.rendered}\n```"
            error_texts.append(error_text)
        
        errors_section = "\n\n".join(error_texts)
        
        prompt = f"""请修复以下 Rust 代码中的编译错误：

## 当前代码

```rust
{code}
```

## 编译错误

{errors_section}

{"## 上下文\n" + context if context else ""}

## 要求

1. 修复所有上述错误
2. 输出完整的修复后代码
3. 只输出代码，不要解释

```rust
// 修复后的代码
```"""
        
        return prompt
    
    def _extract_code(self, response: str) -> Optional[str]:
        """Extract Rust code from response."""
        patterns = [
            r'```rust\n(.*?)```',
            r'```\n(.*?)```',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, response, re.DOTALL)
            if match:
                return match.group(1).strip()
        
        return None
    
    async def fix_iterative(
        self,
        code: str,
        check_fn,  # async def check_fn(code: str) -> list[CompileError]
        max_iterations: int = 5,
        context: str = "",
    ) -> FixResult:
        """
        Iteratively fix code until it compiles or max iterations reached.
        
        Args:
            code: Initial Rust code.
            check_fn: Async function that returns compilation errors.
            max_iterations: Maximum fix iterations.
            context: Additional context.
            
        Returns:
            FixResult with final code state.
        """
        current_code = code
        all_fixed = []
        
        for iteration in range(max_iterations):
            logger.debug(f"Fix iteration {iteration + 1}")
            
            # Check for errors
            errors = await check_fn(current_code)
            
            if not errors:
                logger.info(f"Code fixed after {iteration + 1} iterations")
                return FixResult(
                    code=current_code,
                    success=True,
                    fixed_errors=all_fixed,
                )
            
            # Try to fix
            result = await self.fix(current_code, errors, context)
            
            if result.success and result.code != current_code:
                current_code = result.code
                all_fixed.extend(result.fixed_errors)
            else:
                # No progress made
                logger.warning(f"Fix iteration {iteration + 1} made no progress")
                break
        
        # Check final state
        final_errors = await check_fn(current_code)
        
        return FixResult(
            code=current_code,
            success=len(final_errors) == 0,
            fixed_errors=all_fixed,
            remaining_errors=[e.message for e in final_errors],
        )
    
    @staticmethod
    def parse_cargo_check_output(output: str) -> list[CompileError]:
        """
        Parse cargo check JSON output into CompileError objects.
        
        Args:
            output: JSON lines from cargo check --message-format=json
            
        Returns:
            List of CompileError objects.
        """
        import json
        
        errors = []
        
        for line in output.split('\n'):
            if not line.strip():
                continue
            
            try:
                msg = json.loads(line)
                if msg.get("reason") != "compiler-message":
                    continue
                
                message = msg.get("message", {})
                if message.get("level") != "error":
                    continue
                
                error = CompileError(
                    code=message.get("code", {}).get("code", ""),
                    message=message.get("message", ""),
                    rendered=message.get("rendered", ""),
                )
                
                # Extract file and line from spans
                spans = message.get("spans", [])
                if spans:
                    error.file = spans[0].get("file_name")
                    error.line = spans[0].get("line_start")
                
                # Extract suggestion
                children = message.get("children", [])
                for child in children:
                    if child.get("level") == "help":
                        error.suggestion = child.get("message")
                        break
                
                errors.append(error)
                
            except json.JSONDecodeError:
                continue
        
        return errors

