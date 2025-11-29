"""
Response Schema - Agent response types and status codes.

"""

from enum import Enum
from typing import Optional, Any, TYPE_CHECKING
from dataclasses import dataclass, field


class AgentResponseStatus(str, Enum):
    """Status of an agent response."""
    
    DONE = "done"
    ERROR = "error"
    RETRY = "retry"


class AgentResponseType(str, Enum):
    """Base type for agent responses."""
    
    CHAT = "chat"
    TOOL = "tool"


# Orchestrator Response Types
class OrchestratorResponseType(str, Enum):
    """Response types for Orchestrator agent."""
    
    LOAD_SOURCE_PROJECT = "load_source_project"
    SUMMARIZE_SOURCE_PROJECT = "summarize_source_project"
    ANALYZE_DEPENDENCIES = "analyze_dependencies"
    CREATE_TARGET_PROJECT = "create_target_project"
    ALL_MODULES_DONE = "all_modules_done"
    ALL_TASKS_DONE = "all_tasks_done"


# Architect Response Types
class ArchitectResponseType(str, Enum):
    """Response types for Architect agent."""
    
    PREPARE_MODULE_TRANSLATION_TASKS = "prepare_module_translation_tasks"
    MODULE_TRANSLATION_COMPLETION_DONE = "module_translation_completion_done"
    MODULE_TRANSLATION_DONE = "module_translation_done"
    MODULE_TEST_DONE = "module_test_done"
    MODULE_BENCH_DONE = "module_bench_done"
    MODULE_DONE = "module_done"


# Translator Response Types
class TranslatorResponseType(str, Enum):
    """Response types for Translator agent."""
    
    PREPARE_TRANSLATION_TASK = "prepare_translation_task"
    TRANSLATION_COMPLETION = "translation_completion"
    TRANSLATION_COMPLETION_FAILED = "translation_completion_failed"
    COMPILE_CHECK_DONE = "compile_check_done"
    COMPILE_CHECK_FAILED = "compile_check_failed"
    FIX_FAILED = "fix_failed"
    TRANSLATION_TASK_DONE = "translation_task_done"


# Validator Response Types
class ValidatorResponseType(str, Enum):
    """Response types for Validator agent."""
    
    TEST_PREPARE_DONE = "test_prepare_done"
    TEST_CODE_TRANSLATION_COMPLETION = "test_code_translation_completion"
    TEST_SYNTAX_FIX_COMPLETION = "test_syntax_fix_completion"
    TEST_LOGIC_FIX_COMPLETION = "test_logic_fix_completion"
    TEST_RUN_FAILURE = "test_run_failure"
    TEST_RUN_DONE = "test_run_done"
    TEST_PASSED = "test_passed"
    TEST_FAILED = "test_failed"


# Benchmarker Response Types
class BenchmarkerResponseType(str, Enum):
    """Response types for Benchmarker agent."""
    
    BENCH_PREPARE_DONE = "bench_prepare_done"
    BENCH_COMPLETION = "bench_completion"
    BENCH_FIX_DONE = "bench_fix_done"
    BENCH_FAILED = "bench_failed"
    BENCH_DONE = "bench_done"


@dataclass
class AgentResponse:
    """
    Response from an agent.
    
    Contains status, type, data, and optional error information.
    """
    
    status: AgentResponseStatus
    type: str  # One of the response type enums
    agent: Any = None  # Reference to the agent
    data: dict = field(default_factory=dict)
    messages: list = field(default_factory=list)
    error: Optional[dict] = None
    
    @classmethod
    def done(
        cls,
        agent: Any,
        response_type: str,
        data: Optional[dict] = None
    ) -> "AgentResponse":
        """Create a successful response."""
        return cls(
            status=AgentResponseStatus.DONE,
            type=response_type,
            agent=agent,
            data=data or {}
        )
    
    @classmethod
    def error(
        cls,
        agent: Any,
        response_type: str,
        error: dict
    ) -> "AgentResponse":
        """Create an error response."""
        return cls(
            status=AgentResponseStatus.ERROR,
            type=response_type,
            agent=agent,
            error=error
        )
    
    def set_messages(self, messages: list) -> "AgentResponse":
        """Set conversation messages."""
        self.messages = messages
        return self
    
    def __str__(self):
        return f"AgentResponse(status={self.status}, type={self.type}, data={self.data})"
    
    def __repr__(self):
        return self.__str__()
