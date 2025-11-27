"""
MCP Client - Client for connecting to MCP servers.
"""

import asyncio
import json
from typing import Any, Optional
from loguru import logger

from rustify.mcp.protocol import (
    MCPMessage,
    MCPRequest,
    MCPResponse,
    MCPProtocol,
)


class MCPClient:
    """
    MCP Client for connecting to translation services.
    
    Supports multiple transport modes:
    - stdio: Connect to a process via stdio
    - http: Connect via HTTP
    - websocket: Connect via WebSocket
    
    Example:
        ```python
        client = MCPClient()
        await client.connect_http("http://localhost:8000")
        
        result = await client.call_tool(
            "translate_to_rust",
            c_code="int main() { return 0; }"
        )
        ```
    """
    
    def __init__(self):
        self.protocol = MCPProtocol()
        self._transport = None
        self._connected = False
        self._pending_requests: dict[str, asyncio.Future] = {}
    
    @property
    def connected(self) -> bool:
        return self._connected
    
    async def connect_http(self, base_url: str) -> None:
        """Connect to an MCP server via HTTP."""
        import httpx
        
        self._transport = HTTPTransport(base_url)
        self._connected = True
        
        # Initialize connection
        await self.initialize()
        
        logger.info(f"Connected to MCP server at {base_url}")
    
    async def connect_websocket(self, url: str) -> None:
        """Connect to an MCP server via WebSocket."""
        import websockets
        
        self._transport = WebSocketTransport(url)
        await self._transport.connect()
        self._connected = True
        
        # Initialize connection
        await self.initialize()
        
        logger.info(f"Connected to MCP server at {url}")
    
    async def connect_stdio(
        self,
        command: list[str],
        cwd: Optional[str] = None
    ) -> None:
        """Connect to an MCP server via stdio (spawn process)."""
        self._transport = StdioTransport(command, cwd)
        await self._transport.start()
        self._connected = True
        
        # Initialize connection
        await self.initialize()
        
        logger.info(f"Connected to MCP server via stdio: {' '.join(command)}")
    
    async def disconnect(self) -> None:
        """Disconnect from the server."""
        if self._connected:
            await self.shutdown()
            if self._transport:
                await self._transport.close()
            self._connected = False
    
    async def initialize(self) -> dict:
        """Send initialize request."""
        request = self.protocol.create_initialize_request()
        return await self.send_request(request)
    
    async def shutdown(self) -> dict:
        """Send shutdown request."""
        request = MCPRequest.create("shutdown")
        return await self.send_request(request)
    
    async def list_tools(self) -> list[dict]:
        """List available tools on the server."""
        request = MCPRequest.create("tools/list")
        response = await self.send_request(request)
        return response.get("tools", [])
    
    async def call_tool(self, name: str, **arguments) -> Any:
        """Call a tool on the server."""
        request = MCPRequest.create(
            "tools/call",
            {"name": name, "arguments": arguments}
        )
        response = await self.send_request(request)
        
        # Extract text content
        content = response.get("content", [])
        if content and content[0].get("type") == "text":
            text = content[0].get("text", "")
            try:
                return json.loads(text)
            except json.JSONDecodeError:
                return text
        
        return response
    
    async def send_request(self, request: MCPRequest) -> dict:
        """Send a request and wait for response."""
        if not self._connected:
            raise RuntimeError("Not connected to server")
        
        response = await self._transport.send(request)
        
        if response.error:
            raise MCPError(
                response.error.get("code", -1),
                response.error.get("message", "Unknown error")
            )
        
        return response.result


class MCPError(Exception):
    """MCP protocol error."""
    
    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message
        super().__init__(f"MCP Error {code}: {message}")


class HTTPTransport:
    """HTTP transport for MCP client."""
    
    def __init__(self, base_url: str):
        self.base_url = base_url.rstrip("/")
        self._client = None
    
    async def send(self, request: MCPRequest) -> MCPResponse:
        """Send request via HTTP POST."""
        import httpx
        
        if self._client is None:
            self._client = httpx.AsyncClient()
        
        response = await self._client.post(
            f"{self.base_url}/mcp",
            json=request.to_dict(),
            timeout=120.0
        )
        response.raise_for_status()
        
        return MCPResponse.from_dict(response.json())
    
    async def close(self) -> None:
        """Close the HTTP client."""
        if self._client:
            await self._client.aclose()


class WebSocketTransport:
    """WebSocket transport for MCP client."""
    
    def __init__(self, url: str):
        self.url = url
        self._ws = None
    
    async def connect(self) -> None:
        """Connect to WebSocket server."""
        import websockets
        self._ws = await websockets.connect(self.url)
    
    async def send(self, request: MCPRequest) -> MCPResponse:
        """Send request via WebSocket."""
        await self._ws.send(request.to_json())
        response_data = await self._ws.recv()
        return MCPResponse.from_json(response_data)
    
    async def close(self) -> None:
        """Close WebSocket connection."""
        if self._ws:
            await self._ws.close()


class StdioTransport:
    """Stdio transport for MCP client."""
    
    def __init__(self, command: list[str], cwd: Optional[str] = None):
        self.command = command
        self.cwd = cwd
        self._process = None
    
    async def start(self) -> None:
        """Start the server process."""
        self._process = await asyncio.create_subprocess_exec(
            *self.command,
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            cwd=self.cwd
        )
    
    async def send(self, request: MCPRequest) -> MCPResponse:
        """Send request via stdio."""
        # Encode message
        protocol = MCPProtocol()
        encoded = protocol.encode(request)
        
        # Send to process
        self._process.stdin.write(encoded)
        await self._process.stdin.drain()
        
        # Read response
        # First, read Content-Length header
        header = await self._process.stdout.readline()
        header_str = header.decode('utf-8').strip()
        
        if header_str.startswith('Content-Length:'):
            length = int(header_str.split(':')[1].strip())
            
            # Read empty line
            await self._process.stdout.readline()
            
            # Read content
            content = await self._process.stdout.read(length)
            
            return MCPResponse.from_json(content.decode('utf-8'))
        
        raise RuntimeError("Invalid response from server")
    
    async def close(self) -> None:
        """Terminate the server process."""
        if self._process:
            self._process.terminate()
            await self._process.wait()

