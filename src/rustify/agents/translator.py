"""
Translator Agent - Handles actual code translation and fixing.

"""

import os
import re
from typing import Optional, List
import logging

from rustify.agents.base import BaseAgent, TranslationMemory
from rustify.agents.analyzer import Analyzer
from rustify.schema.response import (
    AgentResponse,
    TranslatorResponseType,
)
from rustify.schema.translation import (
    ModuleTranslation,
    TranslationTask,
    TranslationTaskStatus,
    TranslationTaskTarget,
)
from rustify.state.state_manager import StateManager


class Translator(BaseAgent):
    """
    Translator agent responsible for:
    - Translating C/C++ code to Rust
    - Fixing compilation errors
    - Evaluating translation quality
    """
    
    ROLE = "translator"
    DESCRIPTION = "An AI assistant specialized in translating C/C++ code to Rust."
    
    DEFAULT_FIX_ATTEMPTS = 10
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        analyzer_config: Optional[dict] = None,
        *,
        max_fix_attempts: Optional[int] = None,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Translator.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            analyzer_config: Analyzer LLM configuration.
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.analyzer_config = analyzer_config or llm_config
        self.analyzer = Analyzer(self.analyzer_config, logger=self.logger)
        
        # Configurable limits
        self.max_fix_attempts = max_fix_attempts or self.DEFAULT_FIX_ATTEMPTS
        
        # Translation memory
        self.memory = TranslationMemory()
        
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
        response = self.translate_with_analyzer()
        if response.status != "done":
            return response
        
        # Step 2: Check compilation
        response = self.check_and_fix()
        
        return response
    
    def translate_with_analyzer(self) -> AgentResponse:
        """Translate the current task using the analyzer."""
        task = self.task
        if not task:
            return AgentResponse.error(
                self,
                TranslatorResponseType.TRANSLATION_COMPLETION_FAILED,
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
        
        # Collect header file context (types, macros, structs)
        header_context = self._collect_header_context(task)
        
        # Get context from memory
        memory_context = self.memory.to_context(node_type)
        
        # Combine contexts: header files + memory
        context = ""
        if header_context:
            context += f"## Header File Definitions\n{header_context}\n\n"
        if memory_context:
            context += memory_context
        
        # Build translation prompt
        prompt = self._build_translation_prompt(source_code, node_type, context)
        
        # Call analyzer for translation
        rust_code = self.analyzer.suggest_translation(
            source_code,
            node_type,
            context
        )
        
        if not rust_code:
            return AgentResponse.error(
                self,
                TranslatorResponseType.TRANSLATION_COMPLETION_FAILED,
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
            TranslatorResponseType.TRANSLATION_COMPLETION,
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
    
    def _cleanup_rust_code(self, code: str) -> str:
        """
        Clean up common issues in generated Rust code.
        
        Fixes:
        - Trailing doc comments without items (error: expected item after doc comment)
        - Empty doc comment lines
        - Multiple consecutive empty lines
        """
        lines = code.split('\n')
        cleaned_lines = []
        i = 0
        
        while i < len(lines):
            line = lines[i]
            stripped = line.strip()
            
            # Check if this is a doc comment
            if stripped.startswith('///'):
                # Collect all consecutive doc comment lines
                doc_block_start = i
                doc_lines = []
                
                while i < len(lines) and lines[i].strip().startswith('///'):
                    doc_lines.append(lines[i])
                    i += 1
                
                # Skip empty lines after doc comments
                while i < len(lines) and lines[i].strip() == '':
                    i += 1
                
                # Check if there's an item after the doc comment
                if i < len(lines):
                    next_line = lines[i].strip()
                    # Check if the next line is a valid item start
                    is_item = (
                        next_line.startswith('pub ') or
                        next_line.startswith('fn ') or
                        next_line.startswith('struct ') or
                        next_line.startswith('enum ') or
                        next_line.startswith('type ') or
                        next_line.startswith('const ') or
                        next_line.startswith('static ') or
                        next_line.startswith('impl ') or
                        next_line.startswith('trait ') or
                        next_line.startswith('mod ') or
                        next_line.startswith('use ') or
                        next_line.startswith('#[')  # attribute
                    )
                    
                    if is_item:
                        # Clean doc lines - remove empty /// lines
                        clean_doc_lines = []
                        for doc_line in doc_lines:
                            doc_stripped = doc_line.strip()
                            # Keep non-empty doc comments
                            if doc_stripped != '///' and doc_stripped != '/// ':
                                clean_doc_lines.append(doc_line)
                            elif clean_doc_lines:  # Keep empty lines in middle of doc block
                                # But not at the end
                                pass
                        
                        # Remove trailing empty doc lines
                        while clean_doc_lines and clean_doc_lines[-1].strip() in ('///', '/// '):
                            clean_doc_lines.pop()
                        
                        cleaned_lines.extend(clean_doc_lines)
                    else:
                        # No valid item follows - convert to regular comments
                        for doc_line in doc_lines:
                            # Convert /// to //
                            if doc_line.strip().startswith('///'):
                                indent = len(doc_line) - len(doc_line.lstrip())
                                comment_content = doc_line.strip()[3:]  # Remove ///
                                cleaned_lines.append(' ' * indent + '//' + comment_content)
                            else:
                                cleaned_lines.append(doc_line)
                else:
                    # End of file - convert to regular comments
                    for doc_line in doc_lines:
                        if doc_line.strip().startswith('///'):
                            indent = len(doc_line) - len(doc_line.lstrip())
                            comment_content = doc_line.strip()[3:]
                            cleaned_lines.append(' ' * indent + '//' + comment_content)
                        else:
                            cleaned_lines.append(doc_line)
                
                continue
            
            cleaned_lines.append(line)
            i += 1
        
        # Remove multiple consecutive empty lines
        result_lines = []
        prev_empty = False
        for line in cleaned_lines:
            is_empty = line.strip() == ''
            if is_empty and prev_empty:
                continue
            result_lines.append(line)
            prev_empty = is_empty
        
        return '\n'.join(result_lines)
    
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
        # First, clean up common Rust code issues
        rust_code = self._cleanup_rust_code(rust_code)
        
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
            
            response = self.analyzer.call_llm(
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
    
    def _collect_header_context(self, task: TranslationTask) -> str:
        """
        Collect relevant header file content for context.
        
        Uses Clang AST parser if available for precise dependency analysis,
        falls back to regex-based parsing otherwise.
        
        This ensures the LLM has access to type definitions, macros, and constants.
        """
        if not task.source.nodes:
            return ""
        
        # Get the source project path
        source_project = self.state_manager.state.source_project
        if not source_project:
            return ""
        
        project_path = source_project.path
        
        # Try to use Clang AST parser for precise analysis
        try:
            from rustify.graph.clang_parser import ClangASTParser
            
            parser = ClangASTParser(project_path)
            
            # Get context for each source file
            context_parts = []
            for node in task.source.nodes:
                source_file = node.filepath
                if not os.path.isabs(source_file):
                    source_file = os.path.join(project_path, source_file)
                
                # Get header context using Clang parser
                header_context = parser.get_header_context(source_file)
                context_str = header_context.to_context_string()
                
                if context_str:
                    context_parts.append(context_str)
            
            if context_parts:
                self.logger.info(f"Collected header context using {'Clang AST' if parser.is_clang_available else 'regex'} parser")
                return "\n\n".join(context_parts)
                
        except Exception as e:
            self.logger.debug(f"Clang parser failed, using fallback: {e}")
        
        # Fallback to original method
        return self._collect_header_context_fallback(task)
    
    def _collect_header_context_fallback(self, task: TranslationTask) -> str:
        """
        Fallback method to collect header context using regex parsing.
        """
        source_project = self.state_manager.state.source_project
        if not source_project:
            return ""
        
        project_path = source_project.path
        collected_headers = {}
        visited = set()
        
        # Start from each source file
        for node in task.source.nodes:
            source_file = node.filepath
            if not os.path.isabs(source_file):
                source_file = os.path.join(project_path, source_file)
            
            self._collect_headers_recursive(
                source_file, project_path, collected_headers, visited
            )
        
        if not collected_headers:
            return ""
        
        # Build context string
        context_parts = []
        for header_name, header_content in collected_headers.items():
            # Extract only important parts: types, structs, enums, macros, constants
            important_content = self._extract_important_definitions(header_content)
            if important_content:
                context_parts.append(f"### {header_name}\n```c\n{important_content}\n```")
        
        return "\n\n".join(context_parts)
    
    def _collect_headers_recursive(
        self,
        filepath: str,
        project_path: str,
        collected: dict,
        visited: set,
        depth: int = 0
    ) -> None:
        """Recursively collect header files from #include directives."""
        if depth > 5:  # Prevent infinite recursion
            return
        
        if filepath in visited:
            return
        visited.add(filepath)
        
        try:
            with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
        except Exception:
            return
        
        # Find #include directives for local headers (with quotes)
        include_pattern = re.compile(r'#include\s*"([^"]+)"')
        
        for match in include_pattern.finditer(content):
            include_path = match.group(1)
            
            # Try to resolve the header path
            header_paths = [
                os.path.join(os.path.dirname(filepath), include_path),
                os.path.join(project_path, include_path),
            ]
            
            for header_path in header_paths:
                if os.path.exists(header_path) and header_path not in visited:
                    # Read header content
                    try:
                        with open(header_path, "r", encoding="utf-8", errors="replace") as f:
                            header_content = f.read()
                        
                        header_name = os.path.basename(header_path)
                        collected[header_name] = header_content
                        
                        # Recursively collect headers included by this header
                        self._collect_headers_recursive(
                            header_path, project_path, collected, visited, depth + 1
                        )
                    except Exception:
                        pass
                    break
    
    def _extract_important_definitions(self, content: str) -> str:
        """
        Extract important definitions from header content.
        
        Focuses on: structs, enums, typedefs, #define macros, function declarations
        """
        lines = content.split('\n')
        result_lines = []
        in_block = False
        block_lines = []
        brace_count = 0
        
        for line in lines:
            stripped = line.strip()
            
            # Skip empty lines and comments when not in a block
            if not in_block:
                if not stripped or stripped.startswith('//'):
                    continue
                # Skip license headers
                if stripped.startswith('/*') and ('Copyright' in line or 'LICENSE' in line):
                    continue
            
            # Check for struct/enum/typedef start
            if not in_block and (
                stripped.startswith('typedef ') or
                stripped.startswith('struct ') or
                stripped.startswith('enum ') or
                stripped.startswith('#define ') or
                # Function declarations (ending with ;)
                (re.match(r'^(extern\s+)?\w+[\s\*]+\w+\s*\([^)]*\)\s*;', stripped))
            ):
                # Handle #define (might be multiline)
                if stripped.startswith('#define '):
                    result_lines.append(line)
                    while stripped.endswith('\\'):
                        # Continue reading multiline macro
                        idx = lines.index(line) + 1
                        if idx < len(lines):
                            line = lines[idx]
                            stripped = line.strip()
                            result_lines.append(line)
                    continue
                
                # Check if it's a single-line declaration
                if stripped.endswith(';') and '{' not in stripped:
                    result_lines.append(line)
                    continue
                
                # Start of a block
                in_block = True
                block_lines = [line]
                brace_count = line.count('{') - line.count('}')
                
                if brace_count == 0 and '{' in line:
                    # Complete single-line block
                    result_lines.append(line)
                    in_block = False
                    block_lines = []
                continue
            
            if in_block:
                block_lines.append(line)
                brace_count += line.count('{') - line.count('}')
                
                if brace_count <= 0 and (stripped.endswith(';') or stripped.endswith('}')):
                    # End of block
                    result_lines.extend(block_lines)
                    result_lines.append('')  # Add empty line after block
                    in_block = False
                    block_lines = []
                    brace_count = 0
        
        # Limit the size to avoid overwhelming the context
        result = '\n'.join(result_lines)
        if len(result) > 15000:  # Limit to ~15KB
            result = result[:15000] + "\n// ... (truncated)"
        
        return result

    def _build_translation_prompt(
        self,
        source_code: str,
        node_type: str,
        context: str
    ) -> str:
        """Build the translation prompt."""
        context_section = ""
        if context:
            context_section = f"""
## Context Information

{context}

IMPORTANT: Use the type definitions, structs, and constants from the header files above 
when translating. These define the types used in the source code.
"""
        
        return f"""Translate the following C/C++ code to idiomatic Rust:
{context_section}
## Source Code (C/C++) to Translate
```c
{source_code}
```

## Node Type
{node_type}

## Translation Guidelines
1. Use idiomatic Rust patterns
2. Replace raw pointers with safe Rust types where possible
3. Use Result/Option for error handling
4. Add documentation comments
5. Use appropriate visibility (pub, pub(crate), etc.)
6. Handle unsafe code properly with `unsafe` blocks when necessary
7. Translate C structs to Rust structs with appropriate #[derive] attributes
8. Translate C enums to Rust enums
9. Translate #define constants to const or enum values

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
        """Check compilation and fix errors if needed, then run tests."""
        task = self.task
        if not task or not task.target:
            return AgentResponse.error(
                self,
                TranslatorResponseType.COMPILE_CHECK_FAILED,
                {"message": "No translation to check"}
            )
        
        from rustify.tools.rust_utils import (
            cargo_check,
            cargo_test,
            add_detected_dependencies,
            auto_add_missing_dependencies,
        )
        
        target_path = self.state_manager.state.target_project.path
        
        # First, auto-detect and add dependencies before checking
        self.logger.info("Scanning for required dependencies...")
        added_deps = add_detected_dependencies(target_path)
        if added_deps:
            self.logger.info(f"Auto-added dependencies: {added_deps}")
        
        # Phase 1: Cargo build/check
        build_success = self._run_build_fix_loop(target_path, cargo_check, auto_add_missing_dependencies)
        
        if not build_success:
            self.logger.error("Max build fix attempts reached")
            self.state_manager.update_translation_task_status(
                self.current_module_id,
                task.id,
                TranslationTaskStatus.FAILED
            )
            return AgentResponse.error(
                self,
                TranslatorResponseType.FIX_FAILED,
                {"message": "Max build fix attempts reached"}
            )
        
        # Phase 2: Cargo test
        self.logger.info("Build successful! Running tests...")
        test_success = self._run_test_fix_loop(target_path, cargo_test, auto_add_missing_dependencies)
        
        if not test_success:
            self.logger.warning("Tests failed after max attempts, but build succeeded")
            # Still mark as done since build passed
        
        # Update task status
        self.state_manager.update_translation_task_status(
            self.current_module_id,
            task.id,
            TranslationTaskStatus.DONE
        )
        
        return AgentResponse.done(
            self,
            TranslatorResponseType.TRANSLATION_TASK_DONE
        )
    
    def _run_build_fix_loop(self, target_path: str, cargo_check, auto_add_missing_dependencies) -> bool:
        """Run cargo check with fix loop. Returns True if successful."""
        attempt = 0
        while attempt < self.max_fix_attempts:
            attempt += 1
            self.logger.info(f"Build check attempt {attempt}/{self.max_fix_attempts}")
            
            # Run cargo check
            result = cargo_check(target_path)
            
            if result["success"]:
                self.logger.info("Compilation successful!")
                return True
            
            # Need to fix errors
            errors = result.get("errors", [])
            self.logger.warning(f"Compilation failed with {len(errors)} errors")
            
            # Try to auto-add missing dependencies from errors
            auto_added = auto_add_missing_dependencies(target_path, errors, dev_only=False)
            if auto_added:
                self.logger.info(f"Auto-added missing dependencies from errors: {auto_added}")
                # Retry compilation after adding dependencies
                result = cargo_check(target_path)
                if result["success"]:
                    self.logger.info("Compilation successful after adding dependencies!")
                    return True
                # Update errors for fixing
                errors = result.get("errors", [])
            
            # Update status to fixing
            task = self.task
            if task:
                self.state_manager.update_translation_task_status(
                    self.current_module_id,
                    task.id,
                    TranslationTaskStatus.FIXING
                )
            
            # Try to fix
            fix_response = self.fix_with_analyzer(errors)
            
            if fix_response.status != "done":
                break
        
        return False
    
    def _run_test_fix_loop(
        self, 
        target_path: str, 
        cargo_test, 
        auto_add_missing_dependencies,
        is_integration_test: bool = False
    ) -> bool:
        """
        Run cargo test with fix loop. Returns True if successful.
        
        Args:
            target_path: Path to the Rust project
            cargo_test: Function to run cargo test
            auto_add_missing_dependencies: Function to auto-add deps
            is_integration_test: If True, we're testing integration tests (tests/*.rs),
                                 if False, we're testing src/*.rs inline tests
        """
        attempt = 0
        while attempt < self.max_fix_attempts:
            attempt += 1
            self.logger.info(f"Test check attempt {attempt}/{self.max_fix_attempts}")
            
            # Run cargo test
            result = cargo_test(target_path)
            
            if result["success"]:
                self.logger.info("All tests passed!")
                return True
            
            # Check for compilation errors first
            errors = result.get("errors", [])
            failed_tests = result.get("failed_tests", [])
            
            if errors:
                self.logger.warning(f"Test compilation failed with {len(errors)} errors")
                
                # Try to auto-add missing dependencies (dev-dependencies for tests)
                auto_added = auto_add_missing_dependencies(target_path, errors, dev_only=True)
                if auto_added:
                    self.logger.info(f"Auto-added test dependencies: {auto_added}")
                    continue
                
                # Try to fix compilation errors
                fix_response = self.fix_with_analyzer(errors)
                if fix_response.status != "done":
                    break
            elif failed_tests:
                self.logger.warning(f"Tests failed: {len(failed_tests)} test(s)")
                
                # Try to fix failed tests
                fix_response = self._fix_failed_tests(
                    failed_tests, 
                    target_path,
                    is_integration_test=is_integration_test
                )
                if fix_response.status != "done":
                    break
            else:
                # Unknown failure
                self.logger.warning("Tests failed with unknown error")
                break
        
        return False
    
    def _fix_failed_tests(
        self, 
        failed_tests: list, 
        target_path: str,
        is_integration_test: bool = False
    ) -> AgentResponse:
        """
        Try to fix failed tests.
        
        Args:
            failed_tests: List of failed test info
            target_path: Path to the Rust project
            is_integration_test: If True, focus on tests/*.rs (integration tests)
                                 If False, focus on src/*.rs (inline tests + main code)
        
        Note: Tests may fail because:
        1. The test code itself is wrong
        2. The main code has bugs (e.g., using extern "C" for missing functions)
        3. Logic errors in the translated code
        """
        from rustify.tools.patch import (
            extract_code_block_changes,
            apply_file_changes,
            CHANGE_BLOCK_FORMAT_PROMPT,
        )
        
        # Format failed test info
        test_failures = []
        for test in failed_tests[:5]:  # Limit to 5 failures
            test_failures.append(f"- Test: {test.get('name', 'unknown')}")
            if 'message' in test:
                test_failures.append(f"  Error: {test['message']}")
            if 'stdout' in test:
                test_failures.append(f"  Output: {test['stdout'][:500]}")
        
        failure_info = "\n".join(test_failures)
        
        source_context = ""
        
        if is_integration_test:
            # Integration test phase: focus on tests/ directory
            # Also include src/*.rs for reference (to see the API being tested)
            self.logger.info("Fixing integration tests (tests/*.rs)")
            
            # Read tests/ directory (primary focus)
            tests_dir = os.path.join(target_path, "tests")
            if os.path.exists(tests_dir):
                for test_file in os.listdir(tests_dir):
                    if test_file.endswith(".rs"):
                        file_path = os.path.join(tests_dir, test_file)
                        try:
                            with open(file_path, "r", encoding="utf-8") as f:
                                source_context += f"\n// File: tests/{test_file}\n{f.read()}\n"
                        except Exception:
                            pass
            
            # Also read lib.rs for API reference
            lib_rs = os.path.join(target_path, "src", "lib.rs")
            if os.path.exists(lib_rs):
                try:
                    with open(lib_rs, "r", encoding="utf-8") as f:
                        source_context += f"\n// File: src/lib.rs (API reference)\n{f.read()}\n"
                except Exception:
                    pass
            
            instructions = """
## Instructions
1. Analyze why the integration tests are failing
2. Fix the TEST code in tests/*.rs
3. Make sure imports are correct (use the right crate name)
4. Make sure the API calls match the actual module exports
"""
        else:
            # Source translation phase: focus on src/*.rs (inline tests + main code)
            self.logger.info("Fixing src inline tests and main code (src/*.rs)")
            
            # Read SOURCE files (src/*.rs) - these contain inline tests (#[cfg(test)])
            src_dir = os.path.join(target_path, "src")
            if os.path.exists(src_dir):
                for src_file in os.listdir(src_dir):
                    if src_file.endswith(".rs"):
                        file_path = os.path.join(src_dir, src_file)
                        try:
                            with open(file_path, "r", encoding="utf-8") as f:
                                content = f.read()
                                # Include files that have tests or are likely related
                                if "#[test]" in content or "#[cfg(test)]" in content or len(content) < 5000:
                                    source_context += f"\n// File: src/{src_file}\n{content}\n"
                        except Exception:
                            pass
            
            instructions = """
## Instructions
1. Analyze why the tests are failing
2. **IMPORTANT**: The bug may be in the MAIN CODE, not just the test code
   - Check for `extern "C"` declarations that reference missing C functions
   - Check for incorrect logic in the main functions
   - Check for missing implementations
3. Fix the appropriate code (main code or test code)
4. If a function uses `extern "C"` to call a C function, replace it with the Rust implementation from this project
"""
        
        # Build fix prompt
        prompt = f"""Fix the following test failures in a Rust project:

## Failed Tests
{failure_info}

## Source Code
```rust
{source_context}
```
{instructions}
{CHANGE_BLOCK_FORMAT_PROMPT}
"""
        
        messages = [{"role": "user", "content": prompt}]
        response = self.call_llm(messages)
        
        if not response.content:
            return AgentResponse.error(
                self,
                TranslatorResponseType.FIX_FAILED,
                {"message": "Empty LLM response"}
            )
        
        # Extract and apply changes
        changes = extract_code_block_changes(response.content)
        
        if changes:
            applied = apply_file_changes(target_path, changes)
            if applied:
                self.logger.info(f"Applied {len(applied)} test fixes")
                return AgentResponse.done(
                    self,
                    TranslatorResponseType.FIX_APPLIED
                )
        
        return AgentResponse.error(
            self,
            TranslatorResponseType.FIX_FAILED,
            {"message": "Failed to extract or apply test fixes"}
        )
    
    def fix_with_analyzer(self, errors: List[dict]) -> AgentResponse:
        """
        Fix compilation errors using the analyzer with line-level precision.
        
        Uses TransFactor-style format: ```rust:filepath:start:end
        Falls back to full file replacement if precise changes fail.
        """
        from rustify.tools.patch import (
            extract_code_block_changes,
            apply_file_changes,
            CHANGE_BLOCK_FORMAT_PROMPT,
        )
        
        task = self.task
        if not task or not task.target:
            return AgentResponse.error(
                self,
                TranslatorResponseType.FIX_FAILED,
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
        
        # Read current code with line numbers
        target_path = os.path.join(
            self.state_manager.state.target_project.path,
            task.target.filepath
        )
        
        with open(target_path, "r", encoding="utf-8") as f:
            current_code = f.read()
        
        lines = current_code.split("\n")
        numbered_code = "\n".join([f"{i+1:4d} | {line}" for i, line in enumerate(lines)])
        
        # Build fix prompt with line-level format
        prompt = f"""Fix the following Rust compilation errors using PRECISE line-level changes.

## Compilation Errors
{error_text}

## Current Code ({task.target.filepath}) - with line numbers
```rust
{numbered_code}
```

{CHANGE_BLOCK_FORMAT_PROMPT}

Analyze each error carefully and provide the minimal changes needed.
Use the exact format above for line-level modifications.
"""
        
        # Call analyzer
        response = self.analyzer.call_llm([{"role": "user", "content": prompt}], temperature=1.0)
        
        # Try to extract and apply line-level changes
        changes = extract_code_block_changes(response.content)
        
        if changes:
            modified = apply_file_changes(changes, self.state_manager.state.target_project.path)
            if modified:
                self.logger.info(f"Applied line-level fixes to: {modified}")
                
                # Read updated code
                with open(target_path, "r", encoding="utf-8") as f:
                    task.target.text = f.read()
                
                return AgentResponse.done(
                    self,
                    TranslatorResponseType.COMPILE_CHECK_DONE,
                    {"modified_files": modified}
                )
        
        # Fallback: full file replacement
        self.logger.warning("No line-level changes found, using full file replacement")
        
        fixed_code = self.extract_rust_code(response.content)
        
        if not fixed_code:
            return AgentResponse.error(
                self,
                TranslatorResponseType.FIX_FAILED,
                {"message": "No fixed code extracted"}
            )
        
        # Clean up common Rust code issues
        fixed_code = self._cleanup_rust_code(fixed_code)
        
        # Write fixed code
        with open(target_path, "w", encoding="utf-8") as f:
            f.write(fixed_code)
        
        # Update target
        task.target.text = fixed_code
        
        return AgentResponse.done(
            self,
            TranslatorResponseType.COMPILE_CHECK_DONE
        )

