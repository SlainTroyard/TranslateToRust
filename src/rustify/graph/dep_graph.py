"""
Dependency Graph - Data structures for dependency analysis.

"""

import uuid
from enum import Enum
from typing import Optional, List, Set, Dict
from dataclasses import dataclass, field
import networkx as nx


class NodeType(str, Enum):
    """Types of nodes in the dependency graph."""
    FILE = "file"
    FUNCTION = "function"
    STRUCT = "struct"
    ENUM = "enum"
    TYPEDEF = "typedef"
    MACRO = "macro"
    VARIABLE = "variable"
    UNKNOWN = "unknown"


@dataclass
class DGNode:
    """
    A node in the dependency graph.
    
    Represents a code element (function, struct, file, etc.)
    """
    
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    type: str = "unknown"
    text: str = ""
    location: str = ""  # File path
    start_line: int = 0
    end_line: int = 0
    
    # Dependencies
    edges: List["DGEdge"] = field(default_factory=list)
    
    def __hash__(self):
        return hash(self.id)
    
    def __eq__(self, other):
        if isinstance(other, DGNode):
            return self.id == other.id
        return False
    
    @property
    def line_count(self) -> int:
        """Number of lines in this node."""
        return len(self.text.split("\n"))


@dataclass
class DGEdge:
    """
    An edge in the dependency graph.
    
    Represents a dependency from src to dst.
    """
    
    src: DGNode
    dst: DGNode
    edge_type: str = "depends"  # depends, includes, calls, etc.
    
    def __hash__(self):
        return hash((self.src.id, self.dst.id))
    
    def __eq__(self, other):
        if isinstance(other, DGEdge):
            return self.src.id == other.src.id and self.dst.id == other.dst.id
        return False


class DependencyGraph:
    """
    Dependency graph for a C/C++ project.
    
    Manages nodes (code elements) and edges (dependencies).
    Provides topological sorting for translation order.
    """
    
    def __init__(self):
        self.nodes: Dict[str, DGNode] = {}
        self._nx_graph: Optional[nx.DiGraph] = None
    
    def add_node(self, node: DGNode) -> None:
        """Add a node to the graph."""
        self.nodes[node.id] = node
        self._nx_graph = None  # Invalidate cached graph
    
    def add_edge(self, src: DGNode, dst: DGNode, edge_type: str = "depends") -> None:
        """Add an edge between nodes."""
        if src.id not in self.nodes:
            self.add_node(src)
        if dst.id not in self.nodes:
            self.add_node(dst)
        
        edge = DGEdge(src=src, dst=dst, edge_type=edge_type)
        src.edges.append(edge)
        self._nx_graph = None
    
    def get_node(self, node_id: str) -> Optional[DGNode]:
        """Get a node by ID."""
        return self.nodes.get(node_id)
    
    def get_node_by_name(self, name: str) -> Optional[DGNode]:
        """Get a node by name."""
        for node in self.nodes.values():
            if node.name == name:
                return node
        return None
    
    def _build_nx_graph(self) -> nx.DiGraph:
        """Build a NetworkX directed graph."""
        if self._nx_graph is not None:
            return self._nx_graph
        
        g = nx.DiGraph()
        
        for node in self.nodes.values():
            g.add_node(node.id, data=node)
        
        for node in self.nodes.values():
            for edge in node.edges:
                g.add_edge(node.id, edge.dst.id, type=edge.edge_type)
        
        self._nx_graph = g
        return g
    
    def topological_sort(self) -> List[List[DGNode]]:
        """
        Perform topological sort on the graph.
        
        Returns a list of "levels" where each level can be processed in parallel.
        Level 0 contains nodes with no dependencies.
        """
        g = self._build_nx_graph()
        
        # Check for cycles
        try:
            cycles = list(nx.simple_cycles(g))
            if cycles:
                # Handle cycles by breaking them
                for cycle in cycles:
                    if len(cycle) > 1:
                        g.remove_edge(cycle[-1], cycle[0])
        except:
            pass
        
        levels = []
        remaining = set(g.nodes())
        
        while remaining:
            # Find nodes with no incoming edges from remaining nodes
            level = []
            for node_id in remaining:
                predecessors = set(g.predecessors(node_id)) & remaining
                if not predecessors:
                    level.append(node_id)
            
            if not level:
                # Cycle detected, just take one node
                level = [next(iter(remaining))]
            
            levels.append([self.nodes[nid] for nid in level])
            remaining -= set(level)
        
        return levels
    
    def get_dependencies(self, node: DGNode) -> List[DGNode]:
        """Get all nodes that a node depends on."""
        deps = []
        for edge in node.edges:
            deps.append(edge.dst)
        return deps
    
    def get_dependents(self, node: DGNode) -> List[DGNode]:
        """Get all nodes that depend on a node."""
        dependents = []
        for other in self.nodes.values():
            for edge in other.edges:
                if edge.dst.id == node.id:
                    dependents.append(other)
                    break
        return dependents
    
    def to_dict(self) -> dict:
        """Convert to dictionary for serialization."""
        return {
            "nodes": [
                {
                    "id": n.id,
                    "name": n.name,
                    "type": n.type,
                    "text": n.text,
                    "location": n.location,
                    "start_line": n.start_line,
                    "end_line": n.end_line,
                    "edges": [
                        {"dst": e.dst.id, "type": e.edge_type}
                        for e in n.edges
                    ]
                }
                for n in self.nodes.values()
            ]
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> "DependencyGraph":
        """Create from dictionary."""
        graph = cls()
        
        # First pass: create all nodes
        for node_data in data.get("nodes", []):
            node = DGNode(
                id=node_data["id"],
                name=node_data["name"],
                type=node_data["type"],
                text=node_data.get("text", ""),
                location=node_data.get("location", ""),
                start_line=node_data.get("start_line", 0),
                end_line=node_data.get("end_line", 0)
            )
            graph.add_node(node)
        
        # Second pass: create edges
        for node_data in data.get("nodes", []):
            src = graph.get_node(node_data["id"])
            for edge_data in node_data.get("edges", []):
                dst = graph.get_node(edge_data["dst"])
                if src and dst:
                    graph.add_edge(src, dst, edge_data.get("type", "depends"))
        
        return graph

