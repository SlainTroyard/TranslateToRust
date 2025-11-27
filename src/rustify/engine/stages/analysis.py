"""
Analysis Stage - Parse and analyze C source code.
"""

import hashlib
from pathlib import Path
from typing import Optional
from loguru import logger

from rustify.engine.stage import Stage
from rustify.engine.pipeline import (
    PipelineContext,
    SourceFile,
    TranslationUnit,
)


class AnalysisStage(Stage[PipelineContext]):
    """
    Analyze C source files using tree-sitter.
    
    This stage:
    1. Discovers all C/C++ files in the source directory
    2. Parses each file using tree-sitter
    3. Extracts translation units (functions, structs, etc.)
    4. Generates file summaries
    """
    
    name = "analysis"
    description = "Parse and analyze C source code"
    
    def __init__(self):
        super().__init__()
        self._parser = None
    
    def _get_parser(self):
        """Lazy-load tree-sitter parser."""
        if self._parser is None:
            try:
                from tree_sitter import Language, Parser
                import tree_sitter_c as tsc
                
                self._parser = Parser()
                self._parser.language = Language(tsc.language())
            except ImportError:
                logger.warning("tree-sitter not available, using fallback parser")
                self._parser = FallbackParser()
        return self._parser
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        """Execute the analysis stage."""
        
        # Check if single file mode
        if context.metadata.get("single_file"):
            source_file = context.metadata["source_file"]
            files = [source_file]
        else:
            # Discover all C/C++ files
            files = self._discover_files(context.source_path)
        
        logger.info(f"Found {len(files)} source files")
        
        # Parse each file
        for file_path in files:
            logger.debug(f"Analyzing: {file_path}")
            
            try:
                source_file = await self._analyze_file(file_path, context)
                context.source_files.append(source_file)
            except Exception as e:
                logger.error(f"Failed to analyze {file_path}: {e}")
                context.warnings.append(f"Failed to analyze {file_path}: {e}")
        
        logger.info(f"Analyzed {len(context.source_files)} files")
        return context
    
    def _discover_files(self, source_path: Path) -> list[Path]:
        """Discover all C/C++ files in the source directory."""
        extensions = {".c", ".h", ".cpp", ".hpp", ".cc", ".hh"}
        files = []
        
        for ext in extensions:
            files.extend(source_path.rglob(f"*{ext}"))
        
        # Filter out common non-source directories
        exclude_dirs = {"build", "test", "tests", "doc", "docs", ".git"}
        files = [
            f for f in files
            if not any(d in f.parts for d in exclude_dirs)
        ]
        
        return sorted(files)
    
    async def _analyze_file(
        self, 
        file_path: Path, 
        context: PipelineContext
    ) -> SourceFile:
        """Analyze a single source file."""
        content = file_path.read_text(encoding="utf-8", errors="replace")
        
        # Determine language
        language = "cpp" if file_path.suffix in {".cpp", ".hpp", ".cc", ".hh"} else "c"
        
        # Create source file object
        source_file = SourceFile(
            path=file_path,
            content=content,
            language=language,
        )
        
        # Parse and extract translation units
        parser = self._get_parser()
        units = self._extract_translation_units(parser, content, file_path)
        source_file.translation_units = units
        
        return source_file
    
    def _extract_translation_units(
        self,
        parser,
        content: str,
        file_path: Path
    ) -> list[TranslationUnit]:
        """Extract translation units from parsed content."""
        units = []
        
        if isinstance(parser, FallbackParser):
            # Use fallback extraction
            return parser.extract_units(content, str(file_path))
        
        # Parse with tree-sitter
        tree = parser.parse(content.encode())
        root = tree.root_node
        
        # Walk the tree and extract units
        for node in self._walk_tree(root):
            unit = self._node_to_unit(node, content, file_path)
            if unit:
                units.append(unit)
        
        return units
    
    def _walk_tree(self, node):
        """Walk tree-sitter tree and yield interesting nodes."""
        # Node types we care about
        interesting_types = {
            "function_definition",
            "struct_specifier",
            "enum_specifier",
            "union_specifier",
            "type_definition",
            "preproc_def",
            "preproc_function_def",
            "declaration",
        }
        
        if node.type in interesting_types:
            yield node
        
        for child in node.children:
            yield from self._walk_tree(child)
    
    def _node_to_unit(
        self,
        node,
        content: str,
        file_path: Path
    ) -> Optional[TranslationUnit]:
        """Convert a tree-sitter node to a TranslationUnit."""
        node_text = content[node.start_byte:node.end_byte]
        
        # Determine type and name
        unit_type = self._get_unit_type(node.type)
        if unit_type is None:
            return None
        
        unit_name = self._extract_name(node, content)
        if not unit_name:
            # Generate a name based on hash for anonymous types
            unit_name = f"anon_{hashlib.md5(node_text.encode()).hexdigest()[:8]}"
        
        return TranslationUnit(
            id=f"{file_path.stem}_{unit_name}_{node.start_point[0]}",
            name=unit_name,
            type=unit_type,
            source_code=node_text,
            source_file=str(file_path),
            metadata={
                "start_line": node.start_point[0],
                "end_line": node.end_point[0],
            }
        )
    
    def _get_unit_type(self, node_type: str) -> Optional[str]:
        """Map tree-sitter node type to our unit type."""
        mapping = {
            "function_definition": "function",
            "struct_specifier": "struct",
            "enum_specifier": "enum",
            "union_specifier": "union",
            "type_definition": "typedef",
            "preproc_def": "macro",
            "preproc_function_def": "macro_function",
            "declaration": "variable",
        }
        return mapping.get(node_type)
    
    def _extract_name(self, node, content: str) -> Optional[str]:
        """Extract the name from a node."""
        # Look for identifier children
        for child in node.children:
            if child.type == "identifier":
                return content[child.start_byte:child.end_byte]
            if child.type == "function_declarator":
                # Look deeper for function name
                for subchild in child.children:
                    if subchild.type == "identifier":
                        return content[subchild.start_byte:subchild.end_byte]
        return None


class FallbackParser:
    """Simple fallback parser when tree-sitter is not available."""
    
    def extract_units(self, content: str, file_path: str) -> list[TranslationUnit]:
        """Extract units using simple regex patterns."""
        import re
        
        units = []
        
        # Simple function pattern
        func_pattern = r'(?:static\s+)?(?:inline\s+)?(\w+)\s+(\w+)\s*\([^)]*\)\s*\{'
        for match in re.finditer(func_pattern, content):
            return_type, name = match.groups()
            if return_type not in {"if", "while", "for", "switch"}:
                units.append(TranslationUnit(
                    id=f"{Path(file_path).stem}_{name}",
                    name=name,
                    type="function",
                    source_code=match.group(0),
                    source_file=file_path,
                ))
        
        # Simple struct pattern
        struct_pattern = r'typedef\s+struct\s*\{[^}]*\}\s*(\w+)\s*;'
        for match in re.finditer(struct_pattern, content):
            name = match.group(1)
            units.append(TranslationUnit(
                id=f"{Path(file_path).stem}_{name}",
                name=name,
                type="struct",
                source_code=match.group(0),
                source_file=file_path,
            ))
        
        return units

