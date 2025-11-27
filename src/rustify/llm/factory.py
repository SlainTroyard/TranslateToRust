"""
LLM Factory - Create LLM adapters based on configuration.
"""

from typing import Optional
from loguru import logger

from rustify.config import LLMConfig
from rustify.llm.adapter import LLMAdapter, LiteLLMAdapter


def get_llm_adapter(config: LLMConfig | dict) -> LLMAdapter:
    """
    Create an LLM adapter based on configuration.
    
    Args:
        config: LLM configuration (LLMConfig or dict).
        
    Returns:
        Configured LLM adapter.
    """
    if isinstance(config, LLMConfig):
        config_dict = config.model_dump()
    else:
        config_dict = config
    
    provider = config_dict.get("provider", "openai")
    
    logger.debug(f"Creating LLM adapter for provider: {provider}")
    
    # Use LiteLLM for all providers
    return LiteLLMAdapter(config_dict)


def create_adapter_for_provider(
    provider: str,
    model: str,
    api_key: Optional[str] = None,
    base_url: Optional[str] = None,
    **kwargs
) -> LLMAdapter:
    """
    Convenience function to create an adapter for a specific provider.
    
    Args:
        provider: Provider name (openai, anthropic, deepseek, etc.)
        model: Model name.
        api_key: API key.
        base_url: Base URL for API.
        **kwargs: Additional configuration.
        
    Returns:
        Configured LLM adapter.
    """
    config = {
        "provider": provider,
        "model": model,
        "api_key": api_key,
        "base_url": base_url,
        **kwargs
    }
    
    return get_llm_adapter(config)


# Preset configurations for common providers
PRESETS = {
    "openai-gpt4": {
        "provider": "openai",
        "model": "gpt-4",
    },
    "openai-gpt4o": {
        "provider": "openai",
        "model": "gpt-4o",
    },
    "anthropic-claude3": {
        "provider": "anthropic",
        "model": "claude-3-opus-20240229",
    },
    "anthropic-claude35": {
        "provider": "anthropic",
        "model": "claude-3-5-sonnet-20241022",
    },
    "deepseek-v3": {
        "provider": "deepseek",
        "model": "deepseek-chat",
        "base_url": "https://api.deepseek.com/v1",
    },
    "deepseek-r1": {
        "provider": "deepseek",
        "model": "deepseek-reasoner",
        "base_url": "https://api.deepseek.com/v1",
    },
}


def get_preset_adapter(
    preset_name: str,
    api_key: Optional[str] = None
) -> LLMAdapter:
    """
    Get an adapter using a preset configuration.
    
    Args:
        preset_name: Name of the preset.
        api_key: Optional API key override.
        
    Returns:
        Configured LLM adapter.
    """
    if preset_name not in PRESETS:
        available = ", ".join(PRESETS.keys())
        raise ValueError(f"Unknown preset: {preset_name}. Available: {available}")
    
    config = PRESETS[preset_name].copy()
    if api_key:
        config["api_key"] = api_key
    
    return get_llm_adapter(config)

