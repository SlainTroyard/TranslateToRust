"""
Translation Cache - Caches translation results for reuse.

Stores:
- Source code hash -> Rust code mapping
- Translation context and dependencies
- Success/failure status
"""

import os
import json
import hashlib
from typing import Dict, Optional, List, Any
from dataclasses import dataclass, field, asdict
from datetime import datetime


@dataclass
class CachedTranslation:
    """A cached translation result."""
    source_hash: str
    source_path: str
    rust_code: str
    rust_path: str
    success: bool
    errors: List[str] = field(default_factory=list)
    created_at: str = field(default_factory=lambda: datetime.now().isoformat())
    model_used: str = ""
    context_hashes: List[str] = field(default_factory=list)  # Dependencies


class TranslationCache:
    """
    Caches translation results to avoid redundant LLM calls.
    
    Key benefits:
    - Skip re-translation of unchanged files
    - Reuse translations when only context changes slightly
    - Enable offline mode with cached results
    """
    
    CACHE_DIR = ".rustify_translation_cache"
    INDEX_FILE = "index.json"
    
    def __init__(self, project_dir: str):
        """
        Initialize the translation cache.
        
        Args:
            project_dir: Target project directory
        """
        self.project_dir = os.path.abspath(project_dir)
        self.cache_dir = os.path.join(self.project_dir, self.CACHE_DIR)
        self.index_path = os.path.join(self.cache_dir, self.INDEX_FILE)
        self._index: Dict[str, CachedTranslation] = {}
        self._load_index()
    
    def _load_index(self) -> None:
        """Load the cache index."""
        if os.path.exists(self.index_path):
            try:
                with open(self.index_path, "r") as f:
                    data = json.load(f)
                    for key, entry in data.get("translations", {}).items():
                        self._index[key] = CachedTranslation(**entry)
            except (json.JSONDecodeError, TypeError) as e:
                print(f"Warning: Could not load cache index: {e}")
    
    def _save_index(self) -> None:
        """Save the cache index."""
        os.makedirs(self.cache_dir, exist_ok=True)
        data = {
            "version": "1.0",
            "updated_at": datetime.now().isoformat(),
            "translations": {
                key: asdict(entry) for key, entry in self._index.items()
            }
        }
        with open(self.index_path, "w") as f:
            json.dump(data, f, indent=2)
    
    @staticmethod
    def compute_source_hash(source_code: str) -> str:
        """Compute hash of source code for cache key."""
        return hashlib.sha256(source_code.encode()).hexdigest()[:16]
    
    def get_cache_key(
        self,
        source_path: str,
        source_code: str,
        context_code: Optional[str] = None
    ) -> str:
        """
        Generate a cache key for a translation.
        
        Args:
            source_path: Path to source file
            source_code: Source code content
            context_code: Optional context (dependencies)
            
        Returns:
            Cache key string
        """
        source_hash = self.compute_source_hash(source_code)
        context_hash = self.compute_source_hash(context_code) if context_code else "none"
        return f"{source_path}:{source_hash}:{context_hash}"
    
    def get(
        self,
        source_path: str,
        source_code: str,
        context_code: Optional[str] = None
    ) -> Optional[CachedTranslation]:
        """
        Get a cached translation if available.
        
        Args:
            source_path: Path to source file
            source_code: Source code content
            context_code: Optional context
            
        Returns:
            CachedTranslation if found, None otherwise
        """
        key = self.get_cache_key(source_path, source_code, context_code)
        return self._index.get(key)
    
    def put(
        self,
        source_path: str,
        source_code: str,
        rust_code: str,
        rust_path: str,
        success: bool,
        errors: Optional[List[str]] = None,
        model_used: str = "",
        context_code: Optional[str] = None
    ) -> None:
        """
        Store a translation result in the cache.
        
        Args:
            source_path: Path to source file
            source_code: Source code content
            rust_code: Translated Rust code
            rust_path: Path to Rust output file
            success: Whether compilation succeeded
            errors: List of error messages if any
            model_used: LLM model identifier
            context_code: Optional context
        """
        key = self.get_cache_key(source_path, source_code, context_code)
        
        self._index[key] = CachedTranslation(
            source_hash=self.compute_source_hash(source_code),
            source_path=source_path,
            rust_code=rust_code,
            rust_path=rust_path,
            success=success,
            errors=errors or [],
            model_used=model_used,
            context_hashes=[self.compute_source_hash(context_code)] if context_code else []
        )
        
        self._save_index()
    
    def invalidate(self, source_path: str) -> int:
        """
        Invalidate all cache entries for a source path.
        
        Args:
            source_path: Path to invalidate
            
        Returns:
            Number of entries removed
        """
        keys_to_remove = [
            key for key in self._index.keys()
            if key.startswith(f"{source_path}:")
        ]
        
        for key in keys_to_remove:
            del self._index[key]
        
        if keys_to_remove:
            self._save_index()
        
        return len(keys_to_remove)
    
    def clear(self) -> None:
        """Clear all cached translations."""
        self._index = {}
        if os.path.exists(self.cache_dir):
            import shutil
            shutil.rmtree(self.cache_dir)
    
    def get_stats(self) -> Dict[str, Any]:
        """Get cache statistics."""
        total = len(self._index)
        successful = sum(1 for t in self._index.values() if t.success)
        failed = total - successful
        
        return {
            "total_entries": total,
            "successful": successful,
            "failed": failed,
            "cache_dir": self.cache_dir,
            "cache_size_mb": self._get_cache_size() / (1024 * 1024)
        }
    
    def _get_cache_size(self) -> int:
        """Get total size of cache directory in bytes."""
        total = 0
        if os.path.exists(self.cache_dir):
            for dirpath, _, filenames in os.walk(self.cache_dir):
                for f in filenames:
                    fp = os.path.join(dirpath, f)
                    total += os.path.getsize(fp)
        return total


