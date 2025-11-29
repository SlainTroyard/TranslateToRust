"""
Benchmarker Agent - Handles benchmark generation and execution.

"""

import os
from typing import Optional, List
import logging

from rustify.agents.base import BaseAgent
from rustify.agents.analyzer import Analyzer
from rustify.schema.response import (
    AgentResponse,
    BenchmarkerResponseType,
)
from rustify.schema.translation import ModuleTranslation
from rustify.state.state_manager import StateManager


class Benchmarker(BaseAgent):
    """
    Benchmarker agent responsible for:
    - Generating benchmark code
    - Running benchmarks
    - Fixing benchmark errors
    """
    
    ROLE = "benchmarker"
    DESCRIPTION = "An AI assistant specialized in generating and running performance benchmarks."
    
    MAX_FIX_ATTEMPTS = 5
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        analyzer_config: Optional[dict] = None,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Benchmarker.
        
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
        
        # Current module
        self.current_module_id: Optional[str] = None
        
        # Benchmark state
        self.bench_file: Optional[str] = None
        self.bench_name: Optional[str] = None
        self.crate_name: Optional[str] = None
        
        # Fix counter
        self.fix_attempts = 0
        
        # Original files for rollback
        self.original_files: List[dict] = []
    
    @property
    def module(self) -> Optional[ModuleTranslation]:
        """Get current module."""
        if self.current_module_id:
            return self.state_manager.get_module_translation_by_id(self.current_module_id)
        return None
    
    def start(self, module: ModuleTranslation) -> AgentResponse:
        """
        Start benchmark generation and execution for a module.
        
        Args:
            module: Module to benchmark.
            
        Returns:
            AgentResponse indicating status.
        """
        self.logger.info(f"Starting benchmark generation for module: {module.name}")
        self.current_module_id = module.id
        self.fix_attempts = 0
        
        # Prepare benchmarks
        response = self._prepare_benchmark(module)
        if response.status != "done":
            return response
        
        # Generate benchmark code
        response = self._generate_benchmark()
        if response.status != "done":
            return response
        
        # Run benchmark loop
        while True:
            response = self._run_benchmark()
            
            if response.type == BenchmarkerResponseType.BENCH_DONE:
                return response
            
            if response.type == BenchmarkerResponseType.BENCH_FAILED:
                if self.fix_attempts >= self.MAX_FIX_ATTEMPTS:
                    self.logger.error("Max fix attempts reached")
                    self._revert_files()
                    return AgentResponse.error(
                        self,
                        BenchmarkerResponseType.BENCH_DONE,
                        {"message": "Max fix attempts reached"}
                    )
                
                self.fix_attempts += 1
                fix_response = self._fix_errors(response.data.get("errors", []))
                if fix_response.status != "done":
                    return fix_response
    
    def _prepare_benchmark(self, module: ModuleTranslation) -> AgentResponse:
        """Prepare the benchmark environment."""
        target_project = self.state_manager.state.target_project
        
        # Save original files
        self.original_files.clear()
        for rust_file in module.related_rust_files:
            try:
                full_path = os.path.join(target_project.path, rust_file)
                with open(full_path, "r", encoding="utf-8") as f:
                    content = f.read()
                self.original_files.append({
                    "path": rust_file,
                    "content": content
                })
            except:
                pass
        
        # Set up benchmark info
        self.bench_name = f"{module.name}_bench"
        self.bench_file = f"benches/{self.bench_name}.rs"
        
        # Create benches directory
        benches_path = os.path.join(target_project.path, "benches")
        os.makedirs(benches_path, exist_ok=True)
        
        # Update Cargo.toml
        self._update_cargo_toml(target_project.path)
        
        return AgentResponse.done(
            self,
            BenchmarkerResponseType.BENCH_PREPARE_DONE
        )
    
    def _update_cargo_toml(self, project_path: str) -> None:
        """Update Cargo.toml with benchmark configuration."""
        import toml
        
        cargo_path = os.path.join(project_path, "Cargo.toml")
        
        with open(cargo_path, "r") as f:
            cargo = toml.load(f)
        
        # Get crate name
        self.crate_name = cargo.get("package", {}).get("name", "unknown")
        
        # Add criterion dependency
        if "dev-dependencies" not in cargo:
            cargo["dev-dependencies"] = {}
        cargo["dev-dependencies"]["criterion"] = "0.5"
        
        # Add bench configuration
        if "bench" not in cargo:
            cargo["bench"] = []
        
        bench_exists = any(
            b.get("name") == self.bench_name
            for b in cargo["bench"]
        )
        
        if not bench_exists:
            cargo["bench"].append({
                "name": self.bench_name,
                "harness": False
            })
        
        with open(cargo_path, "w") as f:
            toml.dump(cargo, f)
    
    def _generate_benchmark(self) -> AgentResponse:
        """Generate benchmark code."""
        module = self.module
        target_project = self.state_manager.state.target_project
        
        # Read Rust module code
        rust_code_parts = []
        for rust_file in module.related_rust_files:
            try:
                full_path = os.path.join(target_project.path, rust_file)
                with open(full_path, "r", encoding="utf-8") as f:
                    content = f.read()
                rust_code_parts.append(f"// {rust_file}\n{content}")
            except:
                pass
        
        rust_context = "\n\n".join(rust_code_parts)
        
        # Look for functions to benchmark
        prompt = f"""Generate Criterion benchmarks for the following Rust module:

## Module Code
```rust
{rust_context}
```

## Crate Name
{self.crate_name}

## Requirements
1. Use criterion crate
2. Benchmark key functions
3. Use criterion_group! and criterion_main! macros
4. Handle any necessary imports

Provide the benchmark code in a ```rust code block.
If there are no suitable functions to benchmark, provide an empty code block.
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        bench_code = self.extract_rust_code(response.content)
        
        if not bench_code or bench_code.strip() == "":
            self.logger.info("No benchmark code generated")
            return AgentResponse.done(
                self,
                BenchmarkerResponseType.BENCH_DONE,
                {"message": "No benchmarks needed"}
            )
        
        # Write benchmark file
        bench_path = os.path.join(target_project.path, self.bench_file)
        with open(bench_path, "w", encoding="utf-8") as f:
            f.write(bench_code)
        
        self.logger.info(f"Generated benchmark: {self.bench_file}")
        
        return AgentResponse.done(
            self,
            BenchmarkerResponseType.BENCH_COMPLETION
        )
    
    def _run_benchmark(self) -> AgentResponse:
        """Run the benchmark."""
        from rustify.tools.rust_utils import cargo_bench
        
        target_project = self.state_manager.state.target_project
        
        result = cargo_bench(target_project.path, self.bench_name)
        
        if result["success"]:
            self.logger.info("Benchmark completed successfully")
            return AgentResponse.done(
                self,
                BenchmarkerResponseType.BENCH_DONE
            )
        else:
            return AgentResponse.done(
                self,
                BenchmarkerResponseType.BENCH_FAILED,
                {"errors": result.get("errors", [])}
            )
    
    def _fix_errors(self, errors: List[dict]) -> AgentResponse:
        """Fix benchmark errors."""
        target_project = self.state_manager.state.target_project
        bench_path = os.path.join(target_project.path, self.bench_file)
        
        # Read current code
        with open(bench_path, "r", encoding="utf-8") as f:
            current_code = f.read()
        
        # Format errors
        error_messages = [e.get("rendered", str(e)) for e in errors[:10]]
        error_text = "\n\n".join(error_messages)
        
        # Read module code for context
        module = self.module
        rust_code_parts = []
        for rust_file in module.related_rust_files:
            try:
                full_path = os.path.join(target_project.path, rust_file)
                with open(full_path, "r", encoding="utf-8") as f:
                    content = f.read()
                rust_code_parts.append(f"// {rust_file}\n{content}")
            except:
                pass
        
        rust_context = "\n\n".join(rust_code_parts)
        
        prompt = f"""Fix the following benchmark compilation errors:

## Errors
{error_text}

## Benchmark Code
```rust
{current_code}
```

## Module Code (for reference)
```rust
{rust_context}
```

Provide the corrected benchmark code in a ```rust code block.
"""
        
        response = self.analyzer.call_llm([{"role": "user", "content": prompt}])
        fixed_code = self.extract_rust_code(response.content)
        
        if not fixed_code:
            return AgentResponse.error(
                self,
                BenchmarkerResponseType.BENCH_FIX_DONE,
                {"message": "No fixed code extracted"}
            )
        
        with open(bench_path, "w", encoding="utf-8") as f:
            f.write(fixed_code)
        
        return AgentResponse.done(
            self,
            BenchmarkerResponseType.BENCH_FIX_DONE
        )
    
    def _revert_files(self) -> None:
        """Revert to original files."""
        target_project = self.state_manager.state.target_project
        
        for file_info in self.original_files:
            full_path = os.path.join(target_project.path, file_info["path"])
            with open(full_path, "w", encoding="utf-8") as f:
                f.write(file_info["content"])
        
        self.logger.info("Reverted to original files")

