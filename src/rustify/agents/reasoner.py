"""
Reasoner Agent - Specialized agent for complex reasoning tasks.

"""

from typing import Optional
import logging

from rustify.agents.base import BaseAgent


class Reasoner(BaseAgent):
    """
    Reasoning agent specialized for complex problem solving.
    
    Uses advanced LLM reasoning capabilities for:
    - Error analysis
    - Code fix suggestions
    - Complex translation decisions
    """
    
    ROLE = "reasoner"
    DESCRIPTION = "A powerful agent specialized in deep reasoning and analysis."
    
    def __init__(
        self,
        llm_config: dict,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the reasoner.
        
        Args:
            llm_config: LLM config (should use a reasoning-capable model).
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
    
    def analyze_error(
        self,
        error_message: str,
        code_context: str
    ) -> str:
        """
        Analyze an error and suggest fixes.
        
        Args:
            error_message: The error to analyze.
            code_context: Relevant code context.
            
        Returns:
            Analysis and fix suggestions.
        """
        prompt = f"""Analyze the following error and provide a detailed fix:

## Error
{error_message}

## Code Context
{code_context}

Provide:
1. Root cause analysis
2. Step-by-step fix instructions
3. The corrected code
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        return response.content
    
    def suggest_translation(
        self,
        source_code: str,
        node_type: str,
        context: str = ""
    ) -> str:
        """
        Suggest a Rust translation for C code.
        
        Args:
            source_code: C source code.
            node_type: Type of node (function, struct, etc.).
            context: Additional context.
            
        Returns:
            Suggested Rust translation.
        """
        prompt = f"""Translate the following C code to idiomatic Rust:

## Node Type
{node_type}

## Source Code (C)
```c
{source_code}
```

## Context
{context if context else "No additional context."}

Requirements:
1. Use idiomatic Rust patterns
2. Ensure memory safety
3. Handle errors with Result/Option
4. Add appropriate documentation
5. Preserve the original functionality

Provide only the Rust code in a code block.
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}], temperature=0.2)
        return self.extract_rust_code(response.content) or response.content

