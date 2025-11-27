"""
MCP Protocol - Message definitions and protocol handling.

Implements a simplified version of the Model Context Protocol.
"""

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Optional
import json
import uuid


class MCPMessageType(str, Enum):
    """Types of MCP messages."""
    
    # Lifecycle
    INITIALIZE = "initialize"
    INITIALIZED = "initialized"
    SHUTDOWN = "shutdown"
    
    # Requests
    REQUEST = "request"
    RESPONSE = "response"
    NOTIFICATION = "notification"
    
    # Errors
    ERROR = "error"
    
    # Streaming
    STREAM_START = "stream_start"
    STREAM_CHUNK = "stream_chunk"
    STREAM_END = "stream_end"


@dataclass
class MCPMessage:
    """
    Base MCP message structure.
    
    All MCP communication is done through messages that follow this format.
    """
    
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    type: MCPMessageType = MCPMessageType.REQUEST
    method: Optional[str] = None
    params: dict[str, Any] = field(default_factory=dict)
    result: Optional[Any] = None
    error: Optional[dict] = None
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())
    
    def to_dict(self) -> dict:
        """Convert to dictionary."""
        return {
            "id": self.id,
            "type": self.type.value,
            "method": self.method,
            "params": self.params,
            "result": self.result,
            "error": self.error,
            "timestamp": self.timestamp,
        }
    
    def to_json(self) -> str:
        """Serialize to JSON."""
        return json.dumps(self.to_dict())
    
    @classmethod
    def from_dict(cls, data: dict) -> "MCPMessage":
        """Create from dictionary."""
        return cls(
            id=data.get("id", str(uuid.uuid4())),
            type=MCPMessageType(data.get("type", "request")),
            method=data.get("method"),
            params=data.get("params", {}),
            result=data.get("result"),
            error=data.get("error"),
            timestamp=data.get("timestamp", datetime.now().isoformat()),
        )
    
    @classmethod
    def from_json(cls, json_str: str) -> "MCPMessage":
        """Deserialize from JSON."""
        return cls.from_dict(json.loads(json_str))


@dataclass
class MCPRequest(MCPMessage):
    """MCP Request message."""
    
    type: MCPMessageType = MCPMessageType.REQUEST
    
    @classmethod
    def create(
        cls,
        method: str,
        params: dict[str, Any] = None
    ) -> "MCPRequest":
        """Create a new request."""
        return cls(
            method=method,
            params=params or {}
        )


@dataclass
class MCPResponse(MCPMessage):
    """MCP Response message."""
    
    type: MCPMessageType = MCPMessageType.RESPONSE
    
    @classmethod
    def success(
        cls,
        request_id: str,
        result: Any
    ) -> "MCPResponse":
        """Create a success response."""
        return cls(
            id=request_id,
            result=result
        )
    
    @classmethod
    def error(
        cls,
        request_id: str,
        code: int,
        message: str,
        data: Any = None
    ) -> "MCPResponse":
        """Create an error response."""
        return cls(
            id=request_id,
            type=MCPMessageType.ERROR,
            error={
                "code": code,
                "message": message,
                "data": data
            }
        )


class MCPProtocol:
    """
    Protocol handler for MCP communication.
    
    Handles encoding/decoding of messages and protocol state.
    """
    
    VERSION = "1.0"
    
    def __init__(self):
        self.initialized = False
        self.capabilities: dict[str, Any] = {}
    
    def encode(self, message: MCPMessage) -> bytes:
        """Encode a message for transmission."""
        json_str = message.to_json()
        # Use length-prefixed format
        length = len(json_str.encode('utf-8'))
        return f"Content-Length: {length}\r\n\r\n{json_str}".encode('utf-8')
    
    def decode(self, data: bytes) -> MCPMessage:
        """Decode a received message."""
        text = data.decode('utf-8')
        
        # Parse length-prefixed format
        if text.startswith("Content-Length:"):
            header_end = text.find("\r\n\r\n")
            if header_end > 0:
                json_str = text[header_end + 4:]
                return MCPMessage.from_json(json_str)
        
        # Fallback to direct JSON
        return MCPMessage.from_json(text)
    
    def create_initialize_request(
        self,
        capabilities: dict[str, Any] = None
    ) -> MCPRequest:
        """Create an initialize request."""
        return MCPRequest.create(
            method="initialize",
            params={
                "protocolVersion": self.VERSION,
                "capabilities": capabilities or {},
            }
        )
    
    def create_initialize_response(
        self,
        request_id: str,
        capabilities: dict[str, Any] = None
    ) -> MCPResponse:
        """Create an initialize response."""
        return MCPResponse.success(
            request_id=request_id,
            result={
                "protocolVersion": self.VERSION,
                "capabilities": capabilities or {},
            }
        )


# Standard MCP error codes
class MCPErrorCode:
    """Standard MCP error codes."""
    
    PARSE_ERROR = -32700
    INVALID_REQUEST = -32600
    METHOD_NOT_FOUND = -32601
    INVALID_PARAMS = -32602
    INTERNAL_ERROR = -32603
    
    # Custom error codes
    NOT_INITIALIZED = -32002
    ALREADY_INITIALIZED = -32003
    TOOL_NOT_FOUND = -32004
    EXECUTION_ERROR = -32005

