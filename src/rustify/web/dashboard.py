"""
Dashboard - High-level dashboard controller.

Coordinates between the translation process and the web UI.
"""

import os
from typing import Optional, Dict, Any, List
from dataclasses import dataclass

from rustify.web.server import WebServer, TranslationEvent


@dataclass
class TaskProgress:
    """Progress information for a task."""
    task_id: str
    task_name: str
    status: str  # 'pending', 'translating', 'fixing', 'done', 'error'
    progress: float  # 0.0 to 1.0
    errors: List[str]


class Dashboard:
    """
    Dashboard controller for Rustify.
    
    Provides a high-level interface for:
    - Starting/stopping the web UI
    - Sending progress updates
    - Managing the dependency graph visualization
    """
    
    def __init__(
        self,
        host: str = "127.0.0.1",
        port: int = 8765,
        auto_open: bool = False
    ):
        """
        Initialize the dashboard.
        
        Args:
            host: Host to bind to
            port: Port to listen on
            auto_open: Open browser automatically
        """
        self.host = host
        self.port = port
        self.auto_open = auto_open
        self._server: Optional[WebServer] = None
        self._tasks: Dict[str, TaskProgress] = {}
        self._total_tasks: int = 0
        self._completed_tasks: int = 0
        self._error_count: int = 0
    
    @property
    def url(self) -> str:
        """Get the dashboard URL."""
        return f"http://{self.host}:{self.port}"
    
    def start(self) -> None:
        """Start the dashboard server."""
        self._server = WebServer(self.host, self.port)
        self._server.start(blocking=False)
        
        if self.auto_open:
            import webbrowser
            webbrowser.open(self.url)
    
    def stop(self) -> None:
        """Stop the dashboard server."""
        if self._server:
            self._server.stop()
    
    def set_project_info(
        self,
        source_dir: str,
        target_dir: str,
        total_files: int
    ) -> None:
        """Set project information."""
        self._total_tasks = total_files
        if self._server:
            self._server.update_state(
                source_dir=source_dir,
                target_dir=target_dir,
                files_total=total_files,
                files_done=0,
                progress=0.0,
                status="Starting",
                error_count=0
            )
    
    def register_task(self, task_id: str, task_name: str) -> None:
        """Register a new translation task."""
        self._tasks[task_id] = TaskProgress(
            task_id=task_id,
            task_name=task_name,
            status="pending",
            progress=0.0,
            errors=[]
        )
    
    def update_task(
        self,
        task_id: str,
        status: Optional[str] = None,
        progress: Optional[float] = None,
        message: str = ""
    ) -> None:
        """Update task progress."""
        if task_id not in self._tasks:
            return
        
        task = self._tasks[task_id]
        
        if status:
            old_status = task.status
            task.status = status
            
            # Update completed count
            if status == "done" and old_status != "done":
                self._completed_tasks += 1
            elif status == "error":
                self._error_count += 1
        
        if progress is not None:
            task.progress = progress
        
        # Calculate overall progress
        overall_progress = self._completed_tasks / max(self._total_tasks, 1)
        
        if self._server:
            # Emit event
            self._server.emit_event(TranslationEvent(
                type="progress" if status != "error" else "error",
                task_id=task_id,
                task_name=task.task_name,
                status=task.status,
                progress=task.progress,
                message=message
            ))
            
            # Update state
            self._server.update_state(
                files_done=self._completed_tasks,
                progress=overall_progress,
                status=f"Translating: {task.task_name}" if status == "translating" else task.status.title(),
                current_file=task.task_name,
                error_count=self._error_count
            )
    
    def task_started(self, task_id: str, task_name: str) -> None:
        """Mark a task as started."""
        self.register_task(task_id, task_name)
        self.update_task(task_id, status="translating", message="Translation started")
    
    def task_completed(self, task_id: str) -> None:
        """Mark a task as completed."""
        self.update_task(task_id, status="done", progress=1.0, message="Translation completed")
    
    def task_failed(self, task_id: str, error: str) -> None:
        """Mark a task as failed."""
        if task_id in self._tasks:
            self._tasks[task_id].errors.append(error)
        self.update_task(task_id, status="error", message=f"Error: {error}")
    
    def set_dependency_graph(
        self,
        files: List[str],
        dependencies: List[tuple]
    ) -> None:
        """
        Set the dependency graph for visualization.
        
        Args:
            files: List of file paths
            dependencies: List of (source, target) dependency tuples
        """
        nodes = []
        for i, file_path in enumerate(files):
            task = self._tasks.get(file_path)
            status = task.status if task else "pending"
            nodes.append({
                "id": file_path,
                "name": os.path.basename(file_path),
                "status": status
            })
        
        edges = [
            {"source": src, "target": tgt}
            for src, tgt in dependencies
        ]
        
        if self._server:
            self._server.set_dependency_graph(nodes, edges)
    
    def get_statistics(self) -> Dict[str, Any]:
        """Get current statistics."""
        return {
            "total_tasks": self._total_tasks,
            "completed_tasks": self._completed_tasks,
            "error_count": self._error_count,
            "progress": self._completed_tasks / max(self._total_tasks, 1),
            "tasks_by_status": self._count_by_status()
        }
    
    def _count_by_status(self) -> Dict[str, int]:
        """Count tasks by status."""
        counts = {}
        for task in self._tasks.values():
            counts[task.status] = counts.get(task.status, 0) + 1
        return counts

