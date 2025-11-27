"""
MCP Tools - Tool definitions and registry for MCP servers.
"""

from dataclasses import dataclass, field
from typing import Any, Callable, Optional
import inspect
import json


@dataclass
class ToolParameter:
    """Definition of a tool parameter."""
    
    name: str
    type: str  # JSON Schema type
    description: str = ""
    required: bool = True
    default: Any = None
    enum: Optional[list[Any]] = None


@dataclass
class Tool:
    """
    Definition of an MCP tool.
    
    Tools are functions that can be invoked by LLM or other MCP clients.
    """
    
    name: str
    description: str
    handler: Callable
    parameters: list[ToolParameter] = field(default_factory=list)
    returns: Optional[str] = None
    
    def to_schema(self) -> dict:
        """Convert to JSON Schema for tool definition."""
        properties = {}
        required = []
        
        for param in self.parameters:
            prop = {"type": param.type, "description": param.description}
            if param.enum:
                prop["enum"] = param.enum
            properties[param.name] = prop
            
            if param.required:
                required.append(param.name)
        
        return {
            "name": self.name,
            "description": self.description,
            "parameters": {
                "type": "object",
                "properties": properties,
                "required": required,
            }
        }
    
    async def execute(self, **kwargs) -> Any:
        """Execute the tool handler."""
        if inspect.iscoroutinefunction(self.handler):
            return await self.handler(**kwargs)
        else:
            return self.handler(**kwargs)


class ToolRegistry:
    """
    Registry for MCP tools.
    
    Manages tool registration, lookup, and execution.
    """
    
    def __init__(self):
        self._tools: dict[str, Tool] = {}
    
    def register(
        self,
        name: Optional[str] = None,
        description: Optional[str] = None
    ) -> Callable:
        """
        Decorator to register a function as a tool.
        
        Example:
            ```python
            registry = ToolRegistry()
            
            @registry.register(
                name="translate_code",
                description="Translate C code to Rust"
            )
            def translate_code(source: str, target_lang: str = "rust") -> str:
                ...
            ```
        """
        def decorator(func: Callable) -> Callable:
            tool = self._create_tool_from_function(func, name, description)
            self._tools[tool.name] = tool
            return func
        
        return decorator
    
    def add_tool(self, tool: Tool) -> None:
        """Add a tool directly."""
        self._tools[tool.name] = tool
    
    def get_tool(self, name: str) -> Optional[Tool]:
        """Get a tool by name."""
        return self._tools.get(name)
    
    def list_tools(self) -> list[Tool]:
        """List all registered tools."""
        return list(self._tools.values())
    
    def get_tools_schema(self) -> list[dict]:
        """Get JSON Schema for all tools."""
        return [tool.to_schema() for tool in self._tools.values()]
    
    async def execute(self, name: str, **kwargs) -> Any:
        """Execute a tool by name."""
        tool = self._tools.get(name)
        if not tool:
            raise KeyError(f"Tool not found: {name}")
        return await tool.execute(**kwargs)
    
    def _create_tool_from_function(
        self,
        func: Callable,
        name: Optional[str] = None,
        description: Optional[str] = None
    ) -> Tool:
        """Create a Tool from a function using introspection."""
        tool_name = name or func.__name__
        tool_description = description or func.__doc__ or ""
        
        # Parse function signature
        sig = inspect.signature(func)
        type_hints = getattr(func, '__annotations__', {})
        
        parameters = []
        for param_name, param in sig.parameters.items():
            if param_name in ('self', 'cls'):
                continue
            
            # Get type hint
            type_hint = type_hints.get(param_name, str)
            param_type = self._python_type_to_json_type(type_hint)
            
            # Check if required
            is_required = param.default == inspect.Parameter.empty
            default_value = None if is_required else param.default
            
            parameters.append(ToolParameter(
                name=param_name,
                type=param_type,
                required=is_required,
                default=default_value,
            ))
        
        return Tool(
            name=tool_name,
            description=tool_description.strip(),
            handler=func,
            parameters=parameters,
        )
    
    def _python_type_to_json_type(self, python_type) -> str:
        """Convert Python type to JSON Schema type."""
        type_map = {
            str: "string",
            int: "integer",
            float: "number",
            bool: "boolean",
            list: "array",
            dict: "object",
        }
        
        # Handle Optional and other typing constructs
        if hasattr(python_type, '__origin__'):
            origin = python_type.__origin__
            if origin in type_map:
                return type_map[origin]
        
        return type_map.get(python_type, "string")


# Pre-defined tools for Rustify

def create_rustify_tools() -> ToolRegistry:
    """Create a registry with Rustify-specific tools."""
    registry = ToolRegistry()
    
    @registry.register(
        name="analyze_c_code",
        description="Analyze C source code and extract structure information (functions, structs, etc.)"
    )
    async def analyze_c_code(source_code: str) -> dict:
        """Analyze C code and return structure."""
        from rustify.agents.chunker import CodeChunker, ChunkKind
        
        chunker = CodeChunker()
        chunks = chunker.chunk(source_code)
        
        result = {
            "functions": [],
            "structs": [],
            "enums": [],
            "macros": [],
            "total_chunks": len(chunks),
        }
        
        for chunk in chunks:
            item = {
                "name": chunk.name,
                "start_line": chunk.start_line,
                "end_line": chunk.end_line,
                "dependencies": chunk.dependencies,
            }
            
            if chunk.kind == ChunkKind.FUNCTION:
                result["functions"].append(item)
            elif chunk.kind == ChunkKind.STRUCT:
                result["structs"].append(item)
            elif chunk.kind == ChunkKind.ENUM:
                result["enums"].append(item)
            elif chunk.kind in (ChunkKind.MACRO, ChunkKind.MACRO_FUNCTION):
                result["macros"].append(item)
        
        return result
    
    @registry.register(
        name="translate_to_rust",
        description="Translate C code to Rust using LLM"
    )
    async def translate_to_rust(
        c_code: str,
        context: str = "",
        output_format: str = "fenced"
    ) -> dict:
        """Translate C code to Rust."""
        from rustify.llm import get_llm_adapter
        from rustify.agents.translate_agent import TranslateAgent
        from rustify.config import get_config
        
        config = get_config()
        llm = get_llm_adapter(config.llm)
        agent = TranslateAgent(llm, output_format=output_format)
        
        result = await agent.translate(c_code, context)
        
        return {
            "success": result.success,
            "rust_code": result.code,
            "explanation": result.explanation,
            "error": result.error,
        }
    
    @registry.register(
        name="translate_chunk",
        description="Translate a single code chunk with incremental context"
    )
    async def translate_chunk(
        chunk_code: str,
        chunk_name: str = "",
        chunk_kind: str = "function",
        previous_code: str = "",
        context: str = ""
    ) -> dict:
        """Translate a code chunk with context from previous translations."""
        from rustify.llm import get_llm_adapter
        from rustify.agents.translate_agent import TranslateAgent
        from rustify.config import get_config
        
        config = get_config()
        llm = get_llm_adapter(config.llm)
        agent = TranslateAgent(llm)
        
        full_context = f"chunk_name={chunk_name}\nchunk_kind={chunk_kind}\n{context}"
        
        result = await agent.translate(
            chunk_code,
            full_context,
            incremental_context=previous_code
        )
        
        return {
            "success": result.success,
            "rust_code": result.code,
            "error": result.error,
        }
    
    @registry.register(
        name="validate_rust_code",
        description="Validate Rust code by running cargo check"
    )
    async def validate_rust_code(rust_code: str, crate_name: str = "temp_check") -> dict:
        """Validate Rust code using cargo check."""
        import tempfile
        import subprocess
        from pathlib import Path
        from rustify.agents.fix_agent import FixAgent
        
        with tempfile.TemporaryDirectory() as tmpdir:
            project_path = Path(tmpdir)
            
            # Create Cargo.toml
            cargo_toml = f'''[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"
'''
            (project_path / "Cargo.toml").write_text(cargo_toml)
            (project_path / "src").mkdir()
            (project_path / "src" / "lib.rs").write_text(rust_code)
            
            try:
                result = subprocess.run(
                    ["cargo", "check", "--message-format=json"],
                    cwd=project_path,
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                errors = FixAgent.parse_cargo_check_output(result.stdout)
                
                return {
                    "valid": len(errors) == 0,
                    "errors": [{"code": e.code, "message": e.message} for e in errors],
                    "error_count": len(errors),
                }
            except subprocess.TimeoutExpired:
                return {"valid": False, "errors": [{"message": "Compilation timed out"}], "error_count": 1}
            except FileNotFoundError:
                return {"valid": False, "errors": [{"message": "cargo not found"}], "error_count": 1}
    
    @registry.register(
        name="fix_rust_errors",
        description="Attempt to fix compilation errors in Rust code using LLM"
    )
    async def fix_rust_errors(
        rust_code: str,
        errors: list[dict]
    ) -> dict:
        """Fix Rust compilation errors."""
        from rustify.llm import get_llm_adapter
        from rustify.agents.fix_agent import FixAgent, CompileError
        from rustify.config import get_config
        
        config = get_config()
        llm = get_llm_adapter(config.llm)
        agent = FixAgent(llm)
        
        # Convert error dicts to CompileError objects
        compile_errors = [
            CompileError(
                code=e.get("code", ""),
                message=e.get("message", ""),
                rendered=e.get("rendered", ""),
            )
            for e in errors
        ]
        
        result = await agent.fix(rust_code, compile_errors)
        
        return {
            "success": result.success,
            "fixed_code": result.code,
            "fixed_errors": result.fixed_errors,
            "error": result.error,
        }
    
    @registry.register(
        name="merge_chunks",
        description="Merge multiple translated code chunks into a single module"
    )
    async def merge_chunks(
        chunks: list[dict],
        preamble: str = ""
    ) -> dict:
        """Merge translated chunks."""
        from rustify.llm import get_llm_adapter
        from rustify.agents.merge_agent import MergeAgent
        from rustify.config import get_config
        
        config = get_config()
        llm = get_llm_adapter(config.llm)
        agent = MergeAgent(llm)
        
        result = await agent.merge(chunks, preamble)
        
        return {
            "success": result.success,
            "merged_code": result.code,
            "conflicts_resolved": result.conflicts_resolved,
            "error": result.error,
        }
    
    @registry.register(
        name="split_code",
        description="Split C code into translatable chunks"
    )
    async def split_code(
        source_code: str,
        filename: str = ""
    ) -> dict:
        """Split code into chunks."""
        from rustify.agents.chunker import CodeChunker
        
        chunker = CodeChunker()
        chunks = chunker.chunk(source_code, filename)
        
        return {
            "chunks": [
                {
                    "name": c.name,
                    "kind": c.kind.value,
                    "text": c.text,
                    "start_line": c.start_line,
                    "end_line": c.end_line,
                    "dependencies": c.dependencies,
                }
                for c in chunks
            ],
            "total": len(chunks),
        }
    
    return registry

