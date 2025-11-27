"""
Change Detector - Detects file changes for incremental translation.

Uses multiple strategies:
1. File hash comparison (MD5/SHA256)
2. Git diff detection
3. Modification time tracking
"""

import os
import hashlib
import json
import subprocess
from typing import Dict, List, Optional, Set, Tuple
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path


@dataclass
class FileState:
    """Represents the state of a file at a point in time."""
    path: str
    hash: str
    mtime: float
    size: int
    last_translated: Optional[str] = None  # ISO timestamp


@dataclass
class ChangeReport:
    """Report of detected changes."""
    added: List[str] = field(default_factory=list)
    modified: List[str] = field(default_factory=list)
    deleted: List[str] = field(default_factory=list)
    unchanged: List[str] = field(default_factory=list)
    
    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.modified or self.deleted)
    
    @property
    def files_to_translate(self) -> List[str]:
        """Files that need translation (added + modified)."""
        return self.added + self.modified
    
    def summary(self) -> str:
        """Human-readable summary."""
        return (
            f"Changes detected:\n"
            f"  + {len(self.added)} new files\n"
            f"  ~ {len(self.modified)} modified files\n"
            f"  - {len(self.deleted)} deleted files\n"
            f"  = {len(self.unchanged)} unchanged files"
        )


class ChangeDetector:
    """
    Detects changes in source files for incremental translation.
    
    This enables Rustify to only re-translate files that have actually changed,
    significantly speeding up iterative development workflows.
    """
    
    CACHE_FILENAME = ".rustify_cache.json"
    
    def __init__(self, project_dir: str, cache_dir: Optional[str] = None):
        """
        Initialize the change detector.
        
        Args:
            project_dir: Source project directory
            cache_dir: Directory to store cache (defaults to project_dir)
        """
        self.project_dir = os.path.abspath(project_dir)
        self.cache_dir = cache_dir or self.project_dir
        self.cache_path = os.path.join(self.cache_dir, self.CACHE_FILENAME)
        self._file_states: Dict[str, FileState] = {}
        self._load_cache()
    
    def _load_cache(self) -> None:
        """Load cached file states from disk."""
        if os.path.exists(self.cache_path):
            try:
                with open(self.cache_path, "r") as f:
                    data = json.load(f)
                    for path, state_dict in data.get("files", {}).items():
                        self._file_states[path] = FileState(**state_dict)
            except (json.JSONDecodeError, KeyError) as e:
                print(f"Warning: Could not load cache: {e}")
    
    def _save_cache(self) -> None:
        """Save file states to disk."""
        os.makedirs(self.cache_dir, exist_ok=True)
        data = {
            "version": "1.0",
            "updated_at": datetime.now().isoformat(),
            "project_dir": self.project_dir,
            "files": {
                path: {
                    "path": state.path,
                    "hash": state.hash,
                    "mtime": state.mtime,
                    "size": state.size,
                    "last_translated": state.last_translated
                }
                for path, state in self._file_states.items()
            }
        }
        with open(self.cache_path, "w") as f:
            json.dump(data, f, indent=2)
    
    @staticmethod
    def compute_hash(filepath: str) -> str:
        """Compute SHA256 hash of a file."""
        hasher = hashlib.sha256()
        try:
            with open(filepath, "rb") as f:
                for chunk in iter(lambda: f.read(8192), b""):
                    hasher.update(chunk)
            return hasher.hexdigest()
        except IOError:
            return ""
    
    def scan_files(
        self,
        extensions: Set[str] = {".c", ".h", ".cpp", ".hpp"}
    ) -> Dict[str, FileState]:
        """
        Scan project directory for source files.
        
        Args:
            extensions: File extensions to include
            
        Returns:
            Dictionary of path -> FileState
        """
        current_states: Dict[str, FileState] = {}
        
        for root, dirs, files in os.walk(self.project_dir):
            # Skip hidden directories and common build directories
            dirs[:] = [d for d in dirs if not d.startswith('.') 
                       and d not in {'build', 'target', 'node_modules', '__pycache__'}]
            
            for filename in files:
                if not any(filename.endswith(ext) for ext in extensions):
                    continue
                
                filepath = os.path.join(root, filename)
                relpath = os.path.relpath(filepath, self.project_dir)
                
                try:
                    stat = os.stat(filepath)
                    file_hash = self.compute_hash(filepath)
                    
                    current_states[relpath] = FileState(
                        path=relpath,
                        hash=file_hash,
                        mtime=stat.st_mtime,
                        size=stat.st_size
                    )
                except OSError:
                    continue
        
        return current_states
    
    def detect_changes(
        self,
        extensions: Set[str] = {".c", ".h", ".cpp", ".hpp"}
    ) -> ChangeReport:
        """
        Detect changes since last scan.
        
        Args:
            extensions: File extensions to check
            
        Returns:
            ChangeReport with categorized files
        """
        current_states = self.scan_files(extensions)
        report = ChangeReport()
        
        current_paths = set(current_states.keys())
        cached_paths = set(self._file_states.keys())
        
        # New files
        for path in current_paths - cached_paths:
            report.added.append(path)
        
        # Deleted files
        for path in cached_paths - current_paths:
            report.deleted.append(path)
        
        # Check for modifications
        for path in current_paths & cached_paths:
            current = current_states[path]
            cached = self._file_states[path]
            
            # Compare by hash (most reliable)
            if current.hash != cached.hash:
                report.modified.append(path)
            else:
                report.unchanged.append(path)
        
        return report
    
    def try_git_diff(self, since_commit: str = "HEAD~1") -> Optional[ChangeReport]:
        """
        Try to use git diff for change detection.
        
        Args:
            since_commit: Git reference to compare against
            
        Returns:
            ChangeReport if git is available, None otherwise
        """
        try:
            # Check if it's a git repository
            result = subprocess.run(
                ["git", "rev-parse", "--git-dir"],
                cwd=self.project_dir,
                capture_output=True,
                text=True
            )
            if result.returncode != 0:
                return None
            
            # Get diff
            result = subprocess.run(
                ["git", "diff", "--name-status", since_commit],
                cwd=self.project_dir,
                capture_output=True,
                text=True
            )
            if result.returncode != 0:
                return None
            
            report = ChangeReport()
            for line in result.stdout.strip().split('\n'):
                if not line:
                    continue
                parts = line.split('\t')
                if len(parts) < 2:
                    continue
                status, path = parts[0], parts[1]
                
                # Filter to source files
                if not any(path.endswith(ext) for ext in ['.c', '.h', '.cpp', '.hpp']):
                    continue
                
                if status == 'A':
                    report.added.append(path)
                elif status == 'M':
                    report.modified.append(path)
                elif status == 'D':
                    report.deleted.append(path)
            
            return report
            
        except FileNotFoundError:
            return None
    
    def mark_translated(self, paths: List[str]) -> None:
        """
        Mark files as successfully translated.
        
        Args:
            paths: List of file paths that were translated
        """
        current_states = self.scan_files()
        timestamp = datetime.now().isoformat()
        
        for path in paths:
            if path in current_states:
                state = current_states[path]
                state.last_translated = timestamp
                self._file_states[path] = state
        
        self._save_cache()
    
    def update_cache(self) -> None:
        """Update cache with current file states."""
        self._file_states = self.scan_files()
        self._save_cache()
    
    def clear_cache(self) -> None:
        """Clear the cache file."""
        self._file_states = {}
        if os.path.exists(self.cache_path):
            os.remove(self.cache_path)
    
    def get_untranslated_files(self) -> List[str]:
        """Get files that have never been translated."""
        return [
            path for path, state in self._file_states.items()
            if state.last_translated is None
        ]


