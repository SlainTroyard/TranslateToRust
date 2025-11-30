"""
Orchestrator Agent - Manages the overall translation project.

"""

import os
import asyncio
from typing import Optional, List
import logging

from pydantic import BaseModel, Field

from rustify.agents.base import BaseAgent
from rustify.schema.response import (
    AgentResponse,
    OrchestratorResponseType,
)
from rustify.schema.translation import ModuleTranslation, ModuleTranslationStatus
from rustify.state.state_manager import StateManager


class ModuleInfo(BaseModel):
    """Module information from LLM analysis."""
    name: str = Field(description="Module name")
    description: str = Field(description="Module description")
    files: List[str] = Field(description="Related source files")


class ModulesResponse(BaseModel):
    """Response containing module information."""
    modules: List[ModuleInfo] = Field(description="List of modules")


class Orchestrator(BaseAgent):
    """
    Orchestrator agent responsible for:
    - Loading and analyzing source projects
    - Creating target Rust projects
    - Managing module translation assignments
    - Coordinating Architects
    """
    
    ROLE = "orchestrator"
    DESCRIPTION = "An AI assistant responsible for managing the C to Rust translation project."
    
    def __init__(
        self,
        state_manager: StateManager,
        llm_config: dict,
        analyzer_config: Optional[dict] = None,
        *,
        name: Optional[str] = None,
        logger: Optional[logging.Logger] = None
    ):
        """
        Initialize the Orchestrator.
        
        Args:
            state_manager: State manager instance.
            llm_config: LLM configuration.
            analyzer_config: Analyzer LLM configuration.
            name: Agent name.
            logger: Logger instance.
        """
        super().__init__(llm_config, name=name, logger=logger)
        self.state_manager = state_manager
        self.analyzer_config = analyzer_config or llm_config
        self.dep_graph = None  # Dependency graph from AST analysis
    
    def start(
        self,
        source_project_dir: str,
        target_project_dir: str,
        overwrite: bool = False
    ) -> AgentResponse:
        """
        Start the translation project.
        
        Args:
            source_project_dir: Source C/C++ project directory.
            target_project_dir: Target Rust project directory.
            overwrite: Whether to overwrite existing target.
            
        Returns:
            AgentResponse indicating status.
        """
        self.logger.info(f"Starting translation project")
        self.logger.info(f"Source: {source_project_dir}")
        self.logger.info(f"Target: {target_project_dir}")
        
        # Step 1: Load source project
        response = self.load_source_project(source_project_dir)
        if response.status != "done":
            return response
        
        # Step 2: Summarize source project
        response = self.summarize_project()
        if response.status != "done":
            return response
        
        # Step 3: Analyze dependencies
        response = self.analyze_dependencies()
        if response.status != "done":
            return response
        
        # Step 4: Create target project
        target_name = os.path.basename(target_project_dir).replace("-", "_")
        response = self.create_target_project(
            target_name,
            target_project_dir,
            overwrite=overwrite
        )
        if response.status != "done":
            return response
        
        # Step 5: Start module translations
        response = self.assign_module_translations()
        
        return response
    
    def load_source_project(self, project_dir: str) -> AgentResponse:
        """Load and validate the source project."""
        self.logger.info(f"Loading source project from {project_dir}")
        
        if not os.path.exists(project_dir):
            return AgentResponse.error(
                self,
                OrchestratorResponseType.LOAD_SOURCE_PROJECT,
                {"message": f"Source project directory not found: {project_dir}"}
            )
        
        # Create source project
        self.state_manager.create_source_project(project_dir)
        
        # List files
        files = self.state_manager.state.source_project.list_files()
        self.logger.info(f"Found {len(files)} files in source project")
        
        return AgentResponse.done(
            self,
            OrchestratorResponseType.LOAD_SOURCE_PROJECT,
            {"file_count": len(files)}
        )
    
    def summarize_project(self) -> AgentResponse:
        """Summarize the source project using LLM."""
        self.logger.info("Summarizing source project")
        
        if not self.state_manager.state.source_project:
            return AgentResponse.error(
                self,
                OrchestratorResponseType.SUMMARIZE_SOURCE_PROJECT,
                {"message": "Source project not loaded"}
            )
        
        project = self.state_manager.state.source_project
        
        # Get project structure
        structure = project.pretty_structure(
            ignore_func=lambda p: not (p.endswith('.c') or p.endswith('.h') or p.endswith('.cpp'))
        )
        
        prompt = f"""Analyze this C/C++ project and provide a summary:

## Project Structure
{structure}

Provide:
1. Project purpose
2. Main components
3. Key dependencies
4. Suggested Rust module organization

Format as JSON:
```json
{{
  "purpose": "...",
  "components": ["..."],
  "dependencies": ["..."],
  "suggested_modules": [
    {{"name": "...", "description": "...", "files": ["..."]}}
  ]
}}
```
"""
        
        response = self.call_llm([{"role": "user", "content": prompt}])
        
        # Try to parse modules
        try:
            import json
            import re
            
            json_match = re.search(r'```json\s*(.*?)\s*```', response.content, re.DOTALL)
            if json_match:
                data = json.loads(json_match.group(1))
                description = data.get("purpose", "C/C++ project")
                self.state_manager.update_source_project_description(description)
        except Exception as e:
            self.logger.warning(f"Failed to parse summary: {e}")
        
        return AgentResponse.done(
            self,
            OrchestratorResponseType.SUMMARIZE_SOURCE_PROJECT
        )
    
    def analyze_dependencies(self) -> AgentResponse:
        """
        Analyze project dependencies using AST parsing.
        
        Uses tree-sitter (if available) or regex to parse C code and
        build a dependency graph for proper translation ordering.
        """
        self.logger.info("Analyzing dependencies with AST parser")
        
        project = self.state_manager.state.source_project
        if not project:
            return AgentResponse.error(
                self,
                OrchestratorResponseType.ANALYZE_DEPENDENCIES,
                {"message": "Source project not loaded"}
            )
        
        # Use AST parser to analyze dependencies
        try:
            from rustify.graph.parser import CParser
            
            parser = CParser(project.path)
            self.dep_graph = parser.parse_project()
            
            # Get topological sort for translation order
            levels = self.dep_graph.topological_sort()
            
            self.logger.info(f"Parsed {len(self.dep_graph.nodes)} code elements")
            self.logger.info(f"Dependency levels: {len(levels)}")
            
            # Store the parsed graph
            self.state_manager.state.dep_graph_data = self.dep_graph.to_dict()
            
        except Exception as e:
            self.logger.warning(f"AST parsing failed, falling back to file-based analysis: {e}")
            self.dep_graph = None
        
        # Get source files
        c_files = []
        h_files = []
        
        for f in project.list_files():
            if f.path.endswith('.c') or f.path.endswith('.cpp'):
                c_files.append(f.path)
            elif f.path.endswith('.h') or f.path.endswith('.hpp'):
                h_files.append(f.path)
        
        self.logger.info(f"Found {len(c_files)} source files, {len(h_files)} header files")
        
        return AgentResponse.done(
            self,
            OrchestratorResponseType.ANALYZE_DEPENDENCIES,
            {
                "source_files": c_files,
                "header_files": h_files,
                "node_count": len(self.dep_graph.nodes) if self.dep_graph else 0
            }
        )
    
    def create_target_project(
        self,
        name: str,
        dirpath: str,
        overwrite: bool = False
    ) -> AgentResponse:
        """Create the target Rust project."""
        self.logger.info(f"Creating target project: {name} at {dirpath}")
        
        if os.path.exists(dirpath) and not overwrite:
            self.logger.warning(f"Target directory exists, using existing")
        
        # Create target project
        target = self.state_manager.create_target_project(
            name=name,
            dirpath=dirpath,
            description=f"Rust translation of {self.state_manager.state.source_project.name}"
        )
        
        return AgentResponse.done(
            self,
            OrchestratorResponseType.CREATE_TARGET_PROJECT,
            {"target_path": target.path}
        )
    
    def _sort_by_dependency(self, files: list, project_path: str) -> list:
        """
        Sort files by dependency order using AST-based dependency graph.
        
        Uses topological sort from the dependency graph if available,
        otherwise falls back to heuristics (main() detection).
        
        Args:
            files: List of ProjectFile objects
            project_path: Path to project root
            
        Returns:
            Sorted list of files
        """
        import re
        
        # Try to use dependency graph if available
        if hasattr(self, 'dep_graph') and self.dep_graph:
            return self._sort_by_dep_graph(files, project_path)
        
        # Fallback: sort by heuristics
        self.logger.info("Using heuristic-based sorting (AST parsing unavailable)")
        
        library_files = []
        main_files = []
        example_files = []
        
        for f in files:
            filepath = os.path.join(project_path, f.path)
            filename = os.path.basename(f.path).lower()
            
            # Skip header files (they'll be included)
            if f.path.endswith('.h') or f.path.endswith('.hpp'):
                continue
            
            # Check if file contains main() function
            try:
                with open(filepath, 'r', encoding='utf-8', errors='replace') as fp:
                    content = fp.read()
                
                # Check for main function
                has_main = bool(re.search(r'\bint\s+main\s*\(', content))
                
                # Check if it's an example file
                is_example = 'example' in filename or 'demo' in filename or 'test' in filename
                
                if has_main or is_example:
                    if is_example:
                        example_files.append(f)
                    else:
                        main_files.append(f)
                else:
                    library_files.append(f)
                    
            except Exception:
                # If we can't read, assume it's a library file
                library_files.append(f)
        
        # Sort order: libraries first, then main files, then examples
        sorted_files = library_files + main_files + example_files
        
        self.logger.info(f"Translation order: {len(library_files)} lib, {len(main_files)} main, {len(example_files)} example")
        for f in sorted_files:
            self.logger.debug(f"  - {f.path}")
        
        return sorted_files
    
    def _sort_by_dep_graph(self, files: list, project_path: str) -> list:
        """
        Sort files using #include-based dependency analysis.
        
        Files are ordered so that dependencies are translated first.
        """
        self.logger.info("Using #include-based topological sort for translation order")
        
        # Use the parser's file-level dependency analysis
        from rustify.graph.parser import CParser
        parser = CParser(project_path)
        
        # Get file dependencies from #include directives
        sorted_paths = parser.topological_sort_files()
        
        # Map file paths to ProjectFile objects
        file_map = {f.path: f for f in files}
        
        # Build ordered file list
        sorted_files = []
        seen_files = set()
        
        for rel_path in sorted_paths:
            if rel_path in file_map and rel_path not in seen_files:
                # Skip headers, we translate .c files
                if not rel_path.endswith('.h') and not rel_path.endswith('.hpp'):
                    sorted_files.append(file_map[rel_path])
                    seen_files.add(rel_path)
        
        # Add any remaining files not in the sorted list
        for f in files:
            if f.path not in seen_files and not f.path.endswith('.h') and not f.path.endswith('.hpp'):
                sorted_files.append(f)
                seen_files.add(f.path)
        
        self.logger.info(f"File dependency sort: {len(sorted_files)} files")
        for i, f in enumerate(sorted_files, 1):
            self.logger.info(f"  {i}. {f.path}")
        
        return sorted_files
    
    def _is_test_file(self, filepath: str) -> bool:
        """
        Check if a file is a test file.
        
        Test files are identified by:
        - Filename starts with 'test-', 'test_', or 'tests-'
        - Filename ends with '_test.c', '-test.c', '_tests.c', '-tests.c'
        - Located in 'test/', 'tests/', 'testing/' directories
        
        Args:
            filepath: Relative file path
            
        Returns:
            True if the file is a test file
        """
        basename = os.path.basename(filepath).lower()
        dirname = os.path.dirname(filepath).lower()
        
        # Check directory patterns
        test_dirs = {'test', 'tests', 'testing', 'test-cases', 'test_cases'}
        path_parts = set(dirname.replace('\\', '/').split('/'))
        if path_parts & test_dirs:
            return True
        
        # Check filename patterns
        # Starts with test
        if basename.startswith(('test-', 'test_', 'tests-', 'tests_')):
            return True
        
        # Ends with test
        name_without_ext = os.path.splitext(basename)[0]
        if name_without_ext.endswith(('_test', '-test', '_tests', '-tests')):
            return True
        
        return False
    
    def _is_build_artifact(self, filepath: str) -> bool:
        """
        Check if a file is a build system artifact that should be skipped.
        
        Build artifacts include:
        - Files in build/, Build/, cmake-build-*/, CMakeFiles/ directories
        - CMake compiler detection files (CMakeCCompilerId.c, etc.)
        - Auto-generated files (*.gen.c, etc.)
        
        Args:
            filepath: Relative file path
            
        Returns:
            True if the file is a build artifact
        """
        filepath_lower = filepath.lower().replace('\\', '/')
        basename = os.path.basename(filepath).lower()
        
        # Check for build directories
        build_dirs = {
            'build', 'builds', '_build',
            'cmake-build-debug', 'cmake-build-release', 'cmake-build',
            'cmakefiles', 'cmake_files',
            '.build', 'out', 'output',
        }
        
        path_parts = filepath_lower.split('/')
        for part in path_parts:
            if part in build_dirs:
                return True
            # CMakeFiles can be nested
            if part.startswith('cmakefiles'):
                return True
        
        # Check for CMake compiler detection files
        if 'compilerid' in basename:
            return True
        
        # Check for auto-generated files
        if basename.endswith(('.gen.c', '.gen.h', '.generated.c', '.generated.h')):
            return True
        
        return False
    
    def assign_module_translations(self) -> AgentResponse:
        """Create and assign module translation tasks."""
        self.logger.info("Assigning module translations")
        
        project = self.state_manager.state.source_project
        if not project:
            return AgentResponse.error(
                self,
                OrchestratorResponseType.ALL_MODULES_DONE,
                {"message": "Source project not loaded"}
            )
        
        # Check if module already exists (avoid duplicates)
        existing_modules = self.state_manager.state.module_translations
        if existing_modules:
            self.logger.info(f"Found {len(existing_modules)} existing module(s), skipping creation")
            return AgentResponse.done(
                self,
                OrchestratorResponseType.ALL_MODULES_DONE,
                {"module_count": len(existing_modules), "skipped": True}
            )
        
        # Get all C files, EXCLUDING test files and build artifacts
        # 测试文件由 Validator 在核心代码翻译完成后单独处理
        all_c_files = [
            f for f in project.list_files()
            if f.path.endswith('.c') or f.path.endswith('.cpp')
        ]
        
        # Filter out test files and build artifacts
        c_files = []
        test_files = []
        build_artifacts = []
        for f in all_c_files:
            if self._is_build_artifact(f.path):
                build_artifacts.append(f.path)
                self.logger.info(f"Excluding build artifact: {f.path}")
            elif self._is_test_file(f.path):
                test_files.append(f.path)
                self.logger.info(f"Excluding test file: {f.path}")
            else:
                c_files.append(f)
        
        if build_artifacts:
            self.logger.info(f"Excluded {len(build_artifacts)} build artifact(s)")
        
        if test_files:
            self.logger.info(f"Excluded {len(test_files)} test file(s), will be handled by Validator later")
            # Store test files for later use by Validator
            self.state_manager.state.test_files = test_files
        
        # Sort files by dependency order:
        # 1. Library files first (no main function)
        # 2. Files with main() last (they depend on libraries)
        # 3. Header files are not translated separately
        c_files = self._sort_by_dependency(c_files, project.path)
        
        if c_files:
            from rustify.schema.translation import (
                TranslationTask,
                TranslationTaskSource,
                TranslationUnitNode,
            )
            import uuid
            
            # Create translation tasks for each file
            tasks = []
            for f in c_files:
                # Read file content
                content = ""
                try:
                    with open(os.path.join(project.path, f.path), "r", encoding="utf-8", errors="replace") as fp:
                        content = fp.read()
                except Exception as e:
                    self.logger.warning(f"Failed to read {f.path}: {e}")
                    continue
                
                node = TranslationUnitNode(
                    filepath=f.path,
                    id=str(uuid.uuid4()),
                    name=os.path.basename(f.path),
                    type="file",
                    text=content
                )
                
                task = TranslationTask(
                    source=TranslationTaskSource(
                        name=os.path.basename(f.path),
                        nodes=[node],
                        description=f"Translate {f.path}"
                    )
                )
                tasks.append(task)
            
            # Create module
            module = ModuleTranslation(
                name=project.name,
                description=project.description or "Main module",
                translation_tasks=tasks,
                related_files=[f.path for f in c_files],
                status=ModuleTranslationStatus.CREATED
            )
            
            self.state_manager.add_module_translation(module)
            self.logger.info(f"Created module '{module.name}' with {len(tasks)} tasks")
        
        return AgentResponse.done(
            self,
            OrchestratorResponseType.ALL_MODULES_DONE,
            {"module_count": len(self.state_manager.state.module_translations)}
        )

