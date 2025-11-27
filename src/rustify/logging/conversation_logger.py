"""
Conversation Logger - Records all LLM interactions.

Saves complete conversation history for debugging and analysis.
"""

import json
import os
from datetime import datetime
from typing import List, Dict, Any, Optional
from threading import Lock


class ConversationLogger:
    """
    Logs all LLM conversations to files.
    
    Features:
    - Saves each conversation turn with timestamp
    - Organized by session and agent
    - JSON format for easy parsing
    - Token usage tracking
    """
    
    def __init__(self, log_dir: str = ".rustify/logs"):
        """
        Initialize the conversation logger.
        
        Args:
            log_dir: Directory to store conversation logs.
        """
        self.log_dir = log_dir
        self.session_id = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.session_dir = os.path.join(log_dir, f"session_{self.session_id}")
        self._lock = Lock()
        self._conversation_count = 0
        
        # Create log directory
        os.makedirs(self.session_dir, exist_ok=True)
        
        # Initialize session summary
        self.session_summary = {
            "session_id": self.session_id,
            "start_time": datetime.now().isoformat(),
            "conversations": [],
            "total_tokens": {
                "prompt_tokens": 0,
                "completion_tokens": 0,
                "total_tokens": 0
            }
        }
        self._save_session_summary()
    
    def log_conversation(
        self,
        agent_name: str,
        agent_role: str,
        messages: List[Dict[str, str]],
        response: Dict[str, Any],
        model: str,
        temperature: float = 1.0,
        max_tokens: Optional[int] = None,
        metadata: Optional[Dict[str, Any]] = None
    ) -> str:
        """
        Log a complete conversation turn.
        
        Args:
            agent_name: Name of the agent.
            agent_role: Role of the agent.
            messages: Input messages to the LLM.
            response: LLM response dict.
            model: Model used.
            temperature: Temperature setting.
            max_tokens: Max tokens setting.
            metadata: Additional metadata.
            
        Returns:
            Path to the log file.
        """
        with self._lock:
            self._conversation_count += 1
            conversation_id = f"{self._conversation_count:04d}"
            timestamp = datetime.now().isoformat()
            
            # Build conversation record
            record = {
                "conversation_id": conversation_id,
                "timestamp": timestamp,
                "agent": {
                    "name": agent_name,
                    "role": agent_role
                },
                "model": model,
                "parameters": {
                    "temperature": temperature,
                    "max_tokens": max_tokens
                },
                "messages": messages,
                "response": {
                    "content": response.get("content", ""),
                    "reasoning_content": response.get("reasoning_content"),
                    "usage": response.get("usage", {})
                },
                "metadata": metadata or {}
            }
            
            # Save individual conversation file
            filename = f"{conversation_id}_{agent_role}_{timestamp.replace(':', '-')}.json"
            filepath = os.path.join(self.session_dir, filename)
            
            with open(filepath, "w", encoding="utf-8") as f:
                json.dump(record, f, ensure_ascii=False, indent=2)
            
            # Update session summary
            usage = response.get("usage", {})
            self.session_summary["conversations"].append({
                "id": conversation_id,
                "agent": agent_name,
                "role": agent_role,
                "model": model,
                "timestamp": timestamp,
                "file": filename,
                "usage": usage
            })
            
            # Update total tokens
            self.session_summary["total_tokens"]["prompt_tokens"] += usage.get("prompt_tokens", 0)
            self.session_summary["total_tokens"]["completion_tokens"] += usage.get("completion_tokens", 0)
            self.session_summary["total_tokens"]["total_tokens"] += usage.get("total_tokens", 0)
            
            self._save_session_summary()
            
            return filepath
    
    def log_error(
        self,
        agent_name: str,
        agent_role: str,
        messages: List[Dict[str, str]],
        error: Exception,
        model: str,
        metadata: Optional[Dict[str, Any]] = None
    ) -> str:
        """
        Log a failed conversation attempt.
        
        Args:
            agent_name: Name of the agent.
            agent_role: Role of the agent.
            messages: Input messages.
            error: Exception that occurred.
            model: Model used.
            metadata: Additional metadata.
            
        Returns:
            Path to the log file.
        """
        with self._lock:
            self._conversation_count += 1
            conversation_id = f"{self._conversation_count:04d}"
            timestamp = datetime.now().isoformat()
            
            record = {
                "conversation_id": conversation_id,
                "timestamp": timestamp,
                "agent": {
                    "name": agent_name,
                    "role": agent_role
                },
                "model": model,
                "messages": messages,
                "error": {
                    "type": type(error).__name__,
                    "message": str(error)
                },
                "metadata": metadata or {}
            }
            
            filename = f"{conversation_id}_{agent_role}_ERROR_{timestamp.replace(':', '-')}.json"
            filepath = os.path.join(self.session_dir, filename)
            
            with open(filepath, "w", encoding="utf-8") as f:
                json.dump(record, f, ensure_ascii=False, indent=2)
            
            return filepath
    
    def _save_session_summary(self) -> None:
        """Save session summary to disk."""
        summary_path = os.path.join(self.session_dir, "session_summary.json")
        with open(summary_path, "w", encoding="utf-8") as f:
            json.dump(self.session_summary, f, ensure_ascii=False, indent=2)
    
    def finalize_session(self) -> Dict[str, Any]:
        """
        Finalize the session and return summary.
        
        Returns:
            Session summary dict.
        """
        self.session_summary["end_time"] = datetime.now().isoformat()
        self.session_summary["total_conversations"] = self._conversation_count
        self._save_session_summary()
        
        return self.session_summary
    
    def get_session_log_path(self) -> str:
        """Get the path to the session log directory."""
        return self.session_dir


# Global conversation logger instance
_global_logger: Optional[ConversationLogger] = None


def get_conversation_logger(log_dir: str = ".rustify/logs") -> ConversationLogger:
    """
    Get or create the global conversation logger.
    
    Args:
        log_dir: Directory for logs.
        
    Returns:
        ConversationLogger instance.
    """
    global _global_logger
    if _global_logger is None:
        _global_logger = ConversationLogger(log_dir)
    return _global_logger


def reset_conversation_logger() -> None:
    """Reset the global conversation logger (start a new session)."""
    global _global_logger
    _global_logger = None

