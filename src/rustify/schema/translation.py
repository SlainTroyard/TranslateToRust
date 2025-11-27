"""
Translation Schema - Data models for translation tasks and modules.

"""

import uuid
from enum import Enum
from typing import Optional, List

from pydantic import BaseModel, Field


class TranslationUnitNode(BaseModel):
    """Represents a single AST node for translation."""
    
    filepath: str = Field(description="File path of the translation unit")
    id: str = Field(description="Node ID")
    name: str = Field(description="Node name")
    type: str = Field(description="Node type (function, struct, enum, etc.)")
    text: str = Field(description="Source code text")
    
    def __hash__(self):
        return hash(self.id)


class TranslationTaskSource(BaseModel):
    """Source information for a translation task."""
    
    name: str = Field(description="Name of the C/C++ node(s) to translate")
    nodes: List[TranslationUnitNode] = Field(description="C/C++ nodes to translate")
    description: Optional[str] = Field(default="", description="Node description")
    
    @property
    def id(self):
        """Generate ID from nodes."""
        if self.nodes:
            return self.nodes[0].id
        return ""


class TranslationTaskTarget(BaseModel):
    """Target information after translation."""
    
    name: str = Field(description="Name of the translated Rust code")
    type: str = Field(description="Type of the translated Rust code")
    text: str = Field(description="Translated Rust code")
    description: Optional[str] = Field(default="", description="Description")
    filepath: Optional[str] = Field(default="", description="Rust file path")


class TranslationTaskStatus(str, Enum):
    """Status of a translation task."""
    
    CREATED = "created"
    TRANSLATING = "translating"
    FIXING = "fixing"
    DONE = "done"
    FAILED = "failed"
    
    def __str__(self):
        return self.value


class TranslationTask(BaseModel):
    """A single translation task."""
    
    id: str = Field(default_factory=lambda: str(uuid.uuid4()), description="Task ID")
    source: TranslationTaskSource = Field(description="C/C++ source to translate")
    target: Optional[TranslationTaskTarget] = Field(default=None, description="Translated Rust code")
    prerequisites: List[str] = Field(default_factory=list, description="IDs of prerequisite tasks")
    status: TranslationTaskStatus = Field(default=TranslationTaskStatus.CREATED, description="Task status")
    
    class Config:
        json_encoders = {
            TranslationTaskStatus: lambda v: v.value
        }
    
    def __hash__(self):
        return hash(self.id)


class ModuleTranslationStatus(str, Enum):
    """Status of a module translation."""
    
    CREATED = "created"
    TRANSPILE = "transpile"
    TEST = "test"
    BENCHMARK = "benchmark"
    DONE = "done"
    FAILED = "failed"
    
    def __str__(self):
        return self.value


class ModuleTranslation(BaseModel):
    """
    Represents a module being translated.
    
    A module is a group of related files that are translated together.
    """
    
    id: str = Field(default_factory=lambda: str(uuid.uuid4()), description="Module ID")
    name: str = Field(default="", description="Module name")
    description: str = Field(default="", description="Module description")
    path: str = Field(default="", description="Module output path")
    
    translation_tasks: List[TranslationTask] = Field(
        default_factory=list, 
        description="Translation tasks for this module"
    )
    related_files: List[str] = Field(
        default_factory=list,
        description="Related C/C++ source files"
    )
    related_rust_files: List[str] = Field(
        default_factory=list,
        description="Generated Rust files"
    )
    
    status: ModuleTranslationStatus = Field(
        default=ModuleTranslationStatus.CREATED,
        description="Module translation status"
    )
    
    class Config:
        json_encoders = {
            ModuleTranslationStatus: lambda v: v.value,
            TranslationTaskStatus: lambda v: v.value,
        }
    
    def get_translation_task_by_id(self, task_id: str) -> Optional[TranslationTask]:
        """Get a translation task by ID."""
        for task in self.translation_tasks:
            if task.id == task_id:
                return task
        return None
    
    @property
    def pending_tasks(self) -> List[TranslationTask]:
        """Get tasks that are not yet done."""
        return [
            task for task in self.translation_tasks
            if task.status not in (TranslationTaskStatus.DONE, TranslationTaskStatus.FAILED)
        ]
    
    @property
    def done_tasks(self) -> List[TranslationTask]:
        """Get completed tasks."""
        return [
            task for task in self.translation_tasks
            if task.status == TranslationTaskStatus.DONE
        ]

