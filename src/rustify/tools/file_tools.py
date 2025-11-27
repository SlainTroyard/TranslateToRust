"""
File Tools - Utility functions for file operations.

"""

import os
from typing import Optional


def read_file(
    filepath: str,
    show_line_numbers: bool = False,
    encoding: str = "utf-8"
) -> str:
    """
    Read a file's content.
    
    Args:
        filepath: Path to the file.
        show_line_numbers: Whether to add line numbers.
        encoding: File encoding.
        
    Returns:
        File content as string.
    """
    try:
        with open(filepath, "r", encoding=encoding, errors="replace") as f:
            content = f.read()
        
        if show_line_numbers:
            content = add_line_numbers(content)
        
        return content
        
    except FileNotFoundError:
        return f"# Error: File not found: {filepath}"
    except Exception as e:
        return f"# Error reading file: {e}"


def write_file(
    filepath: str,
    content: str,
    encoding: str = "utf-8",
    create_dirs: bool = True
) -> bool:
    """
    Write content to a file.
    
    Args:
        filepath: Path to the file.
        content: Content to write.
        encoding: File encoding.
        create_dirs: Whether to create parent directories.
        
    Returns:
        True if successful.
    """
    try:
        if create_dirs:
            os.makedirs(os.path.dirname(filepath), exist_ok=True)
        
        with open(filepath, "w", encoding=encoding) as f:
            f.write(content)
        
        return True
        
    except Exception as e:
        print(f"Error writing file: {e}")
        return False


def add_line_numbers(content: str, start: int = 1) -> str:
    """
    Add line numbers to content.
    
    Args:
        content: The content to number.
        start: Starting line number.
        
    Returns:
        Content with line numbers.
    """
    lines = content.split("\n")
    width = len(str(len(lines) + start))
    
    numbered = []
    for i, line in enumerate(lines, start):
        numbered.append(f"{i:>{width}} | {line}")
    
    return "\n".join(numbered)


def remove_line_numbers(content: str) -> str:
    """
    Remove line numbers from content.
    
    Args:
        content: Content with line numbers.
        
    Returns:
        Content without line numbers.
    """
    import re
    
    lines = content.split("\n")
    result = []
    
    for line in lines:
        # Match pattern: "  123 | code"
        match = re.match(r"^\s*\d+\s*\|\s?(.*)$", line)
        if match:
            result.append(match.group(1))
        else:
            result.append(line)
    
    return "\n".join(result)

