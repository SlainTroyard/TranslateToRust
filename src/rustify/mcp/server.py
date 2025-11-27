"""
MCP Server - Server implementation for hosting tools and resources.
"""

import asyncio
import json
from typing import Any, Callable, Optional
from loguru import logger

from rustify.mcp.protocol import (
    MCPMessage,
    MCPMessageType,
    MCPRequest,
    MCPResponse,
    MCPProtocol,
    MCPErrorCode,
)
from rustify.mcp.tools import Tool, ToolRegistry


class MCPServer:
    """
    MCP Server for hosting translation tools.
    
    Supports multiple transport modes:
    - stdio: Standard input/output (for CLI integration)
    - http: HTTP/SSE (for web integration)
    - websocket: WebSocket (for real-time communication)
    
    Example:
        ```python
        server = MCPServer()
        
        @server.tool("translate")
        async def translate(code: str) -> str:
            return translated_code
        
        await server.run_stdio()
        ```
    """
    
    def __init__(
        self,
        name: str = "rustify-mcp",
        version: str = "1.0.0"
    ):
        self.name = name
        self.version = version
        self.protocol = MCPProtocol()
        self.tool_registry = ToolRegistry()
        self._handlers: dict[str, Callable] = {}
        self._running = False
        
        # Register built-in handlers
        self._register_builtin_handlers()
    
    def _register_builtin_handlers(self) -> None:
        """Register built-in protocol handlers."""
        self._handlers["initialize"] = self._handle_initialize
        self._handlers["shutdown"] = self._handle_shutdown
        self._handlers["tools/list"] = self._handle_list_tools
        self._handlers["tools/call"] = self._handle_call_tool
    
    def tool(
        self,
        name: Optional[str] = None,
        description: Optional[str] = None
    ) -> Callable:
        """Decorator to register a tool."""
        return self.tool_registry.register(name, description)
    
    def add_tool(self, tool: Tool) -> None:
        """Add a tool to the registry."""
        self.tool_registry.add_tool(tool)
    
    def handler(self, method: str) -> Callable:
        """Decorator to register a custom handler."""
        def decorator(func: Callable) -> Callable:
            self._handlers[method] = func
            return func
        return decorator
    
    async def handle_message(self, message: MCPMessage) -> MCPResponse:
        """Process an incoming message and return response."""
        try:
            if message.type == MCPMessageType.REQUEST:
                return await self._handle_request(message)
            elif message.type == MCPMessageType.NOTIFICATION:
                await self._handle_notification(message)
                return None
            else:
                return MCPResponse.error(
                    message.id,
                    MCPErrorCode.INVALID_REQUEST,
                    f"Unsupported message type: {message.type}"
                )
        except Exception as e:
            logger.exception(f"Error handling message: {e}")
            return MCPResponse.error(
                message.id,
                MCPErrorCode.INTERNAL_ERROR,
                str(e)
            )
    
    async def _handle_request(self, request: MCPMessage) -> MCPResponse:
        """Handle a request message."""
        method = request.method
        
        if method not in self._handlers:
            return MCPResponse.error(
                request.id,
                MCPErrorCode.METHOD_NOT_FOUND,
                f"Method not found: {method}"
            )
        
        handler = self._handlers[method]
        
        try:
            result = await handler(request.params)
            return MCPResponse.success(request.id, result)
        except Exception as e:
            logger.error(f"Handler error for {method}: {e}")
            return MCPResponse.error(
                request.id,
                MCPErrorCode.EXECUTION_ERROR,
                str(e)
            )
    
    async def _handle_notification(self, message: MCPMessage) -> None:
        """Handle a notification (no response expected)."""
        method = message.method
        
        if method in self._handlers:
            await self._handlers[method](message.params)
    
    # Built-in handlers
    
    async def _handle_initialize(self, params: dict) -> dict:
        """Handle initialize request."""
        self.protocol.initialized = True
        self.protocol.capabilities = params.get("capabilities", {})
        
        return {
            "protocolVersion": self.protocol.VERSION,
            "serverInfo": {
                "name": self.name,
                "version": self.version,
            },
            "capabilities": {
                "tools": True,
            }
        }
    
    async def _handle_shutdown(self, params: dict) -> dict:
        """Handle shutdown request."""
        self._running = False
        return {}
    
    async def _handle_list_tools(self, params: dict) -> dict:
        """Handle tools/list request."""
        return {
            "tools": self.tool_registry.get_tools_schema()
        }
    
    async def _handle_call_tool(self, params: dict) -> dict:
        """Handle tools/call request."""
        tool_name = params.get("name")
        arguments = params.get("arguments", {})
        
        if not tool_name:
            raise ValueError("Tool name is required")
        
        tool = self.tool_registry.get_tool(tool_name)
        if not tool:
            raise KeyError(f"Tool not found: {tool_name}")
        
        result = await tool.execute(**arguments)
        
        return {
            "content": [
                {
                    "type": "text",
                    "text": json.dumps(result) if not isinstance(result, str) else result
                }
            ]
        }
    
    # Transport implementations
    
    async def run_stdio(self) -> None:
        """Run server using stdio transport."""
        import sys
        
        self._running = True
        logger.info("Starting MCP server on stdio")
        
        reader = asyncio.StreamReader()
        protocol = asyncio.StreamReaderProtocol(reader)
        
        await asyncio.get_event_loop().connect_read_pipe(
            lambda: protocol, sys.stdin
        )
        
        while self._running:
            try:
                # Read Content-Length header
                header = await reader.readline()
                if not header:
                    break
                
                header_str = header.decode('utf-8').strip()
                if header_str.startswith('Content-Length:'):
                    length = int(header_str.split(':')[1].strip())
                    
                    # Read empty line
                    await reader.readline()
                    
                    # Read content
                    content = await reader.read(length)
                    
                    # Parse and handle message
                    message = MCPMessage.from_json(content.decode('utf-8'))
                    response = await self.handle_message(message)
                    
                    if response:
                        # Send response
                        encoded = self.protocol.encode(response)
                        sys.stdout.buffer.write(encoded)
                        sys.stdout.buffer.flush()
                        
            except asyncio.CancelledError:
                break
            except Exception as e:
                logger.error(f"Stdio error: {e}")
    
    async def run_http(
        self,
        host: str = "localhost",
        port: int = 8000
    ) -> None:
        """Run server using HTTP transport with SSE."""
        from fastapi import FastAPI, Request
        from fastapi.responses import StreamingResponse
        from sse_starlette.sse import EventSourceResponse
        import uvicorn
        
        app = FastAPI(title=self.name, version=self.version)
        
        @app.post("/mcp")
        async def handle_mcp(request: Request):
            body = await request.json()
            message = MCPMessage.from_dict(body)
            response = await self.handle_message(message)
            return response.to_dict() if response else {}
        
        @app.get("/mcp/tools")
        async def list_tools():
            return {"tools": self.tool_registry.get_tools_schema()}
        
        @app.get("/health")
        async def health():
            return {"status": "ok", "server": self.name}
        
        self._running = True
        logger.info(f"Starting MCP server on http://{host}:{port}")
        
        config = uvicorn.Config(app, host=host, port=port, log_level="info")
        server = uvicorn.Server(config)
        await server.serve()
    
    async def run_websocket(
        self,
        host: str = "localhost",
        port: int = 8001
    ) -> None:
        """Run server using WebSocket transport."""
        import websockets
        
        async def handler(websocket, path):
            async for message in websocket:
                try:
                    msg = MCPMessage.from_json(message)
                    response = await self.handle_message(msg)
                    if response:
                        await websocket.send(response.to_json())
                except Exception as e:
                    error = MCPResponse.error(
                        "unknown",
                        MCPErrorCode.INTERNAL_ERROR,
                        str(e)
                    )
                    await websocket.send(error.to_json())
        
        self._running = True
        logger.info(f"Starting MCP server on ws://{host}:{port}")
        
        async with websockets.serve(handler, host, port):
            await asyncio.Future()  # run forever
    
    def stop(self) -> None:
        """Stop the server."""
        self._running = False

