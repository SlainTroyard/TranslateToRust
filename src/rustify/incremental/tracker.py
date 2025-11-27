"""
Change Tracker - Track file changes for incremental translation.
"""

import os
import json
import hashlib
from enum import Enum
from typing import Dict, List, Optional, Set
from dataclasses import dataclass, field, asdict
from pathlib import Path
from datetime import datetime


class ChangeType(str, Enum):
    """Type of file change."""
    ADDED = "added"
    MODIFIED = "modified"
    DELETED = "deleted"
    UNCHANGED = "unchanged"


@dataclass
class FileChange:
    """Represents a change to a source file."""
    
    path: str
    change_type: ChangeType
    old_hash: Optional[str] = None
    new_hash: Optional[str] = None
    old_mtime: Optional[float] = None
    new_mtime: Optional[float] = None
    affected_symbols: List[str] = field(default_factory=list)
    
    @property
    def needs_translation(self) -> bool:
        """Check if this change requires translation."""
        return self.change_type in (ChangeType.ADDED, ChangeType.MODIFIED)


@dataclass
class FileSnapshot:
    """Snapshot of a file's state."""
    
    path: str
    content_hash: str
    mtime: float
    size: int
    symbols: List[str] = field(default_factory=list)  # Functions, structs, etc.


class ChangeTracker:
    """
    Track changes in a C/C++ project for incremental translation.
    
    Features:
    - Content-based change detection using SHA256
    - Symbol-level change detection
    - Dependency tracking for cascading changes
    """
    
    SNAPSHOT_FILE = ".rustify_snapshot.json"
    
    def __init__(self, project_dir: str):
        """
        Initialize the change tracker.
        
        Args:
            project_dir: Path to the C/C++ project directory.
        """
        self.project_dir = Path(project_dir).resolve()
        self.snapshot_path = self.project_dir / self.SNAPSHOT_FILE
        self.snapshots: Dict[str, FileSnapshot] = {}
        self._load_snapshots()
    
    def _load_snapshots(self) -> None:
        """Load existing snapshots from disk."""
        if self.snapshot_path.exists():
            try:
                with open(self.snapshot_path, 'r') as f:
                    data = json.load(f)
                    for path, snap_data in data.get("files", {}).items():
                        self.snapshots[path] = FileSnapshot(**snap_data)
            except Exception:
                self.snapshots = {}
    
    def save_snapshots(self) -> None:
        """Save current snapshots to disk."""
        data = {
            "version": "1.0",
            "timestamp": datetime.now().isoformat(),
            "files": {
                path: asdict(snap) 
                for path, snap in self.snapshots.items()
            }
        }
        with open(self.snapshot_path, 'w') as f:
            json.dump(data, f, indent=2)
    
    def _compute_hash(self, filepath: Path) -> str:
        """Compute SHA256 hash of file content."""
        hasher = hashlib.sha256()
        with open(filepath, 'rb') as f:
            while chunk := f.read(8192):
                hasher.update(chunk)
        return hasher.hexdigest()
    
    def _extract_symbols(self, filepath: Path) -> List[str]:
        """
        Extract symbol names from a C/C++ file.
        
        Returns function names, struct names, etc.
        """
        import re
        
        symbols = []
        try:
            with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                content = f.read()
            
            # Find function definitions
            func_pattern = re.compile(
                r'^\s*(?:static\s+|inline\s+|extern\s+)*'
                r'(?:const\s+)?[\w\s\*]+\s+(\w+)\s*\([^)]*\)\s*\{',
                re.MULTILINE
            )
            for match in func_pattern.finditer(content):
                symbols.append(f"func:{match.group(1)}")
            
            # Find struct definitions
            struct_pattern = re.compile(
                r'(?:typedef\s+)?struct\s+(\w+)',
                re.MULTILINE
            )
            for match in struct_pattern.finditer(content):
                symbols.append(f"struct:{match.group(1)}")
            
            # Find enum definitions
            enum_pattern = re.compile(
                r'(?:typedef\s+)?enum\s+(\w+)',
                re.MULTILINE
            )
            for match in enum_pattern.finditer(content):
                symbols.append(f"enum:{match.group(1)}")
                
        except Exception:
            pass
        
        return symbols
    
    def _get_source_files(self) -> List[Path]:
        """Get all C/C++ source files in the project."""
        extensions = {'.c', '.h', '.cpp', '.hpp', '.cc', '.cxx'}
        files = []
        
        for root, _, filenames in os.walk(self.project_dir):
            for filename in filenames:
                if Path(filename).suffix.lower() in extensions:
                    filepath = Path(root) / filename
                    files.append(filepath)
        
        return files
    
    def detect_changes(self) -> List[FileChange]:
        """
        Detect all changes since last snapshot.
        
        Returns:
            List of FileChange objects describing what changed.
        """
        changes = []
        current_files: Set[str] = set()
        
        for filepath in self._get_source_files():
            rel_path = str(filepath.relative_to(self.project_dir))
            current_files.add(rel_path)
            
            stat = filepath.stat()
            new_hash = self._compute_hash(filepath)
            new_symbols = self._extract_symbols(filepath)
            
            if rel_path not in self.snapshots:
                # New file
                changes.append(FileChange(
                    path=rel_path,
                    change_type=ChangeType.ADDED,
                    new_hash=new_hash,
                    new_mtime=stat.st_mtime,
                    affected_symbols=new_symbols,
                ))
            else:
                old_snap = self.snapshots[rel_path]
                
                if old_snap.content_hash != new_hash:
                    # Modified file - find changed symbols
                    old_symbols_set = set(old_snap.symbols)
                    new_symbols_set = set(new_symbols)
                    changed_symbols = list(
                        old_symbols_set.symmetric_difference(new_symbols_set)
                    )
                    
                    changes.append(FileChange(
                        path=rel_path,
                        change_type=ChangeType.MODIFIED,
                        old_hash=old_snap.content_hash,
                        new_hash=new_hash,
                        old_mtime=old_snap.mtime,
                        new_mtime=stat.st_mtime,
                        affected_symbols=changed_symbols or new_symbols,
                    ))
                else:
                    changes.append(FileChange(
                        path=rel_path,
                        change_type=ChangeType.UNCHANGED,
                        old_hash=old_snap.content_hash,
                        new_hash=new_hash,
                    ))
        
        # Find deleted files
        for old_path in self.snapshots.keys():
            if old_path not in current_files:
                old_snap = self.snapshots[old_path]
                changes.append(FileChange(
                    path=old_path,
                    change_type=ChangeType.DELETED,
                    old_hash=old_snap.content_hash,
                    affected_symbols=old_snap.symbols,
                ))
        
        return changes
    
    def update_snapshot(self, filepath: str) -> None:
        """
        Update snapshot for a single file after translation.
        
        Args:
            filepath: Relative path to the file.
        """
        full_path = self.project_dir / filepath
        
        if not full_path.exists():
            # File was deleted
            if filepath in self.snapshots:
                del self.snapshots[filepath]
        else:
            stat = full_path.stat()
            self.snapshots[filepath] = FileSnapshot(
                path=filepath,
                content_hash=self._compute_hash(full_path),
                mtime=stat.st_mtime,
                size=stat.st_size,
                symbols=self._extract_symbols(full_path),
            )
    
    def update_all_snapshots(self) -> None:
        """Update snapshots for all current files."""
        self.snapshots.clear()
        
        for filepath in self._get_source_files():
            rel_path = str(filepath.relative_to(self.project_dir))
            stat = filepath.stat()
            
            self.snapshots[rel_path] = FileSnapshot(
                path=rel_path,
                content_hash=self._compute_hash(filepath),
                mtime=stat.st_mtime,
                size=stat.st_size,
                symbols=self._extract_symbols(filepath),
            )
        
        self.save_snapshots()
    
    def get_changed_files(self) -> List[str]:
        """Get list of files that need translation."""
        changes = self.detect_changes()
        return [c.path for c in changes if c.needs_translation]
    
    def get_change_summary(self) -> Dict:
        """Get a summary of all changes."""
        changes = self.detect_changes()
        
        return {
            "total_files": len(changes),
            "added": len([c for c in changes if c.change_type == ChangeType.ADDED]),
            "modified": len([c for c in changes if c.change_type == ChangeType.MODIFIED]),
            "deleted": len([c for c in changes if c.change_type == ChangeType.DELETED]),
            "unchanged": len([c for c in changes if c.change_type == ChangeType.UNCHANGED]),
            "needs_translation": len([c for c in changes if c.needs_translation]),
            "files": [
                {
                    "path": c.path,
                    "status": c.change_type.value,
                    "symbols": c.affected_symbols[:5],  # Limit for display
                }
                for c in changes
                if c.change_type != ChangeType.UNCHANGED
            ]
        }

