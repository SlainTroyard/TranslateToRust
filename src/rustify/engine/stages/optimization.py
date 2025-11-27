"""
Optimization Stage - Optimize and finalize translated Rust code.
"""

from pathlib import Path
from typing import Optional
from loguru import logger

from rustify.engine.stage import Stage
from rustify.engine.pipeline import (
    PipelineContext,
    TranslationModule,
)


class OptimizationStage(Stage[PipelineContext]):
    """
    Optimize and finalize translated Rust code.
    
    This stage:
    1. Generates final Rust project structure
    2. Applies Rust idioms and best practices
    3. Adds documentation and tests
    4. Runs rustfmt for formatting
    """
    
    name = "optimization"
    description = "Optimize and finalize translated Rust code"
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        """Execute the optimization stage."""
        target_path = context.target_path
        
        # Create project structure
        self._create_project_structure(target_path, context)
        
        # Write modules
        for module in context.modules:
            if module.status not in ("translated", "validated"):
                continue
            
            self._write_module(target_path, module, context)
        
        # Create workspace Cargo.toml if multiple modules
        if len(context.modules) > 1:
            self._create_workspace(target_path, context)
        
        # Run rustfmt
        self._run_rustfmt(target_path)
        
        # Generate report
        self._generate_report(target_path, context)
        
        logger.info(f"Output written to: {target_path}")
        
        return context
    
    def _create_project_structure(
        self,
        target_path: Path,
        context: PipelineContext
    ) -> None:
        """Create the target project structure."""
        target_path.mkdir(parents=True, exist_ok=True)
        
        # Create common directories
        (target_path / "src").mkdir(exist_ok=True)
        (target_path / "tests").mkdir(exist_ok=True)
    
    def _write_module(
        self,
        target_path: Path,
        module: TranslationModule,
        context: PipelineContext
    ) -> None:
        """Write a translated module to the target directory."""
        if len(context.modules) == 1:
            # Single module - write directly to src/
            module_path = target_path
        else:
            # Multiple modules - create subdirectory
            module_path = target_path / module.name
            module_path.mkdir(exist_ok=True)
        
        # Create Cargo.toml
        self._write_cargo_toml(module_path, module)
        
        # Create src directory
        src_path = module_path / "src"
        src_path.mkdir(exist_ok=True)
        
        # Collect all translated code
        code_sections = []
        
        for sf in module.source_files:
            for unit in sf.translation_units:
                if unit.rust_code:
                    code_sections.append({
                        "name": unit.name,
                        "type": unit.type,
                        "code": unit.rust_code,
                    })
        
        # Determine main file
        has_main = any(
            "fn main" in section["code"]
            for section in code_sections
        )
        
        main_file = src_path / ("main.rs" if has_main else "lib.rs")
        
        # Write code with organization
        self._write_organized_code(main_file, code_sections, module)
        
        logger.debug(f"Wrote module {module.name} to {module_path}")
    
    def _write_cargo_toml(
        self,
        module_path: Path,
        module: TranslationModule
    ) -> None:
        """Write Cargo.toml for a module."""
        cargo_toml = f'''[package]
name = "{module.name}"
version = "0.1.0"
edition = "2021"
description = "{module.description or 'Translated from C'}"

[dependencies]
libc = "0.2"

[dev-dependencies]
'''
        
        (module_path / "Cargo.toml").write_text(cargo_toml)
    
    def _write_organized_code(
        self,
        main_file: Path,
        code_sections: list[dict],
        module: TranslationModule
    ) -> None:
        """Write organized Rust code."""
        lines = [
            "//! Translated from C using Rustify",
            "//!",
            f"//! Original module: {module.name}",
            "",
            "#![allow(unused)]",
            "#![allow(non_snake_case)]",
            "#![allow(non_camel_case_types)]",
            "",
        ]
        
        # Group by type
        type_order = ["typedef", "struct", "enum", "union", "macro", "variable", "function"]
        
        for unit_type in type_order:
            type_sections = [s for s in code_sections if s["type"] == unit_type]
            if not type_sections:
                continue
            
            lines.append(f"// ==================== {unit_type.upper()}S ====================")
            lines.append("")
            
            for section in type_sections:
                lines.append(f"// {section['name']}")
                lines.append(section["code"])
                lines.append("")
        
        main_file.write_text("\n".join(lines))
    
    def _create_workspace(
        self,
        target_path: Path,
        context: PipelineContext
    ) -> None:
        """Create Cargo workspace for multiple modules."""
        members = [
            m.name for m in context.modules
            if m.status in ("translated", "validated")
        ]
        
        members_str = ", ".join(f'"{m}"' for m in members)
        
        workspace_toml = f'''[workspace]
members = [{members_str}]
resolver = "2"
'''
        
        (target_path / "Cargo.toml").write_text(workspace_toml)
    
    def _run_rustfmt(self, target_path: Path) -> None:
        """Run rustfmt on the generated code."""
        import subprocess
        
        try:
            # Find all Rust files
            rust_files = list(target_path.rglob("*.rs"))
            
            for rust_file in rust_files:
                subprocess.run(
                    ["rustfmt", str(rust_file)],
                    capture_output=True,
                    timeout=30
                )
        except (subprocess.TimeoutExpired, FileNotFoundError):
            logger.warning("rustfmt not available or timed out")
    
    def _generate_report(
        self,
        target_path: Path,
        context: PipelineContext
    ) -> None:
        """Generate translation report."""
        report_lines = [
            "# Translation Report",
            "",
            f"## Summary",
            f"- Source: {context.source_path}",
            f"- Target: {context.target_path}",
            f"- Duration: {context.duration:.2f}s" if context.duration else "",
            f"- Status: {'Success' if context.is_success else 'Completed with warnings'}",
            "",
            "## Modules",
            "",
        ]
        
        for module in context.modules:
            report_lines.append(f"### {module.name}")
            report_lines.append(f"- Status: {module.status}")
            
            unit_count = sum(
                len(sf.translation_units) for sf in module.source_files
            )
            translated_count = sum(
                len([u for u in sf.translation_units if u.rust_code])
                for sf in module.source_files
            )
            
            report_lines.append(f"- Units: {translated_count}/{unit_count} translated")
            report_lines.append("")
        
        if context.warnings:
            report_lines.append("## Warnings")
            report_lines.append("")
            for warning in context.warnings:
                report_lines.append(f"- {warning}")
            report_lines.append("")
        
        if context.errors:
            report_lines.append("## Errors")
            report_lines.append("")
            for error in context.errors:
                report_lines.append(f"- {error}")
            report_lines.append("")
        
        report_lines.append("## Stage Results")
        report_lines.append("")
        for stage_name, result in context.stage_results.items():
            duration = f"({result.duration:.2f}s)" if result.duration else ""
            report_lines.append(f"- {stage_name}: {result.status.value} {duration}")
        
        report_path = target_path / "TRANSLATION_REPORT.md"
        report_path.write_text("\n".join(report_lines))
        
        logger.info(f"Report written to: {report_path}")

