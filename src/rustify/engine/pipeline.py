"""
Pipeline implementation for the translation workflow.
"""

from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Any, Optional
from loguru import logger

from rustify.config import RustifyConfig, get_config
from rustify.engine.stage import Stage, StageResult, StageStatus


@dataclass
class TranslationUnit:
    """Represents a single translation unit (e.g., a function, struct)."""
    
    id: str
    name: str
    type: str  # function, struct, enum, typedef, macro
    source_code: str
    source_file: str
    dependencies: list[str] = field(default_factory=list)
    rust_code: Optional[str] = None
    status: str = "pending"
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass
class SourceFile:
    """Represents a source file."""
    
    path: Path
    content: str
    language: str = "c"
    summary: Optional[str] = None
    translation_units: list[TranslationUnit] = field(default_factory=list)


@dataclass
class TranslationModule:
    """Represents a translation module (group of related files)."""
    
    id: str
    name: str
    description: Optional[str] = None
    source_files: list[SourceFile] = field(default_factory=list)
    dependencies: list[str] = field(default_factory=list)
    status: str = "pending"


@dataclass
class PipelineContext:
    """
    Context object passed through the pipeline stages.
    Contains all state and data needed for translation.
    """
    
    # Source project information
    source_path: Path
    target_path: Path
    
    # Configuration
    config: RustifyConfig = field(default_factory=get_config)
    
    # Project state
    source_files: list[SourceFile] = field(default_factory=list)
    modules: list[TranslationModule] = field(default_factory=list)
    
    # Dependency graph (NetworkX graph will be stored here)
    dependency_graph: Any = None
    
    # Translation state
    current_module: Optional[TranslationModule] = None
    current_unit: Optional[TranslationUnit] = None
    
    # Results and metrics
    stage_results: dict[str, StageResult] = field(default_factory=dict)
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)
    
    # Timing
    started_at: Optional[datetime] = None
    finished_at: Optional[datetime] = None
    
    # Metadata
    metadata: dict[str, Any] = field(default_factory=dict)
    
    @property
    def is_success(self) -> bool:
        """Check if all stages completed successfully."""
        return all(
            r.status == StageStatus.SUCCESS 
            for r in self.stage_results.values()
        )
    
    @property
    def duration(self) -> Optional[float]:
        """Get total pipeline duration in seconds."""
        if self.started_at and self.finished_at:
            return (self.finished_at - self.started_at).total_seconds()
        return None
    
    def get_file_by_path(self, path: Path | str) -> Optional[SourceFile]:
        """Get a source file by path."""
        path = Path(path)
        for f in self.source_files:
            if f.path == path:
                return f
        return None
    
    def get_module_by_id(self, module_id: str) -> Optional[TranslationModule]:
        """Get a module by ID."""
        for m in self.modules:
            if m.id == module_id:
                return m
        return None


class Pipeline:
    """
    Translation pipeline that orchestrates the execution of stages.
    
    Example:
        ```python
        pipeline = Pipeline([
            AnalysisStage(),
            PlanningStage(),
            TranslationStage(),
            ValidationStage(),
        ])
        
        context = PipelineContext(
            source_path=Path("./c_project"),
            target_path=Path("./rust_project")
        )
        
        result = await pipeline.run(context)
        ```
    """
    
    def __init__(
        self, 
        stages: list[Stage],
        config: Optional[RustifyConfig] = None
    ):
        self.stages = stages
        self.config = config or get_config()
        self._current_stage: Optional[Stage] = None
    
    @property
    def current_stage(self) -> Optional[Stage]:
        """Get the currently executing stage."""
        return self._current_stage
    
    async def run(self, context: PipelineContext) -> PipelineContext:
        """
        Run all stages in sequence.
        
        Args:
            context: The pipeline context.
            
        Returns:
            Updated context after all stages complete.
        """
        context.started_at = datetime.now()
        logger.info(f"Starting pipeline with {len(self.stages)} stages")
        
        for stage in self.stages:
            self._current_stage = stage
            stage_name = stage.name
            
            logger.info(f"Running stage: {stage_name}")
            
            try:
                context, result = await stage.run(context)
                context.stage_results[stage_name] = result
                
                if result.status == StageStatus.FAILED:
                    logger.error(f"Stage {stage_name} failed: {result.error}")
                    context.errors.append(f"Stage {stage_name}: {result.error}")
                    # Stop pipeline on failure
                    break
                elif result.status == StageStatus.SKIPPED:
                    logger.info(f"Stage {stage_name} skipped: {result.error}")
                else:
                    logger.info(
                        f"Stage {stage_name} completed in {result.duration:.2f}s"
                    )
                    
            except Exception as e:
                logger.exception(f"Unexpected error in stage {stage_name}")
                context.stage_results[stage_name] = StageResult.failed(str(e))
                context.errors.append(f"Stage {stage_name}: {str(e)}")
                break
        
        self._current_stage = None
        context.finished_at = datetime.now()
        
        logger.info(
            f"Pipeline completed in {context.duration:.2f}s "
            f"with {'success' if context.is_success else 'errors'}"
        )
        
        return context
    
    async def run_stage(
        self, 
        stage_name: str, 
        context: PipelineContext
    ) -> PipelineContext:
        """
        Run a specific stage by name.
        
        Args:
            stage_name: Name of the stage to run.
            context: The pipeline context.
            
        Returns:
            Updated context.
        """
        for stage in self.stages:
            if stage.name == stage_name:
                context, result = await stage.run(context)
                context.stage_results[stage_name] = result
                return context
        
        raise ValueError(f"Stage not found: {stage_name}")
    
    def add_stage(self, stage: Stage, position: int = -1) -> None:
        """Add a stage to the pipeline."""
        if position < 0:
            self.stages.append(stage)
        else:
            self.stages.insert(position, stage)
    
    def remove_stage(self, stage_name: str) -> bool:
        """Remove a stage by name."""
        for i, stage in enumerate(self.stages):
            if stage.name == stage_name:
                self.stages.pop(i)
                return True
        return False
    
    def get_stage(self, stage_name: str) -> Optional[Stage]:
        """Get a stage by name."""
        for stage in self.stages:
            if stage.name == stage_name:
                return stage
        return None
    
    def __repr__(self) -> str:
        stage_names = [s.name for s in self.stages]
        return f"<Pipeline stages={stage_names}>"

