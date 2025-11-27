"""
Web UI Module - Visual dashboard for translation progress.

Provides:
- Real-time translation progress monitoring
- Interactive dependency graph visualization
- Error analysis dashboard
- Translation history and statistics
"""

from rustify.web.server import WebServer
from rustify.web.dashboard import Dashboard

__all__ = ["WebServer", "Dashboard"]
