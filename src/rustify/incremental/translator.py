"""
Incremental Translator - Translate only changed parts of a project.
"""

import os
import logging
from typing import Optional, List, Dict, Any
from pathlib import Path

from rustify.incremental.tracker import ChangeTracker, FileChange, ChangeType
from rustify.state.state_manager import StateManager
from rustify.agents.orchestrator import Orchestrator
from rustify.agents.architect import Architect
from rustify.schema.translation import (
    TranslationTask,
    TranslationTaskSource,
    TranslationUnitNode,
    ModuleTranslation,
    ModuleTranslationStatus,
)

logger = logging.getLogger("rustify.incremental")


class IncrementalTranslator:
    """
    Incremental translator that only translates changed files.
    
    Features:
    - Detects file changes using content hashing
    - Only translates modified/added files
    - Updates Rust code for deleted C files
    - Tracks dependencies for cascading updates
    """
    
    def __init__(
        self,
        llm_config: Dict[str, Any],
        analyzer_config: Optional[Dict[str, Any]] = None,
    ):
        """
        Initialize the incremental translator.
        
        Args:
            llm_config: LLM configuration.
            analyzer_config: Analyzer configuration.
        """
        self.llm_config = llm_config
        self.analyzer_config = analyzer_config or llm_config
        self.tracker: Optional[ChangeTracker] = None
        self.state_manager: Optional[StateManager] = None
    
    def translate_incremental(
        self,
        source_dir: str,
        target_dir: str,
        force_files: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """
        Perform incremental translation.
        
        Args:
            source_dir: Path to C/C++ source project.
            target_dir: Path to Rust target project.
            force_files: Optional list of files to force re-translate.
            
        Returns:
            Translation result summary.
        """
        source_path = Path(source_dir).resolve()
        target_path = Path(target_dir).resolve()
        
        # Initialize change tracker
        self.tracker = ChangeTracker(str(source_path))
        
        # Detect changes
        changes = self.tracker.detect_changes()
        
        # Filter to only files needing translation
        files_to_translate = []
        for change in changes:
            if change.needs_translation:
                files_to_translate.append(change)
            elif force_files and change.path in force_files:
                files_to_translate.append(change)
        
        if not files_to_translate:
            logger.info("No changes detected, nothing to translate")
            return {
                "status": "no_changes",
                "translated_files": 0,
                "summary": self.tracker.get_change_summary(),
            }
        
        logger.info(f"Found {len(files_to_translate)} files to translate")
        
        # Initialize state manager
        state_file = str(target_path / "states.json")
        self.state_manager = StateManager(state_file)
        
        # If target project doesn't exist, do full translation first
        if not target_path.exists() or not self.state_manager.state.target_project:
            logger.info("Target project not found, performing initial translation")
            return self._do_full_translation(source_path, target_path)
        
        # Translate only changed files
        results = self._translate_changed_files(
            source_path, target_path, files_to_translate
        )
        
        # Handle deleted files
        deleted = [c for c in changes if c.change_type == ChangeType.DELETED]
        if deleted:
            self._handle_deleted_files(target_path, deleted)
        
        # Update snapshots
        self.tracker.update_all_snapshots()
        
        return {
            "status": "success",
            "translated_files": len(files_to_translate),
            "deleted_files": len(deleted),
            "results": results,
            "summary": self.tracker.get_change_summary(),
        }
    
    def _do_full_translation(
        self,
        source_path: Path,
        target_path: Path,
    ) -> Dict[str, Any]:
        """Perform full translation for initial setup."""
        from rustify import Rustify
        
        rustify = Rustify(
            llm_config=self.llm_config,
            state_file=str(target_path / "states.json"),
        )
        
        success = rustify.translate(
            source_dir=str(source_path),
            target_dir=str(target_path),
            overwrite=True,
        )
        
        # Update snapshots after full translation
        if self.tracker:
            self.tracker.update_all_snapshots()
        
        return {
            "status": "full_translation",
            "success": success,
        }
    
    def _translate_changed_files(
        self,
        source_path: Path,
        target_path: Path,
        changes: List[FileChange],
    ) -> List[Dict[str, Any]]:
        """Translate only the changed files."""
        results = []
        
        # Create a module for incremental translation
        import uuid
        
        tasks = []
        for change in changes:
            filepath = source_path / change.path
            
            if not filepath.exists():
                continue
            
            try:
                with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                    content = f.read()
            except Exception as e:
                logger.warning(f"Failed to read {change.path}: {e}")
                continue
            
            node = TranslationUnitNode(
                filepath=change.path,
                id=str(uuid.uuid4()),
                name=os.path.basename(change.path),
                type="file",
                text=content,
            )
            
            task = TranslationTask(
                source=TranslationTaskSource(
                    name=os.path.basename(change.path),
                    nodes=[node],
                    description=f"Incremental: {change.change_type.value} - {change.path}"
                )
            )
            tasks.append(task)
        
        if not tasks:
            return results
        
        # Create incremental module
        module = ModuleTranslation(
            name="incremental_update",
            description=f"Incremental translation of {len(tasks)} files",
            translation_tasks=tasks,
            related_files=[c.path for c in changes],
            status=ModuleTranslationStatus.CREATED,
        )
        
        # Add module to state
        self.state_manager.add_module_translation(module)
        
        # Translate using Architect
        architect = Architect(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            analyzer_config=self.analyzer_config,
        )
        
        response = architect.start(module)
        
        results.append({
            "module": module.name,
            "status": response.status,
            "files": [c.path for c in changes],
        })
        
        return results
    
    def _handle_deleted_files(
        self,
        target_path: Path,
        deleted: List[FileChange],
    ) -> None:
        """
        Handle deleted source files.
        
        For now, we log warnings. A more advanced implementation
        could remove corresponding Rust code.
        """
        for change in deleted:
            logger.warning(
                f"Source file deleted: {change.path}. "
                f"You may need to manually remove corresponding Rust code."
            )
            
            # Try to find and warn about corresponding Rust file
            base_name = Path(change.path).stem
            rust_file = target_path / "src" / f"{base_name}.rs"
            
            if rust_file.exists():
                logger.warning(
                    f"Corresponding Rust file exists: {rust_file}. "
                    f"Consider removing it."
                )
    
    def get_pending_changes(self, source_dir: str) -> Dict[str, Any]:
        """
        Get a summary of pending changes without translating.
        
        Args:
            source_dir: Path to C/C++ source project.
            
        Returns:
            Change summary dict.
        """
        tracker = ChangeTracker(source_dir)
        return tracker.get_change_summary()

