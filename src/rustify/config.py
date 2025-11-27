"""
Rustify Configuration - Settings management using Pydantic.

"""

import os
from typing import Optional, List
from pathlib import Path

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class LLMConfig(BaseSettings):
    """LLM provider configuration."""
    
    model: str = Field(default="gpt-4", description="LLM model name")
    api_key: Optional[str] = Field(default=None, description="API key")
    base_url: Optional[str] = Field(default=None, description="API base URL")
    temperature: float = Field(default=1.0, description="Sampling temperature")
    top_p: float = Field(default=1.0, description="Top-p sampling")
    max_tokens: int = Field(default=8192, description="Max tokens to generate")
    
    model_config = SettingsConfigDict(
        env_prefix="RUSTIFY_LLM_",
        env_file=".env",
        extra="ignore"
    )


class ReasonerConfig(BaseSettings):
    """Reasoner LLM configuration (for complex reasoning tasks)."""
    
    model: str = Field(default="gpt-4", description="Reasoner model name")
    api_key: Optional[str] = Field(default=None, description="API key")
    base_url: Optional[str] = Field(default=None, description="API base URL")
    temperature: float = Field(default=1.0, description="Sampling temperature")
    top_p: float = Field(default=1.0, description="Top-p sampling")
    max_tokens: int = Field(default=8192, description="More tokens for reasoning")
    
    model_config = SettingsConfigDict(
        env_prefix="RUSTIFY_REASONER_",
        env_file=".env",
        extra="ignore"
    )


class RustConfig(BaseSettings):
    """Rust toolchain configuration."""
    
    cargo_bin: str = Field(default="cargo", description="Path to cargo")
    rustc_bin: str = Field(default="rustc", description="Path to rustc")
    edition: str = Field(default="2021", description="Rust edition")
    
    model_config = SettingsConfigDict(
        env_prefix="RUSTIFY_RUST_",
        env_file=".env",
        extra="ignore"
    )


class RustifyConfig(BaseSettings):
    """Main Rustify configuration."""
    
    # Sub-configs
    llm: LLMConfig = Field(default_factory=LLMConfig)
    reasoner: ReasonerConfig = Field(default_factory=ReasonerConfig)
    rust: RustConfig = Field(default_factory=RustConfig)
    
    # General settings
    log_level: str = Field(default="INFO", description="Logging level")
    
    # Translation settings
    max_fix_attempts: int = Field(default=10, description="Max error fix attempts")
    max_test_fix_attempts: int = Field(default=20, description="Max test fix attempts")
    parallel_tasks: int = Field(default=1, description="Parallel translation tasks")
    
    # Paths
    prompt_dirs: List[str] = Field(
        default_factory=list,
        description="Additional prompt template directories"
    )
    
    model_config = SettingsConfigDict(
        env_prefix="RUSTIFY_",
        env_file=".env",
        toml_file="rustify.toml",
        extra="ignore"
    )
    
    @classmethod
    def load(cls, config_path: Optional[str] = None) -> "RustifyConfig":
        """
        Load configuration from file.
        
        Args:
            config_path: Path to config file (TOML format).
            
        Returns:
            RustifyConfig instance.
        """
        if config_path and os.path.exists(config_path):
            import toml
            with open(config_path, 'r') as f:
                data = toml.load(f)
            
            # Build config from dict
            return cls(
                llm=LLMConfig(**data.get('llm', {})),
                reasoner=ReasonerConfig(**data.get('reasoner', {})),
                rust=RustConfig(**data.get('rust', {})),
                **{k: v for k, v in data.items() if k not in ('llm', 'reasoner', 'rust')}
            )
        
        return cls()
    
    def save(self, config_path: str) -> None:
        """
        Save configuration to file.
        
        Args:
            config_path: Path to save config file.
        """
        import toml
        
        data = {
            'llm': self.llm.model_dump(),
            'reasoner': self.reasoner.model_dump(),
            'rust': self.rust.model_dump(),
            'log_level': self.log_level,
            'max_fix_attempts': self.max_fix_attempts,
            'max_test_fix_attempts': self.max_test_fix_attempts,
            'parallel_tasks': self.parallel_tasks,
            'prompt_dirs': self.prompt_dirs,
        }
        
        os.makedirs(os.path.dirname(config_path), exist_ok=True)
        with open(config_path, 'w') as f:
            toml.dump(data, f)
