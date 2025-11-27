"""
Jinja2 Template Engine for Rustify.
"""

from pathlib import Path
from typing import Any, Optional

from jinja2 import Environment, FileSystemLoader, select_autoescape


class TemplateEngine:
    """
    Jinja2-based template engine for generating prompts.
    
    Example:
        ```python
        engine = TemplateEngine(Path("templates"))
        prompt = engine.render(
            "translation/direct.jinja2",
            unit=translation_unit,
            context=pipeline_context,
        )
        ```
    """
    
    def __init__(
        self,
        templates_dir: Path | str,
        auto_reload: bool = True
    ):
        self.templates_dir = Path(templates_dir)
        
        # Create Jinja2 environment
        self.env = Environment(
            loader=FileSystemLoader(str(self.templates_dir)),
            autoescape=select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
            auto_reload=auto_reload,
        )
        
        # Register custom filters
        self._register_filters()
        
        # Register custom globals
        self._register_globals()
    
    def _register_filters(self) -> None:
        """Register custom Jinja2 filters."""
        
        def indent_code(code: str, spaces: int = 4) -> str:
            """Indent code by specified spaces."""
            indent = " " * spaces
            lines = code.split("\n")
            return "\n".join(indent + line for line in lines)
        
        def truncate_code(code: str, max_lines: int = 50) -> str:
            """Truncate code to max lines."""
            lines = code.split("\n")
            if len(lines) > max_lines:
                return "\n".join(lines[:max_lines]) + "\n// ... truncated ..."
            return code
        
        def escape_rust_string(s: str) -> str:
            """Escape string for Rust."""
            return s.replace("\\", "\\\\").replace('"', '\\"')
        
        def to_rust_identifier(name: str) -> str:
            """Convert name to valid Rust identifier."""
            import re
            name = re.sub(r'[^a-zA-Z0-9_]', '_', name)
            if name and name[0].isdigit():
                name = '_' + name
            return name.lower()
        
        self.env.filters["indent_code"] = indent_code
        self.env.filters["truncate_code"] = truncate_code
        self.env.filters["escape_rust_string"] = escape_rust_string
        self.env.filters["to_rust_identifier"] = to_rust_identifier
    
    def _register_globals(self) -> None:
        """Register custom global functions."""
        
        def now() -> str:
            """Get current timestamp."""
            from datetime import datetime
            return datetime.now().isoformat()
        
        self.env.globals["now"] = now
    
    def render(
        self,
        template_name: str,
        **kwargs: Any
    ) -> str:
        """
        Render a template with the given context.
        
        Args:
            template_name: Path to template relative to templates_dir.
            **kwargs: Template context variables.
            
        Returns:
            Rendered template string.
        """
        template = self.env.get_template(template_name)
        return template.render(**kwargs)
    
    def render_string(
        self,
        template_string: str,
        **kwargs: Any
    ) -> str:
        """
        Render a template string directly.
        
        Args:
            template_string: Template content as string.
            **kwargs: Template context variables.
            
        Returns:
            Rendered string.
        """
        template = self.env.from_string(template_string)
        return template.render(**kwargs)
    
    def get_template(self, template_name: str):
        """Get a template object for later rendering."""
        return self.env.get_template(template_name)
    
    def has_template(self, template_name: str) -> bool:
        """Check if a template exists."""
        try:
            self.env.get_template(template_name)
            return True
        except Exception:
            return False


# Default templates directory
_default_templates_dir = Path(__file__).parent / "prompts"


def get_template_engine(
    templates_dir: Optional[Path | str] = None
) -> TemplateEngine:
    """Get or create a template engine instance."""
    if templates_dir is None:
        templates_dir = _default_templates_dir
    return TemplateEngine(templates_dir)

