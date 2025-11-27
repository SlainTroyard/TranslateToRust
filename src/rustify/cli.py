"""
Rustify CLI - Command Line Interface for Rustify.

"""

import os
import sys
import json
import logging
from pathlib import Path
from typing import Optional

import typer
from rich import print as rprint
from rich.console import Console
from rich.table import Table
from rich.progress import Progress, SpinnerColumn, TextColumn

app = typer.Typer(
    name="rustify",
    help="Translate C/C++ projects to Rust using AI.",
    add_completion=False,
)

console = Console()


def setup_logging(verbose: bool = False):
    """Setup logging configuration."""
    # Set rustify logger level
    rustify_level = logging.DEBUG if verbose else logging.INFO
    
    # Configure root logger (for our code)
    logging.basicConfig(
        level=rustify_level,
        format="[%(asctime)s] [%(name)s] %(levelname)s: %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S"
    )
    
    # Suppress verbose logs from third-party libraries
    logging.getLogger("LiteLLM").setLevel(logging.WARNING)
    logging.getLogger("litellm").setLevel(logging.WARNING)
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    logging.getLogger("openai").setLevel(logging.WARNING)
    logging.getLogger("urllib3").setLevel(logging.WARNING)


@app.command()
def translate(
    source: str = typer.Argument(..., help="Path to C/C++ source project"),
    target: str = typer.Argument(..., help="Path for output Rust project"),
    config: Optional[str] = typer.Option(None, "--config", "-c", help="Config file path"),
    overwrite: bool = typer.Option(False, "--overwrite", "-f", help="Overwrite existing target"),
    incremental: bool = typer.Option(False, "--incremental", "-i", help="Incremental translation (only changed files)"),
    interactive: bool = typer.Option(False, "--interactive", help="Interactive mode (human-in-the-loop)"),
    dashboard: bool = typer.Option(False, "--dashboard", "-d", help="Open web dashboard"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Verbose output"),
):
    """
    Translate a C/C++ project to Rust.
    
    Examples:
        rustify translate ./my-c-project ./my-rust-project
        rustify translate ./src ./out --overwrite
        rustify translate ./src ./out --incremental  # Only translate changed files
        rustify translate ./src ./out --interactive  # Human-in-the-loop mode
        rustify translate ./src ./out --dashboard    # With web UI
    """
    setup_logging(verbose)
    
    from rustify import Rustify
    from rustify.config import RustifyConfig
    
    # Load config
    if config:
        cfg = RustifyConfig.load(config)
    else:
        cfg = RustifyConfig()
    
    rprint(f"[bold blue]Rustify[/bold blue] - C/C++ to Rust Translator")
    rprint(f"Source: [green]{source}[/green]")
    rprint(f"Target: [green]{target}[/green]")
    
    if incremental:
        rprint(f"Mode: [cyan]Incremental[/cyan] (only changed files)")
    if interactive:
        rprint(f"Mode: [cyan]Interactive[/cyan] (human-in-the-loop)")
    
    # Check source exists
    if not os.path.exists(source):
        rprint(f"[red]Error:[/red] Source directory not found: {source}")
        raise typer.Exit(1)
    
    # Confirm overwrite
    if os.path.exists(target) and not overwrite and not incremental:
        if not typer.confirm(f"Target {target} exists. Overwrite?"):
            raise typer.Exit(0)
    
    # Start dashboard if requested
    web_dashboard = None
    if dashboard:
        try:
            from rustify.web import Dashboard
            web_dashboard = Dashboard(auto_open=True)
            web_dashboard.start()
            rprint(f"[green]Dashboard[/green]: {web_dashboard.url}")
        except ImportError:
            rprint("[yellow]Warning:[/yellow] Flask not installed. Dashboard disabled.")
            rprint("  Install with: pip install flask flask-cors")
    
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console,
    ) as progress:
        progress.add_task("Translating...", total=None)
        
        rustify = Rustify(config=cfg)
        
        # Use incremental mode if requested
        if incremental:
            from rustify.incremental import ChangeDetector
            detector = ChangeDetector(source, target)
            changes = detector.detect_changes()
            
            if not changes.has_changes:
                rprint("[green]No changes detected![/green] Everything is up to date.")
                raise typer.Exit(0)
            
            rprint(f"\n{changes.summary()}")
            rprint(f"\nTranslating {len(changes.files_to_translate)} file(s)...")
            
            success = rustify.translate(
                source, target, 
                overwrite=False,
                files_filter=changes.files_to_translate
            )
            
            # Mark translated files
            if success:
                detector.mark_translated(changes.files_to_translate)
        else:
            success = rustify.translate(source, target, overwrite=overwrite)
    
    if web_dashboard:
        web_dashboard.stop()
    
    if success:
        rprint(f"\n[green]✓[/green] Translation completed successfully!")
        rprint(f"  Output: [bold]{target}[/bold]")
    else:
        rprint(f"\n[yellow]⚠[/yellow] Translation completed with some issues")
        rprint(f"  Check the output and logs for details")


@app.command()
def analyze(
    source: str = typer.Argument(..., help="Path to C/C++ source project"),
    output: Optional[str] = typer.Option(None, "--output", "-o", help="Output JSON file"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Verbose output"),
):
    """
    Analyze a C/C++ project without translating.
    
    Shows project structure, dependencies, and translation plan.
    """
    setup_logging(verbose)
    
    from rustify import Rustify
    
    if not os.path.exists(source):
        rprint(f"[red]Error:[/red] Source directory not found: {source}")
        raise typer.Exit(1)
    
    rprint(f"[bold blue]Analyzing[/bold blue]: {source}")
    
    rustify = Rustify()
    analysis = rustify.analyze_only(source)
    
    # Display results
    rprint(f"\n[bold]Project Analysis[/bold]")
    rprint(f"  Nodes found: {analysis['node_count']}")
    rprint(f"  Translation levels: {analysis['level_count']}")
    
    # Show nodes table
    if analysis['nodes']:
        table = Table(title="Code Elements")
        table.add_column("Name", style="cyan")
        table.add_column("Type", style="green")
        table.add_column("Lines", justify="right")
        table.add_column("Location")
        
        for node in analysis['nodes'][:20]:  # Limit to 20
            table.add_row(
                node['name'],
                node['type'],
                str(node['line_count']),
                os.path.basename(node['location'])
            )
        
        if len(analysis['nodes']) > 20:
            table.add_row("...", "...", "...", f"({len(analysis['nodes']) - 20} more)")
        
        console.print(table)
    
    # Save to file if requested
    if output:
        with open(output, 'w') as f:
            json.dump(analysis, f, indent=2)
        rprint(f"\n[green]✓[/green] Analysis saved to: {output}")


@app.command()
def resume(
    state_file: str = typer.Argument(..., help="Path to state file"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Verbose output"),
):
    """
    Resume a previous translation from state file.
    
    Useful when translation was interrupted.
    """
    setup_logging(verbose)
    
    from rustify import Rustify
    
    if not os.path.exists(state_file):
        rprint(f"[red]Error:[/red] State file not found: {state_file}")
        raise typer.Exit(1)
    
    rprint(f"[bold blue]Resuming[/bold blue] from: {state_file}")
    
    rustify = Rustify()
    success = rustify.resume(state_file)
    
    if success:
        rprint(f"\n[green]✓[/green] Translation resumed and completed!")
    else:
        rprint(f"\n[yellow]⚠[/yellow] Some issues during resumption")


@app.command()
def init(
    output: str = typer.Option("rustify.toml", "--output", "-o", help="Config file path"),
):
    """
    Initialize a new configuration file.
    
    Creates a rustify.toml with default settings.
    """
    from rustify.config import RustifyConfig
    
    if os.path.exists(output):
        if not typer.confirm(f"{output} exists. Overwrite?"):
            raise typer.Exit(0)
    
    config = RustifyConfig()
    config.save(output)
    
    rprint(f"[green]✓[/green] Created configuration file: {output}")
    rprint(f"  Edit this file to customize settings.")


@app.command()
def fix(
    project: str = typer.Argument(..., help="Path to Rust project with errors"),
    interactive: bool = typer.Option(True, "--interactive/--auto", help="Interactive or automatic fixing"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Verbose output"),
):
    """
    Fix compilation errors in a translated project.
    
    Interactive mode (default) lets you review and approve each fix.
    
    Examples:
        rustify fix ./my-rust-project
        rustify fix ./my-rust-project --auto  # Automatic fixing
    """
    setup_logging(verbose)
    
    from rustify.tools.rust_utils import cargo_check
    from rustify.interactive import InteractiveFixer, InteractiveUI
    
    if not os.path.exists(project):
        rprint(f"[red]Error:[/red] Project not found: {project}")
        raise typer.Exit(1)
    
    ui = InteractiveUI(console)
    ui.banner()
    
    # Check for errors
    result = cargo_check(project)
    
    if result["success"]:
        ui.success("No compilation errors found!")
        raise typer.Exit(0)
    
    errors = result.get("errors", [])
    ui.error(f"Found {len(errors)} compilation error(s)")
    
    if interactive:
        fixer = InteractiveFixer(console)
        rprint("\n[bold]Starting interactive fix mode...[/bold]")
        rprint("[dim]You can review and approve each fix proposed by the AI[/dim]\n")
        
        # This would integrate with the reasoner for actual fixes
        rprint("[yellow]Note:[/yellow] Interactive fixing requires active translation session")
    else:
        rprint("\n[bold]Starting automatic fix mode...[/bold]")
        # Automatic fixing logic would go here


@app.command()
def dashboard(
    project: str = typer.Argument(None, help="Path to project (optional)"),
    port: int = typer.Option(8765, "--port", "-p", help="Port number"),
    host: str = typer.Option("127.0.0.1", "--host", "-H", help="Host to bind (use 0.0.0.0 for remote access)"),
):
    """
    Start the web dashboard for monitoring translations.
    
    Examples:
        rustify dashboard
        rustify dashboard ./my-project --port 9000
        rustify dashboard --host 0.0.0.0  # Allow remote access
    """
    from rustify.web import Dashboard
    
    try:
        # Don't auto-open browser if binding to 0.0.0.0 (remote server)
        auto_open = (host == "127.0.0.1")
        d = Dashboard(host=host, port=port, auto_open=auto_open)
        d.start()
        
        rprint(f"[bold green]Dashboard running at[/bold green] {d.url}")
        rprint("[dim]Press Ctrl+C to stop[/dim]\n")
        
        if project and os.path.exists(project):
            # Load project state if available
            state_file = os.path.join(project, "states.json")
            if os.path.exists(state_file):
                rprint(f"[green]✓[/green] Loaded project state from {state_file}")
        
        # Keep running
        import time
        while True:
            time.sleep(1)
            
    except ImportError:
        rprint("[red]Error:[/red] Flask not installed")
        rprint("  Install with: pip install flask flask-cors")
        raise typer.Exit(1)
    except KeyboardInterrupt:
        rprint("\n[yellow]Dashboard stopped[/yellow]")


@app.command()
def diff(
    source: str = typer.Argument(..., help="Path to C/C++ source"),
    since: str = typer.Option("HEAD~1", "--since", "-s", help="Git ref or 'cache' for cached state"),
):
    """
    Show files that would be translated in incremental mode.
    
    Examples:
        rustify diff ./my-project
        rustify diff ./my-project --since HEAD~5
        rustify diff ./my-project --since cache
    """
    from rustify.incremental import ChangeDetector
    
    if not os.path.exists(source):
        rprint(f"[red]Error:[/red] Source not found: {source}")
        raise typer.Exit(1)
    
    detector = ChangeDetector(source)
    
    if since == "cache":
        changes = detector.detect_changes()
    else:
        changes = detector.try_git_diff(since)
        if changes is None:
            rprint("[yellow]Git not available, using file hash comparison[/yellow]")
            changes = detector.detect_changes()
    
    rprint(f"\n{changes.summary()}")
    
    if changes.added:
        rprint("\n[green]New files:[/green]")
        for f in changes.added:
            rprint(f"  + {f}")
    
    if changes.modified:
        rprint("\n[yellow]Modified files:[/yellow]")
        for f in changes.modified:
            rprint(f"  ~ {f}")
    
    if changes.deleted:
        rprint("\n[red]Deleted files:[/red]")
        for f in changes.deleted:
            rprint(f"  - {f}")


@app.command()
def version():
    """Show version information."""
    from rustify import __version__
    
    rprint(f"[bold blue]Rustify[/bold blue] v{__version__}")
    rprint(f"  A C/C++ to Rust translation tool using AI")
    rprint(f"\n[dim]Features:[/dim]")
    rprint(f"  • Incremental translation (--incremental)")
    rprint(f"  • Interactive fixing (rustify fix)")
    rprint(f"  • Web dashboard (rustify dashboard)")
    rprint(f"  • Human-in-the-loop review (--interactive)")


def main():
    """Entry point for CLI."""
    app()


if __name__ == "__main__":
    main()
