"""
Translation Stage - Translate C code to Rust using LLM.
"""

import asyncio
from typing import Optional
from loguru import logger

from rustify.engine.stage import Stage
from rustify.engine.pipeline import (
    PipelineContext,
    TranslationModule,
    TranslationUnit,
)


class TranslationStage(Stage[PipelineContext]):
    """
    Translate C code to Rust using LLM.
    
    This stage:
    1. Iterates through modules in dependency order
    2. For each unit, generates Rust code using LLM
    3. Applies multi-strategy translation if configured
    4. Stores results in context
    """
    
    name = "translation"
    description = "Translate C code to Rust using LLM"
    
    def __init__(self):
        super().__init__()
        self._llm_adapter = None
        self._template_engine = None
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        """Execute the translation stage."""
        from rustify.llm import get_llm_adapter
        from rustify.templates import TemplateEngine
        
        # Initialize LLM adapter and template engine
        self._llm_adapter = get_llm_adapter(context.config.llm)
        self._template_engine = TemplateEngine(context.config.templates_dir)
        
        # Get translation strategy
        strategy = context.config.translation.strategy
        max_workers = context.config.translation.parallel_workers
        
        logger.info(f"Using translation strategy: {strategy}")
        
        # Create semaphore for parallel execution
        semaphore = asyncio.Semaphore(max_workers)
        
        # Translate each module
        for module in context.modules:
            context.current_module = module
            logger.info(f"Translating module: {module.name}")
            
            # Get all units from module
            units = []
            for sf in module.source_files:
                units.extend(sf.translation_units)
            
            # Sort units by dependencies
            units = self._sort_units_by_deps(units, context.dependency_graph)
            
            # Translate units
            if strategy == "multi":
                await self._translate_multi_strategy(units, context, semaphore)
            else:
                await self._translate_sequential(units, context, semaphore)
            
            module.status = "translated"
        
        context.current_module = None
        return context
    
    def _sort_units_by_deps(self, units: list[TranslationUnit], graph) -> list[TranslationUnit]:
        """Sort units by dependency order."""
        if graph is None:
            return units
        
        try:
            import networkx as nx
            
            unit_ids = {u.id for u in units}
            subgraph = graph.subgraph(unit_ids)
            sorted_ids = list(nx.topological_sort(subgraph))
            
            unit_map = {u.id: u for u in units}
            return [unit_map[uid] for uid in sorted_ids if uid in unit_map]
            
        except Exception:
            return units
    
    async def _translate_sequential(
        self,
        units: list[TranslationUnit],
        context: PipelineContext,
        semaphore: asyncio.Semaphore
    ) -> None:
        """Translate units sequentially with optional parallelism."""
        tasks = []
        
        for unit in units:
            task = self._translate_unit_with_semaphore(unit, context, semaphore)
            tasks.append(task)
        
        await asyncio.gather(*tasks, return_exceptions=True)
    
    async def _translate_multi_strategy(
        self,
        units: list[TranslationUnit],
        context: PipelineContext,
        semaphore: asyncio.Semaphore
    ) -> None:
        """Translate using multiple strategies and select best result."""
        for unit in units:
            async with semaphore:
                context.current_unit = unit
                
                # Try multiple strategies
                strategies = [
                    self._translate_direct,
                    self._translate_with_context,
                    # self._translate_with_reasoning,
                ]
                
                results = []
                for strategy_fn in strategies:
                    try:
                        result = await strategy_fn(unit, context)
                        if result:
                            results.append(result)
                    except Exception as e:
                        logger.debug(f"Strategy failed: {e}")
                
                # Select best result
                if results:
                    best_result = self._select_best_translation(results)
                    unit.rust_code = best_result
                    unit.status = "translated"
                else:
                    unit.status = "failed"
                    context.warnings.append(f"Failed to translate: {unit.name}")
        
        context.current_unit = None
    
    async def _translate_unit_with_semaphore(
        self,
        unit: TranslationUnit,
        context: PipelineContext,
        semaphore: asyncio.Semaphore
    ) -> None:
        """Translate a single unit with semaphore control."""
        async with semaphore:
            context.current_unit = unit
            
            try:
                result = await self._translate_direct(unit, context)
                if result:
                    unit.rust_code = result
                    unit.status = "translated"
                    logger.debug(f"Translated: {unit.name}")
                else:
                    unit.status = "failed"
                    context.warnings.append(f"Failed to translate: {unit.name}")
            except Exception as e:
                unit.status = "failed"
                context.warnings.append(f"Error translating {unit.name}: {e}")
                logger.error(f"Translation error for {unit.name}: {e}")
            
            context.current_unit = None
    
    async def _translate_direct(
        self,
        unit: TranslationUnit,
        context: PipelineContext
    ) -> Optional[str]:
        """Direct translation using LLM."""
        # Build prompt using template
        prompt = self._template_engine.render(
            "translation/direct.jinja2",
            unit=unit,
            context=context,
        )
        
        # Call LLM
        response = await self._llm_adapter.complete(
            prompt=prompt,
            temperature=context.config.llm.temperature,
            max_tokens=context.config.llm.max_tokens,
        )
        
        # Extract Rust code from response
        rust_code = self._extract_rust_code(response)
        
        return rust_code
    
    async def _translate_with_context(
        self,
        unit: TranslationUnit,
        context: PipelineContext
    ) -> Optional[str]:
        """Translation with additional context from dependencies."""
        # Get dependency context
        dep_context = self._build_dependency_context(unit, context)
        
        # Build prompt with context
        prompt = self._template_engine.render(
            "translation/with_context.jinja2",
            unit=unit,
            context=context,
            dependency_context=dep_context,
        )
        
        response = await self._llm_adapter.complete(
            prompt=prompt,
            temperature=context.config.llm.temperature,
            max_tokens=context.config.llm.max_tokens,
        )
        
        return self._extract_rust_code(response)
    
    def _build_dependency_context(
        self,
        unit: TranslationUnit,
        context: PipelineContext
    ) -> str:
        """Build context from translated dependencies."""
        dep_codes = []
        
        for dep_id in unit.dependencies:
            # Find the translated dependency
            for sf in context.source_files:
                for u in sf.translation_units:
                    if u.id == dep_id and u.rust_code:
                        dep_codes.append(f"// {u.name}\n{u.rust_code}")
                        break
        
        return "\n\n".join(dep_codes)
    
    def _extract_rust_code(self, response: str) -> Optional[str]:
        """Extract Rust code from LLM response."""
        import re
        
        # Try to find code block
        patterns = [
            r'```rust\n(.*?)```',
            r'```\n(.*?)```',
            r'```rs\n(.*?)```',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, response, re.DOTALL)
            if match:
                return match.group(1).strip()
        
        # If no code block, return the whole response if it looks like Rust
        if response.strip().startswith(("fn ", "struct ", "enum ", "impl ", "pub ", "use ", "mod ")):
            return response.strip()
        
        return None
    
    def _select_best_translation(self, results: list[str]) -> str:
        """Select the best translation from multiple results."""
        if len(results) == 1:
            return results[0]
        
        # Simple heuristic: prefer shorter code that compiles
        # In practice, you'd run cargo check here
        scored = []
        for result in results:
            score = 0
            # Prefer code with fewer unsafe blocks
            score -= result.count("unsafe") * 10
            # Prefer idiomatic patterns
            score += result.count("Option<") * 2
            score += result.count("Result<") * 2
            score += result.count("impl ") * 3
            # Penalize very long code
            score -= len(result) / 1000
            scored.append((score, result))
        
        scored.sort(key=lambda x: x[0], reverse=True)
        return scored[0][1]

