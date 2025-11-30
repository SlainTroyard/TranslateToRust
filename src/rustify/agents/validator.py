"""
Validator Agent - Handles test translation and execution.

Features:
- Double-loop error fixing (syntax + logic separation)
- State rollback mechanism
- Line-level code modification
- Deep analysis for logic errors
"""

import os
import re
from typing import Optional, List, Dict, Any
import logging
import copy

from rustify.agents.base import BaseAgent
from rustify.agents.analyzer import Analyzer
from rustify.schema.response import (
    AgentResponse,
    ValidatorResponseType,
)
from rustify.schema.translation import ModuleTranslation
from rustify.state.state_manager import StateManager
from rustify.tools.patch import (
    extract_code_block_changes,
    apply_file_changes,
    CHANGE_BLOCK_FORMAT_PROMPT,
)


class Validator(BaseAgent):
    """
    Validator agent responsible for:
    - Finding and translating test files
    - Running tests with double-loop error fixing
    - Syntax error fixing (inner loop)
    - Logic error fixing with deep analysis (outer loop)
    - State rollback on persistent failures
    """
    
    ROLE = "validator"
    DESCRIPTION = "An AI assistant specialized in validating and testing translated Rust code."
    
    # Default limits (can be overridden via config)
    DEFAULT_SYNTAX_FIX_ATTEMPTS = 20  # Inner loop limit
    DEFAULT_LOGIC_FIX_ATTEMPTS = 10   # Outer loop limit
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        analyzer_config: Optional[dict] = None,
        *,
        max_syntax_attempts: Optional[int] = None,
        max_logic_attempts: Optional[int] = None,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Validator.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            analyzer_config: Analyzer LLM configuration (for deep analysis).
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.analyzer_config = analyzer_config or llm_config
        self.analyzer = Analyzer(self.analyzer_config, logger=self.logger)
        
        # Configurable limits
        self.max_syntax_attempts = max_syntax_attempts or self.DEFAULT_SYNTAX_FIX_ATTEMPTS
        self.max_logic_attempts = max_logic_attempts or self.DEFAULT_LOGIC_FIX_ATTEMPTS
        
        # Current module
        self.current_module_id: Optional[str] = None
        
        # Test state
        self.source_test_files: List[str] = []
        self.test_file: Optional[str] = None
        self.test_name: Optional[str] = None
        
        # Attempt counters
        self.syntax_attempts = 0
        self.logic_attempts = 0
        
        # History for rollback - stores complete states
        self.state_history: List[Dict[str, Any]] = []
        self.best_state: Optional[Dict[str, Any]] = None
        self.best_failed_count: int = float('inf')
    
    @property
    def module(self) -> Optional[ModuleTranslation]:
        """Get current module."""
        if self.current_module_id:
            return self.state_manager.get_module_translation_by_id(self.current_module_id)
        return None
    
    def start(self, module: ModuleTranslation) -> AgentResponse:
        """
        Start test translation and execution for a module.
        
        Implements double-loop error fixing:
        - Outer loop: Logic error fixing
        - Inner loop: Syntax error fixing
        
        Args:
            module: Module to test.
            
        Returns:
            AgentResponse indicating status.
        """
        self.logger.info(f"Starting test validation for module: {module.name}")
        self.current_module_id = module.id
        self._reset_state()
        
        # Find test files
        self.source_test_files = self._find_source_tests()
        
        if not self.source_test_files:
            self.logger.info("No test files found, skipping test phase")
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_PASSED,
                {"message": "No tests found"}
            )
        
        # Translate test code
        response = self._translate_test_code()
        if response.status != "done":
            return response
        
        # Save initial state
        self._save_state("initial")
        
        # ========== OUTER LOOP: Logic Error Fixing ==========
        while self.logic_attempts < self.max_logic_attempts:
            self.logger.info(f"Logic fix attempt {self.logic_attempts + 1}/{self.max_logic_attempts}")
            
            # Reset syntax counter for each logic iteration
            self.syntax_attempts = 0
            
            # ========== INNER LOOP: Syntax Error Fixing ==========
            syntax_result = self._syntax_fix_loop()
            
            if syntax_result is None:
                # Syntax fix loop exhausted without success
                self.logger.warning("Syntax fix loop exhausted, trying logic fix")
                self.logic_attempts += 1
                
                # Rollback to best state before continuing
                if self.best_state:
                    self._restore_state(self.best_state)
                    self.logger.info("Rolled back to best state before logic fix")
                continue
            
            if syntax_result.type == ValidatorResponseType.TEST_PASSED:
                self.logger.info("All tests passed!")
                return syntax_result
            
            # Tests compiled but some failed - this is a logic error
            failed_tests = syntax_result.data.get("failed_tests", [])
            failed_count = len(failed_tests)
            
            self.logger.info(f"Tests ran with {failed_count} failures")
            
            # Save state and track best
            self._save_state(f"logic_attempt_{self.logic_attempts}", failed_tests)
            
            if failed_count < self.best_failed_count:
                self.best_failed_count = failed_count
                self.best_state = self._capture_state()
                self.logger.info(f"New best state: {failed_count} failed tests")
            
            # Try to fix logic errors using deep analysis
            self.logic_attempts += 1
            fix_response = self._fix_logic_errors_deep(failed_tests)
            
            if fix_response.status != "done":
                self.logger.warning("Logic fix failed")
                # Rollback to best state
                if self.best_state:
                    self._restore_state(self.best_state)
        
        # ========== Max attempts reached ==========
        self.logger.error("Max logic fix attempts reached")
        
        # Restore best state if available
        if self.best_state:
            self._restore_state(self.best_state)
            self.logger.info(f"Restored best state with {self.best_failed_count} failed tests")
        
        return AgentResponse.error(
            self,
            ValidatorResponseType.TEST_FAILED,
            {
                "message": "Max logic fix attempts reached",
                "best_failed_count": self.best_failed_count
            }
        )
    
    def _reset_state(self) -> None:
        """Reset all state tracking."""
        self.syntax_attempts = 0
        self.logic_attempts = 0
        self.state_history.clear()
        self.best_state = None
        self.best_failed_count = float('inf')
    
    def _syntax_fix_loop(self) -> Optional[AgentResponse]:
        """
        Inner loop: Fix syntax/compilation errors.
        
        Returns:
            AgentResponse if tests ran (passed or with logic errors),
            None if syntax fix loop exhausted.
        """
        while self.syntax_attempts < self.max_syntax_attempts:
            response = self._run_test()
            
            if response.type == ValidatorResponseType.TEST_PASSED:
                return response
            
            if response.type == ValidatorResponseType.TEST_RUN_DONE:
                # Tests compiled and ran (but may have failures)
                return response
            
            if response.type == ValidatorResponseType.TEST_RUN_FAILURE:
                # Compilation error - try to fix
                self.syntax_attempts += 1
                self.logger.info(f"Syntax fix attempt {self.syntax_attempts}/{self.max_syntax_attempts}")
                
                errors = response.data.get("errors", [])
                fix_response = self._fix_syntax_errors_precise(errors)
                
                if fix_response.status != "done":
                    self.logger.warning("Syntax fix failed to generate fix")
                    continue
        
        # Exhausted syntax attempts
        return None
    
    def _capture_state(self) -> Dict[str, Any]:
        """Capture current state of all relevant files."""
        module = self.module
        target_project = self.state_manager.state.target_project
        
        state = {
            "files": {},
            "timestamp": os.times().elapsed
        }
        
        # Capture module files
        if module:
            for rust_file in module.related_rust_files:
                try:
                    full_path = os.path.join(target_project.path, rust_file)
                    with open(full_path, "r", encoding="utf-8") as f:
                        state["files"][rust_file] = f.read()
                except Exception:
                    pass
        
        # Capture test file
        if self.test_file:
            try:
                test_path = os.path.join(target_project.path, self.test_file)
                with open(test_path, "r", encoding="utf-8") as f:
                    state["files"][self.test_file] = f.read()
            except Exception:
                pass
        
        return state
    
    def _save_state(self, label: str, failed_tests: List[dict] = None) -> None:
        """Save current state to history."""
        state = self._capture_state()
        state["label"] = label
        state["failed_tests"] = failed_tests or []
        state["failed_count"] = len(failed_tests) if failed_tests else 0
        
        self.state_history.append(state)
        self.logger.debug(f"Saved state: {label}")
    
    def _restore_state(self, state: Dict[str, Any]) -> None:
        """Restore files from a saved state."""
        target_project = self.state_manager.state.target_project
        
        for filepath, content in state.get("files", {}).items():
            try:
                full_path = os.path.join(target_project.path, filepath)
                os.makedirs(os.path.dirname(full_path), exist_ok=True)
                with open(full_path, "w", encoding="utf-8") as f:
                    f.write(content)
            except Exception as e:
                self.logger.warning(f"Failed to restore {filepath}: {e}")
        
        self.logger.info(f"Restored state: {state.get('label', 'unknown')}")
    
    def _find_source_tests(self) -> List[str]:
        """Find test files in the source project."""
        # First, check if test files were already identified
        if self.state_manager.state.test_files:
            self.logger.info(f"Using {len(self.state_manager.state.test_files)} test files from project analysis")
            return self.state_manager.state.test_files
        
        source_project = self.state_manager.state.source_project
        if not source_project:
            return []
        
        test_files = []
        
        for f in source_project.list_files():
            if not (f.path.endswith('.c') or f.path.endswith('.cpp')):
                continue
            
            basename = os.path.basename(f.path).lower()
            dirname = os.path.dirname(f.path).lower()
            
            is_test = False
            
            # Directory patterns
            if any(d in dirname.split('/') for d in ['test', 'tests', 'testing']):
                is_test = True
            
            # Filename patterns
            if basename.startswith(('test-', 'test_', 'tests-', 'tests_')):
                is_test = True
            
            name_without_ext = os.path.splitext(basename)[0]
            if name_without_ext.endswith(('_test', '-test', '_tests', '-tests')):
                is_test = True
            
            if is_test:
                test_files.append(f.path)
        
        self.logger.info(f"Found {len(test_files)} test files")
        return test_files
    
    def _translate_test_code(self) -> AgentResponse:
        """Translate test files to Rust."""
        if not self.source_test_files:
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_PREPARE_DONE
            )
        
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                ValidatorResponseType.TEST_CODE_TRANSLATION_COMPLETION,
                {"message": "No module selected"}
            )
        
        # Read source test files
        source_project = self.state_manager.state.source_project
        test_code_parts = []
        
        for test_file in self.source_test_files:
            try:
                full_path = os.path.join(source_project.path, test_file)
                with open(full_path, "r", encoding="utf-8", errors="replace") as f:
                    content = f.read()
                test_code_parts.append(f"// File: {test_file}\n{content}")
            except Exception as e:
                self.logger.warning(f"Failed to read test file {test_file}: {e}")
        
        source_test_code = "\n\n".join(test_code_parts)
        
        # Get Rust module code for context
        target_project = self.state_manager.state.target_project
        rust_code_parts = []
        
        for rust_file in module.related_rust_files:
            try:
                full_path = os.path.join(target_project.path, rust_file)
                with open(full_path, "r", encoding="utf-8") as f:
                    content = f.read()
                rust_code_parts.append(f"// File: {rust_file}\n{content}")
            except Exception as e:
                self.logger.warning(f"Failed to read rust file {rust_file}: {e}")
        
        rust_context = "\n\n".join(rust_code_parts)
        
        # Get crate name for correct imports
        crate_name = target_project.name
        
        # Determine available modules from lib.rs
        lib_rs_path = os.path.join(target_project.path, "src", "lib.rs")
        available_modules = []
        if os.path.exists(lib_rs_path):
            try:
                with open(lib_rs_path, "r", encoding="utf-8") as f:
                    for line in f:
                        if line.strip().startswith("pub mod "):
                            mod_name = line.strip().replace("pub mod ", "").replace(";", "").strip()
                            available_modules.append(mod_name)
            except Exception:
                pass
        
        # Build import examples
        import_examples = f"use {crate_name}::*;"
        if available_modules:
            import_examples += f"\n// Or for specific modules:\n"
            for mod_name in available_modules[:3]:  # Show first 3 as examples
                import_examples += f"// use {crate_name}::{mod_name}::*;\n"
        
        # Build prompt
        prompt = f"""Translate the following C/C++ test code to Rust tests:

## C/C++ Test Code
```c
{source_test_code}
```

## Rust Module Code (for reference)
```rust
{rust_context}
```

## Crate Information
- Crate name: `{crate_name}`
- Available modules: {available_modules if available_modules else "See lib.rs exports"}

## Import Examples
```rust
{import_examples}
```

## Requirements
1. Use Rust's #[test] attribute
2. Use assert!, assert_eq!, assert_ne! macros
3. **IMPORTANT**: Import from the correct crate: `{crate_name}` (NOT the original C library name)
4. Group tests in a tests module
5. Handle any setup/teardown logic

Provide the test code in a ```rust code block.
Start with: // filepath: tests/{module.name}_tests.rs
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        rust_test_code = self.extract_rust_code(response.content)
        
        if not rust_test_code:
            return AgentResponse.error(
                self,
                ValidatorResponseType.TEST_CODE_TRANSLATION_COMPLETION,
                {"message": "Failed to extract test code"}
            )
        
        # Parse filepath
        self.test_file = f"tests/{module.name}_tests.rs"
        if "// filepath:" in rust_test_code:
            first_line = rust_test_code.split("\n")[0]
            if "filepath:" in first_line:
                self.test_file = first_line.split("filepath:")[1].strip()
                rust_test_code = "\n".join(rust_test_code.split("\n")[1:])
        
        self.test_name = os.path.splitext(os.path.basename(self.test_file))[0]
        
        # Write test file
        test_path = os.path.join(target_project.path, self.test_file)
        os.makedirs(os.path.dirname(test_path), exist_ok=True)
        
        with open(test_path, "w", encoding="utf-8") as f:
            f.write(rust_test_code)
        
        self.logger.info(f"Written test file: {self.test_file}")
        
        return AgentResponse.done(
            self,
            ValidatorResponseType.TEST_CODE_TRANSLATION_COMPLETION
        )
    
    def _run_test(self) -> AgentResponse:
        """Run the tests."""
        from rustify.tools.rust_utils import (
            cargo_test, 
            auto_add_missing_dependencies,
            detect_missing_crates
        )
        
        target_project = self.state_manager.state.target_project
        
        result = cargo_test(target_project.path, self.test_name)
        
        if result["success"]:
            failed_tests = result.get("failed_tests", [])
            
            if not failed_tests:
                self.logger.info("All tests passed!")
                return AgentResponse.done(
                    self,
                    ValidatorResponseType.TEST_PASSED
                )
            
            # Tests ran but some failed
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_RUN_DONE,
                {"failed_tests": failed_tests}
            )
        else:
            # Compilation failed - first try auto-add dependencies
            errors = result.get("errors", [])
            missing_crates = detect_missing_crates(errors)
            
            if missing_crates:
                self.logger.info(f"Detected missing crates: {missing_crates}")
                added = auto_add_missing_dependencies(
                    target_project.path, 
                    errors, 
                    dev_only=True
                )
                if added:
                    self.logger.info(f"Auto-added dependencies: {added}")
                    # Retry after adding dependencies
                    result = cargo_test(target_project.path, self.test_name)
                    if result["success"]:
                        failed_tests = result.get("failed_tests", [])
                        if not failed_tests:
                            return AgentResponse.done(
                                self,
                                ValidatorResponseType.TEST_PASSED
                            )
                        return AgentResponse.done(
                            self,
                            ValidatorResponseType.TEST_RUN_DONE,
                            {"failed_tests": failed_tests}
                        )
                    errors = result.get("errors", [])
            
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_RUN_FAILURE,
                {"errors": errors}
            )
    
    def _fix_syntax_errors_precise(self, errors: List[dict]) -> AgentResponse:
        """
        Fix syntax/compilation errors using line-level modifications.
        
        Uses the TransFactor-style format: ```rust:filepath:start:end
        """
        target_project = self.state_manager.state.target_project
        module = self.module
        
        # Collect all relevant file contents with line numbers
        file_contents = {}
        
        # Test file
        if self.test_file:
            test_path = os.path.join(target_project.path, self.test_file)
            if os.path.exists(test_path):
                with open(test_path, "r", encoding="utf-8") as f:
                    file_contents[self.test_file] = f.read()
        
        # Module files
        if module:
            for rust_file in module.related_rust_files:
                full_path = os.path.join(target_project.path, rust_file)
                if os.path.exists(full_path):
                    with open(full_path, "r", encoding="utf-8") as f:
                        file_contents[rust_file] = f.read()
        
        # Format errors
        error_messages = [e.get("rendered", str(e)) for e in errors[:10]]
        error_text = "\n\n".join(error_messages)
        
        # Build context with line numbers
        context_parts = []
        for filepath, content in file_contents.items():
            lines = content.split("\n")
            numbered_lines = [f"{i+1:4d} | {line}" for i, line in enumerate(lines)]
            context_parts.append(f"### {filepath}\n```rust\n" + "\n".join(numbered_lines) + "\n```")
        
        file_context = "\n\n".join(context_parts)
        
        prompt = f"""Fix the following Rust compilation errors using PRECISE line-level changes.

## Compilation Errors
{error_text}

## Current Code (with line numbers)
{file_context}

{CHANGE_BLOCK_FORMAT_PROMPT}

Analyze each error carefully and provide the minimal changes needed to fix them.
Use the exact format above for each change.
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        
        # Extract and apply changes
        changes = extract_code_block_changes(response.content)
        
        if changes:
            modified = apply_file_changes(changes, target_project.path)
            self.logger.info(f"Applied changes to {len(modified)} files: {modified}")
            
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_SYNTAX_FIX_COMPLETION,
                {"modified_files": modified}
            )
        
        # Fallback: try full file replacement
        self.logger.warning("No precise changes found, trying full file replacement")
        return self._fix_syntax_errors_fallback(errors)
    
    def _fix_syntax_errors_fallback(self, errors: List[dict]) -> AgentResponse:
        """Fallback: fix syntax errors with full file replacement."""
        target_project = self.state_manager.state.target_project
        test_path = os.path.join(target_project.path, self.test_file)
        
        with open(test_path, "r", encoding="utf-8") as f:
            current_code = f.read()
        
        error_messages = [e.get("rendered", str(e)) for e in errors[:10]]
        error_text = "\n\n".join(error_messages)
        
        prompt = f"""Fix the following compilation errors in the Rust test code:

## Errors
{error_text}

## Test Code
```rust
{current_code}
```

Provide the COMPLETE corrected code in a ```rust code block.
"""
        
        response = self.analyzer.call_llm([{"role": "user", "content": prompt}])
        fixed_code = self.extract_rust_code(response.content)
        
        if not fixed_code:
            return AgentResponse.error(
                self,
                ValidatorResponseType.TEST_SYNTAX_FIX_COMPLETION,
                {"message": "No fixed code extracted"}
            )
        
        with open(test_path, "w", encoding="utf-8") as f:
            f.write(fixed_code)
        
        return AgentResponse.done(
            self,
            ValidatorResponseType.TEST_SYNTAX_FIX_COMPLETION
        )
    
    def _fix_logic_errors_deep(self, failed_tests: List[dict]) -> AgentResponse:
        """
        Fix logic errors using deep analysis with Analyzer.
        
        This is the outer loop fix that uses more sophisticated reasoning.
        """
        target_project = self.state_manager.state.target_project
        module = self.module
        
        # Read test code with line numbers
        test_path = os.path.join(target_project.path, self.test_file)
        with open(test_path, "r", encoding="utf-8") as f:
            test_code = f.read()
        
        test_lines = test_code.split("\n")
        numbered_test = "\n".join([f"{i+1:4d} | {line}" for i, line in enumerate(test_lines)])
        
        # Read module code with line numbers
        module_code_parts = []
        for rust_file in module.related_rust_files:
            full_path = os.path.join(target_project.path, rust_file)
            with open(full_path, "r", encoding="utf-8") as f:
                content = f.read()
            lines = content.split("\n")
            numbered = "\n".join([f"{i+1:4d} | {line}" for i, line in enumerate(lines)])
            module_code_parts.append(f"### {rust_file}\n```rust\n{numbered}\n```")
        
        module_code = "\n\n".join(module_code_parts)
        
        # Format failed tests with details
        failed_info = []
        for t in failed_tests:
            test_name = t.get('name', 'unknown')
            error_msg = t.get('error', 'unknown error')
            stdout = t.get('stdout', '')
            failed_info.append(f"""
### Test: {test_name}
**Error**: {error_msg}
**Output**:
```
{stdout}
```
""")
        
        failed_details = "\n".join(failed_info)
        
        # Deep analysis prompt
        prompt = f"""Perform deep analysis of the following test failures and provide fixes.

## Failed Tests (with details)
{failed_details}

## Test Code (with line numbers)
```rust
{numbered_test}
```

## Module Implementation (with line numbers)
{module_code}

## Analysis Tasks

1. **Root Cause Analysis**: For each failed test, identify:
   - What the test expects vs what actually happens
   - Whether the bug is in the test or the implementation
   - The specific line(s) causing the issue

2. **Fix Strategy**: Determine the minimal changes needed:
   - If the implementation is wrong, fix the module code
   - If the test expectation is wrong, fix the test
   - Prefer fixing the implementation over the test when both are valid approaches

3. **Provide Changes**: Use precise line-level modifications.

{CHANGE_BLOCK_FORMAT_PROMPT}

Think step-by-step and provide your analysis before the code changes.
"""
        
        # Use Analyzer for deep reasoning
        response = self.analyzer.call_llm(
            [{"role": "user", "content": prompt}],
            temperature=0.7
        )
        
        # Extract and apply changes
        changes = extract_code_block_changes(response.content)
        
        if changes:
            modified = apply_file_changes(changes, target_project.path)
            self.logger.info(f"Deep analysis applied changes to: {modified}")
            
            return AgentResponse.done(
                self,
                ValidatorResponseType.TEST_LOGIC_FIX_COMPLETION,
                {"modified_files": modified}
            )
        
        # Fallback: try to extract any rust code blocks with filepath
        self.logger.warning("No precise changes found in deep analysis, trying fallback")
        return self._fix_logic_errors_fallback(failed_tests)
    
    def _fix_logic_errors_fallback(self, failed_tests: List[dict]) -> AgentResponse:
        """Fallback logic error fix with full file replacement."""
        target_project = self.state_manager.state.target_project
        module = self.module
        
        # Read test code
        test_path = os.path.join(target_project.path, self.test_file)
        with open(test_path, "r", encoding="utf-8") as f:
            test_code = f.read()
        
        # Read module code
        module_code_parts = []
        for rust_file in module.related_rust_files:
            full_path = os.path.join(target_project.path, rust_file)
            with open(full_path, "r", encoding="utf-8") as f:
                content = f.read()
            module_code_parts.append(f"// {rust_file}\n{content}")
        
        module_code = "\n\n".join(module_code_parts)
        
        # Format failed tests
        failed_info = "\n".join([
            f"- {t.get('name', 'unknown')}: {t.get('error', 'unknown error')}"
            for t in failed_tests
        ])
        
        prompt = f"""Some tests are failing. Analyze and fix the issues:

## Failed Tests
{failed_info}

## Test Code
```rust
{test_code}
```

## Module Code
```rust
{module_code}
```

Determine if the issue is in:
1. The test code itself
2. The module implementation

Provide fixes for the appropriate file(s).
Use code blocks with filepath comments:
// filepath: <path>
```rust
<code>
```
"""
        
        response = self.analyzer.call_llm([{"role": "user", "content": prompt}])
        
        # Files that should NOT be modified by LLM code changes
        PROTECTED_FILES = {"Cargo.toml", "Cargo.lock", "rust-toolchain.toml"}
        
        # Parse and apply fixes
        content = response.content
        blocks = re.findall(
            r'// filepath:\s*(\S+)\s*\n```rust\s*(.*?)\s*```',
            content,
            re.DOTALL
        )
        
        for filepath, code in blocks:
            # Skip protected files
            if os.path.basename(filepath.strip()) in PROTECTED_FILES:
                self.logger.warning(f"Skipping protected file: {filepath}")
                continue
            
            full_path = os.path.join(target_project.path, filepath.strip())
            if os.path.exists(full_path) or filepath.startswith("tests/"):
                os.makedirs(os.path.dirname(full_path), exist_ok=True)
                with open(full_path, "w", encoding="utf-8") as f:
                    f.write(code.strip())
                self.logger.info(f"Updated: {filepath}")
        
        return AgentResponse.done(
            self,
            ValidatorResponseType.TEST_LOGIC_FIX_COMPLETION
        )
