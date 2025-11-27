"""
Cache - Caching layer for translation results and embeddings.
"""

import hashlib
import json
from pathlib import Path
from typing import Any, Optional
from datetime import datetime, timedelta
from loguru import logger

from rustify.config import get_config


class Cache:
    """
    Simple file-based cache for translation results.
    
    Provides caching for:
    - LLM responses
    - Translation results
    - Embeddings
    """
    
    def __init__(self, cache_dir: Path = None):
        self.cache_dir = cache_dir or get_config().storage.cache_dir
        self.cache_dir = Path(self.cache_dir)
        self._ensure_dirs()
    
    def _ensure_dirs(self) -> None:
        """Ensure cache directories exist."""
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        (self.cache_dir / "llm").mkdir(exist_ok=True)
        (self.cache_dir / "translations").mkdir(exist_ok=True)
        (self.cache_dir / "embeddings").mkdir(exist_ok=True)
    
    def _get_key(self, *args) -> str:
        """Generate cache key from arguments."""
        content = json.dumps(args, sort_keys=True)
        return hashlib.sha256(content.encode()).hexdigest()
    
    def _get_path(self, category: str, key: str) -> Path:
        """Get cache file path."""
        return self.cache_dir / category / f"{key}.json"
    
    def get(
        self,
        category: str,
        key: str,
        max_age: timedelta = None
    ) -> Optional[Any]:
        """
        Get item from cache.
        
        Args:
            category: Cache category (llm, translations, embeddings)
            key: Cache key
            max_age: Maximum age of cached item
            
        Returns:
            Cached value or None if not found/expired
        """
        path = self._get_path(category, key)
        
        if not path.exists():
            return None
        
        try:
            with open(path, "r") as f:
                data = json.load(f)
            
            # Check expiration
            if max_age:
                cached_at = datetime.fromisoformat(data.get("cached_at", ""))
                if datetime.now() - cached_at > max_age:
                    logger.debug(f"Cache expired: {category}/{key}")
                    return None
            
            return data.get("value")
            
        except (json.JSONDecodeError, KeyError, ValueError) as e:
            logger.warning(f"Cache read error: {e}")
            return None
    
    def set(
        self,
        category: str,
        key: str,
        value: Any
    ) -> None:
        """
        Set item in cache.
        
        Args:
            category: Cache category
            key: Cache key
            value: Value to cache
        """
        path = self._get_path(category, key)
        
        data = {
            "value": value,
            "cached_at": datetime.now().isoformat(),
        }
        
        try:
            with open(path, "w") as f:
                json.dump(data, f)
        except Exception as e:
            logger.warning(f"Cache write error: {e}")
    
    def delete(self, category: str, key: str) -> bool:
        """Delete item from cache."""
        path = self._get_path(category, key)
        
        if path.exists():
            path.unlink()
            return True
        return False
    
    def clear(self, category: str = None) -> int:
        """
        Clear cache.
        
        Args:
            category: Optional category to clear. If None, clears all.
            
        Returns:
            Number of items cleared.
        """
        count = 0
        
        if category:
            category_dir = self.cache_dir / category
            if category_dir.exists():
                for path in category_dir.glob("*.json"):
                    path.unlink()
                    count += 1
        else:
            for category_dir in self.cache_dir.iterdir():
                if category_dir.is_dir():
                    for path in category_dir.glob("*.json"):
                        path.unlink()
                        count += 1
        
        logger.info(f"Cleared {count} cache entries")
        return count
    
    # Convenience methods
    
    def get_llm_response(
        self,
        prompt: str,
        model: str = None,
        max_age: timedelta = timedelta(days=7)
    ) -> Optional[str]:
        """Get cached LLM response."""
        key = self._get_key(prompt, model)
        return self.get("llm", key, max_age)
    
    def set_llm_response(
        self,
        prompt: str,
        response: str,
        model: str = None
    ) -> None:
        """Cache LLM response."""
        key = self._get_key(prompt, model)
        self.set("llm", key, response)
    
    def get_translation(
        self,
        source_code: str,
        context: str = None
    ) -> Optional[str]:
        """Get cached translation."""
        key = self._get_key(source_code, context)
        return self.get("translations", key)
    
    def set_translation(
        self,
        source_code: str,
        rust_code: str,
        context: str = None
    ) -> None:
        """Cache translation result."""
        key = self._get_key(source_code, context)
        self.set("translations", key, rust_code)
    
    def get_embedding(self, text: str) -> Optional[list[float]]:
        """Get cached embedding."""
        key = self._get_key(text)
        return self.get("embeddings", key)
    
    def set_embedding(self, text: str, embedding: list[float]) -> None:
        """Cache embedding."""
        key = self._get_key(text)
        self.set("embeddings", key, embedding)


# Global cache instance
_cache: Optional[Cache] = None


def get_cache() -> Cache:
    """Get or create global cache instance."""
    global _cache
    if _cache is None:
        _cache = Cache()
    return _cache

