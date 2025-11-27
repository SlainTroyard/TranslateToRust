"""
State Manager - Manages the translation state and persistence.

"""

import json
import os
from enum import Enum
from threading import Lock
from typing import Optional, List
from dataclasses import dataclass, field

from rustify.schema.translation import (
    ModuleTranslation,
    ModuleTranslationStatus,
    TranslationTask,
    TranslationTaskStatus,
    TranslationTaskTarget,
)
from rustify.schema.project import Project, TargetProject


@dataclass
class State:
    """
    Holds the current translation state.
    
    Contains source/target projects and module translations.
    """
    
    project_translation_id: Optional[str] = None
    source_project: Optional[Project] = None
    target_project: Optional[TargetProject] = None
    module_translations: List[ModuleTranslation] = field(default_factory=list)
    test_files: List[str] = field(default_factory=list)  # 测试文件列表，由 TestEngineer 后续处理
    dep_graph_data: Optional[dict] = None  # AST 依赖图数据
    
    @property
    def ready_module_translations(self) -> List[tuple]:
        """Get modules that are not yet done or failed."""
        return [
            (index, module)
            for index, module in enumerate(self.module_translations)
            if module.status not in (ModuleTranslationStatus.DONE, ModuleTranslationStatus.FAILED)
        ]
    
    def to_dict(self) -> dict:
        """Convert state to dictionary for serialization."""
        return {
            "project_translation_id": self.project_translation_id,
            "source_project": self.source_project.to_dict() if self.source_project else None,
            "target_project": self.target_project.to_dict() if self.target_project else None,
            "module_translations": [m.model_dump() for m in self.module_translations],
            "test_files": self.test_files,
            "dep_graph_data": self.dep_graph_data
        }
    
    def load_from_json(self, json_str: str) -> None:
        """Load state from JSON string."""
        data = json.loads(json_str)
        
        if data.get("project_translation_id"):
            self.project_translation_id = data["project_translation_id"]
        
        if data.get("source_project"):
            self.source_project = Project(**data["source_project"])
        
        if data.get("target_project"):
            self.target_project = TargetProject(**data["target_project"])
        
        if data.get("module_translations"):
            self.module_translations = [
                ModuleTranslation(**m) for m in data["module_translations"]
            ]
        
        if data.get("test_files"):
            self.test_files = data["test_files"]
        
        if data.get("dep_graph_data"):
            self.dep_graph_data = data["dep_graph_data"]


class FileLockManager:
    """Thread-safe file lock manager."""
    
    def __init__(self):
        self.file_locks: dict[str, Lock] = {}
    
    def file_lock(self, filepath: str) -> Lock:
        """Get or create a lock for a file."""
        if filepath not in self.file_locks:
            self.file_locks[filepath] = Lock()
        return self.file_locks[filepath]
    
    def is_locked(self, filepath: str) -> bool:
        """Check if a file is locked."""
        if filepath not in self.file_locks:
            return False
        return self.file_locks[filepath].locked()


class EnumEncoder(json.JSONEncoder):
    """JSON encoder that handles Enum values."""
    
    def default(self, obj):
        if isinstance(obj, Enum):
            return obj.value
        return super().default(obj)


class StateManager:
    """
    Manages the translation state and persistence.
    
    Handles:
    - Loading/saving state to disk
    - Managing source and target projects
    - Tracking module translations and their status
    """
    
    def __init__(self, filepath: str):
        """
        Initialize the state manager.
        
        Args:
            filepath: Path to the state file.
        """
        self.bound_filepath = filepath
        self.state = State()
        self._lock = Lock()
        self.file_lock_manager = FileLockManager()
        
        # Load existing state if available
        if os.path.exists(filepath):
            try:
                with open(filepath, "r", encoding="utf-8") as f:
                    content = f.read()
                    if content:
                        self.state.load_from_json(content)
            except Exception as e:
                print(f"Warning: Could not load state from {filepath}: {e}")
    
    def sync_to_disk(self) -> None:
        """Save current state to disk."""
        if os.path.dirname(self.bound_filepath):
            os.makedirs(os.path.dirname(self.bound_filepath), exist_ok=True)
        
        with self._lock:
            with open(self.bound_filepath, "w", encoding="utf-8") as f:
                json.dump(
                    self.state.to_dict(),
                    f,
                    ensure_ascii=False,
                    indent=4,
                    cls=EnumEncoder
                )
    
    # ============ Project Management ============
    
    def create_source_project(self, project_dir: str) -> Project:
        """Create source project from directory."""
        import uuid
        
        project = Project(
            id=str(uuid.uuid4()),
            name=os.path.basename(project_dir),
            path=project_dir,
            description=""
        )
        self.state.source_project = project
        self.sync_to_disk()
        return project
    
    def create_target_project(
        self,
        name: str,
        dirpath: str,
        description: str = ""
    ) -> TargetProject:
        """Create target Rust project."""
        import uuid
        
        os.makedirs(dirpath, exist_ok=True)
        
        project = TargetProject(
            id=str(uuid.uuid4()),
            name=name,
            path=dirpath,
            description=description
        )
        project.ensure_structure()
        project.write_cargo_toml()
        
        self.state.target_project = project
        self.sync_to_disk()
        return project
    
    def update_source_project_description(self, description: str) -> None:
        """Update source project description."""
        if self.state.source_project:
            self.state.source_project.description = description
            self.sync_to_disk()
    
    # ============ Module Translation Management ============
    
    def add_module_translation(self, module: ModuleTranslation) -> None:
        """Add a module translation."""
        self.state.module_translations.append(module)
        self.sync_to_disk()
    
    def get_module_translation_by_id(self, module_id: str) -> Optional[ModuleTranslation]:
        """Get a module translation by ID."""
        for module in self.state.module_translations:
            if module.id == module_id:
                return module
        return None
    
    def update_module_translation_info(
        self,
        module_id: str,
        name: str,
        description: str
    ) -> None:
        """Update module translation info."""
        module = self.get_module_translation_by_id(module_id)
        if module and self.state.target_project:
            module.name = name
            module.description = description
            module.path = os.path.join(self.state.target_project.path, name)
            self.sync_to_disk()
    
    def update_module_translation_status(
        self,
        module_id: str,
        status: ModuleTranslationStatus
    ) -> None:
        """Update module translation status."""
        module = self.get_module_translation_by_id(module_id)
        if module:
            module.status = status
            self.sync_to_disk()
            
            # Backup state when module is done or failed
            if status in (ModuleTranslationStatus.DONE, ModuleTranslationStatus.FAILED):
                self._backup_module_state(module)
    
    def _backup_module_state(self, module: ModuleTranslation) -> None:
        """Backup state for a completed module."""
        import copy
        
        backup_path = os.path.join(
            os.path.dirname(self.bound_filepath),
            f"{module.name}_state.json"
        )
        
        with open(backup_path, "w", encoding="utf-8") as f:
            backup_state = State()
            backup_state.source_project = self.state.source_project
            backup_state.target_project = self.state.target_project
            backup_state.module_translations = [module]
            
            json.dump(
                backup_state.to_dict(),
                f,
                ensure_ascii=False,
                indent=4,
                cls=EnumEncoder
            )
    
    def add_module_rust_files(
        self,
        module_id: str,
        rust_files: List[str]
    ) -> None:
        """Add generated Rust files to a module."""
        module = self.get_module_translation_by_id(module_id)
        if module:
            for rust_file in rust_files:
                if rust_file not in module.related_rust_files:
                    module.related_rust_files.append(rust_file)
            self.sync_to_disk()
    
    # ============ Translation Task Management ============
    
    def update_translation_task_status(
        self,
        module_id: str,
        task_id: str,
        status: TranslationTaskStatus
    ) -> None:
        """Update a translation task status."""
        module = self.get_module_translation_by_id(module_id)
        if module:
            task = module.get_translation_task_by_id(task_id)
            if task:
                task.status = status
                self.sync_to_disk()
    
    def set_translation_task_target(
        self,
        module_id: str,
        task_id: str,
        target: TranslationTaskTarget
    ) -> None:
        """Set the translation result for a task."""
        module = self.get_module_translation_by_id(module_id)
        if module:
            task = module.get_translation_task_by_id(task_id)
            if task:
                task.target = target
                self.sync_to_disk()

