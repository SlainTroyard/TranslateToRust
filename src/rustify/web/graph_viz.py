"""
Graph Visualizer - Generate graph data for web visualization.
"""

import json
from typing import Dict, List, Any, Optional
from dataclasses import dataclass, asdict

from rustify.graph.dep_graph import DependencyGraph, DGNode


@dataclass
class VisNode:
    """Node for visualization."""
    
    id: str
    label: str
    type: str
    file: str
    line_count: int
    status: str = "pending"  # pending, translating, completed, error
    
    # Visual properties
    color: Optional[str] = None
    size: Optional[int] = None
    x: Optional[float] = None
    y: Optional[float] = None


@dataclass
class VisEdge:
    """Edge for visualization."""
    
    source: str
    target: str
    type: str = "depends"
    color: Optional[str] = None


class GraphVisualizer:
    """
    Generate visualization data from dependency graph.
    
    Outputs data compatible with D3.js, Cytoscape.js, or vis.js.
    """
    
    # Color scheme for different node types
    TYPE_COLORS = {
        "file": "#4CAF50",       # Green
        "function": "#2196F3",   # Blue
        "struct": "#FF9800",     # Orange
        "enum": "#9C27B0",       # Purple
        "typedef": "#00BCD4",    # Cyan
        "macro": "#F44336",      # Red
        "unknown": "#9E9E9E",    # Gray
    }
    
    # Color scheme for status
    STATUS_COLORS = {
        "pending": "#9E9E9E",     # Gray
        "translating": "#FFC107", # Amber
        "completed": "#4CAF50",   # Green
        "error": "#F44336",       # Red
    }
    
    def __init__(self, graph: DependencyGraph):
        """
        Initialize the visualizer.
        
        Args:
            graph: The dependency graph to visualize.
        """
        self.graph = graph
        self.node_status: Dict[str, str] = {}
    
    def set_node_status(self, node_id: str, status: str) -> None:
        """Set the status of a node."""
        self.node_status[node_id] = status
    
    def set_statuses_from_state(self, state) -> None:
        """
        Set node statuses from translation state.
        
        Args:
            state: StateManager state object.
        """
        # Check module translations for status
        if hasattr(state, 'module_translations'):
            for module in state.module_translations:
                for task in module.translation_tasks:
                    for node in task.source.nodes:
                        if task.status == "completed":
                            self.node_status[node.id] = "completed"
                        elif task.status == "translating":
                            self.node_status[node.id] = "translating"
                        elif task.status == "error":
                            self.node_status[node.id] = "error"
    
    def to_d3_format(self) -> Dict[str, Any]:
        """
        Convert to D3.js force-directed graph format.
        
        Returns:
            Dict with 'nodes' and 'links' arrays.
        """
        nodes = []
        links = []
        
        for node in self.graph.nodes.values():
            status = self.node_status.get(node.id, "pending")
            
            nodes.append({
                "id": node.id,
                "name": node.name,
                "type": node.type,
                "file": node.location,
                "lineCount": node.line_count,
                "status": status,
                "color": self.STATUS_COLORS.get(status, "#9E9E9E"),
                "typeColor": self.TYPE_COLORS.get(node.type, "#9E9E9E"),
            })
            
            for edge in node.edges:
                links.append({
                    "source": node.id,
                    "target": edge.dst.id,
                    "type": edge.edge_type,
                })
        
        return {
            "nodes": nodes,
            "links": links,
        }
    
    def to_cytoscape_format(self) -> List[Dict[str, Any]]:
        """
        Convert to Cytoscape.js format.
        
        Returns:
            List of elements (nodes and edges).
        """
        elements = []
        
        for node in self.graph.nodes.values():
            status = self.node_status.get(node.id, "pending")
            
            elements.append({
                "data": {
                    "id": node.id,
                    "label": node.name,
                    "type": node.type,
                    "file": node.location,
                    "lineCount": node.line_count,
                    "status": status,
                },
                "classes": f"{node.type} {status}",
            })
        
        for node in self.graph.nodes.values():
            for edge in node.edges:
                elements.append({
                    "data": {
                        "source": node.id,
                        "target": edge.dst.id,
                        "type": edge.edge_type,
                    },
                    "classes": edge.edge_type,
                })
        
        return elements
    
    def to_visjs_format(self) -> Dict[str, List]:
        """
        Convert to vis.js format.
        
        Returns:
            Dict with 'nodes' and 'edges' arrays.
        """
        nodes = []
        edges = []
        
        for node in self.graph.nodes.values():
            status = self.node_status.get(node.id, "pending")
            
            nodes.append({
                "id": node.id,
                "label": node.name,
                "title": f"{node.type}: {node.name}\n{node.location}:{node.line_count} lines",
                "group": node.type,
                "color": self.STATUS_COLORS.get(status, "#9E9E9E"),
            })
        
        edge_id = 0
        for node in self.graph.nodes.values():
            for edge in node.edges:
                edges.append({
                    "id": edge_id,
                    "from": node.id,
                    "to": edge.dst.id,
                    "arrows": "to",
                })
                edge_id += 1
        
        return {
            "nodes": nodes,
            "edges": edges,
        }
    
    def get_statistics(self) -> Dict[str, Any]:
        """
        Get graph statistics.
        
        Returns:
            Statistics dict.
        """
        type_counts = {}
        status_counts = {}
        
        for node in self.graph.nodes.values():
            # Count by type
            type_counts[node.type] = type_counts.get(node.type, 0) + 1
            
            # Count by status
            status = self.node_status.get(node.id, "pending")
            status_counts[status] = status_counts.get(status, 0) + 1
        
        total_lines = sum(n.line_count for n in self.graph.nodes.values())
        
        return {
            "total_nodes": len(self.graph.nodes),
            "total_edges": sum(len(n.edges) for n in self.graph.nodes.values()),
            "total_lines": total_lines,
            "by_type": type_counts,
            "by_status": status_counts,
            "progress": {
                "completed": status_counts.get("completed", 0),
                "total": len(self.graph.nodes),
                "percentage": round(
                    status_counts.get("completed", 0) / max(len(self.graph.nodes), 1) * 100,
                    1
                ),
            }
        }
    
    def get_levels(self) -> List[List[str]]:
        """
        Get topological levels for visualization.
        
        Returns:
            List of levels, each containing node IDs.
        """
        levels = self.graph.topological_sort()
        return [[n.id for n in level] for level in levels]

