"""
Merge Agent - Intelligently merge multiple code chunks.
"""

import re
from typing import Optional
from dataclasses import dataclass, field
from loguru import logger

from rustify.llm import LLMAdapter
from rustify.agents.chunker import CodeChunk


@dataclass 
class MergeResult:
    """Result of a merge operation."""
    
    code: str
    success: bool
    conflicts_resolved: list[str] = field(default_factory=list)
    error: Optional[str] = None


class MergeAgent:
    """
    Agent responsible for intelligently merging translated code chunks.
    
    Features:
    - Resolves naming conflicts
    - Deduplicates types and functions
    - Organizes code structure
    - Adds missing imports
    """
    
    SYSTEM_PROMPT = """你是一名 Rust 专家。你的任务是将多个代码块合并成一个完整、可编译的 Rust 模块。

# 合并策略

1. **去重**：
   - 合并重复的 use 语句
   - 保留类型定义的第一次出现
   - 检测并解决命名冲突

2. **组织结构**：
   - use 语句放在最前面
   - 类型定义（struct, enum, type）在中间
   - 函数实现在最后
   - impl 块紧跟其对应的类型定义

3. **解决冲突**：
   - 同名但不同的类型：添加后缀区分
   - 同名函数：保留最完整的实现
   - 循环依赖：重新组织定义顺序

4. **添加必要的代码**：
   - 添加缺失的 use 语句
   - 添加必要的 #[derive(...)] 属性
   - 添加 #![allow(unused)] 等模块属性

# 输出格式
输出完整的合并后代码，使用 ```rust ... ``` 格式"""

    def __init__(
        self,
        llm_adapter: LLMAdapter,
        use_llm_merge: bool = True
    ):
        self.llm = llm_adapter
        self.use_llm_merge = use_llm_merge
    
    async def merge(
        self,
        chunks: list[dict],  # [{"name": str, "code": str, "kind": str}]
        preamble: str = "",
        context: str = "",
    ) -> MergeResult:
        """
        Merge multiple code chunks into a single module.
        
        Args:
            chunks: List of code chunks with name, code, and kind.
            preamble: Preamble to add (common imports, etc.)
            context: Additional context.
            
        Returns:
            MergeResult with merged code.
        """
        if not chunks:
            return MergeResult(code="", success=True)
        
        if len(chunks) == 1:
            code = chunks[0]["code"]
            if preamble:
                code = preamble + "\n\n" + code
            return MergeResult(code=code, success=True)
        
        # First, try rule-based merge
        rule_merged = self._rule_based_merge(chunks, preamble)
        
        # If LLM merge is enabled and there are potential conflicts, use LLM
        if self.use_llm_merge and len(chunks) >= 2:
            try:
                return await self._llm_merge(chunks, preamble, context, rule_merged)
            except Exception as e:
                logger.warning(f"LLM merge failed, using rule-based result: {e}")
        
        return MergeResult(code=rule_merged, success=True)
    
    def _rule_based_merge(
        self,
        chunks: list[dict],
        preamble: str = ""
    ) -> str:
        """
        Simple rule-based merge without LLM.
        
        Performs:
        - Deduplication of use statements
        - Basic ordering
        - Removal of redundant blank lines
        """
        all_code = "\n\n".join(c["code"] for c in chunks)
        if preamble:
            all_code = preamble + "\n\n" + all_code
        
        return self._postprocess(all_code)
    
    def _postprocess(self, code: str) -> str:
        """
        Post-process merged code.
        
        - Deduplicate use statements
        - Remove excessive blank lines
        - Deduplicate type definitions
        """
        lines = code.splitlines()
        
        # Collect and deduplicate use statements
        uses = set()
        other_lines = []
        
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("use ") and stripped.endswith(";"):
                uses.add(stripped)
            else:
                other_lines.append(line)
        
        # Rebuild code
        result_lines = []
        
        # Add uses first
        if uses:
            for use in sorted(uses):
                result_lines.append(use)
            result_lines.append("")
        
        # Add other code, removing excessive blank lines
        blank_count = 0
        for line in other_lines:
            if not line.strip():
                blank_count += 1
                if blank_count <= 1:
                    result_lines.append(line)
            else:
                blank_count = 0
                result_lines.append(line)
        
        # Deduplicate struct/enum definitions
        result = "\n".join(result_lines)
        result = self._deduplicate_types(result)
        
        return result.strip() + "\n"
    
    def _deduplicate_types(self, code: str) -> str:
        """Remove duplicate type definitions, keeping the first occurrence."""
        lines = code.splitlines()
        seen_types = {"struct": set(), "enum": set(), "union": set(), "type": set()}
        result = []
        
        i = 0
        while i < len(lines):
            line = lines[i]
            stripped = line.strip()
            
            # Check for type definitions
            skip = False
            for kind in ["struct", "enum", "union"]:
                match = re.match(rf'^(?:pub\s+)?{kind}\s+(\w+)', stripped)
                if match:
                    name = match.group(1)
                    if name in seen_types[kind]:
                        # Skip this definition
                        skip = True
                        # Skip until closing brace
                        brace_count = 0
                        while i < len(lines):
                            brace_count += lines[i].count('{') - lines[i].count('}')
                            i += 1
                            if brace_count <= 0 and '{' in code[code.find(line):]:
                                break
                        break
                    seen_types[kind].add(name)
            
            # Check for type alias
            alias_match = re.match(r'^(?:pub\s+)?type\s+(\w+)\s*=', stripped)
            if alias_match:
                name = alias_match.group(1)
                if name in seen_types["type"]:
                    skip = True
                seen_types["type"].add(name)
            
            if not skip:
                result.append(line)
            i += 1
        
        return "\n".join(result)
    
    async def _llm_merge(
        self,
        chunks: list[dict],
        preamble: str,
        context: str,
        fallback_code: str
    ) -> MergeResult:
        """
        Use LLM to intelligently merge chunks.
        """
        # Build chunks description
        chunks_text = []
        for i, chunk in enumerate(chunks, 1):
            chunk_desc = f"### 代码块 {i}: {chunk.get('name', 'unknown')} ({chunk.get('kind', 'unknown')})\n```rust\n{chunk['code']}\n```"
            chunks_text.append(chunk_desc)
        
        chunks_section = "\n\n".join(chunks_text)
        
        prompt = f"""请将以下 {len(chunks)} 个 Rust 代码块合并成一个完整的模块：

{chunks_section}

{"## 前置代码\n```rust\n" + preamble + "\n```\n" if preamble else ""}
{"## 上下文\n" + context if context else ""}

## 要求
1. 去除重复的类型定义和函数
2. 保持代码功能完整
3. 添加必要的 use 语句
4. 输出完整的合并后代码

```rust
// 合并后的代码
```"""

        try:
            response = await self.llm.complete(
                prompt=prompt,
                system_prompt=self.SYSTEM_PROMPT,
                temperature=0.1,
                max_tokens=8192,
            )
            
            merged_code = self._extract_code(response)
            
            if merged_code and len(merged_code) > len(fallback_code) * 0.5:
                return MergeResult(
                    code=self._postprocess(merged_code),
                    success=True,
                )
            else:
                logger.warning("LLM merge result too short, using fallback")
                return MergeResult(code=fallback_code, success=True)
                
        except Exception as e:
            logger.error(f"LLM merge error: {e}")
            return MergeResult(code=fallback_code, success=True)
    
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
    
    async def resolve_conflicts(
        self,
        code: str,
        conflicts: list[str],
    ) -> MergeResult:
        """
        Resolve specific conflicts in merged code.
        
        Args:
            code: Code with conflicts.
            conflicts: List of conflict descriptions.
            
        Returns:
            MergeResult with resolved code.
        """
        if not conflicts:
            return MergeResult(code=code, success=True)
        
        conflicts_text = "\n".join(f"- {c}" for c in conflicts)
        
        prompt = f"""以下 Rust 代码存在一些问题需要解决：

```rust
{code}
```

## 问题
{conflicts_text}

## 要求
1. 解决上述所有问题
2. 保持代码功能不变
3. 输出完整的修复后代码

```rust
// 修复后的代码
```"""

        try:
            response = await self.llm.complete(
                prompt=prompt,
                system_prompt=self.SYSTEM_PROMPT,
                temperature=0.2,
                max_tokens=8192,
            )
            
            fixed_code = self._extract_code(response)
            
            if fixed_code:
                return MergeResult(
                    code=fixed_code,
                    success=True,
                    conflicts_resolved=conflicts,
                )
            else:
                return MergeResult(
                    code=code,
                    success=False,
                    error="Failed to extract resolved code",
                )
                
        except Exception as e:
            return MergeResult(
                code=code,
                success=False,
                error=str(e),
            )

