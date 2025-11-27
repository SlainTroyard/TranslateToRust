"""
Interactive Fixer - Human-in-the-loop error fixing.

Allows users to:
1. Review compilation errors
2. Suggest fixes manually
3. Guide the LLM with context
4. Accept/reject proposed changes
"""

import os
from typing import Optional, List, Callable, Dict, Any
from dataclasses import dataclass, field
from enum import Enum
from rich.console import Console
from rich.panel import Panel
from rich.syntax import Syntax
from rich.prompt import Prompt, Confirm
from rich.table import Table


class FixAction(Enum):
    """User action for a fix."""
    ACCEPT = "accept"
    REJECT = "reject"
    EDIT = "edit"
    SKIP = "skip"
    RETRY_WITH_HINT = "retry_with_hint"
    MANUAL_FIX = "manual_fix"


@dataclass
class CompilationError:
    """Represents a compilation error."""
    code: Optional[str]
    message: str
    file_path: str
    line: int
    column: int
    rendered: str
    suggestion: Optional[str] = None


@dataclass
class FixProposal:
    """A proposed fix from the LLM."""
    original_code: str
    fixed_code: str
    explanation: str
    error: CompilationError
    confidence: float = 0.0


@dataclass 
class FixResult:
    """Result of a fix attempt."""
    action: FixAction
    final_code: str
    user_feedback: Optional[str] = None
    manual_edit: bool = False


class InteractiveFixer:
    """
    Interactive error fixer with human-in-the-loop.
    
    This unique feature allows users to:
    - Review LLM-proposed fixes before applying
    - Provide hints to guide the LLM
    - Make manual edits when LLM fails
    - Build a feedback loop for better translations
    """
    
    def __init__(
        self,
        console: Optional[Console] = None,
        auto_accept_threshold: float = 0.9
    ):
        """
        Initialize the interactive fixer.
        
        Args:
            console: Rich console for output
            auto_accept_threshold: Confidence threshold for auto-accepting fixes
        """
        self.console = console or Console()
        self.auto_accept_threshold = auto_accept_threshold
        self.fix_history: List[FixResult] = []
        self._callbacks: Dict[str, Callable] = {}
    
    def on(self, event: str, callback: Callable) -> None:
        """Register an event callback."""
        self._callbacks[event] = callback
    
    def _emit(self, event: str, **kwargs) -> Any:
        """Emit an event."""
        if event in self._callbacks:
            return self._callbacks[event](**kwargs)
    
    def display_error(self, error: CompilationError) -> None:
        """Display a compilation error with context."""
        self.console.print()
        self.console.print(Panel(
            f"[bold red]Compilation Error[/bold red]\n\n"
            f"[yellow]File:[/yellow] {error.file_path}:{error.line}:{error.column}\n"
            f"[yellow]Code:[/yellow] {error.code or 'N/A'}\n\n"
            f"{error.message}",
            title="❌ Error",
            border_style="red"
        ))
        
        if error.rendered:
            self.console.print(Syntax(
                error.rendered,
                "rust",
                theme="monokai",
                line_numbers=True
            ))
    
    def display_proposal(self, proposal: FixProposal) -> None:
        """Display a fix proposal."""
        self.console.print()
        self.console.print(Panel(
            f"[bold green]Proposed Fix[/bold green]\n\n"
            f"[yellow]Confidence:[/yellow] {proposal.confidence:.1%}\n\n"
            f"[bold]Explanation:[/bold]\n{proposal.explanation}",
            title="💡 LLM Suggestion",
            border_style="green"
        ))
        
        # Show diff
        self.console.print("\n[bold]Code Changes:[/bold]")
        self.console.print(Syntax(
            proposal.fixed_code,
            "rust",
            theme="monokai",
            line_numbers=True
        ))
    
    def prompt_action(self) -> FixAction:
        """Prompt user for action on a fix proposal."""
        self.console.print()
        
        table = Table(show_header=False, box=None)
        table.add_column("Key", style="bold cyan")
        table.add_column("Action")
        table.add_row("a", "Accept this fix")
        table.add_row("r", "Reject and try again")
        table.add_row("e", "Edit the fix manually")
        table.add_row("h", "Retry with a hint for the LLM")
        table.add_row("m", "Fix it manually (open editor)")
        table.add_row("s", "Skip this error")
        
        self.console.print(table)
        
        choice = Prompt.ask(
            "Choose action",
            choices=["a", "r", "e", "h", "m", "s"],
            default="a"
        )
        
        action_map = {
            "a": FixAction.ACCEPT,
            "r": FixAction.REJECT,
            "e": FixAction.EDIT,
            "h": FixAction.RETRY_WITH_HINT,
            "m": FixAction.MANUAL_FIX,
            "s": FixAction.SKIP
        }
        
        return action_map[choice]
    
    def get_user_hint(self) -> str:
        """Get a hint from the user for the LLM."""
        self.console.print()
        self.console.print("[yellow]Provide a hint to help the LLM fix this error:[/yellow]")
        self.console.print("[dim]Examples: 'Use Option instead of raw pointer', 'This should be a Vec<u8>'[/dim]")
        return Prompt.ask("Hint")
    
    def edit_code(self, code: str, file_path: str) -> str:
        """Allow user to edit code directly."""
        import tempfile
        import subprocess
        
        # Create temp file with the code
        with tempfile.NamedTemporaryFile(
            mode='w',
            suffix='.rs',
            delete=False
        ) as f:
            f.write(code)
            temp_path = f.name
        
        # Open in editor
        editor = os.environ.get('EDITOR', 'nano')
        self.console.print(f"[yellow]Opening {editor}... Save and close when done.[/yellow]")
        
        try:
            subprocess.run([editor, temp_path], check=True)
            
            with open(temp_path, 'r') as f:
                edited_code = f.read()
            
            return edited_code
        except subprocess.CalledProcessError:
            self.console.print("[red]Editor closed with error. Using original code.[/red]")
            return code
        finally:
            os.unlink(temp_path)
    
    def fix_with_interaction(
        self,
        error: CompilationError,
        current_code: str,
        get_llm_fix: Callable[[CompilationError, str, Optional[str]], FixProposal]
    ) -> FixResult:
        """
        Interactively fix an error with user involvement.
        
        Args:
            error: The compilation error
            current_code: Current code content
            get_llm_fix: Callback to get LLM fix proposal
            
        Returns:
            FixResult with the final decision
        """
        self.display_error(error)
        
        hint = None
        max_retries = 5
        retries = 0
        
        while retries < max_retries:
            # Get LLM proposal
            self.console.print("\n[cyan]Asking LLM for a fix...[/cyan]")
            proposal = get_llm_fix(error, current_code, hint)
            
            # Auto-accept if confidence is high
            if proposal.confidence >= self.auto_accept_threshold:
                self.console.print(f"[green]Auto-accepting fix (confidence: {proposal.confidence:.1%})[/green]")
                result = FixResult(
                    action=FixAction.ACCEPT,
                    final_code=proposal.fixed_code
                )
                self.fix_history.append(result)
                return result
            
            self.display_proposal(proposal)
            action = self.prompt_action()
            
            if action == FixAction.ACCEPT:
                result = FixResult(
                    action=action,
                    final_code=proposal.fixed_code
                )
                self.fix_history.append(result)
                return result
            
            elif action == FixAction.REJECT:
                retries += 1
                self.console.print(f"[yellow]Retrying... ({retries}/{max_retries})[/yellow]")
                continue
            
            elif action == FixAction.EDIT:
                edited = self.edit_code(proposal.fixed_code, error.file_path)
                result = FixResult(
                    action=action,
                    final_code=edited,
                    manual_edit=True
                )
                self.fix_history.append(result)
                return result
            
            elif action == FixAction.RETRY_WITH_HINT:
                hint = self.get_user_hint()
                retries += 1
                continue
            
            elif action == FixAction.MANUAL_FIX:
                edited = self.edit_code(current_code, error.file_path)
                result = FixResult(
                    action=action,
                    final_code=edited,
                    manual_edit=True
                )
                self.fix_history.append(result)
                return result
            
            elif action == FixAction.SKIP:
                result = FixResult(
                    action=action,
                    final_code=current_code
                )
                self.fix_history.append(result)
                return result
        
        # Max retries reached
        self.console.print("[red]Max retries reached. Skipping this error.[/red]")
        return FixResult(
            action=FixAction.SKIP,
            final_code=current_code
        )
    
    def get_statistics(self) -> Dict[str, int]:
        """Get fix statistics."""
        stats = {action.value: 0 for action in FixAction}
        for result in self.fix_history:
            stats[result.action.value] += 1
        stats["total"] = len(self.fix_history)
        stats["manual_edits"] = sum(1 for r in self.fix_history if r.manual_edit)
        return stats
