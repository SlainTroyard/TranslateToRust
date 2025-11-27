"""
Tech Leader Agent - Manages module translation workflow.

"""

import os
import asyncio
from typing import Optional, List
import logging

from rustify.agents.base import BaseAgent
from rustify.agents.reasoner import Reasoner
from rustify.schema.response import (
    AgentResponse,
    TechLeaderResponseType,
)
from rustify.schema.translation import (
    ModuleTranslation,
    ModuleTranslationStatus,
    TranslationTask,
    TranslationTaskStatus,
)
from rustify.state.state_manager import StateManager


class TechLeader(BaseAgent):
    """
    Tech Leader agent responsible for:
    - Managing translation workflow for a module
    - Coordinating CodeMonkey agents
    - Initiating test and benchmark phases
    """
    
    ROLE = "tech_leader"
    DESCRIPTION = "An AI assistant responsible for managing the translation process of a module."
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        reasoner_config: Optional[dict] = None,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Tech Leader.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            reasoner_config: Reasoner LLM configuration.
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.reasoner_config = reasoner_config or llm_config
        self.reasoner = Reasoner(self.reasoner_config, logger=self.logger)
        
        self.current_module_id: Optional[str] = None
    
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
            TechLeaderResponseType.MODULE_DONE,
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
            TechLeaderResponseType.PREPARE_MODULE_TRANSLATION_TASKS,
            {"task_count": len(module.translation_tasks)}
        )
    
    def execute_translations(self) -> AgentResponse:
        """Execute all translation tasks."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                TechLeaderResponseType.MODULE_TRANSLATION_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Executing {len(module.translation_tasks)} translation tasks")
        
        # Import CodeMonkey here to avoid circular imports
        from rustify.agents.code_monkey import CodeMonkey
        
        # Create CodeMonkey
        code_monkey = CodeMonkey(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            reasoner_config=self.reasoner_config,
            logger=self.logger
        )
        
        # Execute each task
        for task in module.translation_tasks:
            if task.status == TranslationTaskStatus.DONE:
                continue
            
            self.logger.info(f"Translating task: {task.source.name}")
            
            response = code_monkey.translate(module.id, task)
            
            if response.status != "done":
                self.logger.error(f"Task failed: {task.source.name}")
            else:
                self.logger.info(f"Task completed: {task.source.name}")
        
        return AgentResponse.done(
            self,
            TechLeaderResponseType.MODULE_TRANSLATION_DONE
        )
    
    def run_tests(self) -> AgentResponse:
        """Run tests for the module."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                TechLeaderResponseType.MODULE_TEST_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Running tests for module: {module.name}")
        
        # Update status
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.TEST
        )
        
        # Import TestEngineer here
        from rustify.agents.test_engineer import TestEngineer
        
        test_engineer = TestEngineer(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            reasoner_config=self.reasoner_config,
            logger=self.logger
        )
        
        response = test_engineer.start(module)
        
        return AgentResponse.done(
            self,
            TechLeaderResponseType.MODULE_TEST_DONE,
            {"test_passed": response.status == "done"}
        )
    
    def run_benchmarks(self) -> AgentResponse:
        """Run benchmarks for the module."""
        module = self.module
        if not module:
            return AgentResponse.error(
                self,
                TechLeaderResponseType.MODULE_BENCH_DONE,
                {"message": "No module selected"}
            )
        
        self.logger.info(f"Running benchmarks for module: {module.name}")
        
        # Update status
        self.state_manager.update_module_translation_status(
            module.id,
            ModuleTranslationStatus.BENCHMARK
        )
        
        # Import BenchEngineer here
        from rustify.agents.bench_engineer import BenchEngineer
        
        bench_engineer = BenchEngineer(
            state_manager=self.state_manager,
            llm_config=self.llm_config,
            reasoner_config=self.reasoner_config,
            logger=self.logger
        )
        
        response = bench_engineer.start(module)
        
        return AgentResponse.done(
            self,
            TechLeaderResponseType.MODULE_BENCH_DONE,
            {"bench_passed": response.status == "done"}
        )

