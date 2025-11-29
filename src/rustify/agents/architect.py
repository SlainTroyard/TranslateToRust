"""
Architect Agent - Manages module translation workflow.

"""

import os
import asyncio
from typing import Optional, List, Callable
import logging

from rustify.agents.base import BaseAgent
from rustify.agents.analyzer import Analyzer
from rustify.schema.response import (
    AgentResponse,
    ArchitectResponseType,
)
from rustify.schema.translation import (
    ModuleTranslation,
    ModuleTranslationStatus,
    TranslationTask,
    TranslationTaskStatus,
)
from rustify.state.state_manager import StateManager


class Architect(BaseAgent):
    """
    Architect agent responsible for:
    - Managing translation workflow for a module
    - Coordinating Translator agents
    - Initiating test and benchmark phases
    """
    
    ROLE = "architect"
    DESCRIPTION = "An AI assistant responsible for managing the translation process of a module."
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        analyzer_config: Optional[dict] = None,
        *,
        max_fix_attempts: int = 10,
        max_syntax_attempts: int = 20,
        max_logic_attempts: int = 10,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None,
        on_task_start: Optional[Callable[[str], None]] = None,
        on_task_done: Optional[Callable[[str], None]] = None,
        on_task_error: Optional[Callable[[str, str], None]] = None,
    ):
        """
        Initialize the Architect.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            analyzer_config: Analyzer LLM configuration.
            max_fix_attempts: Max compilation fix attempts for Translator.
            max_syntax_attempts: Max syntax fix attempts for Validator.
            max_logic_attempts: Max logic fix attempts for Validator.
            name: Agent name.
            logger: Logger instance.
            on_task_start: Callback when a task starts.
            on_task_done: Callback when a task completes.
            on_task_error: Callback when a task fails.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.analyzer_config = analyzer_config or llm_config
        self.analyzer = Analyzer(self.analyzer_config, logger=self.logger)
        
        # Fix attempt limits
        self.max_fix_attempts = max_fix_attempts
        self.max_syntax_attempts = max_syntax_attempts
        self.max_logic_attempts = max_logic_attempts
        
        self.current_module_id: Optional[str] = None
        
        # Dashboard callbacks
        self.on_task_start = on_task_start
        self.on_task_done = on_task_done
        self.on_task_error = on_task_error
    
    @property
    def module(self) -> Optional[ModuleTranslation]:
        """Get the current module being worked on."""
        if self.current_module_id:
            return self.state_manager.get_module_translation_by_id(self.current_module_id)
        return None
    
    def start(self, module: ModuleTranslation) -> AgentResponse:
        """
        Start translation for a module.
        
        Args:
            module: Module to translate.
            
        Returns:
            AgentResponse indicating status.
        """
        self.logger.info(f"Starting translation for module: {module.name}")
        self.current_module_id = module.id
        
        # Prepare translation tasks
        response = self.prepare_translation_tasks(module)
        if response.status != "done":
            return response
        
        # Execute translation
        response = self.execute_translations()
        if response.status != "done":
            return response
        
        # Run tests
        response = self.run_tests()
        if response.status != "done":
            self.logger.warning("Tests failed, but continuing")
        
        # Run benchmarks
        response = self.run_benchmarks()
        
        # Mark module as done
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.DONE
        )
        
        return AgentResponse.done(
            self,
            ArchitectResponseType.MODULE_DONE,
            {"module_name": module.name}
        )
    
    def prepare_translation_tasks(
        self,
        module: ModuleTranslation
    ) -> AgentResponse:
        """Prepare translation tasks for the module."""
        self.logger.info(f"Preparing {len(module.translation_tasks)} translation tasks")
        
        # Update module status
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.TRANSPILE
        )
        
        # Set module path
        if self.state_manager.state.target_project:
            target_path = self.state_manager.state.target_project.path
            module.path = target_path  # Use main project as module path
        
        return AgentResponse.done(
            self,
            ArchitectResponseType.PREPARE_MODULE_TRANSLATION_TASKS,
            {"task_count": len(module.translation_tasks)}
        )
    
    def execute_translations(self) -> AgentResponse:
        """Execute all translation tasks."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                ArchitectResponseType.MODULE_TRANSLATION_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Executing {len(module.translation_tasks)} translation tasks")
        
        # Import Translator here to avoid circular imports
        from rustify.agents.translator import Translator
        
        # Create Translator
        translator = Translator(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            analyzer_config=self.analyzer_config,
            max_fix_attempts=self.max_fix_attempts,
            logger=self.logger
        )
        
        # Execute each task
        for task in module.translation_tasks:
            if task.status == TranslationTaskStatus.DONE:
                continue
            
            task_name = task.source.name
            self.logger.info(f"Translating task: {task_name}")
            
            # Notify dashboard: task started
            if self.on_task_start:
                try:
                    self.on_task_start(task_name)
                except Exception:
                    pass
            
            response = translator.translate(module.id, task)
            
            if response.status != "done":
                self.logger.error(f"Task failed: {task_name}")
                # Notify dashboard: task error
                if self.on_task_error:
                    try:
                        self.on_task_error(task_name, response.error or "Unknown error")
                    except Exception:
                        pass
            else:
                self.logger.info(f"Task completed: {task_name}")
                # Notify dashboard: task done
                if self.on_task_done:
                    try:
                        self.on_task_done(task_name)
                    except Exception:
                        pass
        
        return AgentResponse.done(
            self,
            ArchitectResponseType.MODULE_TRANSLATION_DONE
        )
    
    def run_tests(self) -> AgentResponse:
        """Run tests for the module."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                ArchitectResponseType.MODULE_TEST_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Running tests for module: {module.name}")
        
        # Update status
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.TEST
        )
        
        # Import Validator here
        from rustify.agents.validator import Validator
        
        validator = Validator(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            analyzer_config=self.analyzer_config,
            max_syntax_attempts=self.max_syntax_attempts,
            max_logic_attempts=self.max_logic_attempts,
            logger=self.logger
        )
        
        response = validator.start(module)
        
        return AgentResponse.done(
            self,
            ArchitectResponseType.MODULE_TEST_DONE,
            {"test_passed": response.status == "done"}
        )
    
    def run_benchmarks(self) -> AgentResponse:
        """Run benchmarks for the module."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                ArchitectResponseType.MODULE_BENCH_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Running benchmarks for module: {module.name}")
        
        # Update status
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.BENCHMARK
        )
        
        # Import Benchmarker here
        from rustify.agents.benchmarker import Benchmarker
        
        benchmarker = Benchmarker(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            analyzer_config=self.analyzer_config,
            logger=self.logger
        )
        
        response = benchmarker.start(module)
        
        return AgentResponse.done(
            self,
            ArchitectResponseType.MODULE_BENCH_DONE,
            {"bench_passed": response.status == "done"}
        )

