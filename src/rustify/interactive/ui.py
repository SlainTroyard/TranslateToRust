"""
Interactive UI - Rich terminal interface for Rustify.

Provides a beautiful, interactive terminal experience.
"""

import os
import time
from typing import Optional, Dict, Any, List
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn, TaskProgressColumn
from rich.live import Live
from rich.table import Table
from rich.panel import Panel
from rich.layout import Layout
from rich.tree import Tree
from rich.syntax import Syntax


class InteractiveUI:
    """
    Rich interactive UI for Rustify.
    
    Features:
    - Real-time progress tracking
    - File tree visualization
    - Error display with syntax highlighting
    - Statistics dashboard
    """
    
    def __init__(self, console: Optional[Console] = None):
        """Initialize the UI."""
        self.console = console or Console()
        self._progress: Optional[Progress] = None
        self._tasks: Dict[str, Any] = {}
    
    def banner(self) -> None:
        """Display the Rustify banner."""
        banner_text = """
[bold cyan]
  ██████╗ ██╗   ██╗███████╗████████╗██╗███████╗██╗   ██╗
  ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██║██╔════╝╚██╗ ██╔╝
  ██████╔╝██║   ██║███████╗   ██║   ██║█████╗   ╚████╔╝ 
  ██╔══██╗██║   ██║╚════██║   ██║   ██║██╔══╝    ╚██╔╝  
  ██║  ██║╚██████╔╝███████║   ██║   ██║██║        ██║   
  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝╚═╝        ╚═╝   
[/bold cyan]
[dim]Intelligent C/C++ to Rust Translation[/dim]
        """
        self.console.print(banner_text)
    
    def show_project_info(
        self,
        source_dir: str,
        target_dir: str,
        file_count: int,
        mode: str = "full"
    ) -> None:
        """Display project information."""
        table = Table(title="Project Information", show_header=False)
        table.add_column("Property", style="cyan")
        table.add_column("Value")
        
        table.add_row("Source", source_dir)
        table.add_row("Target", target_dir)
        table.add_row("Files", str(file_count))
        table.add_row("Mode", mode)
        
        self.console.print(table)
    
    def show_file_tree(
        self,
        root_path: str,
        files: List[str],
        status_map: Optional[Dict[str, str]] = None
    ) -> None:
        """Display a file tree with optional status indicators."""
        tree = Tree(f"[bold]{os.path.basename(root_path)}[/bold]")
        
        # Build tree structure
        dirs: Dict[str, Any] = {}
        
        for file_path in sorted(files):
            parts = file_path.split(os.sep)
            current = dirs
            
            for i, part in enumerate(parts[:-1]):
                if part not in current:
                    current[part] = {}
                current = current[part]
            
            # Add file with status
            status = ""
            if status_map and file_path in status_map:
                status_icons = {
                    "done": "[green]✓[/green]",
                    "translating": "[yellow]⟳[/yellow]",
                    "error": "[red]✗[/red]",
                    "pending": "[dim]○[/dim]"
                }
                status = status_icons.get(status_map[file_path], "")
            
            current[parts[-1]] = f"{status} {parts[-1]}" if status else parts[-1]
        
        def add_to_tree(node: Tree, items: Dict) -> None:
            for name, value in items.items():
                if isinstance(value, dict):
                    branch = node.add(f"[blue]{name}/[/blue]")
                    add_to_tree(branch, value)
                else:
                    node.add(value)
        
        add_to_tree(tree, dirs)
        self.console.print(tree)
    
    def create_progress(self) -> Progress:
        """Create a progress bar context."""
        return Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            BarColumn(),
            TaskProgressColumn(),
            console=self.console
        )
    
    def show_statistics(
        self,
        stats: Dict[str, Any],
        title: str = "Translation Statistics"
    ) -> None:
        """Display statistics in a nice table."""
        table = Table(title=title)
        table.add_column("Metric", style="cyan")
        table.add_column("Value", justify="right")
        
        for key, value in stats.items():
            if isinstance(value, float):
                value = f"{value:.2f}"
            table.add_row(key.replace("_", " ").title(), str(value))
        
        self.console.print(table)
    
    def show_error_summary(
        self,
        errors: List[Dict[str, Any]],
        max_display: int = 10
    ) -> None:
        """Display a summary of errors."""
        if not errors:
            self.console.print("[green]No errors! 🎉[/green]")
            return
        
        self.console.print(f"\n[bold red]Errors ({len(errors)} total):[/bold red]")
        
        for error in errors[:max_display]:
            self.console.print(Panel(
                f"[yellow]{error.get('file', 'unknown')}:{error.get('line', '?')}[/yellow]\n"
                f"{error.get('message', 'Unknown error')}",
                border_style="red"
            ))
        
        if len(errors) > max_display:
            self.console.print(f"[dim]... and {len(errors) - max_display} more errors[/dim]")
    
    def show_diff(
        self,
        original: str,
        modified: str,
        filename: str = "file"
    ) -> None:
        """Display a diff between two code versions."""
        import difflib
        
        diff = difflib.unified_diff(
            original.splitlines(keepends=True),
            modified.splitlines(keepends=True),
            fromfile=f"a/{filename}",
            tofile=f"b/{filename}"
        )
        
        diff_text = "".join(diff)
        if diff_text:
            self.console.print(Syntax(diff_text, "diff", theme="monokai"))
        else:
            self.console.print("[dim]No changes[/dim]")
    
    def confirm(self, message: str, default: bool = True) -> bool:
        """Ask for confirmation."""
        from rich.prompt import Confirm
        return Confirm.ask(message, default=default)
    
    def prompt(self, message: str, default: str = "") -> str:
        """Ask for user input."""
        from rich.prompt import Prompt
        return Prompt.ask(message, default=default)
    
    def status(self, message: str):
        """Create a status context manager."""
        from rich.status import Status
        return Status(message, console=self.console)
    
    def success(self, message: str) -> None:
        """Display a success message."""
        self.console.print(f"[bold green]✓[/bold green] {message}")
    
    def error(self, message: str) -> None:
        """Display an error message."""
        self.console.print(f"[bold red]✗[/bold red] {message}")
    
    def warning(self, message: str) -> None:
        """Display a warning message."""
        self.console.print(f"[bold yellow]⚠[/bold yellow] {message}")
    
    def info(self, message: str) -> None:
        """Display an info message."""
        self.console.print(f"[bold blue]ℹ[/bold blue] {message}")
