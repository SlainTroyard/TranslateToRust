"""
Test Engineer Agent - Handles test translation and execution.

"""

import os
import re
from typing import Optional, List
import logging

from rustify.agents.base import BaseAgent
from rustify.agents.reasoner import Reasoner
from rustify.schema.response import (
    AgentResponse,
    TestEngineerResponseType,
)
from rustify.schema.translation import ModuleTranslation
from rustify.state.state_manager import StateManager


class TestEngineer(BaseAgent):
    """
    Test Engineer agent responsible for:
    - Finding and translating test files
    - Running tests
    - Fixing test failures
    """
    
    ROLE = "test_engineer"
    DESCRIPTION = "An AI assistant specialized in translating and running tests."
    
    MAX_SYNTAX_FIX_ATTEMPTS = 20
    MAX_LOGIC_FIX_ATTEMPTS = 10
    
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
        Initialize the Test Engineer.
        
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
        
        # Current module
        self.current_module_id: Optional[str] = None
        
        # Test state
        self.source_test_files: List[str] = []
        self.test_file: Optional[str] = None
        self.test_name: Optional[str] = None
        
        # Attempt counters
        self.syntax_attempts = 0
        self.logic_attempts = 0
        
        # History for rollback
        self.change_history: List[dict] = []
    
    @property
    def module(self) -> Optional[ModuleTranslation]:
        """Get current module."""
        if self.current_module_id:
            return self.state_manager.get_module_translation_by_id(self.current_module_id)
        return None
    
    def start(self, module: ModuleTranslation) -> AgentResponse:
        """
        Start test translation and execution for a module.
        
        Args:
            module: Module to test.
            
        Returns:
            AgentResponse indicating status.
        """
        self.logger.info(f"Starting test translation for module: {module.name}")
        self.current_module_id = module.id
        self.syntax_attempts = 0
        self.logic_attempts = 0
        self.change_history.clear()
        
        # Find test files
        self.source_test_files = self._find_source_tests()
        
        if not self.source_test_files:
            self.logger.info("No test files found, skipping test phase")
            return AgentResponse.done(
                self,
                TestEngineerResponseType.TEST_PASSED,
                {"message": "No tests found"}
            )
        
        # Translate test code
        response = self._translate_test_code()
        if response.status != "done":
            return response
        
        # Run test loop
        while True:
            response = self._run_test()
            
            if response.type == TestEngineerResponseType.TEST_PASSED:
                return response
            
            if response.type == TestEngineerResponseType.TEST_RUN_FAILURE:
                # Syntax error
                if self.syntax_attempts >= self.MAX_SYNTAX_FIX_ATTEMPTS:
                    self.logger.error("Max syntax fix attempts reached")
                    return AgentResponse.error(
                        self,
                        TestEngineerResponseType.TEST_FAILED,
                        {"message": "Max syntax fix attempts reached"}
                    )
                
                self.syntax_attempts += 1
                fix_response = self._fix_syntax_errors(response.data.get("errors", []))
                if fix_response.status != "done":
                    return fix_response
            
            elif response.type == TestEngineerResponseType.TEST_RUN_DONE:
                # Logic error (tests failed)
                if self.logic_attempts >= self.MAX_LOGIC_FIX_ATTEMPTS:
                    self.logger.error("Max logic fix attempts reached")
                    self._save_best_state()
                    return AgentResponse.error(
                        self,
                        TestEngineerResponseType.TEST_FAILED,
                        {"message": "Max logic fix attempts reached"}
                    )
                
                self.logic_attempts += 1
                self.syntax_attempts = 0  # Reset syntax counter
                
                failed_tests = response.data.get("failed_tests", [])
                fix_response = self._fix_logic_errors(failed_tests)
                if fix_response.status != "done":
                    return fix_response
    
    def _find_source_tests(self) -> List[str]:
        """
        Find test files in the source project.
        
        Uses test files that were filtered during project analysis (stored in state.test_files)
        or searches in the test directory.
        """
        # First, check if test files were already identified during project analysis
        if self.state_manager.state.test_files:
            self.logger.info(f"Using {len(self.state_manager.state.test_files)} test files from project analysis")
            return self.state_manager.state.test_files
        
        source_project = self.state_manager.state.source_project
        if not source_project:
            return []
        
        test_files = []
        
        # Search in all project files for test patterns
        for f in source_project.list_files():
            if not (f.path.endswith('.c') or f.path.endswith('.cpp')):
                continue
            
            basename = os.path.basename(f.path).lower()
            dirname = os.path.dirname(f.path).lower()
            
            # Check if it's a test file by name or directory
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
                TestEngineerResponseType.TEST_PREPARE_DONE
            )
        
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                TestEngineerResponseType.TEST_CODE_TRANSLATION_COMPLETION,
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

## Requirements
1. Use Rust's #[test] attribute
2. Use assert!, assert_eq!, assert_ne! macros
3. Group tests in a tests module
4. Handle any setup/teardown logic

Provide the test code in a ```rust code block.
Start with: // filepath: tests/{module.name}_tests.rs
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        rust_test_code = self.extract_rust_code(response.content)
        
        if not rust_test_code:
            return AgentResponse.error(
                self,
                TestEngineerResponseType.TEST_CODE_TRANSLATION_COMPLETION,
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
            TestEngineerResponseType.TEST_CODE_TRANSLATION_COMPLETION
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
                    TestEngineerResponseType.TEST_PASSED
                )
            
            # Tests ran but some failed
            self._record_history(failed_tests=failed_tests)
            
            return AgentResponse.done(
                self,
                TestEngineerResponseType.TEST_RUN_DONE,
                {"failed_tests": failed_tests}
            )
        else:
            # Compilation failed - first try to auto-add missing dependencies
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
                    # Retry compilation after adding dependencies
                    result = cargo_test(target_project.path, self.test_name)
                    if result["success"]:
                        failed_tests = result.get("failed_tests", [])
                        if not failed_tests:
                            return AgentResponse.done(
                                self,
                                TestEngineerResponseType.TEST_PASSED
                            )
                        self._record_history(failed_tests=failed_tests)
                        return AgentResponse.done(
                            self,
                            TestEngineerResponseType.TEST_RUN_DONE,
                            {"failed_tests": failed_tests}
                        )
                    errors = result.get("errors", [])
            
            return AgentResponse.done(
                self,
                TestEngineerResponseType.TEST_RUN_FAILURE,
                {"errors": errors}
            )
    
    def _fix_syntax_errors(self, errors: List[dict]) -> AgentResponse:
        """Fix syntax/compilation errors in test code."""
        target_project = self.state_manager.state.target_project
        test_path = os.path.join(target_project.path, self.test_file)
        
        # Read current test code
        with open(test_path, "r", encoding="utf-8") as f:
            current_code = f.read()
        
        # Format errors
        error_messages = [e.get("rendered", str(e)) for e in errors[:10]]
        error_text = "\n\n".join(error_messages)
        
        prompt = f"""Fix the following compilation errors in the Rust test code:

## Errors
{error_text}

## Test Code
```rust
{current_code}
```

Provide the corrected code in a ```rust code block.
"""
        
        response = self.reasoner.call_llm([{"role": "user", "content": prompt}])
        fixed_code = self.extract_rust_code(response.content)
        
        if not fixed_code:
            return AgentResponse.error(
                self,
                TestEngineerResponseType.TEST_SYNTAX_FIX_COMPLETION,
                {"message": "No fixed code extracted"}
            )
        
        with open(test_path, "w", encoding="utf-8") as f:
            f.write(fixed_code)
        
        return AgentResponse.done(
            self,
            TestEngineerResponseType.TEST_SYNTAX_FIX_COMPLETION
        )
    
    def _fix_logic_errors(self, failed_tests: List[dict]) -> AgentResponse:
        """Fix logic errors (failed tests)."""
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
        
        response = self.reasoner.call_llm([{"role": "user", "content": prompt}])
        
        # Parse and apply fixes
        # Simple approach: look for rust code blocks with filepath
        content = response.content
        blocks = re.findall(
            r'// filepath:\s*(\S+)\s*\n```rust\s*(.*?)\s*```',
            content,
            re.DOTALL
        )
        
        for filepath, code in blocks:
            full_path = os.path.join(target_project.path, filepath.strip())
            if os.path.exists(full_path) or filepath.startswith("tests/"):
                os.makedirs(os.path.dirname(full_path), exist_ok=True)
                with open(full_path, "w", encoding="utf-8") as f:
                    f.write(code.strip())
                self.logger.info(f"Updated: {filepath}")
        
        return AgentResponse.done(
            self,
            TestEngineerResponseType.TEST_LOGIC_FIX_COMPLETION
        )
    
    def _record_history(self, failed_tests: List[dict] = None) -> None:
        """Record current state for potential rollback."""
        module = self.module
        target_project = self.state_manager.state.target_project
        
        state = {
            "failed_tests": failed_tests or [],
            "files": {}
        }
        
        # Save current file contents
        for rust_file in module.related_rust_files:
            try:
                full_path = os.path.join(target_project.path, rust_file)
                with open(full_path, "r", encoding="utf-8") as f:
                    state["files"][rust_file] = f.read()
            except:
                pass
        
        if self.test_file:
            try:
                test_path = os.path.join(target_project.path, self.test_file)
                with open(test_path, "r", encoding="utf-8") as f:
                    state["files"][self.test_file] = f.read()
            except:
                pass
        
        self.change_history.append(state)
    
    def _save_best_state(self) -> None:
        """Restore the best state from history."""
        if not self.change_history:
            return
        
        # Find state with fewest failed tests
        best_state = min(
            self.change_history,
            key=lambda s: len(s.get("failed_tests", []))
        )
        
        target_project = self.state_manager.state.target_project
        
        for filepath, content in best_state.get("files", {}).items():
            full_path = os.path.join(target_project.path, filepath)
            with open(full_path, "w", encoding="utf-8") as f:
                f.write(content)
        
        self.logger.info("Restored best state from history")

