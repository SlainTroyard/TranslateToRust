"""
Example: Using MCP client to connect to translation server.
"""

import asyncio
from rustify.mcp import MCPClient


async def main():
    # Create MCP client
    client = MCPClient()
    
    try:
        # Connect to server (make sure server is running)
        print("Connecting to MCP server...")
        await client.connect_http("http://localhost:8000")
        
        # List available tools
        print("\nAvailable tools:")
        tools = await client.list_tools()
        for tool in tools:
            print(f"  - {tool['name']}: {tool['description']}")
        
        # Call translation tool
        print("\nCalling translate_to_rust tool...")
        result = await client.call_tool(
            "translate_to_rust",
            c_code="int add(int a, int b) { return a + b; }",
            style="idiomatic"
        )
        
        print("\nResult:")
        print(result)
        
    finally:
        await client.disconnect()


if __name__ == "__main__":
    asyncio.run(main())

