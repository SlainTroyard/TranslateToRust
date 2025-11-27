"""
Rustify Prompts - Jinja2 template-based prompt system.

"""

import os
from typing import Optional, Any, Dict
from jinja2 import Environment, FileSystemLoader, select_autoescape


class PromptLoader:
    """
    Load and render prompt templates using Jinja2.
    
    Templates are organized by agent role:
    - prompts/project_manager/
    - prompts/tech_leader/
    - prompts/code_monkey/
    - prompts/test_engineer/
    - prompts/bench_engineer/
    """
    
    _instance: Optional["PromptLoader"] = None
    _env: Optional[Environment] = None
    
    def __init__(self, template_dirs: Optional[list] = None):
        """
        Initialize the prompt loader.
        
        Args:
            template_dirs: List of template directories.
        """
        if template_dirs is None:
            # Default to prompts directory in this package
            template_dirs = [
                os.path.join(os.path.dirname(__file__), "templates")
            ]
        
        self._env = Environment(
            loader=FileSystemLoader(template_dirs),
            autoescape=select_autoescape(['html', 'xml']),
            trim_blocks=True,
            lstrip_blocks=True
        )
    
    @classmethod
    def get_instance(cls) -> "PromptLoader":
        """Get the singleton instance."""
        if cls._instance is None:
            cls._instance = PromptLoader()
        return cls._instance
    
    @classmethod
    def get_prompt(cls, template_name: str, **kwargs) -> str:
        """
        Load and render a prompt template.
        
        Args:
            template_name: Template name (e.g., 'code_monkey/translate.jinja2').
            **kwargs: Variables to pass to the template.
            
        Returns:
            Rendered prompt string.
        """
        instance = cls.get_instance()
        
        # Add .jinja2 extension if not present
        if not template_name.endswith('.jinja2') and not template_name.endswith('.prompt'):
            template_name += '.jinja2'
        
        try:
            template = instance._env.get_template(template_name)
            return template.render(**kwargs)
        except Exception as e:
            # Return a fallback prompt
            return f"# Error loading template {template_name}: {e}\n\n{kwargs}"
    
    def add_template_dir(self, directory: str) -> None:
        """Add a template directory."""
        if self._env:
            loader = self._env.loader
            if hasattr(loader, 'searchpath'):
                loader.searchpath.append(directory)
    
    def list_templates(self) -> list:
        """List all available templates."""
        if self._env:
            return list(self._env.list_templates())
        return []


# Convenience function
def get_prompt(template_name: str, **kwargs) -> str:
    """Load and render a prompt template."""
    return PromptLoader.get_prompt(template_name, **kwargs)

