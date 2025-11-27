"""
Base Stage definition for the translation pipeline.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Generic, TypeVar, Optional
from datetime import datetime


class StageStatus(str, Enum):
    """Status of a pipeline stage."""
    
    PENDING = "pending"
    RUNNING = "running"
    SUCCESS = "success"
    FAILED = "failed"
    SKIPPED = "skipped"


@dataclass
class StageResult:
    """Result of a stage execution."""
    
    status: StageStatus
    data: dict[str, Any] = field(default_factory=dict)
    error: Optional[str] = None
    started_at: Optional[datetime] = None
    finished_at: Optional[datetime] = None
    
    @property
    def duration(self) -> Optional[float]:
        """Get execution duration in seconds."""
        if self.started_at and self.finished_at:
            return (self.finished_at - self.started_at).total_seconds()
        return None
    
    @classmethod
    def success(cls, data: dict[str, Any] = None) -> "StageResult":
        return cls(status=StageStatus.SUCCESS, data=data or {})
    
    @classmethod
    def failed(cls, error: str, data: dict[str, Any] = None) -> "StageResult":
        return cls(status=StageStatus.FAILED, error=error, data=data or {})
    
    @classmethod
    def skipped(cls, reason: str = "") -> "StageResult":
        return cls(status=StageStatus.SKIPPED, error=reason)


# Type variable for context
T = TypeVar("T")


class Stage(ABC, Generic[T]):
    """
    Base class for all pipeline stages.
    
    A stage represents a single step in the translation pipeline.
    Each stage receives a context, performs its operation, and returns
    an updated context.
    """
    
    name: str = "base_stage"
    description: str = "Base stage"
    
    def __init__(self):
        self.result: Optional[StageResult] = None
    
    @abstractmethod
    async def execute(self, context: T) -> T:
        """
        Execute the stage logic.
        
        Args:
            context: The pipeline context containing all necessary data.
            
        Returns:
            Updated context after stage execution.
        """
        pass
    
    async def before_execute(self, context: T) -> T:
        """Hook called before execute. Can be overridden."""
        return context
    
    async def after_execute(self, context: T, result: StageResult) -> T:
        """Hook called after execute. Can be overridden."""
        return context
    
    async def on_error(self, context: T, error: Exception) -> T:
        """Hook called on error. Can be overridden."""
        return context
    
    async def run(self, context: T) -> tuple[T, StageResult]:
        """
        Run the stage with lifecycle hooks.
        
        Args:
            context: The pipeline context.
            
        Returns:
            Tuple of (updated context, stage result).
        """
        result = StageResult(status=StageStatus.RUNNING)
        result.started_at = datetime.now()
        
        try:
            context = await self.before_execute(context)
            context = await self.execute(context)
            result.status = StageStatus.SUCCESS
            result.finished_at = datetime.now()
            context = await self.after_execute(context, result)
        except Exception as e:
            result.status = StageStatus.FAILED
            result.error = str(e)
            result.finished_at = datetime.now()
            context = await self.on_error(context, e)
        
        self.result = result
        return context, result
    
    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}: {self.name}>"

