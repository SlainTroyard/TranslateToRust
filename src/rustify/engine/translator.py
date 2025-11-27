"""
Main Rustify translator class.
"""

from pathlib import Path
from typing import Optional
from loguru import logger

from rustify.config import RustifyConfig, get_config
from rustify.engine.pipeline import Pipeline, PipelineContext
from rustify.engine.stages import (
    AnalysisStage,
    PlanningStage,
    TranslationStage,
    ValidationStage,
    OptimizationStage,
)


class Rustify:
    """
    Main entry point for C to Rust translation.
    
    Example:
        ```python
        rustify = Rustify()
        
        # Translate a single file
        result = await rustify.translate_file("main.c", "main.rs")
        
        # Translate a project
        result = await rustify.translate_project(
            source="./c_project",
            target="./rust_project"
        )
        ```
    """
    
    def __init__(self, config: Optional[RustifyConfig] = None):
        self.config = config or get_config()
        self._setup_logging()
    
    def _setup_logging(self) -> None:
        """Configure logging based on config."""
        logger.remove()
        logger.add(
            lambda msg: print(msg, end=""),
            level=self.config.log_level,
            format="<green>{time:YYYY-MM-DD HH:mm:ss}</green> | "
                   "<level>{level: <8}</level> | "
                   "<cyan>{name}</cyan>:<cyan>{function}</cyan> - "
                   "<level>{message}</level>"
        )
    
    def create_pipeline(
        self,
        stages: Optional[list] = None
    ) -> Pipeline:
        """
        Create a translation pipeline.
        
        Args:
            stages: Optional list of custom stages.
            
        Returns:
            Configured Pipeline instance.
        """
        if stages is None:
            stages = [
                AnalysisStage(),
                PlanningStage(),
                TranslationStage(),
                ValidationStage(),
                OptimizationStage(),
            ]
        
        return Pipeline(stages=stages, config=self.config)
    
    async def translate_file(
        self,
        source: str | Path,
        target: Optional[str | Path] = None
    ) -> PipelineContext:
        """
        Translate a single C file to Rust.
        
        Args:
            source: Path to the source C file.
            target: Path for the output Rust file.
            
        Returns:
            PipelineContext with translation results.
        """
        source = Path(source)
        if target is None:
            target = source.with_suffix(".rs")
        else:
            target = Path(target)
        
        # Create a minimal context for single file
        context = PipelineContext(
            source_path=source.parent,
            target_path=target.parent,
            config=self.config,
        )
        
        # Mark this as single file mode
        context.metadata["single_file"] = True
        context.metadata["source_file"] = source
        context.metadata["target_file"] = target
        
        pipeline = self.create_pipeline()
        return await pipeline.run(context)
    
    async def translate_project(
        self,
        source: str | Path,
        target: str | Path,
        incremental: bool = False
    ) -> PipelineContext:
        """
        Translate an entire C project to Rust.
        
        Args:
            source: Path to the source C project directory.
            target: Path for the output Rust project directory.
            incremental: If True, only translate changed files.
            
        Returns:
            PipelineContext with translation results.
        """
        source = Path(source)
        target = Path(target)
        
        if not source.exists():
            raise ValueError(f"Source path does not exist: {source}")
        
        # Create target directory
        target.mkdir(parents=True, exist_ok=True)
        
        # Create context
        context = PipelineContext(
            source_path=source,
            target_path=target,
            config=self.config,
        )
        
        context.metadata["incremental"] = incremental
        
        pipeline = self.create_pipeline()
        return await pipeline.run(context)
    
    async def translate_code(self, c_code: str) -> str:
        """
        Translate C code string directly to Rust.
        
        Args:
            c_code: C source code as string.
            
        Returns:
            Translated Rust code.
        """
        import tempfile
        
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)
            source_file = tmpdir / "source.c"
            target_file = tmpdir / "output.rs"
            
            source_file.write_text(c_code)
            
            result = await self.translate_file(source_file, target_file)
            
            if result.is_success and target_file.exists():
                return target_file.read_text()
            else:
                errors = "\n".join(result.errors)
                raise RuntimeError(f"Translation failed: {errors}")

