"""
Translation Reviewer - Review and approve translations before finalizing.

Enables quality control through human review.
"""

import os
from typing import Optional, List, Dict, Callable
from dataclasses import dataclass, field
from enum import Enum
from rich.console import Console
from rich.panel import Panel
from rich.syntax import Syntax
from rich.prompt import Prompt, Confirm
from rich.columns import Columns
from rich.table import Table


class ReviewDecision(Enum):
    """Review decision for a translation."""
    APPROVE = "approve"
    REJECT = "reject"
    REQUEST_CHANGES = "request_changes"
    SKIP = "skip"


@dataclass
class TranslationUnit:
    """A unit of translation to review."""
    source_path: str
    source_code: str
    rust_path: str
    rust_code: str
    node_type: str
    node_name: str
    compilation_success: bool
    errors: List[str] = field(default_factory=list)


@dataclass
class ReviewResult:
    """Result of a review."""
    decision: ReviewDecision
    comments: Optional[str] = None
    suggested_changes: Optional[str] = None
    rating: Optional[int] = None  # 1-5 quality rating


class TranslationReviewer:
    """
    Human review system for translations.
    
    Features:
    - Side-by-side C and Rust code comparison
    - Quality ratings and comments
    - Batch review mode
    - Export review reports
    """
    
    def __init__(self, console: Optional[Console] = None):
        """Initialize the reviewer."""
        self.console = console or Console()
        self.reviews: List[tuple[TranslationUnit, ReviewResult]] = []
    
    def display_translation(self, unit: TranslationUnit) -> None:
        """Display a translation for review."""
        self.console.print()
        
        # Header
        status = "✅" if unit.compilation_success else "❌"
        self.console.print(Panel(
            f"[bold]{unit.node_name}[/bold] ({unit.node_type}) {status}",
            title="Translation Review",
            border_style="blue"
        ))
        
        # Side by side code
        self.console.print("\n[bold cyan]Source (C/C++):[/bold cyan]")
        self.console.print(Syntax(
            unit.source_code[:2000] + ("..." if len(unit.source_code) > 2000 else ""),
            "c",
            theme="monokai",
            line_numbers=True
        ))
        
        self.console.print("\n[bold green]Translated (Rust):[/bold green]")
        self.console.print(Syntax(
            unit.rust_code[:2000] + ("..." if len(unit.rust_code) > 2000 else ""),
            "rust",
            theme="monokai",
            line_numbers=True
        ))
        
        if unit.errors:
            self.console.print("\n[bold red]Compilation Errors:[/bold red]")
            for error in unit.errors[:5]:
                self.console.print(f"  • {error}")
    
    def prompt_review(self, unit: TranslationUnit) -> ReviewResult:
        """Prompt user for review decision."""
        self.console.print()
        
        table = Table(show_header=False, box=None)
        table.add_column("Key", style="bold cyan")
        table.add_column("Action")
        table.add_row("a", "Approve translation")
        table.add_row("r", "Reject (needs retranslation)")
        table.add_row("c", "Request specific changes")
        table.add_row("s", "Skip (review later)")
        
        self.console.print(table)
        
        choice = Prompt.ask(
            "Decision",
            choices=["a", "r", "c", "s"],
            default="a" if unit.compilation_success else "r"
        )
        
        decision_map = {
            "a": ReviewDecision.APPROVE,
            "r": ReviewDecision.REJECT,
            "c": ReviewDecision.REQUEST_CHANGES,
            "s": ReviewDecision.SKIP
        }
        
        decision = decision_map[choice]
        comments = None
        suggested_changes = None
        rating = None
        
        if decision == ReviewDecision.APPROVE:
            if Confirm.ask("Add a quality rating?", default=False):
                rating = int(Prompt.ask("Rating (1-5)", choices=["1", "2", "3", "4", "5"]))
        
        elif decision == ReviewDecision.REQUEST_CHANGES:
            suggested_changes = Prompt.ask("Describe the changes needed")
        
        elif decision == ReviewDecision.REJECT:
            comments = Prompt.ask("Reason for rejection", default="")
        
        return ReviewResult(
            decision=decision,
            comments=comments,
            suggested_changes=suggested_changes,
            rating=rating
        )
    
    def review(self, unit: TranslationUnit) -> ReviewResult:
        """Review a single translation unit."""
        self.display_translation(unit)
        result = self.prompt_review(unit)
        self.reviews.append((unit, result))
        return result
    
    def batch_review(
        self,
        units: List[TranslationUnit],
        filter_fn: Optional[Callable[[TranslationUnit], bool]] = None
    ) -> List[ReviewResult]:
        """
        Review multiple translations in batch.
        
        Args:
            units: List of translation units
            filter_fn: Optional filter (e.g., only failed compilations)
            
        Returns:
            List of review results
        """
        filtered = units if filter_fn is None else [u for u in units if filter_fn(u)]
        
        self.console.print(f"\n[bold]Batch Review: {len(filtered)} translations[/bold]\n")
        
        results = []
        for i, unit in enumerate(filtered, 1):
            self.console.print(f"\n[dim]─── {i}/{len(filtered)} ───[/dim]")
            
            result = self.review(unit)
            results.append(result)
            
            if i < len(filtered):
                if not Confirm.ask("Continue to next?", default=True):
                    break
        
        return results
    
    def export_report(self, output_path: str) -> None:
        """Export review report to file."""
        import json
        
        report = {
            "summary": self.get_summary(),
            "reviews": [
                {
                    "source_path": unit.source_path,
                    "rust_path": unit.rust_path,
                    "node_name": unit.node_name,
                    "compilation_success": unit.compilation_success,
                    "decision": result.decision.value,
                    "comments": result.comments,
                    "suggested_changes": result.suggested_changes,
                    "rating": result.rating
                }
                for unit, result in self.reviews
            ]
        }
        
        with open(output_path, "w") as f:
            json.dump(report, f, indent=2)
        
        self.console.print(f"[green]Report exported to {output_path}[/green]")
    
    def get_summary(self) -> Dict[str, int]:
        """Get review summary statistics."""
        summary = {d.value: 0 for d in ReviewDecision}
        ratings = []
        
        for _, result in self.reviews:
            summary[result.decision.value] += 1
            if result.rating:
                ratings.append(result.rating)
        
        summary["total"] = len(self.reviews)
        summary["avg_rating"] = sum(ratings) / len(ratings) if ratings else 0
        
        return summary


