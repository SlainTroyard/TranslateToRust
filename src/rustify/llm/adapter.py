"""
LLM Adapter - Unified interface for LLM providers.

Uses LiteLLM for multi-provider support.
"""

import os
import logging
from typing import Optional, List, Dict, Any

logger = logging.getLogger("rustify.llm")

# Global flag for conversation logging
_enable_conversation_logging = True


class LiteLLMAdapter:
    """
    Adapter for LLM providers using LiteLLM.
    
    Supports:
    - OpenAI (gpt-4, gpt-3.5-turbo)
    - DeepSeek (deepseek-chat, deepseek-reasoner)
    - Anthropic (claude-3)
    - And many more via LiteLLM
    """
    
    def __init__(
        self,
        model: str,
        api_key: Optional[str] = None,
        base_url: Optional[str] = None,
        **kwargs
    ):
        """
        Initialize the LLM adapter.
        
        Args:
            model: Model name (e.g., 'gpt-4', 'deepseek-chat').
            api_key: API key (or from environment).
            base_url: API base URL (optional).
            **kwargs: Additional parameters.
        """
        self.model = model
        self.api_key = api_key or os.getenv("OPENAI_API_KEY")
        self.base_url = base_url
        self.extra_params = kwargs
        
        # Set environment for LiteLLM
        if self.api_key:
            os.environ["OPENAI_API_KEY"] = self.api_key
        if self.base_url:
            os.environ["OPENAI_API_BASE"] = self.base_url
    
    def chat(
        self,
        messages: List[Dict[str, str]],
        temperature: float = 1.0,
        top_p: float = 1.0,
        max_tokens: Optional[int] = None,
        agent_name: str = "default",
        agent_role: str = "assistant",
        **kwargs
    ) -> Dict[str, Any]:
        """
        Send a chat completion request.
        
        Args:
            messages: Conversation messages.
            temperature: Sampling temperature.
            max_tokens: Max tokens to generate.
            agent_name: Name of the calling agent (for logging).
            agent_role: Role of the calling agent (for logging).
            **kwargs: Additional parameters.
            
        Returns:
            Response dict with 'content', 'reasoning_content', 'usage'.
        """
        try:
            import litellm
            
            # Build parameters
            params = {
                "model": self.model,
                "messages": messages,
                "temperature": temperature,
                "top_p": top_p,
            }
            
            if max_tokens:
                params["max_tokens"] = max_tokens
            
            if self.base_url:
                params["api_base"] = self.base_url
            
            if self.api_key:
                params["api_key"] = self.api_key
            
            # Merge extra params
            params.update(self.extra_params)
            params.update(kwargs)
            
            # Call LiteLLM
            response = litellm.completion(**params)
            
            # Extract content
            choice = response.choices[0]
            content = choice.message.content or ""
            
            # Check for reasoning content (DeepSeek R1, etc.)
            reasoning_content = None
            if hasattr(choice.message, 'reasoning_content'):
                reasoning_content = choice.message.reasoning_content
            
            result = {
                "content": content,
                "reasoning_content": reasoning_content,
                "usage": dict(response.usage) if response.usage else None,
                "model": response.model,
            }
            
            # Log conversation
            self._log_conversation(
                agent_name, agent_role, messages, result,
                temperature, max_tokens
            )
            
            return result
            
        except ImportError:
            logger.warning("LiteLLM not installed, falling back to OpenAI client")
            return self._openai_fallback(messages, temperature, max_tokens)
        except Exception as e:
            logger.error(f"LLM call failed: {e}")
            self._log_error(agent_name, agent_role, messages, e)
            raise
    
    def _log_conversation(
        self,
        agent_name: str,
        agent_role: str,
        messages: List[Dict[str, str]],
        response: Dict[str, Any],
        temperature: float,
        max_tokens: Optional[int]
    ) -> None:
        """Log conversation to file."""
        if not _enable_conversation_logging:
            return
        
        try:
            from rustify.logging.conversation_logger import get_conversation_logger
            conv_logger = get_conversation_logger()
            conv_logger.log_conversation(
                agent_name=agent_name,
                agent_role=agent_role,
                messages=messages,
                response=response,
                model=self.model,
                temperature=temperature,
                max_tokens=max_tokens
            )
        except Exception as e:
            logger.debug(f"Failed to log conversation: {e}")
    
    def _log_error(
        self,
        agent_name: str,
        agent_role: str,
        messages: List[Dict[str, str]],
        error: Exception
    ) -> None:
        """Log error to file."""
        if not _enable_conversation_logging:
            return
        
        try:
            from rustify.logging.conversation_logger import get_conversation_logger
            conv_logger = get_conversation_logger()
            conv_logger.log_error(
                agent_name=agent_name,
                agent_role=agent_role,
                messages=messages,
                error=error,
                model=self.model
            )
        except Exception as e:
            logger.debug(f"Failed to log error: {e}")
    
    def _openai_fallback(
        self,
        messages: List[Dict[str, str]],
        temperature: float,
        max_tokens: Optional[int]
    ) -> Dict[str, Any]:
        """Fallback to direct OpenAI client."""
        try:
            from openai import OpenAI
            
            client_params = {}
            if self.api_key:
                client_params["api_key"] = self.api_key
            if self.base_url:
                client_params["base_url"] = self.base_url
            
            client = OpenAI(**client_params)
            
            params = {
                "model": self.model,
                "messages": messages,
                "temperature": temperature,
            }
            
            if max_tokens:
                params["max_tokens"] = max_tokens
            
            response = client.chat.completions.create(**params)
            
            content = response.choices[0].message.content or ""
            
            return {
                "content": content,
                "reasoning_content": None,
                "usage": dict(response.usage) if response.usage else None,
                "model": response.model,
            }
            
        except Exception as e:
            logger.error(f"OpenAI fallback failed: {e}")
            raise
    
    def stream_chat(
        self,
        messages: List[Dict[str, str]],
        temperature: float = 1.0,
        **kwargs
    ):
        """
        Stream a chat completion response.
        
        Yields content chunks as they arrive.
        """
        try:
            import litellm
            
            params = {
                "model": self.model,
                "messages": messages,
                "temperature": temperature,
                "stream": True,
            }
            
            if self.base_url:
                params["api_base"] = self.base_url
            if self.api_key:
                params["api_key"] = self.api_key
            
            params.update(kwargs)
            
            response = litellm.completion(**params)
            
            for chunk in response:
                if chunk.choices and chunk.choices[0].delta.content:
                    yield chunk.choices[0].delta.content
                    
        except Exception as e:
            logger.error(f"Stream failed: {e}")
            raise


class LLMFactory:
    """Factory for creating LLM adapters."""
    
    @staticmethod
    def create(
        model: str,
        api_key: Optional[str] = None,
        base_url: Optional[str] = None,
        **kwargs
    ) -> LiteLLMAdapter:
        """
        Create an LLM adapter.
        
        Args:
            model: Model name.
            api_key: API key.
            base_url: API base URL.
            **kwargs: Additional parameters.
            
        Returns:
            LiteLLMAdapter instance.
        """
        return LiteLLMAdapter(
            model=model,
            api_key=api_key,
            base_url=base_url,
            **kwargs
        )
    
    @staticmethod
    def from_config(config: Dict[str, Any]) -> LiteLLMAdapter:
        """
        Create an LLM adapter from config dict.
        
        Args:
            config: Configuration dict.
            
        Returns:
            LiteLLMAdapter instance.
        """
        return LiteLLMAdapter(
            model=config.get("model", "gpt-4"),
            api_key=config.get("api_key"),
            base_url=config.get("base_url"),
            **config.get("extra_params", {})
        )
