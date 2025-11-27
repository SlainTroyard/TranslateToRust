"""
Tests for the Template module.
"""

import pytest
from pathlib import Path
import tempfile

from rustify.templates import TemplateEngine


@pytest.fixture
def temp_templates():
    """Create temporary template directory."""
    with tempfile.TemporaryDirectory() as tmpdir:
        tmpdir = Path(tmpdir)
        
        # Create test template
        (tmpdir / "test.jinja2").write_text(
            "Hello {{ name }}! Count: {{ items | length }}"
        )
        
        # Create nested template
        (tmpdir / "nested").mkdir()
        (tmpdir / "nested" / "sub.jinja2").write_text(
            "{% for item in items %}{{ item }}{% endfor %}"
        )
        
        yield tmpdir


def test_template_engine_render(temp_templates):
    """Test basic template rendering."""
    engine = TemplateEngine(temp_templates)
    
    result = engine.render(
        "test.jinja2",
        name="World",
        items=[1, 2, 3]
    )
    
    assert result == "Hello World! Count: 3"


def test_template_engine_nested(temp_templates):
    """Test nested template rendering."""
    engine = TemplateEngine(temp_templates)
    
    result = engine.render(
        "nested/sub.jinja2",
        items=["a", "b", "c"]
    )
    
    assert result == "abc"


def test_template_engine_render_string():
    """Test string template rendering."""
    engine = TemplateEngine(Path("."))
    
    result = engine.render_string(
        "{{ x + y }}",
        x=1,
        y=2
    )
    
    assert result == "3"


def test_template_engine_custom_filters(temp_templates):
    """Test custom filters."""
    engine = TemplateEngine(temp_templates)
    
    # Test indent_code filter
    result = engine.render_string(
        "{{ code | indent_code(2) }}",
        code="fn main() {}"
    )
    
    assert result == "  fn main() {}"


def test_template_engine_has_template(temp_templates):
    """Test template existence check."""
    engine = TemplateEngine(temp_templates)
    
    assert engine.has_template("test.jinja2")
    assert not engine.has_template("nonexistent.jinja2")

