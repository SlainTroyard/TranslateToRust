"""
Base Agent - Foundation for all agents in the system.

"""

import re
import json
import logging
from typing import Optional, Any, List, Type
from abc import ABC
from dataclasses import dataclass

from pydantic import BaseModel

from rustify.schema.response import AgentResponse, AgentResponseStatus


@dataclass
class LLMMessage:
    """A message in an LLM conversation."""
    role: str
    content: str
    reasoning_content: Optional[str] = None


@dataclass
class LLMChoice:
    """A choice from an LLM response."""
    message: LLMMessage
    index: int = 0


@dataclass
class LLMResponse:
    """Response from an LLM call."""
    choices: List[LLMChoice]
    format_object: Optional[Any] = None
    usage: Optional[dict] = None
    
    @property
    def content(self) -> str:
        """Get the first choice content."""
        if self.choices:
            return self.choices[0].message.content
        return ""


class BaseAgent(ABC):
    """
    Base class for all agents.
    
    Provides:
    - LLM interaction
    - JSON parsing
    - Response handling
    - Logging
    """
    
    ROLE = "base"
    DESCRIPTION = "A powerful AI assistant that completes tasks accurately."
    
    # Pattern to extract JSON from markdown code blocks
    JSON_PATTERN = re.compile(r"```json\s*(.*?)\s*```", re.DOTALL)
    # Pattern to extract Rust code from markdown code blocks
    RUST_PATTERN = re.compile(r"```rust\s*(.*?)\s*```", re.DOTALL)
    
    def __init__(
        self,
        llm_config: dict,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the agent.
        
        Args:
            llm_config: LLM configuration dict.
            name: Agent name (defaults to ROLE).
            logger: Logger instance.
        """
        self.llm_config = llm_config
        self.name = name or self.ROLE
        self.logger = logger or self._setup_logger()
        
        # Messages history
        self.messages: List[dict] = []
    
    def _setup_logger(self) -> logging.Logger:
        """Set up a default logger."""
        logger = logging.getLogger(f"rustify.{self.ROLE}.{self.name}")
        if not logger.handlers:
            handler = logging.StreamHandler()
            handler.setFormatter(logging.Formatter(
                f"[%(asctime)s] [{self.ROLE}] %(levelname)s: %(message)s"
            ))
            logger.addHandler(handler)
            logger.setLevel(logging.INFO)
        return logger
    
    def call_llm(
        self,
        messages: List[dict],
        temperature: float = 0.7,
        max_tokens: Optional[int] = None,
        json_format: Optional[Type[BaseModel]] = None,
        **kwargs
    ) -> LLMResponse:
        """
        Call the LLM with messages.
        
        Args:
            messages: Conversation messages.
            temperature: Sampling temperature.
            max_tokens: Maximum tokens to generate.
            json_format: Pydantic model for JSON output parsing.
            
        Returns:
            LLMResponse with the model's response.
        """
        from rustify.llm.adapter import LiteLLMAdapter
        
        # Create adapter from config
        adapter = LiteLLMAdapter(
            model=self.llm_config.get("model", "gpt-4"),
            api_key=self.llm_config.get("api_key"),
            base_url=self.llm_config.get("base_url"),
            **self.llm_config.get("extra_params", {})
        )
        
        # Build system message
        system_message = {
            "role": "system",
            "content": f"You are {self.DESCRIPTION}"
        }
        
        # Prepare messages
        full_messages = [system_message] + messages
        
        # Call LLM with agent info for logging
        try:
            response = adapter.chat(
                messages=full_messages,
                temperature=temperature,
                max_tokens=max_tokens,
                agent_name=self.name,
                agent_role=self.ROLE
            )
            
            # Parse response
            content = response.get("content", "")
            reasoning = response.get("reasoning_content")
            
            llm_response = LLMResponse(
                choices=[LLMChoice(
                    message=LLMMessage(
                        role="assistant",
                        content=content,
                        reasoning_content=reasoning
                    )
                )],
                usage=response.get("usage")
            )
            
            # Try to parse JSON if format specified
            if json_format:
                llm_response.format_object = self._parse_json(content, json_format)
            
            return llm_response
            
        except Exception as e:
            self.logger.error(f"LLM call failed: {e}")
            raise
    
    def _parse_json(
        self,
        content: str,
        model_class: Type[BaseModel]
    ) -> Optional[BaseModel]:
        """
        Parse JSON from LLM response content.
        
        Args:
            content: Raw response content.
            model_class: Pydantic model class.
            
        Returns:
            Parsed model instance or None.
        """
        # Try to find JSON in code block
        match = self.JSON_PATTERN.search(content)
        if match:
            json_str = match.group(1)
        else:
            # Try to find raw JSON
            json_str = content.strip()
        
        try:
            data = json.loads(json_str)
            return model_class(**data)
        except (json.JSONDecodeError, ValueError) as e:
            self.logger.warning(f"Failed to parse JSON: {e}")
            return None
    
    def extract_rust_code(self, content: str) -> Optional[str]:
        """
        Extract Rust code from markdown content.
        
        Args:
            content: Content with code blocks.
            
        Returns:
            Extracted Rust code or None.
        """
        matches = self.RUST_PATTERN.findall(content)
        if matches:
            # Return the last Rust code block
            return matches[-1].strip()
        return None
    
    def add_message(self, role: str, content: str) -> None:
        """Add a message to the conversation history."""
        self.messages.append({"role": role, "content": content})
    
    def clear_messages(self) -> None:
        """Clear the conversation history."""
        self.messages.clear()


class TranspileMemory:
    """
    Memory system for storing translation experiences.
    
    Helps agents learn from past translations and fixes.
    """
    
    def __init__(self):
        self.experiences: List[dict] = []
    
    def add_experience(
        self,
        source_code: str,
        target_code: str,
        source_type: str,
        success: bool,
        notes: Optional[str] = None
    ) -> None:
        """Add a translation experience."""
        self.experiences.append({
            "source_code": source_code,
            "target_code": target_code,
            "source_type": source_type,
            "success": success,
            "notes": notes
        })
    
    def find_similar(
        self,
        source_type: str,
        limit: int = 5
    ) -> List[dict]:
        """Find similar successful experiences."""
        matches = [
            exp for exp in self.experiences
            if exp["source_type"] == source_type and exp["success"]
        ]
        return matches[-limit:]
    
    def to_context(self, source_type: str) -> str:
        """Generate context string from similar experiences."""
        similar = self.find_similar(source_type)
        if not similar:
            return ""
        
        context_parts = ["## Previous Translation Examples\n"]
        for exp in similar:
            context_parts.append(f"### Source ({exp['source_type']}):\n```c\n{exp['source_code']}\n```\n")
            context_parts.append(f"### Target:\n```rust\n{exp['target_code']}\n```\n")
            if exp.get("notes"):
                context_parts.append(f"Notes: {exp['notes']}\n")
        
        return "\n".join(context_parts)

