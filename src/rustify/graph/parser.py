"""
C Parser - Parse C/C++ code to extract nodes and dependencies.

Uses tree-sitter for parsing when available, falls back to regex.
"""

import os
import re
from typing import List, Optional, Set, Dict, Tuple
from pathlib import Path

from rustify.graph.dep_graph import DGNode, DependencyGraph, NodeType


class CParser:
    """
    Parser for C/C++ code.
    
    Extracts:
    - Functions
    - Structs
    - Enums
    - Typedefs
    - Includes
    - Dependencies between nodes
    """
    
    # Regex patterns for parsing
    FUNCTION_PATTERN = re.compile(
        r'^(?P<ret_type>[\w\s\*]+?)\s+'
        r'(?P<name>\w+)\s*'
        r'\((?P<params>[^)]*)\)\s*'
        r'(?:{\s*(?P<body>.*?)\s*})?',
        re.MULTILINE | re.DOTALL
    )
    
    STRUCT_PATTERN = re.compile(
        r'(?:typedef\s+)?struct\s+(?P<name>\w+)?\s*'
        r'{(?P<body>[^}]*)}(?:\s*(?P<typedef_name>\w+))?\s*;',
        re.DOTALL
    )
    
    ENUM_PATTERN = re.compile(
        r'(?:typedef\s+)?enum\s+(?P<name>\w+)?\s*'
        r'{(?P<body>[^}]*)}(?:\s*(?P<typedef_name>\w+))?\s*;',
        re.DOTALL
    )
    
    TYPEDEF_PATTERN = re.compile(
        r'typedef\s+(?P<type>[\w\s\*]+)\s+(?P<name>\w+)\s*;'
    )
    
    INCLUDE_PATTERN = re.compile(
        r'#include\s*[<"](?P<path>[^>"]+)[>"]'
    )
    
    MACRO_PATTERN = re.compile(
        r'#define\s+(?P<name>\w+)(?:\((?P<params>[^)]*)\))?\s*(?P<body>.*?)(?=\n(?!\\)|\Z)',
        re.DOTALL
    )
    
    def __init__(self, project_path: str):
        """
        Initialize the parser.
        
        Args:
            project_path: Path to the C/C++ project.
        """
        self.project_path = project_path
        self.graph = DependencyGraph()
        
        # Cache for parsed files
        self._file_cache: Dict[str, str] = {}
        
        # Try to import tree-sitter
        self._tree_sitter = None
        try:
            import tree_sitter_c as tsc
            from tree_sitter import Language, Parser
            self._tree_sitter = Parser(Language(tsc.language()))
        except ImportError:
            pass
    
    def parse_project(self) -> DependencyGraph:
        """
        Parse the entire project.
        
        Returns:
            DependencyGraph with all nodes and edges.
        """
        # Find all C/C++ files
        c_files = self._find_source_files()
        
        # Parse each file
        for filepath in c_files:
            self._parse_file(filepath)
        
        # Build dependencies
        self._build_dependencies()
        
        return self.graph
    
    def _find_source_files(self) -> List[str]:
        """Find all C/C++ source files."""
        extensions = {'.c', '.h', '.cpp', '.hpp', '.cc', '.cxx'}
        files = []
        
        for root, dirs, filenames in os.walk(self.project_path):
            # Skip hidden directories
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
        content = self._read_file(filepath)
        
        if self._tree_sitter:
            self._parse_with_tree_sitter(filepath, content)
        else:
            self._parse_with_regex(filepath, content)
    
    def _parse_with_tree_sitter(self, filepath: str, content: str) -> None:
        """Parse using tree-sitter for accurate AST."""
        tree = self._tree_sitter.parse(content.encode())
        root = tree.root_node
        
        self._extract_nodes_from_tree(root, filepath, content)
    
    def _extract_nodes_from_tree(self, node, filepath: str, content: str) -> None:
        """Extract nodes from tree-sitter AST."""
        if node.type == 'function_definition':
            self._extract_function_node(node, filepath, content)
        elif node.type == 'struct_specifier':
            self._extract_struct_node(node, filepath, content)
        elif node.type == 'enum_specifier':
            self._extract_enum_node(node, filepath, content)
        elif node.type == 'type_definition':
            self._extract_typedef_node(node, filepath, content)
        
        for child in node.children:
            self._extract_nodes_from_tree(child, filepath, content)
    
    def _extract_function_node(self, node, filepath: str, content: str) -> None:
        """Extract a function from tree-sitter node."""
        name = None
        for child in node.children:
            if child.type == 'function_declarator':
                for subchild in child.children:
                    if subchild.type == 'identifier':
                        name = content[subchild.start_byte:subchild.end_byte]
                        break
        
        if name:
            text = content[node.start_byte:node.end_byte]
            dg_node = DGNode(
                name=name,
                type=NodeType.FUNCTION,
                text=text,
                location=filepath,
                start_line=node.start_point[0] + 1,
                end_line=node.end_point[0] + 1
            )
            self.graph.add_node(dg_node)
    
    def _extract_struct_node(self, node, filepath: str, content: str) -> None:
        """Extract a struct from tree-sitter node."""
        name = None
        for child in node.children:
            if child.type == 'type_identifier':
                name = content[child.start_byte:child.end_byte]
                break
        
        if name:
            text = content[node.start_byte:node.end_byte]
            dg_node = DGNode(
                name=name,
                type=NodeType.STRUCT,
                text=text,
                location=filepath,
                start_line=node.start_point[0] + 1,
                end_line=node.end_point[0] + 1
            )
            self.graph.add_node(dg_node)
    
    def _extract_enum_node(self, node, filepath: str, content: str) -> None:
        """Extract an enum from tree-sitter node."""
        name = None
        for child in node.children:
            if child.type == 'type_identifier':
                name = content[child.start_byte:child.end_byte]
                break
        
        if name:
            text = content[node.start_byte:node.end_byte]
            dg_node = DGNode(
                name=name,
                type=NodeType.ENUM,
                text=text,
                location=filepath,
                start_line=node.start_point[0] + 1,
                end_line=node.end_point[0] + 1
            )
            self.graph.add_node(dg_node)
    
    def _extract_typedef_node(self, node, filepath: str, content: str) -> None:
        """Extract a typedef from tree-sitter node."""
        text = content[node.start_byte:node.end_byte]
        # Try to extract name from typedef
        match = re.search(r'typedef\s+.*?\s+(\w+)\s*;', text)
        if match:
            name = match.group(1)
            dg_node = DGNode(
                name=name,
                type=NodeType.TYPEDEF,
                text=text,
                location=filepath,
                start_line=node.start_point[0] + 1,
                end_line=node.end_point[0] + 1
            )
            self.graph.add_node(dg_node)
    
    def _parse_with_regex(self, filepath: str, content: str) -> None:
        """Parse using regex (fallback when tree-sitter unavailable)."""
        # Parse functions
        for match in self.FUNCTION_PATTERN.finditer(content):
            name = match.group('name')
            if name and not self._is_keyword(name):
                start = content[:match.start()].count('\n') + 1
                end = content[:match.end()].count('\n') + 1
                
                dg_node = DGNode(
                    name=name,
                    type=NodeType.FUNCTION,
                    text=match.group(0),
                    location=filepath,
                    start_line=start,
                    end_line=end
                )
                self.graph.add_node(dg_node)
        
        # Parse structs
        for match in self.STRUCT_PATTERN.finditer(content):
            name = match.group('typedef_name') or match.group('name')
            if name:
                start = content[:match.start()].count('\n') + 1
                end = content[:match.end()].count('\n') + 1
                
                dg_node = DGNode(
                    name=name,
                    type=NodeType.STRUCT,
                    text=match.group(0),
                    location=filepath,
                    start_line=start,
                    end_line=end
                )
                self.graph.add_node(dg_node)
        
        # Parse enums
        for match in self.ENUM_PATTERN.finditer(content):
            name = match.group('typedef_name') or match.group('name')
            if name:
                start = content[:match.start()].count('\n') + 1
                end = content[:match.end()].count('\n') + 1
                
                dg_node = DGNode(
                    name=name,
                    type=NodeType.ENUM,
                    text=match.group(0),
                    location=filepath,
                    start_line=start,
                    end_line=end
                )
                self.graph.add_node(dg_node)
    
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
        # Build name lookup
        name_to_node: Dict[str, DGNode] = {}
        for node in self.graph.nodes.values():
            name_to_node[node.name] = node
        
        # Find dependencies based on name references
        for node in self.graph.nodes.values():
            text = node.text
            
            # Find function calls and type references
            for other_name, other_node in name_to_node.items():
                if other_node.id == node.id:
                    continue
                
                # Check if this node references the other
                pattern = rf'\b{re.escape(other_name)}\b'
                if re.search(pattern, text):
                    self.graph.add_edge(node, other_node, "depends")
    
    def get_file_dependencies(self) -> Dict[str, Set[str]]:
        """
        Analyze #include directives to build file-level dependencies.
        
        Returns:
            Dict mapping file path -> set of files it depends on
        """
        deps: Dict[str, Set[str]] = {}
        
        for filepath in self._find_source_files():
            content = self._read_file(filepath)
            rel_path = os.path.relpath(filepath, self.project_path)
            deps[rel_path] = set()
            
            # Find all #include directives
            for match in self.INCLUDE_PATTERN.finditer(content):
                include_path = match.group('path')
                
                # Skip system headers (angle brackets or standard names)
                if '<' in content[max(0, match.start()-5):match.start()]:
                    continue
                
                # Find the corresponding .c file
                # e.g., "tinyexpr.h" -> "tinyexpr.c"
                if include_path.endswith('.h') or include_path.endswith('.hpp'):
                    base = include_path[:-2] if include_path.endswith('.h') else include_path[:-4]
                    c_file = base + '.c'
                    cpp_file = base + '.cpp'
                    
                    # Check various locations
                    for candidate in [c_file, cpp_file, os.path.basename(c_file), os.path.basename(cpp_file)]:
                        candidate_abs = os.path.join(self.project_path, candidate)
                        if os.path.exists(candidate_abs):
                            deps[rel_path].add(candidate)
                            break
        
        return deps
    
    def topological_sort_files(self) -> List[str]:
        """
        Sort files by dependency order using #include analysis.
        
        Files that are included by others (libraries) come first.
        Files that include others (applications) come last.
        
        Returns:
            List of file paths in dependency order (dependencies first)
        """
        deps = self.get_file_dependencies()
        
        # Count how many other files depend on each file (被依赖次数)
        # Higher count = more important library = should be translated first
        dependency_count: Dict[str, int] = {f: 0 for f in deps}
        
        for file, file_deps in deps.items():
            for dep in file_deps:
                if dep in dependency_count:
                    dependency_count[dep] += 1
        
        # Also count outgoing dependencies (依赖别人的次数)
        # Higher count = more dependent on others = should be translated later
        depends_on_count: Dict[str, int] = {f: len(d) for f, d in deps.items()}
        
        # Sort: 
        # 1. Files with more dependents first (libraries)
        # 2. Files with fewer dependencies first
        # 3. Non-example files before example files
        # 4. Alphabetically
        all_files = list(deps.keys())
        all_files.sort(key=lambda f: (
            -dependency_count.get(f, 0),  # More dependents = earlier (negative for descending)
            depends_on_count.get(f, 0),   # Fewer dependencies = earlier
            1 if any(x in f.lower() for x in ['example', 'test', 'bench', 'demo', 'sample']) else 0,
            f  # Alphabetically
        ))
        
        return all_files

