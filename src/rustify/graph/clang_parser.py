"""
Clang AST Parser - Parse C/C++ code using libclang for precise dependency analysis.

Uses libclang for accurate AST parsing, providing:
- Precise function, struct, enum, typedef extraction
- Accurate dependency relationships
- Macro definition handling
- Include directive analysis

Falls back to regex parsing if libclang is not available.
"""

import os
import re
import logging
from typing import List, Optional, Set, Dict, Tuple, Any
from pathlib import Path
from dataclasses import dataclass, field
from enum import Enum

logger = logging.getLogger("rustify.clang_parser")


class ClangNodeType(Enum):
    """Node types recognized by Clang parser."""
    FILE = "file"
    FUNCTION = "function"
    STRUCT = "struct"
    UNION = "union"
    ENUM = "enum"
    TYPEDEF = "typedef"
    MACRO = "macro"
    MACRO_FUNCTION = "macro_function"
    VARIABLE = "variable"
    DECLARATION = "declaration"


@dataclass
class ClangNode:
    """A node in the Clang-parsed dependency graph."""
    id: str
    name: str
    type: ClangNodeType
    location: str  # File path
    text: str  # Source code text
    start_line: int = 0
    end_line: int = 0
    extra: Dict[str, Any] = field(default_factory=dict)
    
    # Edges
    depends_on: List[str] = field(default_factory=list)  # Node IDs this depends on
    depended_by: List[str] = field(default_factory=list)  # Node IDs that depend on this
    
    @classmethod
    def generate_id(cls, type: ClangNodeType, name: str, location: str) -> str:
        """Generate a unique ID for a node."""
        import hashlib
        key = f"{type.value}:{name}:{location}"
        return hashlib.md5(key.encode()).hexdigest()[:12]


@dataclass
class HeaderContext:
    """Context extracted from header files."""
    structs: Dict[str, str] = field(default_factory=dict)  # name -> definition
    enums: Dict[str, str] = field(default_factory=dict)
    typedefs: Dict[str, str] = field(default_factory=dict)
    macros: Dict[str, str] = field(default_factory=dict)
    function_decls: Dict[str, str] = field(default_factory=dict)
    constants: Dict[str, str] = field(default_factory=dict)
    
    def to_context_string(self) -> str:
        """Convert to a context string for LLM."""
        parts = []
        
        if self.structs:
            parts.append("### Struct Definitions")
            for name, defn in self.structs.items():
                parts.append(f"```c\n{defn}\n```")
        
        if self.enums:
            parts.append("### Enum Definitions")
            for name, defn in self.enums.items():
                parts.append(f"```c\n{defn}\n```")
        
        if self.typedefs:
            parts.append("### Type Definitions")
            for name, defn in self.typedefs.items():
                parts.append(f"```c\n{defn}\n```")
        
        if self.macros:
            parts.append("### Macro Definitions")
            for name, defn in list(self.macros.items())[:50]:  # Limit macros
                parts.append(f"```c\n{defn}\n```")
        
        if self.constants:
            parts.append("### Constants")
            for name, defn in list(self.constants.items())[:30]:
                parts.append(f"```c\n{defn}\n```")
        
        return "\n\n".join(parts)


class ClangASTParser:
    """
    Parser using libclang for precise C/C++ AST analysis.
    
    Features:
    - Accurate AST parsing using libclang
    - Precise dependency analysis at node level
    - Header file context extraction
    - Macro and constant handling
    """
    
    def __init__(self, project_path: str, clang_args: List[str] = None):
        """
        Initialize the Clang parser.
        
        Args:
            project_path: Path to the C/C++ project.
            clang_args: Additional arguments for clang (e.g., ["-std=c11"]).
        """
        self.project_path = os.path.abspath(project_path)
        self.clang_args = clang_args or ["-std=c11"]
        
        # Nodes and edges
        self.nodes: Dict[str, ClangNode] = {}
        self.file_nodes: Dict[str, List[ClangNode]] = {}  # file -> nodes in that file
        
        # File content cache
        self._file_cache: Dict[str, str] = {}
        
        # Header context for each source file
        self._header_contexts: Dict[str, HeaderContext] = {}
        
        # Try to import libclang
        self._clang = None
        self._clang_available = False
        try:
            import clang.cindex as clang
            self._clang = clang
            self._clang_available = True
            logger.info("libclang available, using Clang AST parsing")
        except ImportError:
            logger.warning("libclang not available, falling back to regex parsing")
    
    @property
    def is_clang_available(self) -> bool:
        """Check if libclang is available."""
        return self._clang_available
    
    def parse_project(self) -> Dict[str, ClangNode]:
        """
        Parse the entire project.
        
        Returns:
            Dict of node_id -> ClangNode.
        """
        source_files = self._find_source_files()
        
        for filepath in source_files:
            self._parse_file(filepath)
        
        # Build cross-file dependencies
        self._build_dependencies()
        
        return self.nodes
    
    def _find_source_files(self) -> List[str]:
        """Find all C/C++ source files."""
        extensions = {'.c', '.h', '.cpp', '.hpp', '.cc', '.cxx'}
        files = []
        
        for root, dirs, filenames in os.walk(self.project_path):
            dirs[:] = [d for d in dirs if not d.startswith('.')]
            
            for filename in filenames:
                if Path(filename).suffix.lower() in extensions:
                    files.append(os.path.join(root, filename))
        
        return files
    
    def _read_file(self, filepath: str) -> str:
        """Read and cache a file."""
        if filepath not in self._file_cache:
            try:
                with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                    self._file_cache[filepath] = f.read()
            except Exception:
                self._file_cache[filepath] = ""
        return self._file_cache[filepath]
    
    def _parse_file(self, filepath: str) -> None:
        """Parse a single file."""
        if self._clang_available:
            self._parse_with_clang(filepath)
        else:
            self._parse_with_regex(filepath)
    
    def _parse_with_clang(self, filepath: str) -> None:
        """Parse using libclang for accurate AST."""
        clang = self._clang
        
        try:
            index = clang.Index.create()
            tu = index.parse(
                filepath,
                args=self.clang_args,
                options=clang.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD
            )
            
            if tu is None:
                logger.warning(f"Failed to parse {filepath}")
                self._parse_with_regex(filepath)
                return
            
            # Visit the AST
            self._visit_clang_node(tu.cursor, filepath)
            
        except Exception as e:
            logger.warning(f"Clang parse error for {filepath}: {e}")
            self._parse_with_regex(filepath)
    
    def _visit_clang_node(self, cursor, filepath: str) -> None:
        """Visit a Clang AST node and extract relevant information."""
        clang = self._clang
        
        # Only process nodes from our file (not included headers)
        if cursor.location.file and cursor.location.file.name != filepath:
            # But record include directives
            if cursor.kind == clang.CursorKind.INCLUSION_DIRECTIVE:
                pass  # Handle includes separately
            return
        
        # Extract based on cursor kind
        if cursor.kind == clang.CursorKind.FUNCTION_DECL:
            self._extract_function(cursor, filepath)
        elif cursor.kind == clang.CursorKind.STRUCT_DECL:
            self._extract_struct(cursor, filepath)
        elif cursor.kind == clang.CursorKind.UNION_DECL:
            self._extract_union(cursor, filepath)
        elif cursor.kind == clang.CursorKind.ENUM_DECL:
            self._extract_enum(cursor, filepath)
        elif cursor.kind == clang.CursorKind.TYPEDEF_DECL:
            self._extract_typedef(cursor, filepath)
        elif cursor.kind == clang.CursorKind.MACRO_DEFINITION:
            self._extract_macro(cursor, filepath)
        elif cursor.kind == clang.CursorKind.VAR_DECL:
            self._extract_variable(cursor, filepath)
        
        # Recurse into children
        for child in cursor.get_children():
            self._visit_clang_node(child, filepath)
    
    def _get_cursor_text(self, cursor, filepath: str) -> str:
        """Get the source text for a cursor."""
        content = self._read_file(filepath)
        
        if cursor.extent.start.file and cursor.extent.end.file:
            start = cursor.extent.start.offset
            end = cursor.extent.end.offset
            return content[start:end]
        
        return ""
    
    def _extract_function(self, cursor, filepath: str) -> None:
        """Extract a function definition/declaration."""
        name = cursor.spelling
        if not name:
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.FUNCTION, name, filepath),
            name=name,
            type=ClangNodeType.FUNCTION,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
            extra={
                "is_definition": cursor.is_definition(),
                "return_type": cursor.result_type.spelling if cursor.result_type else None,
            }
        )
        
        self._add_node(node, filepath)
    
    def _extract_struct(self, cursor, filepath: str) -> None:
        """Extract a struct definition."""
        name = cursor.spelling
        if not name:
            # Anonymous struct, try to get from typedef
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        # Add semicolon if missing
        if not text.strip().endswith(';'):
            text = text + ';'
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.STRUCT, name, filepath),
            name=name,
            type=ClangNodeType.STRUCT,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
        )
        
        self._add_node(node, filepath)
    
    def _extract_union(self, cursor, filepath: str) -> None:
        """Extract a union definition."""
        name = cursor.spelling
        if not name:
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.UNION, name, filepath),
            name=name,
            type=ClangNodeType.UNION,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
        )
        
        self._add_node(node, filepath)
    
    def _extract_enum(self, cursor, filepath: str) -> None:
        """Extract an enum definition."""
        name = cursor.spelling
        if not name:
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.ENUM, name, filepath),
            name=name,
            type=ClangNodeType.ENUM,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
        )
        
        self._add_node(node, filepath)
    
    def _extract_typedef(self, cursor, filepath: str) -> None:
        """Extract a typedef."""
        name = cursor.spelling
        if not name:
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.TYPEDEF, name, filepath),
            name=name,
            type=ClangNodeType.TYPEDEF,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
            extra={
                "underlying_type": cursor.underlying_typedef_type.spelling 
                    if cursor.underlying_typedef_type else None,
            }
        )
        
        self._add_node(node, filepath)
    
    def _extract_macro(self, cursor, filepath: str) -> None:
        """Extract a macro definition."""
        name = cursor.spelling
        if not name:
            return
        
        # Get macro text from file
        content = self._read_file(filepath)
        lines = content.split('\n')
        
        if cursor.location.line <= len(lines):
            line_idx = cursor.location.line - 1
            text = lines[line_idx]
            
            # Handle multiline macros
            while text.rstrip().endswith('\\') and line_idx + 1 < len(lines):
                line_idx += 1
                text += '\n' + lines[line_idx]
            
            # Determine if it's a function-like macro
            parts = text.split()
            is_function_macro = len(parts) > 1 and '(' in parts[1]
            node_type = ClangNodeType.MACRO_FUNCTION if is_function_macro else ClangNodeType.MACRO
            
            node = ClangNode(
                id=ClangNode.generate_id(node_type, name, filepath),
                name=name,
                type=node_type,
                location=filepath,
                text=text,
                start_line=cursor.location.line,
                end_line=line_idx + 1,
            )
            
            self._add_node(node, filepath)
    
    def _extract_variable(self, cursor, filepath: str) -> None:
        """Extract a variable declaration (global/static)."""
        name = cursor.spelling
        if not name:
            return
        
        # Only extract global/file-scope variables
        if cursor.semantic_parent.kind != self._clang.CursorKind.TRANSLATION_UNIT:
            return
        
        text = self._get_cursor_text(cursor, filepath)
        if not text:
            return
        
        node = ClangNode(
            id=ClangNode.generate_id(ClangNodeType.VARIABLE, name, filepath),
            name=name,
            type=ClangNodeType.VARIABLE,
            location=filepath,
            text=text,
            start_line=cursor.location.line,
            end_line=cursor.extent.end.line,
        )
        
        self._add_node(node, filepath)
    
    def _add_node(self, node: ClangNode, filepath: str) -> None:
        """Add a node to the graph."""
        if node.id not in self.nodes:
            self.nodes[node.id] = node
            
            if filepath not in self.file_nodes:
                self.file_nodes[filepath] = []
            self.file_nodes[filepath].append(node)
    
    def _parse_with_regex(self, filepath: str) -> None:
        """Fallback regex parsing when libclang is not available."""
        content = self._read_file(filepath)
        
        # Function pattern
        func_pattern = re.compile(
            r'(?:^|\n)\s*(?:static\s+|extern\s+|inline\s+)*'
            r'([\w\s\*]+?)\s+'
            r'(\w+)\s*'
            r'\(([^)]*)\)\s*'
            r'(\{[^}]*\}|;)',
            re.MULTILINE | re.DOTALL
        )
        
        for match in func_pattern.finditer(content):
            name = match.group(2)
            if name and not self._is_keyword(name):
                text = match.group(0).strip()
                start_line = content[:match.start()].count('\n') + 1
                end_line = content[:match.end()].count('\n') + 1
                
                node = ClangNode(
                    id=ClangNode.generate_id(ClangNodeType.FUNCTION, name, filepath),
                    name=name,
                    type=ClangNodeType.FUNCTION,
                    location=filepath,
                    text=text,
                    start_line=start_line,
                    end_line=end_line,
                )
                self._add_node(node, filepath)
        
        # Struct pattern
        struct_pattern = re.compile(
            r'(?:typedef\s+)?struct\s+(\w+)?\s*'
            r'\{([^}]*)\}\s*(\w+)?\s*;',
            re.DOTALL
        )
        
        for match in struct_pattern.finditer(content):
            name = match.group(3) or match.group(1)
            if name:
                text = match.group(0)
                start_line = content[:match.start()].count('\n') + 1
                end_line = content[:match.end()].count('\n') + 1
                
                node = ClangNode(
                    id=ClangNode.generate_id(ClangNodeType.STRUCT, name, filepath),
                    name=name,
                    type=ClangNodeType.STRUCT,
                    location=filepath,
                    text=text,
                    start_line=start_line,
                    end_line=end_line,
                )
                self._add_node(node, filepath)
        
        # Enum pattern
        enum_pattern = re.compile(
            r'(?:typedef\s+)?enum\s+(\w+)?\s*'
            r'\{([^}]*)\}\s*(\w+)?\s*;',
            re.DOTALL
        )
        
        for match in enum_pattern.finditer(content):
            name = match.group(3) or match.group(1)
            if name:
                text = match.group(0)
                start_line = content[:match.start()].count('\n') + 1
                end_line = content[:match.end()].count('\n') + 1
                
                node = ClangNode(
                    id=ClangNode.generate_id(ClangNodeType.ENUM, name, filepath),
                    name=name,
                    type=ClangNodeType.ENUM,
                    location=filepath,
                    text=text,
                    start_line=start_line,
                    end_line=end_line,
                )
                self._add_node(node, filepath)
        
        # Macro pattern
        macro_pattern = re.compile(
            r'^#define\s+(\w+)(?:\([^)]*\))?\s*(.*?)(?=\n(?!\\)|\Z)',
            re.MULTILINE | re.DOTALL
        )
        
        for match in macro_pattern.finditer(content):
            name = match.group(1)
            text = match.group(0)
            start_line = content[:match.start()].count('\n') + 1
            end_line = content[:match.end()].count('\n') + 1
            
            node_type = ClangNodeType.MACRO_FUNCTION if '(' in text.split('\n')[0] else ClangNodeType.MACRO
            
            node = ClangNode(
                id=ClangNode.generate_id(node_type, name, filepath),
                name=name,
                type=node_type,
                location=filepath,
                text=text,
                start_line=start_line,
                end_line=end_line,
            )
            self._add_node(node, filepath)
    
    def _is_keyword(self, name: str) -> bool:
        """Check if a name is a C keyword."""
        keywords = {
            'if', 'else', 'while', 'for', 'do', 'switch', 'case',
            'break', 'continue', 'return', 'goto', 'default',
            'sizeof', 'typedef', 'struct', 'union', 'enum',
            'static', 'extern', 'const', 'volatile', 'register',
            'auto', 'void', 'char', 'short', 'int', 'long',
            'float', 'double', 'signed', 'unsigned'
        }
        return name in keywords
    
    def _build_dependencies(self) -> None:
        """Build dependency edges between nodes."""
        # Build name -> node lookup
        name_to_nodes: Dict[str, List[ClangNode]] = {}
        for node in self.nodes.values():
            if node.name not in name_to_nodes:
                name_to_nodes[node.name] = []
            name_to_nodes[node.name].append(node)
        
        # Find dependencies based on name references
        for node in self.nodes.values():
            text = node.text
            
            for other_name, other_nodes in name_to_nodes.items():
                if other_name == node.name:
                    continue
                
                # Check if this node references the other
                pattern = rf'\b{re.escape(other_name)}\b'
                if re.search(pattern, text):
                    for other_node in other_nodes:
                        if other_node.id != node.id:
                            if other_node.id not in node.depends_on:
                                node.depends_on.append(other_node.id)
                            if node.id not in other_node.depended_by:
                                other_node.depended_by.append(node.id)
    
    def get_header_context(self, source_file: str) -> HeaderContext:
        """
        Get header context for a source file.
        
        Collects type definitions, macros, and constants from included headers.
        
        Args:
            source_file: Path to the source file.
            
        Returns:
            HeaderContext with collected definitions.
        """
        if source_file in self._header_contexts:
            return self._header_contexts[source_file]
        
        context = HeaderContext()
        visited_headers: Set[str] = set()
        
        self._collect_header_context(source_file, context, visited_headers, 0)
        
        self._header_contexts[source_file] = context
        return context
    
    def _collect_header_context(
        self,
        filepath: str,
        context: HeaderContext,
        visited: Set[str],
        depth: int
    ) -> None:
        """Recursively collect context from header files."""
        if depth > 10:  # Limit recursion depth
            return
        
        if filepath in visited:
            return
        visited.add(filepath)
        
        content = self._read_file(filepath)
        if not content:
            return
        
        # Find #include directives
        include_pattern = re.compile(r'#include\s*"([^"]+)"')
        
        for match in include_pattern.finditer(content):
            include_path = match.group(1)
            
            # Try to resolve header path
            header_paths = [
                os.path.join(os.path.dirname(filepath), include_path),
                os.path.join(self.project_path, include_path),
            ]
            
            for header_path in header_paths:
                if os.path.exists(header_path):
                    # Read header content and extract definitions
                    self._extract_header_definitions(header_path, context)
                    
                    # Recursively process included headers
                    self._collect_header_context(
                        header_path, context, visited, depth + 1
                    )
                    break
    
    def _extract_header_definitions(self, header_path: str, context: HeaderContext) -> None:
        """Extract type definitions from a header file."""
        content = self._read_file(header_path)
        if not content:
            return
        
        # Extract structs
        struct_pattern = re.compile(
            r'(?:typedef\s+)?struct\s+(\w+)?\s*\{[^}]*\}\s*(\w+)?\s*;',
            re.DOTALL
        )
        for match in struct_pattern.finditer(content):
            name = match.group(2) or match.group(1)
            if name and name not in context.structs:
                context.structs[name] = match.group(0)
        
        # Extract enums
        enum_pattern = re.compile(
            r'(?:typedef\s+)?enum\s+(\w+)?\s*\{[^}]*\}\s*(\w+)?\s*;',
            re.DOTALL
        )
        for match in enum_pattern.finditer(content):
            name = match.group(2) or match.group(1)
            if name and name not in context.enums:
                context.enums[name] = match.group(0)
        
        # Extract typedefs (simple ones)
        typedef_pattern = re.compile(
            r'typedef\s+([\w\s\*]+)\s+(\w+)\s*;'
        )
        for match in typedef_pattern.finditer(content):
            name = match.group(2)
            if name and name not in context.typedefs:
                context.typedefs[name] = match.group(0)
        
        # Extract macros (constant-like)
        macro_pattern = re.compile(
            r'^#define\s+(\w+)\s+(.+?)(?=\n(?!\\)|\Z)',
            re.MULTILINE
        )
        for match in macro_pattern.finditer(content):
            name = match.group(1)
            value = match.group(2).strip()
            # Skip function-like macros and complex ones
            if '(' not in match.group(0).split('\n')[0] and len(value) < 100:
                if name not in context.macros:
                    context.macros[name] = match.group(0)
        
        # Extract constant definitions (#define NAME value)
        const_pattern = re.compile(
            r'^#define\s+(BZ_\w+|[A-Z][A-Z0-9_]+)\s+(-?\d+|0x[0-9a-fA-F]+)',
            re.MULTILINE
        )
        for match in const_pattern.finditer(content):
            name = match.group(1)
            if name not in context.constants:
                context.constants[name] = match.group(0)
    
    def get_file_dependencies(self) -> Dict[str, Set[str]]:
        """
        Get file-level dependencies based on #include directives.
        
        Returns:
            Dict mapping file path -> set of header files it includes.
        """
        deps: Dict[str, Set[str]] = {}
        
        for filepath in self._find_source_files():
            content = self._read_file(filepath)
            rel_path = os.path.relpath(filepath, self.project_path)
            deps[rel_path] = set()
            
            include_pattern = re.compile(r'#include\s*"([^"]+)"')
            for match in include_pattern.finditer(content):
                include_path = match.group(1)
                deps[rel_path].add(include_path)
        
        return deps
    
    def topological_sort(self) -> List[str]:
        """
        Sort files by dependency order.
        
        Returns:
            List of file paths in dependency order (dependencies first).
        """
        deps = self.get_file_dependencies()
        
        # Count incoming edges (how many files depend on this)
        in_degree: Dict[str, int] = {f: 0 for f in deps}
        for file_deps in deps.values():
            for dep in file_deps:
                if dep in in_degree:
                    in_degree[dep] += 1
        
        # Sort by dependency count (most depended upon first)
        sorted_files = sorted(deps.keys(), key=lambda f: (
            -in_degree.get(f, 0),
            len(deps.get(f, set())),
            f
        ))
        
        return sorted_files
    
    def get_translation_context(self, source_file: str) -> str:
        """
        Get complete translation context for a source file.
        
        Combines:
        1. Header file definitions (structs, enums, macros)
        2. Related node definitions from the dependency graph
        
        Args:
            source_file: Path to the source file to translate.
            
        Returns:
            Context string for LLM translation.
        """
        parts = []
        
        # 1. Header context
        header_context = self.get_header_context(source_file)
        header_str = header_context.to_context_string()
        if header_str:
            parts.append("## Header File Definitions\n" + header_str)
        
        # 2. Related nodes from dependency graph
        rel_path = source_file
        if os.path.isabs(source_file):
            rel_path = os.path.relpath(source_file, self.project_path)
        
        if rel_path in self.file_nodes or source_file in self.file_nodes:
            nodes = self.file_nodes.get(rel_path, self.file_nodes.get(source_file, []))
            
            # Get dependencies for these nodes
            dep_nodes = []
            for node in nodes:
                for dep_id in node.depends_on:
                    dep_node = self.nodes.get(dep_id)
                    if dep_node and dep_node not in dep_nodes:
                        dep_nodes.append(dep_node)
            
            if dep_nodes:
                dep_parts = ["## Dependencies"]
                for dep in dep_nodes[:20]:  # Limit to 20 dependencies
                    dep_parts.append(f"### {dep.name} ({dep.type.value})")
                    dep_parts.append(f"```c\n{dep.text}\n```")
                parts.append("\n\n".join(dep_parts))
        
        return "\n\n".join(parts)

