"""
Patch Tools - Utility functions for applying code changes.

"""

import re
from typing import Dict, List, Tuple, Optional


def apply_changes(
    content: str,
    changes: List[Dict]
) -> str:
    """
    Apply a list of changes to content.
    
    Args:
        content: Original content.
        changes: List of change dicts with 'start_line', 'end_line', 'new_code'.
        
    Returns:
        Modified content.
    """
    lines = content.split("\n")
    
    # Sort changes by start line in reverse order
    # to avoid line number shifts
    sorted_changes = sorted(
        changes,
        key=lambda c: c.get("start_line", 0),
        reverse=True
    )
    
    for change in sorted_changes:
        start = change.get("start_line", 1) - 1  # Convert to 0-indexed
        end = change.get("end_line", start + 1)
        new_code = change.get("new_code", "")
        
        # Ensure valid range
        start = max(0, min(start, len(lines)))
        end = max(start, min(end, len(lines)))
        
        # Replace lines
        new_lines = new_code.split("\n") if new_code else []
        lines = lines[:start] + new_lines + lines[end:]
    
    return "\n".join(lines)


def extract_code_block_changes(
    content: str
) -> Dict[str, List[Dict]]:
    """
    Extract code changes from LLM response.
    
    Supports multiple formats:
    
    Format 1 (TransFactor style - most precise):
    ```rust:filepath:start_line:end_line
    new code
    ```
    
    Format 2: ```rust:filepath with line markers
    ```rust:filepath
    // [start_line:end_line]
    new code
    ```
    
    Format 3: // filepath: comment followed by code block
    // filepath: path/to/file.rs
    ```rust
    code
    ```
    
    Args:
        content: LLM response content.
        
    Returns:
        Dict mapping filepath to list of changes.
    """
    changes: Dict[str, List[Dict]] = {}
    
    # Pattern 0: TransFactor style - ```lang:filepath:start_line:end_line (most precise)
    # This is the format used by the winning competition entry
    pattern0 = re.compile(
        r'```(\w+):([^:]+):(\d+):(\d+)\s*\n'
        r'([\s\S]*?)'
        r'```',
        re.DOTALL
    )
    
    for match in pattern0.finditer(content):
        language = match.group(1)
        filepath = match.group(2).strip()
        start_line = int(match.group(3))
        end_line = int(match.group(4))
        new_code = match.group(5)
        
        # Remove trailing newline if present
        if new_code.endswith('\n'):
            new_code = new_code[:-1]
        
        if filepath not in changes:
            changes[filepath] = []
        
        changes[filepath].append({
            "start_line": start_line,
            "end_line": end_line,
            "new_code": new_code,
            "language": language
        })
    
    # If we found TransFactor style changes, prefer those
    if changes:
        return changes
    
    # Pattern 1: ```rust:filepath with line markers
    pattern1 = re.compile(
        r'```rust:(\S+)\s*\n'
        r'// \[(\d+):(\d+)\]\s*\n'
        r'(.*?)'
        r'```',
        re.DOTALL
    )
    
    for match in pattern1.finditer(content):
        filepath = match.group(1)
        start_line = int(match.group(2))
        end_line = int(match.group(3))
        new_code = match.group(4).strip()
        
        if filepath not in changes:
            changes[filepath] = []
        
        changes[filepath].append({
            "start_line": start_line,
            "end_line": end_line,
            "new_code": new_code
        })
    
    # Pattern 2: // filepath: comment followed by code block
    pattern2 = re.compile(
        r'// filepath:\s*(\S+)\s*\n'
        r'```rust\s*\n'
        r'(.*?)'
        r'```',
        re.DOTALL
    )
    
    for match in pattern2.finditer(content):
        filepath = match.group(1)
        new_code = match.group(2).strip()
        
        # Full file replacement
        if filepath not in changes:
            changes[filepath] = []
        
        changes[filepath].append({
            "start_line": 1,
            "end_line": 999999,  # Replace whole file
            "new_code": new_code,
            "full_replace": True
        })
    
    return changes


# Change block format prompt for LLM
CHANGE_BLOCK_FORMAT_PROMPT = """
请使用以下格式返回代码变更：

```lang:filepath:start_line:end_line
code
```

其中：
- `lang` 是编程语言 (如 rust, c, cpp)
- `filepath` 是文件的相对路径
- `start_line` 是变更的起始行号（从 1 开始）
- `end_line` 是变更的结束行号（从 1 开始）
- `code` 是新的代码内容

示例：

创建新文件：
```rust:src/utils.rs:1:1
fn helper() -> i32 {
    42
}
```

修改现有代码（替换第5-7行）：
```rust:src/main.rs:5:7
    let result = calculate();
    println!("{}", result);
```

删除代码（第10-12行）：
```rust:src/main.rs:10:12
```
"""


def apply_file_changes(
    changes: Dict[str, List[Dict]],
    base_path: str
) -> List[str]:
    """
    Apply changes to files on disk.
    
    Args:
        changes: Dict mapping filepath to changes.
        base_path: Base path for files.
        
    Returns:
        List of modified file paths.
    """
    import os
    from rustify.tools.file_tools import read_file, write_file
    
    modified = []
    
    for filepath, file_changes in changes.items():
        full_path = os.path.join(base_path, filepath)
        
        # Read current content
        if os.path.exists(full_path):
            content = read_file(full_path)
        else:
            content = ""
        
        # Check for full replacement
        full_replace = any(c.get("full_replace") for c in file_changes)
        
        if full_replace:
            # Use the last full replacement
            for change in reversed(file_changes):
                if change.get("full_replace"):
                    content = change.get("new_code", "")
                    break
        else:
            # Apply incremental changes
            content = apply_changes(content, file_changes)
        
        # Write back
        if write_file(full_path, content):
            modified.append(filepath)
    
    return modified


def create_diff(
    old_content: str,
    new_content: str,
    filepath: str = "file"
) -> str:
    """
    Create a unified diff between old and new content.
    
    Args:
        old_content: Original content.
        new_content: New content.
        filepath: File path for diff header.
        
    Returns:
        Unified diff string.
    """
    import difflib
    
    old_lines = old_content.splitlines(keepends=True)
    new_lines = new_content.splitlines(keepends=True)
    
    diff = difflib.unified_diff(
        old_lines,
        new_lines,
        fromfile=f"a/{filepath}",
        tofile=f"b/{filepath}"
    )
    
    return "".join(diff)

