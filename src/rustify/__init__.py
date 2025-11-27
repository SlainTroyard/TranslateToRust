"""
Rustify - C/C++ to Rust Automatic Translation Tool

A multi-agent system for translating C/C++ code to idiomatic Rust.

"""

import os
import logging
from typing import Optional, Dict, Any, List
from pathlib import Path

from rustify.config import RustifyConfig
from rustify.state.state_manager import StateManager
from rustify.agents.project_manager import ProjectManager
from rustify.agents.tech_leader import TechLeader
from rustify.schema.translation import ModuleTranslation, ModuleTranslationStatus

__version__ = "0.1.0"
__author__ = "Rustify Team"

logger = logging.getLogger("rustify")


class Rustify:
    """
    Main Rustify translator class.
    
    Orchestrates the multi-agent translation workflow:
    1. ProjectManager: Loads and analyzes source project
    2. TechLeader: Manages module translation
    3. CodeMonkey: Performs actual code translation
    4. TestEngineer: Translates and runs tests
    5. BenchEngineer: Generates and runs benchmarks
    """
    
    def __init__(
        self,
        config: Optional[RustifyConfig] = None,
        *,
        llm_config: Optional[Dict[str, Any]] = None,
        reasoner_config: Optional[Dict[str, Any]] = None,
        state_file: Optional[str] = None,
    ):
        """
        Initialize Rustify.
        
        Args:
            config: RustifyConfig instance (or loads from file).
            llm_config: LLM configuration dict.
            reasoner_config: Reasoner LLM configuration (defaults to llm_config).
            state_file: Path to state file for persistence.
        """
        # Load config
        self.config = config or RustifyConfig()
        
        # LLM configs
        self.llm_config = llm_config or self._build_llm_config()
        self.reasoner_config = reasoner_config or self.llm_config
        
        # State file
        self.state_file = state_file
        self.state_manager: Optional[StateManager] = None
        
        # Agents
        self.project_manager: Optional[ProjectManager] = None
        
        # Incremental translation filter
        self._files_filter: Optional[List[str]] = None
        
        # Setup logging
        self._setup_logging()
    
    def _build_llm_config(self) -> Dict[str, Any]:
        """Build LLM config from settings."""
        return {
            "model": self.config.llm.model,
            "api_key": self.config.llm.api_key,
            "base_url": self.config.llm.base_url,
            "extra_params": {
                "temperature": self.config.llm.temperature,
                "top_p": self.config.llm.top_p,
                "max_tokens": self.config.llm.max_tokens,
            }
        }
    
    def _setup_logging(self) -> None:
        """Setup logging based on config."""
        level = getattr(logging, self.config.log_level.upper(), logging.INFO)
        
        logging.basicConfig(
            level=level,
            format="[%(asctime)s] [%(name)s] %(levelname)s: %(message)s",
            datefmt="%Y-%m-%d %H:%M:%S"
        )
    
    def translate(
        self,
        source_dir: str,
        target_dir: str,
        *,
        overwrite: bool = False,
        files_filter: Optional[List[str]] = None,
    ) -> bool:
        """
        Translate a C/C++ project to Rust.
        
        Args:
            source_dir: Path to C/C++ source project.
            target_dir: Path for generated Rust project.
            overwrite: Whether to overwrite existing target.
            files_filter: Optional list of filenames to translate (for incremental).
            
        Returns:
            True if translation succeeded.
        """
        self._files_filter = files_filter
        # Validate source
        source_path = Path(source_dir).resolve()
        if not source_path.exists():
            logger.error(f"Source directory not found: {source_dir}")
            return False
        
        # Setup target
        target_path = Path(target_dir).resolve()
        
        # Setup state file
        if self.state_file:
            state_path = self.state_file
        else:
            state_path = str(target_path / "states.json")
        
        # If overwrite is True, remove old state file
        if overwrite and os.path.exists(state_path):
            os.remove(state_path)
            logger.info("Removed old state file")
        
        # Initialize state manager
        self.state_manager = StateManager(state_path)
        
        logger.info(f"Starting translation")
        logger.info(f"Source: {source_path}")
        logger.info(f"Target: {target_path}")
        
        # Initialize ProjectManager
        self.project_manager = ProjectManager(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            reasoner_config=self.reasoner_config,
        )
        
        # Start project (loads, analyzes, creates target)
        response = self.project_manager.start(
            source_project_dir=str(source_path),
            target_project_dir=str(target_path),
            overwrite=overwrite,
        )
        
        if response.status != "done":
            logger.error(f"Project initialization failed: {response.error}")
            return False
        
        # Process each module
        success = self._process_modules()
        
        if success:
            logger.info("Translation completed successfully!")
        else:
            logger.warning("Translation completed with some failures")
        
        return success
    
    def _process_modules(self) -> bool:
        """Process all modules through translation."""
        all_success = True
        
        for index, module in self.state_manager.state.ready_module_translations:
            # Check if we should skip this module based on files_filter
            if self._files_filter:
                # Get the source file for this module
                module_file = Path(module.source_file).name if hasattr(module, 'source_file') else module.name
                if not any(f in module_file or module_file in f for f in self._files_filter):
                    logger.debug(f"Skipping module {module.name} (not in files_filter)")
                    continue
            
            logger.info(f"Processing module {index + 1}: {module.name}")
            
            # Create TechLeader for this module
            tech_leader = TechLeader(
                state_manager=self.state_manager,
                llm_config=self.llm_config,
                reasoner_config=self.reasoner_config,
            )
            
            # Start module translation
            response = tech_leader.start(module)
            
            if response.status != "done":
                logger.error(f"Module {module.name} failed: {response.error}")
                all_success = False
            else:
                logger.info(f"Module {module.name} completed")
        
        return all_success
    
    def analyze_only(self, source_dir: str) -> Dict[str, Any]:
        """
        Only analyze a project without translating.
        
        Args:
            source_dir: Path to C/C++ source project.
            
        Returns:
            Analysis results dict.
        """
        from rustify.graph.parser import CParser
        
        parser = CParser(source_dir)
        graph = parser.parse_project()
        
        # Get levels for translation order
        levels = graph.topological_sort()
        
        return {
            "node_count": len(graph.nodes),
            "level_count": len(levels),
            "nodes": [
                {
                    "name": n.name,
                    "type": n.type,
                    "location": n.location,
                    "line_count": n.line_count,
                }
                for n in graph.nodes.values()
            ],
            "translation_order": [
                [n.name for n in level]
                for level in levels
            ]
        }
    
    def resume(self, state_file: str) -> bool:
        """
        Resume a previous translation from state file.
        
        Args:
            state_file: Path to state file.
            
        Returns:
            True if resumed successfully.
        """
        self.state_manager = StateManager(state_file)
        
        if not self.state_manager.state.source_project:
            logger.error("No source project in state file")
            return False
        
        if not self.state_manager.state.target_project:
            logger.error("No target project in state file")
            return False
        
        logger.info(f"Resuming translation")
        logger.info(f"Source: {self.state_manager.state.source_project.path}")
        logger.info(f"Target: {self.state_manager.state.target_project.path}")
        
        return self._process_modules()


def translate(
    source_dir: str,
    target_dir: str,
    *,
    llm_config: Optional[Dict[str, Any]] = None,
    overwrite: bool = False,
) -> bool:
    """
    Convenience function to translate a C/C++ project to Rust.
    
    Args:
        source_dir: Path to C/C++ source project.
        target_dir: Path for generated Rust project.
        llm_config: LLM configuration dict.
        overwrite: Whether to overwrite existing target.
        
    Returns:
        True if translation succeeded.
    """
    rustify = Rustify(llm_config=llm_config)
    return rustify.translate(source_dir, target_dir, overwrite=overwrite)


__all__ = [
    "Rustify",
    "translate",
    "RustifyConfig",
    "__version__",
]
