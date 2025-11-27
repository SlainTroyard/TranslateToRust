"""
Translate Agent - Core translation logic using LLM.
"""

import re
from typing import Optional
from dataclasses import dataclass
from loguru import logger

from rustify.llm import LLMAdapter
from rustify.templates import TemplateEngine


@dataclass
class TranslationResult:
    """Result of a translation."""
    
    code: str
    success: bool
    explanation: Optional[str] = None
    error: Optional[str] = None
    tokens_used: int = 0


class TranslateAgent:
    """
    Agent responsible for translating C code to Rust.
    
    Features:
    - Multiple output formats (fenced, json)
    - Context-aware translation
    - Incremental translation with previous context
    - Best practices enforcement
    """
    
    SYSTEM_PROMPT_FENCED = """你是一名出色的 C/C++ 和 Rust 开发者。你的任务是将 C/C++ 代码重写为地道的 Rust 代码。

**输出格式**：只输出一个 ```rust ... ``` 代码块，禁止任何解释或说明

# Rust 编码最佳实践

1. **泛型替代 void 指针**：使用泛型 T 替代 void*，保持类型安全
2. **智能指针管理内存**：
   - Box<T>：堆分配，单一所有权
   - Rc<RefCell<T>>：共享且可变，适用于复杂数据结构
3. **标准库数据结构**：Vec<T>、String、HashMap 等替代手动实现
4. **错误处理**：Result<T, E> 和 Option<T> 替代错误码和 NULL
5. **枚举与模式匹配**：enum + match 替代整数常量和 if/else 链
6. **迭代器**：使用迭代器替代手动循环和指针算术
7. **特性派生**：派生 Debug、Clone、PartialEq、Default 等
8. **方法化**：将函数转换为结构体的关联方法（impl 块）

# 完整实现
- 提供完整的代码实现，展示所有必要的逻辑细节"""

    SYSTEM_PROMPT_JSON = """你是一名出色的 C/C++ 和 Rust 开发者。你的任务是将 C/C++ 代码重写为地道的 Rust 代码。

**输出格式**：严格输出 JSON 对象 {"code":"...", "explanation":"..."}

# Rust 编码最佳实践

1. 使用泛型 T 替代 void*，保持类型安全
2. 使用 Box<T> 进行堆分配，Rc<RefCell<T>> 处理共享可变数据
3. 使用 Vec、String、HashMap 等标准库类型
4. 使用 Result/Option 处理错误和可空值
5. 使用 enum + match 替代整数常量
6. 使用迭代器替代手动循环
7. 派生 Debug、Clone、PartialEq 等特性
8. 将函数转换为 impl 块中的方法"""
    
    def __init__(
        self,
        llm_adapter: LLMAdapter,
        template_engine: Optional[TemplateEngine] = None,
        output_format: str = "fenced"  # fenced, json
    ):
        self.llm = llm_adapter
        self.template_engine = template_engine
        self.output_format = output_format
    
    async def translate(
        self,
        code: str,
        context: str = "",
        *,
        temperature: float = 0.1,
        max_tokens: int = 8192,
        incremental_context: str = "",
    ) -> TranslationResult:
        """
        Translate C code to Rust.
        
        Args:
            code: C/C++ source code.
            context: Additional context (file path, chunk info, etc.)
            temperature: LLM temperature.
            max_tokens: Maximum tokens to generate.
            incremental_context: Previously translated code for consistency.
            
        Returns:
            TranslationResult with Rust code.
        """
        # Build prompt
        system_prompt = (
            self.SYSTEM_PROMPT_FENCED 
            if self.output_format == "fenced" 
            else self.SYSTEM_PROMPT_JSON
        )
        
        user_prompt = self._build_user_prompt(code, context, incremental_context)
        
        try:
            response = await self.llm.complete(
                prompt=user_prompt,
                system_prompt=system_prompt,
                temperature=temperature,
                max_tokens=max_tokens,
            )
            
            # Extract code from response
            rust_code = self._extract_code(response)
            
            if rust_code:
                return TranslationResult(
                    code=rust_code,
                    success=True,
                    tokens_used=len(response.split()),
                )
            else:
                return TranslationResult(
                    code="",
                    success=False,
                    error="Failed to extract Rust code from response",
                )
                
        except Exception as e:
            logger.error(f"Translation failed: {e}")
            return TranslationResult(
                code="",
                success=False,
                error=str(e),
            )
    
    def _build_user_prompt(
        self,
        code: str,
        context: str,
        incremental_context: str
    ) -> str:
        """Build the user prompt for translation."""
        parts = [f"请将以下 C/C++ 代码重写为地道的 Rust 代码：\n\n```c\n{code}\n```"]
        
        if context:
            parts.append(f"\n上下文：{context}")
        
        if incremental_context:
            # Truncate if too long
            if len(incremental_context) > 4000:
                incremental_context = incremental_context[:4000] + "\n// ... (truncated)"
            parts.append(
                f"\n\n// === 已翻译的代码（供参考，保持一致性） ===\n"
                f"{incremental_context}"
            )
        
        if self.output_format == "fenced":
            parts.append("\n\n只输出一个 ```rust ... ``` 代码块。")
        else:
            parts.append('\n\n请输出 JSON 格式: {"code": "...", "explanation": "..."}')
        
        return "\n".join(parts)
    
    def _extract_code(self, response: str) -> Optional[str]:
        """Extract Rust code from LLM response."""
        if self.output_format == "json":
            return self._extract_from_json(response)
        else:
            return self._extract_from_fenced(response)
    
    def _extract_from_fenced(self, response: str) -> Optional[str]:
        """Extract code from fenced code block."""
        patterns = [
            r'```rust\n(.*?)```',
            r'```rs\n(.*?)```',
            r'```\n(.*?)```',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, response, re.DOTALL)
            if match:
                return match.group(1).strip()
        
        # If no code block, check if response looks like Rust code
        if response.strip().startswith(("fn ", "struct ", "enum ", "impl ", "pub ", "use ", "mod ")):
            return response.strip()
        
        return None
    
    def _extract_from_json(self, response: str) -> Optional[str]:
        """Extract code from JSON response."""
        import json
        
        # Find JSON object in response
        json_match = re.search(r'\{[^{}]*"code"[^{}]*\}', response, re.DOTALL)
        if not json_match:
            # Try to find any JSON object
            json_match = re.search(r'\{.*\}', response, re.DOTALL)
        
        if json_match:
            try:
                data = json.loads(json_match.group())
                return data.get("code", "")
            except json.JSONDecodeError:
                pass
        
        # Fallback to fenced extraction
        return self._extract_from_fenced(response)
    
    async def translate_with_retry(
        self,
        code: str,
        context: str = "",
        max_retries: int = 3,
        **kwargs
    ) -> TranslationResult:
        """
        Translate with automatic retry on failure.
        
        Args:
            code: C/C++ source code.
            context: Additional context.
            max_retries: Maximum retry attempts.
            **kwargs: Additional arguments passed to translate().
            
        Returns:
            Best TranslationResult from attempts.
        """
        last_error = None
        temperatures = [0.1, 0.3, 0.5]  # Increase temperature on retry
        
        for attempt in range(max_retries):
            temp = temperatures[min(attempt, len(temperatures) - 1)]
            
            result = await self.translate(
                code,
                context,
                temperature=temp,
                **kwargs
            )
            
            if result.success:
                return result
            
            last_error = result.error
            logger.debug(f"Translation attempt {attempt + 1} failed: {last_error}")
        
        return TranslationResult(
            code="",
            success=False,
            error=f"All {max_retries} attempts failed. Last error: {last_error}",
        )

