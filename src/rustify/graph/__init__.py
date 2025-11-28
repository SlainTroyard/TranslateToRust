"""
Rustify Graph - Dependency graph analysis for C/C++ projects.

Supports two parsing backends:
- ClangASTParser: Uses libclang for precise AST analysis (recommended)
- CParser: Uses tree-sitter or regex as fallback
"""

from rustify.graph.dep_graph import DGNode, DGEdge, DependencyGraph
from rustify.graph.parser import CParser

# Import ClangASTParser with graceful fallback
try:
    from rustify.graph.clang_parser import ClangASTParser, ClangNode, HeaderContext
except ImportError:
    ClangASTParser = None
    ClangNode = None
    HeaderContext = None

__all__ = [
    "DGNode",
    "DGEdge",
    "DependencyGraph",
    "CParser",
    "ClangASTParser",
    "ClangNode",
    "HeaderContext",
]

