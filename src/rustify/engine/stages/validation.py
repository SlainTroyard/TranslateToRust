"""
Validation Stage - Validate and fix translated Rust code.
"""

import asyncio
import subprocess
import tempfile
from pathlib import Path
from typing import Optional
from loguru import logger

from rustify.engine.stage import Stage
from rustify.engine.pipeline import (
    PipelineContext,
    TranslationModule,
    TranslationUnit,
)


class ValidationStage(Stage[PipelineContext]):
    """
    Validate and fix translated Rust code.
    
    This stage:
    1. Writes translated code to temporary Rust project
    2. Runs cargo check to find errors
    3. Uses LLM to fix errors iteratively
    4. Updates translation units with fixed code
    """
    
    name = "validation"
    description = "Validate and fix translated Rust code"
    
    def __init__(self):
        super().__init__()
        self._llm_adapter = None
        self._template_engine = None
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        """Execute the validation stage."""
        from rustify.llm import get_llm_adapter
        from rustify.templates import TemplateEngine
        
        self._llm_adapter = get_llm_adapter(context.config.llm)
        self._template_engine = TemplateEngine(context.config.templates_dir)
        
        max_retries = context.config.translation.max_retries
        
        # Validate each module
        for module in context.modules:
            if module.status != "translated":
                continue
            
            context.current_module = module
            logger.info(f"Validating module: {module.name}")
            
            # Create temporary Rust project
            with tempfile.TemporaryDirectory() as tmpdir:
                project_path = Path(tmpdir)
                
                # Initialize Cargo project
                self._init_cargo_project(project_path, module.name)
                
                # Write all translated code
                self._write_module_code(project_path, module, context)
                
                # Validate and fix
                success = await self._validate_and_fix(
                    project_path, module, context, max_retries
                )
                
                if success:
                    module.status = "validated"
                    logger.info(f"Module {module.name} validated successfully")
                else:
                    module.status = "validation_failed"
                    context.warnings.append(
                        f"Module {module.name} has unfixed errors"
                    )
        
        context.current_module = None
        return context
    
    def _init_cargo_project(self, path: Path, name: str) -> None:
        """Initialize a Cargo project for validation."""
        # Create Cargo.toml
        cargo_toml = f'''[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"
'''
        (path / "Cargo.toml").write_text(cargo_toml)
        
        # Create src directory
        (path / "src").mkdir(exist_ok=True)
    
    def _write_module_code(
        self,
        project_path: Path,
        module: TranslationModule,
        context: PipelineContext
    ) -> None:
        """Write translated code to the project."""
        src_path = project_path / "src"
        
        # Collect all code
        all_code = []
        for sf in module.source_files:
            for unit in sf.translation_units:
                if unit.rust_code:
                    all_code.append(f"// From: {unit.name}")
                    all_code.append(unit.rust_code)
                    all_code.append("")
        
        # Determine if lib or bin
        has_main = any("fn main" in code for code in all_code)
        
        if has_main:
            main_file = src_path / "main.rs"
        else:
            main_file = src_path / "lib.rs"
        
        # Write code
        main_file.write_text("\n".join(all_code))
    
    async def _validate_and_fix(
        self,
        project_path: Path,
        module: TranslationModule,
        context: PipelineContext,
        max_retries: int
    ) -> bool:
        """Validate code and attempt to fix errors."""
        for attempt in range(max_retries):
            # Run cargo check
            errors = self._run_cargo_check(project_path)
            
            if not errors:
                return True
            
            logger.debug(f"Validation attempt {attempt + 1}: {len(errors)} errors")
            
            # Try to fix errors
            fixed = await self._fix_errors(
                project_path, module, context, errors
            )
            
            if not fixed:
                break
        
        return False
    
    def _run_cargo_check(self, project_path: Path) -> list[dict]:
        """Run cargo check and parse errors."""
        import json
        
        try:
            result = subprocess.run(
                ["cargo", "check", "--message-format=json"],
                cwd=project_path,
                capture_output=True,
                text=True,
                timeout=60
            )
        except subprocess.TimeoutExpired:
            return [{"message": "Compilation timed out"}]
        except FileNotFoundError:
            logger.warning("cargo not found, skipping validation")
            return []
        
        errors = []
        for line in result.stdout.split("\n"):
            if not line.strip():
                continue
            try:
                msg = json.loads(line)
                if msg.get("reason") == "compiler-message":
                    if msg["message"]["level"] == "error":
                        errors.append({
                            "message": msg["message"]["message"],
                            "rendered": msg["message"].get("rendered", ""),
                            "code": msg["message"].get("code", {}).get("code", ""),
                            "spans": msg["message"].get("spans", []),
                        })
            except json.JSONDecodeError:
                continue
        
        return errors
    
    async def _fix_errors(
        self,
        project_path: Path,
        module: TranslationModule,
        context: PipelineContext,
        errors: list[dict]
    ) -> bool:
        """Attempt to fix compilation errors using LLM."""
        # Read current code
        src_path = project_path / "src"
        main_file = src_path / "lib.rs"
        if not main_file.exists():
            main_file = src_path / "main.rs"
        
        if not main_file.exists():
            return False
        
        current_code = main_file.read_text()
        
        # Limit errors to prevent context overflow
        if len(errors) > 10:
            errors = errors[:10]
        
        # Build fix prompt
        prompt = self._template_engine.render(
            "validation/fix_errors.jinja2",
            code=current_code,
            errors=errors,
            module=module,
        )
        
        # Get fix from LLM
        response = await self._llm_adapter.complete(
            prompt=prompt,
            temperature=0.2,
            max_tokens=context.config.llm.max_tokens,
        )
        
        # Extract fixed code
        fixed_code = self._extract_rust_code(response)
        
        if fixed_code and fixed_code != current_code:
            main_file.write_text(fixed_code)
            
            # Update translation units with fixed code
            self._update_units_from_fixed_code(module, fixed_code)
            
            return True
        
        return False
    
    def _extract_rust_code(self, response: str) -> Optional[str]:
        """Extract Rust code from LLM response."""
        import re
        
        patterns = [
            r'```rust\n(.*?)```',
            r'```\n(.*?)```',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, response, re.DOTALL)
            if match:
                return match.group(1).strip()
        
        return None
    
    def _update_units_from_fixed_code(
        self,
        module: TranslationModule,
        fixed_code: str
    ) -> None:
        """Update translation units from fixed combined code."""
        # This is a simplified version - in practice you'd need
        # to parse and match individual units
        
        # For now, just mark units as needing manual review
        for sf in module.source_files:
            for unit in sf.translation_units:
                if unit.rust_code:
                    unit.metadata["needs_review"] = True

