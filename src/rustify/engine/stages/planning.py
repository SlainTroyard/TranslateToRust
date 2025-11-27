"""
Planning Stage - Analyze dependencies and create translation plan.
"""

import hashlib
from pathlib import Path
from typing import Optional
from loguru import logger

from rustify.engine.stage import Stage
from rustify.engine.pipeline import (
    PipelineContext,
    TranslationModule,
    TranslationUnit,
)


class PlanningStage(Stage[PipelineContext]):
    """
    Create translation plan by analyzing dependencies.
    
    This stage:
    1. Builds dependency graph using NetworkX
    2. Groups translation units into modules
    3. Determines translation order (topological sort)
    4. Creates translation schedule
    """
    
    name = "planning"
    description = "Analyze dependencies and create translation plan"
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        """Execute the planning stage."""
        
        # Build dependency graph
        graph = self._build_dependency_graph(context)
        context.dependency_graph = graph
        
        # Group into modules
        modules = self._group_into_modules(context, graph)
        context.modules = modules
        
        logger.info(f"Created {len(modules)} translation modules")
        
        # Generate module summaries if LLM is available
        # This is optional and can be done asynchronously
        
        return context
    
    def _build_dependency_graph(self, context: PipelineContext):
        """Build dependency graph from translation units."""
        try:
            import networkx as nx
        except ImportError:
            logger.warning("NetworkX not available, using simple list ordering")
            return None
        
        graph = nx.DiGraph()
        
        # Add all units as nodes
        all_units: dict[str, TranslationUnit] = {}
        for source_file in context.source_files:
            for unit in source_file.translation_units:
                graph.add_node(unit.id, unit=unit)
                all_units[unit.name] = unit
        
        # Analyze dependencies
        for source_file in context.source_files:
            for unit in source_file.translation_units:
                deps = self._find_dependencies(unit, all_units)
                unit.dependencies = deps
                for dep in deps:
                    if dep in [u.id for u in all_units.values()]:
                        graph.add_edge(dep, unit.id)
        
        # Check for cycles
        if not nx.is_directed_acyclic_graph(graph):
            logger.warning("Dependency graph has cycles, breaking cycles")
            graph = self._break_cycles(graph)
        
        return graph
    
    def _find_dependencies(
        self, 
        unit: TranslationUnit,
        all_units: dict[str, TranslationUnit]
    ) -> list[str]:
        """Find dependencies of a translation unit by analyzing its source code."""
        import re
        
        dependencies = []
        code = unit.source_code
        
        # Look for function calls
        call_pattern = r'\b(\w+)\s*\('
        for match in re.finditer(call_pattern, code):
            name = match.group(1)
            # Skip C keywords and common functions
            if name in {"if", "while", "for", "switch", "sizeof", "return"}:
                continue
            if name in all_units and name != unit.name:
                dep_unit = all_units[name]
                if dep_unit.id not in dependencies:
                    dependencies.append(dep_unit.id)
        
        # Look for type references
        type_pattern = r'\b(struct|enum|union)\s+(\w+)'
        for match in re.finditer(type_pattern, code):
            name = match.group(2)
            if name in all_units and name != unit.name:
                dep_unit = all_units[name]
                if dep_unit.id not in dependencies:
                    dependencies.append(dep_unit.id)
        
        return dependencies
    
    def _break_cycles(self, graph):
        """Break cycles in the dependency graph."""
        import networkx as nx
        
        while True:
            try:
                cycle = nx.find_cycle(graph)
                # Remove the last edge in the cycle
                graph.remove_edge(*cycle[-1][:2])
                logger.debug(f"Broke cycle by removing edge: {cycle[-1]}")
            except nx.NetworkXNoCycle:
                break
        
        return graph
    
    def _group_into_modules(
        self,
        context: PipelineContext,
        graph
    ) -> list[TranslationModule]:
        """Group translation units into modules based on file relationships."""
        modules = []
        
        # Group by source file for simplicity
        # More sophisticated grouping can be done with graph analysis
        file_groups: dict[str, list[TranslationUnit]] = {}
        
        for source_file in context.source_files:
            # Use file stem as group key
            key = source_file.path.stem
            
            # Check if there's a matching .h and .c pair
            paired_key = None
            for existing_key in file_groups:
                if key.replace("_impl", "") == existing_key or existing_key.replace("_impl", "") == key:
                    paired_key = existing_key
                    break
            
            if paired_key:
                file_groups[paired_key].extend(source_file.translation_units)
            else:
                file_groups[key] = list(source_file.translation_units)
        
        # Create modules
        for name, units in file_groups.items():
            if not units:
                continue
            
            module_id = hashlib.md5(name.encode()).hexdigest()[:12]
            
            # Collect all source files for this module
            source_files = []
            for sf in context.source_files:
                if any(u in sf.translation_units for u in units):
                    if sf not in source_files:
                        source_files.append(sf)
            
            module = TranslationModule(
                id=module_id,
                name=self._sanitize_module_name(name),
                source_files=source_files,
            )
            
            # Calculate module dependencies
            if graph is not None:
                module.dependencies = self._get_module_dependencies(
                    units, modules, graph
                )
            
            modules.append(module)
        
        # Sort modules by dependency order
        modules = self._sort_modules(modules)
        
        return modules
    
    def _sanitize_module_name(self, name: str) -> str:
        """Convert a name to a valid Rust module name."""
        import re
        # Replace non-alphanumeric with underscore
        name = re.sub(r'[^a-zA-Z0-9_]', '_', name)
        # Ensure it starts with a letter or underscore
        if name and name[0].isdigit():
            name = '_' + name
        return name.lower()
    
    def _get_module_dependencies(
        self,
        units: list[TranslationUnit],
        existing_modules: list[TranslationModule],
        graph
    ) -> list[str]:
        """Get dependencies of a module based on its units."""
        import networkx as nx
        
        deps = set()
        unit_ids = {u.id for u in units}
        
        for unit in units:
            if unit.id not in graph:
                continue
            for pred in graph.predecessors(unit.id):
                if pred not in unit_ids:
                    # Find which module this unit belongs to
                    for module in existing_modules:
                        for sf in module.source_files:
                            if any(u.id == pred for u in sf.translation_units):
                                deps.add(module.id)
                                break
        
        return list(deps)
    
    def _sort_modules(
        self, 
        modules: list[TranslationModule]
    ) -> list[TranslationModule]:
        """Sort modules by dependency order."""
        try:
            import networkx as nx
            
            graph = nx.DiGraph()
            module_map = {m.id: m for m in modules}
            
            for module in modules:
                graph.add_node(module.id)
                for dep in module.dependencies:
                    if dep in module_map:
                        graph.add_edge(dep, module.id)
            
            # Topological sort
            sorted_ids = list(nx.topological_sort(graph))
            return [module_map[mid] for mid in sorted_ids if mid in module_map]
            
        except Exception as e:
            logger.warning(f"Failed to sort modules: {e}")
            return modules

