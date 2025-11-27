"""
Code Monkey Agent - Handles actual code translation and fixing.

"""

import os
import re
from typing import Optional, List
import logging

from rustify.agents.base import BaseAgent, TranspileMemory
from rustify.agents.reasoner import Reasoner
from rustify.schema.response import (
    AgentResponse,
    CodeMonkeyResponseType,
)
from rustify.schema.translation import (
    ModuleTranslation,
    TranslationTask,
    TranslationTaskStatus,
    TranslationTaskTarget,
)
from rustify.state.state_manager import StateManager


class CodeMonkey(BaseAgent):
    """
    Code Monkey agent responsible for:
    - Translating C/C++ code to Rust
    - Fixing compilation errors
    - Evaluating translation quality
    """
    
    ROLE = "code_monkey"
    DESCRIPTION = "An AI assistant specialized in translating C/C++ code to Rust."
    
    MAX_FIX_ATTEMPTS = 10
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        reasoner_config: Optional[dict] = None,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Code Monkey.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            reasoner_config: Reasoner LLM configuration.
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.reasoner_config = reasoner_config or llm_config
        self.reasoner = Reasoner(self.reasoner_config, logger=self.logger)
        
        # Translation memory
        self.memory = TranspileMemory()
        
        # Current task info
        self.current_module_id: Optional[str] = None
        self.current_task_id: Optional[str] = None
    
    @property
    def module(self) -> Optional[ModuleTranslation]:
        """Get current module."""
        if self.current_module_id:
            return self.state_manager.get_module_translation_by_id(self.current_module_id)
        return None
    
    @property
    def task(self) -> Optional[TranslationTask]:
        """Get current task."""
        if self.module and self.current_task_id:
            return self.module.get_translation_task_by_id(self.current_task_id)
        return None
    
    def translate(
        self,
        module_id: str,
        task: TranslationTask
    ) -> AgentResponse:
        """
        Translate a single task.
        
        Args:
            module_id: Module ID.
            task: Translation task.
            
        Returns:
            AgentResponse indicating status.
        """
        self.current_module_id = module_id
        self.current_task_id = task.id
        
        self.logger.info(f"Translating: {task.source.name}")
        
        # Update task status
        self.state_manager.update_translation_task_status(
            module_id,
            task.id,
            TranslationTaskStatus.TRANSLATING
        )
        
        # Step 1: Perform translation
        response = self.translate_with_reasoner()
        if response.status != "done":
            return response
        
        # Step 2: Check compilation
        response = self.check_and_fix()
        
        return response
    
    def translate_with_reasoner(self) -> AgentResponse:
        """Translate the current task using the reasoner."""
        task = self.task
        if not task:
            return AgentResponse.error(
                self,
                CodeMonkeyResponseType.TRANSLATION_COMPLETION_FAILED,
                {"message": "No task selected"}
            )
        
        # Collect source code
        source_code_parts = []
        node_type = "mixed"
        
        for node in task.source.nodes:
            source_code_parts.append(f"// From: {node.filepath}")
            source_code_parts.append(f"// Type: {node.type}, Name: {node.name}")
            source_code_parts.append(node.text)
            source_code_parts.append("")
            node_type = node.type
        
        source_code = "\n".join(source_code_parts)
        
        # Get context from memory
        context = self.memory.to_context(node_type)
        
        # Build translation prompt
        prompt = self._build_translation_prompt(source_code, node_type, context)
        
        # Call reasoner for translation
        rust_code = self.reasoner.suggest_translation(
            source_code,
            node_type,
            context
        )
        
        if not rust_code:
            return AgentResponse.error(
                self,
                CodeMonkeyResponseType.TRANSLATION_COMPLETION_FAILED,
                {"message": "Translation returned empty result"}
            )
        
        # Check code completeness and fix if needed
        rust_code = self._ensure_code_completeness(rust_code, source_code)
        
        # Set translation target
        target = TranslationTaskTarget(
            name=task.source.name,
            type="rust_module",
            text=rust_code,
            description=f"Translated from {task.source.name}",
            filepath=self._get_target_filepath(task)
        )
        
        self.state_manager.set_translation_task_target(
            self.current_module_id,
            task.id,
            target
        )
        
        # Write to file
        self._write_translation(target)
        
        # Add to memory
        self.memory.add_experience(
            source_code=source_code,
            target_code=rust_code,
            source_type=node_type,
            success=True
        )
        
        return AgentResponse.done(
            self,
            CodeMonkeyResponseType.TRANSLATION_COMPLETION,
            {"rust_code": rust_code}
        )
    
    def _is_code_complete(self, code: str) -> bool:
        """
        Check if the Rust code is syntactically complete.
        
        Checks for balanced braces, brackets, and parentheses.
        """
        # Count delimiters
        open_braces = code.count('{')
        close_braces = code.count('}')
        open_parens = code.count('(')
        close_parens = code.count(')')
        open_brackets = code.count('[')
        close_brackets = code.count(']')
        
        # Check balance
        if open_braces != close_braces:
            return False
        if open_parens != close_parens:
            return False
        if open_brackets != close_brackets:
            return False
        
        # Check for obvious truncation patterns
        stripped = code.strip()
        if stripped.endswith('///') or stripped.endswith('//'):
            return False
        if stripped.endswith('*'):
            return False
        
        return True
    
    def _ensure_code_completeness(
        self,
        rust_code: str,
        source_code: str,
        max_attempts: int = 3
    ) -> str:
        """
        Ensure the translated code is complete.
        
        If the code appears truncated, request the LLM to complete it.
        """
        if self._is_code_complete(rust_code):
            return rust_code
        
        self.logger.warning("Detected incomplete code, requesting completion...")
        
        for attempt in range(max_attempts):
            prompt = f"""The following Rust code appears to be truncated/incomplete.
Please complete the code to make it syntactically valid:

## Incomplete Rust Code
```rust
{rust_code}
```

## Original C Source (for reference)
```c
{source_code}
```

IMPORTANT: 
1. Return the COMPLETE code, not just the missing parts
2. Ensure all braces {{ }} are properly closed
3. Ensure all impl blocks and functions are complete

Provide the complete code in a ```rust code block.
"""
            
            response = self.reasoner.call_llm(
                [{"role": "user", "content": prompt}],
                temperature=0.3  # Lower temperature for more consistent output
            )
            
            completed_code = self.extract_rust_code(response.content)
            
            if completed_code and self._is_code_complete(completed_code):
                self.logger.info("Code completion successful")
                return completed_code
            
            if completed_code:
                rust_code = completed_code  # Use the new code for next attempt
        
        self.logger.warning("Could not complete code after max attempts, using last version")
        return rust_code
    
    def _build_translation_prompt(
        self,
        source_code: str,
        node_type: str,
        context: str
    ) -> str:
        """Build the translation prompt."""
        return f"""Translate the following C/C++ code to idiomatic Rust:

## Source Code (C/C++)
```c
{source_code}
```

## Node Type
{node_type}

{context if context else ""}

## Translation Guidelines
1. Use idiomatic Rust patterns
2. Replace raw pointers with safe Rust types where possible
3. Use Result/Option for error handling
4. Add documentation comments
5. Use appropriate visibility (pub, pub(crate), etc.)
6. Handle unsafe code properly with `unsafe` blocks when necessary

Provide only the Rust code in a ```rust code block.
"""
    
    def _get_target_filepath(self, task: TranslationTask) -> str:
        """
        Generate target filepath for the translation.
        
        Rust module names cannot contain hyphens, so we convert them to underscores.
        e.g., 'avl-tree.c' -> 'src/avl_tree.rs'
        """
        if task.source.nodes:
            source_path = task.source.nodes[0].filepath
            # Convert .c/.cpp to .rs
            base = os.path.splitext(os.path.basename(source_path))[0]
            # Rust module names must use underscores, not hyphens
            base = base.replace("-", "_").replace(".", "_")
            return f"src/{base}.rs"
        return "src/lib.rs"
    
    def _write_translation(self, target: TranslationTaskTarget) -> None:
        """Write the translation to file."""
        module = self.module
        if not module or not self.state_manager.state.target_project:
            return
        
        target_path = os.path.join(
            self.state_manager.state.target_project.path,
            target.filepath
        )
        
        os.makedirs(os.path.dirname(target_path), exist_ok=True)
        
        with open(target_path, "w", encoding="utf-8") as f:
            f.write(target.text)
        
        self.logger.info(f"Written translation to: {target.filepath}")
        
        # Track the rust file
        self.state_manager.add_module_rust_files(
            self.current_module_id,
            [target.filepath]
        )
        
        # Update lib.rs to export this module
        self._update_lib_rs(target.filepath)
    
    def _update_lib_rs(self, rust_filepath: str) -> None:
        """
        Update lib.rs to include the new module.
        
        Args:
            rust_filepath: Path to the new Rust file (e.g., 'src/tinyexpr.rs')
        """
        if not self.state_manager.state.target_project:
            return
        
        # Skip if it's lib.rs or main.rs itself
        base_name = os.path.basename(rust_filepath)
        if base_name in ("lib.rs", "main.rs"):
            return
        
        # Extract module name from filepath (e.g., 'src/tinyexpr.rs' -> 'tinyexpr')
        module_name = os.path.splitext(base_name)[0]
        
        # Make it a valid Rust identifier
        module_name = module_name.replace("-", "_").replace(".", "_")
        
        lib_rs_path = os.path.join(
            self.state_manager.state.target_project.path,
            "src",
            "lib.rs"
        )
        
        # Read existing lib.rs content
        if os.path.exists(lib_rs_path):
            with open(lib_rs_path, "r", encoding="utf-8") as f:
                lib_content = f.read()
        else:
            lib_content = "//! Auto-generated library root\n\n"
        
        # Check if module is already declared
        mod_declaration = f"pub mod {module_name};"
        if mod_declaration in lib_content:
            return
        
        # Add module declaration
        # Insert after any existing mod declarations or at the end
        lines = lib_content.split("\n")
        
        # Find the last mod declaration line
        last_mod_line = -1
        for i, line in enumerate(lines):
            if line.strip().startswith("pub mod ") or line.strip().startswith("mod "):
                last_mod_line = i
        
        if last_mod_line >= 0:
            # Insert after the last mod declaration
            lines.insert(last_mod_line + 1, mod_declaration)
        else:
            # No existing mod declarations, add after header comments
            insert_pos = 0
            for i, line in enumerate(lines):
                if not line.strip().startswith("//") and line.strip():
                    insert_pos = i
                    break
                insert_pos = i + 1
            lines.insert(insert_pos, mod_declaration)
            if insert_pos == 0 or (insert_pos > 0 and lines[insert_pos - 1].strip()):
                lines.insert(insert_pos, "")  # Add blank line before
        
        new_content = "\n".join(lines)
        
        with open(lib_rs_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        
        self.logger.info(f"Updated lib.rs: added '{mod_declaration}'")
    
    def check_and_fix(self) -> AgentResponse:
        """Check compilation and fix errors if needed."""
        task = self.task
        if not task or not task.target:
            return AgentResponse.error(
                self,
                CodeMonkeyResponseType.COMPILE_CHECK_FAILED,
                {"message": "No translation to check"}
            )
        
        from rustify.tools.rust_utils import cargo_check
        
        target_path = self.state_manager.state.target_project.path
        
        attempt = 0
        while attempt < self.MAX_FIX_ATTEMPTS:
            attempt += 1
            self.logger.info(f"Compile check attempt {attempt}/{self.MAX_FIX_ATTEMPTS}")
            
            # Run cargo check
            result = cargo_check(target_path)
            
            if result["success"]:
                self.logger.info("Compilation successful!")
                
                # Update task status
                self.state_manager.update_translation_task_status(
                    self.current_module_id,
                    task.id,
                    TranslationTaskStatus.DONE
                )
                
                return AgentResponse.done(
                    self,
                    CodeMonkeyResponseType.TRANSLATION_TASK_DONE
                )
            
            # Need to fix errors
            errors = result.get("errors", [])
            self.logger.warning(f"Compilation failed with {len(errors)} errors")
            
            # Update status to fixing
            self.state_manager.update_translation_task_status(
                self.current_module_id,
                task.id,
                TranslationTaskStatus.FIXING
            )
            
            # Try to fix
            fix_response = self.fix_with_reasoner(errors)
            
            if fix_response.status != "done":
                break
        
        # Max attempts reached
        self.logger.error("Max fix attempts reached")
        
        self.state_manager.update_translation_task_status(
            self.current_module_id,
            task.id,
            TranslationTaskStatus.FAILED
        )
        
        return AgentResponse.error(
            self,
            CodeMonkeyResponseType.FIX_FAILED,
            {"message": "Max fix attempts reached"}
        )
    
    def fix_with_reasoner(self, errors: List[dict]) -> AgentResponse:
        """Fix compilation errors using the reasoner."""
        task = self.task
        if not task or not task.target:
            return AgentResponse.error(
                self,
                CodeMonkeyResponseType.FIX_FAILED,
                {"message": "No task to fix"}
            )
        
        # Limit errors
        if len(errors) > 10:
            errors = errors[:10]
        
        # Format errors
        error_messages = []
        for error in errors:
            rendered = error.get("rendered", str(error))
            error_messages.append(rendered)
        
        error_text = "\n\n".join(error_messages)
        
        # Read current code
        target_path = os.path.join(
            self.state_manager.state.target_project.path,
            task.target.filepath
        )
        
        with open(target_path, "r", encoding="utf-8") as f:
            current_code = f.read()
        
        # Build fix prompt
        prompt = f"""Fix the following Rust compilation errors:

## Errors
{error_text}

## Current Code
```rust
{current_code}
```

Provide the corrected code in a ```rust code block.
Explain your fixes briefly before the code.
"""
        
        # Call reasoner default temperature 1.0, 0.2 is too low
        response = self.reasoner.call_llm([{"role": "user", "content": prompt}], temperature=1.0)
        
        # Extract fixed code
        fixed_code = self.extract_rust_code(response.content)
        
        if not fixed_code:
            return AgentResponse.error(
                self,
                CodeMonkeyResponseType.FIX_FAILED,
                {"message": "No fixed code extracted"}
            )
        
        # Write fixed code
        with open(target_path, "w", encoding="utf-8") as f:
            f.write(fixed_code)
        
        # Update target
        task.target.text = fixed_code
        
        return AgentResponse.done(
            self,
            CodeMonkeyResponseType.COMPILE_CHECK_DONE
        )

