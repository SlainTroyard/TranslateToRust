"""
Project Schema - Data models for source and target projects.

"""

import os
from pathlib import Path
from typing import Optional, List, Callable, Literal

from pydantic import BaseModel, Field


class ProjectFile(BaseModel):
    """Represents a file in a project."""
    
    type: Literal["file"] = "file"
    path: str
    content: Optional[str] = None
    summary: Optional[str] = None


class Project:
    """
    Represents a source C/C++ project.
    
    Manages file listing, structure display, and metadata.
    """
    
    DEFAULT_IGNORES = [".git", ".vcs", ".gitignore", "target", "Cargo.lock", "__pycache__", ".venv"]
    
    def __init__(
        self,
        id: str,
        name: str,
        path: str,
        description: Optional[str] = None,
        file_summaries: Optional[dict[str, str]] = None,
        **kwargs
    ):
        self.id = id
        self.name = name
        self.path = path
        self.description = description
        self.file_summaries = file_summaries or {}
        self.details = kwargs
    
    def list_files(
        self,
        show_content: bool = False,
        show_summary: bool = False,
        show_line_numbers: bool = False,
        ignore_func: Optional[Callable[[str], bool]] = None,
        relpath: Optional[str] = None
    ) -> List[ProjectFile]:
        """
        List all files in the project directory.
        
        Args:
            show_content: Whether to include file content.
            show_summary: Whether to include file summary.
            show_line_numbers: Whether to add line numbers to content.
            ignore_func: Function to filter files (returns True to ignore).
            relpath: Relative path base for file paths.
            
        Returns:
            List of ProjectFile objects.
        """
        path = self.path
        file_list = []
        
        for root, dirs, files in os.walk(path):
            # Remove ignored directories
            for ignore in Project.DEFAULT_IGNORES:
                if ignore in dirs:
                    dirs.remove(ignore)
            
            if relpath and not root.startswith(relpath):
                continue
            
            for file in files:
                if os.path.basename(file) in Project.DEFAULT_IGNORES:
                    continue
                
                abs_filepath = os.path.join(root, file)
                
                if relpath:
                    filepath = os.path.relpath(abs_filepath, relpath)
                else:
                    filepath = os.path.relpath(abs_filepath, self.path)
                
                if ignore_func and ignore_func(filepath):
                    continue
                
                project_file = ProjectFile(path=filepath)
                
                if show_content:
                    try:
                        with open(abs_filepath, "r", encoding="utf-8", errors="replace") as f:
                            content = f.read()
                        if show_line_numbers:
                            content = self._add_line_numbers(content)
                        project_file.content = content
                    except Exception:
                        project_file.content = "# Error reading file"
                
                if show_summary:
                    rel_path = os.path.relpath(abs_filepath, self.path)
                    project_file.summary = self.file_summaries.get(rel_path, "")
                
                file_list.append(project_file)
        
        return file_list
    
    def _add_line_numbers(self, content: str) -> str:
        """Add line numbers to content."""
        lines = content.split("\n")
        numbered = []
        for i, line in enumerate(lines, 1):
            numbered.append(f"{i:4d} | {line}")
        return "\n".join(numbered)
    
    def pretty_structure(self, ignore_func: Optional[Callable[[str], bool]] = None) -> str:
        """Return a pretty-printed project structure."""
        
        def build_tree(dirpath: str) -> List[dict]:
            items = []
            try:
                entries = sorted(os.listdir(dirpath))
            except PermissionError:
                return items
            
            for entry in entries:
                if entry in Project.DEFAULT_IGNORES:
                    continue
                
                full_path = os.path.join(dirpath, entry)
                rel_path = os.path.relpath(full_path, self.path)
                
                if ignore_func and ignore_func(rel_path):
                    continue
                
                item = {"path": rel_path, "children": []}
                
                if os.path.isdir(full_path):
                    item["type"] = "dir"
                    item["children"] = build_tree(full_path)
                else:
                    item["type"] = "file"
                
                items.append(item)
            
            return items
        
        def format_tree(node: dict, indent: int = 0) -> str:
            result = " " * indent + f"[{node['type'].upper()}] {node['path']}\n"
            for child in node.get("children", []):
                result += format_tree(child, indent + 2)
            return result
        
        root = {
            "path": os.path.basename(self.path),
            "type": "dir",
            "children": build_tree(self.path)
        }
        
        return format_tree(root)
    
    def to_dict(self) -> dict:
        """Convert to dictionary."""
        return {
            "id": self.id,
            "name": self.name,
            "path": self.path,
            "description": self.description,
            "file_summaries": self.file_summaries,
            **self.details
        }
    
    @property
    def src_path(self) -> str:
        """Source directory path."""
        return os.path.join(self.path, "src")
    
    @property
    def test_path(self) -> str:
        """Test directory path."""
        return os.path.join(self.path, "test")


class TargetProject(Project):
    """
    Represents the target Rust project.
    
    Extends Project with Rust-specific functionality.
    """
    
    def __init__(
        self,
        id: str,
        name: str,
        path: str,
        description: Optional[str] = None,
        **kwargs
    ):
        super().__init__(id, name, path, description, **kwargs)
    
    @property
    def test_path(self) -> str:
        """Test directory path (Rust convention)."""
        return os.path.join(self.path, "tests")
    
    @property
    def benches_path(self) -> str:
        """Benchmarks directory path."""
        return os.path.join(self.path, "benches")
    
    def ensure_structure(self, crate_type: str = "lib") -> None:
        """
        Ensure the Rust project structure exists.
        
        Creates:
        - Project directory
        - src/ directory
        - lib.rs or main.rs (based on crate_type)
        
        Args:
            crate_type: 'lib' for library or 'bin' for binary
        """
        os.makedirs(self.path, exist_ok=True)
        os.makedirs(os.path.join(self.path, "src"), exist_ok=True)
        
        # Create lib.rs or main.rs based on crate type
        if crate_type == "lib":
            lib_path = os.path.join(self.path, "src", "lib.rs")
            if not os.path.exists(lib_path):
                with open(lib_path, "w") as f:
                    f.write("//! Auto-generated library root\n\n")
        else:
            main_path = os.path.join(self.path, "src", "main.rs")
            if not os.path.exists(main_path):
                with open(main_path, "w") as f:
                    f.write("fn main() {\n    println!(\"Hello, world!\");\n}\n")
    
    def write_cargo_toml(
        self,
        crate_type: str = "lib",
        dependencies: Optional[dict] = None
    ) -> None:
        """
        Write or update Cargo.toml.
        
        Args:
            crate_type: 'lib' for library or 'bin' for binary
            dependencies: Additional dependencies to add
        """
        import toml
        
        cargo_path = os.path.join(self.path, "Cargo.toml")
        
        if os.path.exists(cargo_path):
            with open(cargo_path, "r") as f:
                cargo = toml.load(f)
        else:
            cargo = {}
        
        # Package section
        if "package" not in cargo:
            cargo["package"] = {}
        cargo["package"]["name"] = self.name
        cargo["package"]["version"] = "0.1.0"
        cargo["package"]["edition"] = "2021"
        if self.description:
            cargo["package"]["description"] = self.description
        
        # Specify lib or bin target explicitly
        if crate_type == "lib":
            if "lib" not in cargo:
                cargo["lib"] = {}
            cargo["lib"]["name"] = self.name.replace("-", "_")
            cargo["lib"]["path"] = "src/lib.rs"
        else:
            # For binary, Cargo auto-detects src/main.rs
            pass
        
        # Dependencies
        if "dependencies" not in cargo:
            cargo["dependencies"] = {}
        cargo["dependencies"]["libc"] = "0.2"
        if dependencies:
            cargo["dependencies"].update(dependencies)
        
        # Dev dependencies
        if "dev-dependencies" not in cargo:
            cargo["dev-dependencies"] = {}
        cargo["dev-dependencies"]["criterion"] = "0.5"
        
        with open(cargo_path, "w") as f:
            toml.dump(cargo, f)

