"""
Code Chunker - Split C code into translatable chunks.
"""

import re
from dataclasses import dataclass, field
from enum import Enum
from typing import Optional
from loguru import logger


class ChunkKind(str, Enum):
    """Type of code chunk."""
    
    FUNCTION = "function"
    STRUCT = "struct"
    ENUM = "enum"
    UNION = "union"
    TYPEDEF = "typedef"
    MACRO = "macro"
    MACRO_FUNCTION = "macro_function"
    GLOBAL = "global"
    INCLUDE = "include"
    UNKNOWN = "unknown"


@dataclass
class CodeChunk:
    """Represents a chunk of code to translate."""
    
    name: str
    kind: ChunkKind
    text: str
    start_line: int = 0
    end_line: int = 0
    dependencies: list[str] = field(default_factory=list)
    metadata: dict = field(default_factory=dict)
    
    def __hash__(self):
        return hash((self.name, self.kind, self.start_line))


class CodeChunker:
    """
    Split C/C++ code into logical chunks for translation.
    
    Supports:
    - Function definitions
    - Struct/enum/union definitions
    - Typedefs
    - Macro definitions
    - Global variables
    """
    
    # Patterns for different code elements
    PATTERNS = {
        ChunkKind.FUNCTION: re.compile(
            r'^(?:static\s+|inline\s+|extern\s+)*'
            r'(?:const\s+)?'
            r'[\w\s\*]+\s+'  # return type
            r'(\w+)\s*\([^)]*\)\s*'  # function name and params
            r'(?:__attribute__\s*\([^)]*\)\s*)?'
            r'\{',
            re.MULTILINE
        ),
        ChunkKind.STRUCT: re.compile(
            r'^(?:typedef\s+)?struct\s+(\w+)?\s*\{',
            re.MULTILINE
        ),
        ChunkKind.ENUM: re.compile(
            r'^(?:typedef\s+)?enum\s+(\w+)?\s*\{',
            re.MULTILINE
        ),
        ChunkKind.UNION: re.compile(
            r'^(?:typedef\s+)?union\s+(\w+)?\s*\{',
            re.MULTILINE
        ),
        ChunkKind.TYPEDEF: re.compile(
            r'^typedef\s+.+?\s+(\w+)\s*;',
            re.MULTILINE
        ),
        ChunkKind.MACRO: re.compile(
            r'^#define\s+(\w+)(?:\s|$)',
            re.MULTILINE
        ),
        ChunkKind.MACRO_FUNCTION: re.compile(
            r'^#define\s+(\w+)\s*\([^)]*\)',
            re.MULTILINE
        ),
    }
    
    def __init__(self, max_chunk_lines: int = 200, min_chunk_lines: int = 5):
        self.max_chunk_lines = max_chunk_lines
        self.min_chunk_lines = min_chunk_lines
    
    def chunk(self, code: str, filename: str = "") -> list[CodeChunk]:
        """
        Split code into chunks.
        
        Args:
            code: C/C++ source code.
            filename: Optional filename for context.
            
        Returns:
            List of CodeChunk objects.
        """
        chunks = []
        lines = code.split('\n')
        
        # First pass: identify chunk boundaries
        boundaries = self._find_boundaries(code, lines)
        
        # Second pass: extract chunks
        for boundary in boundaries:
            chunk = self._extract_chunk(lines, boundary)
            if chunk:
                chunks.append(chunk)
        
        # If no chunks found, treat whole file as one chunk
        if not chunks and lines:
            chunks.append(CodeChunk(
                name=filename or "main",
                kind=ChunkKind.UNKNOWN,
                text=code,
                start_line=1,
                end_line=len(lines),
            ))
        
        logger.debug(f"Split into {len(chunks)} chunks")
        return chunks
    
    def _find_boundaries(self, code: str, lines: list[str]) -> list[dict]:
        """Find chunk boundaries in code."""
        boundaries = []
        
        # Find functions
        for match in self.PATTERNS[ChunkKind.FUNCTION].finditer(code):
            name = match.group(1)
            start_pos = match.start()
            start_line = code[:start_pos].count('\n') + 1
            end_line = self._find_block_end(lines, start_line - 1)
            
            boundaries.append({
                'name': name,
                'kind': ChunkKind.FUNCTION,
                'start_line': start_line,
                'end_line': end_line,
            })
        
        # Find structs
        for match in self.PATTERNS[ChunkKind.STRUCT].finditer(code):
            name = match.group(1) or "anonymous_struct"
            start_pos = match.start()
            start_line = code[:start_pos].count('\n') + 1
            end_line = self._find_block_end(lines, start_line - 1)
            
            # Check for typedef at end
            if end_line < len(lines):
                end_text = lines[end_line - 1] if end_line <= len(lines) else ""
                if ';' in end_text:
                    typedef_match = re.search(r'\}\s*(\w+)\s*;', end_text)
                    if typedef_match:
                        name = typedef_match.group(1)
            
            boundaries.append({
                'name': name,
                'kind': ChunkKind.STRUCT,
                'start_line': start_line,
                'end_line': end_line,
            })
        
        # Find enums
        for match in self.PATTERNS[ChunkKind.ENUM].finditer(code):
            name = match.group(1) or "anonymous_enum"
            start_pos = match.start()
            start_line = code[:start_pos].count('\n') + 1
            end_line = self._find_block_end(lines, start_line - 1)
            
            boundaries.append({
                'name': name,
                'kind': ChunkKind.ENUM,
                'start_line': start_line,
                'end_line': end_line,
            })
        
        # Find macros (multi-line)
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            if line.startswith('#define'):
                match = re.match(r'#define\s+(\w+)', line)
                if match:
                    name = match.group(1)
                    start_line = i + 1
                    end_line = i + 1
                    
                    # Handle multi-line macros
                    while i < len(lines) and lines[i].rstrip().endswith('\\'):
                        i += 1
                        end_line = i + 1
                    
                    kind = ChunkKind.MACRO_FUNCTION if '(' in line.split('//')[0] else ChunkKind.MACRO
                    
                    boundaries.append({
                        'name': name,
                        'kind': kind,
                        'start_line': start_line,
                        'end_line': end_line,
                    })
            i += 1
        
        # Sort by start line and remove overlaps
        boundaries.sort(key=lambda x: x['start_line'])
        return self._remove_overlaps(boundaries)
    
    def _find_block_end(self, lines: list[str], start_idx: int) -> int:
        """Find the end of a brace-delimited block."""
        brace_count = 0
        started = False
        
        for i in range(start_idx, len(lines)):
            line = lines[i]
            
            # Skip string contents (simplified)
            for char in line:
                if char == '{':
                    brace_count += 1
                    started = True
                elif char == '}':
                    brace_count -= 1
            
            if started and brace_count <= 0:
                return i + 1
        
        return len(lines)
    
    def _remove_overlaps(self, boundaries: list[dict]) -> list[dict]:
        """Remove overlapping boundaries, keeping the larger chunk."""
        if not boundaries:
            return []
        
        result = []
        current = boundaries[0]
        
        for next_bound in boundaries[1:]:
            if next_bound['start_line'] < current['end_line']:
                # Overlap - keep the larger one
                current_size = current['end_line'] - current['start_line']
                next_size = next_bound['end_line'] - next_bound['start_line']
                if next_size > current_size:
                    current = next_bound
            else:
                result.append(current)
                current = next_bound
        
        result.append(current)
        return result
    
    def _extract_chunk(self, lines: list[str], boundary: dict) -> Optional[CodeChunk]:
        """Extract a chunk from lines based on boundary info."""
        start = boundary['start_line'] - 1
        end = boundary['end_line']
        
        if start < 0 or end > len(lines):
            return None
        
        chunk_lines = lines[start:end]
        
        # Skip if too small
        if len(chunk_lines) < self.min_chunk_lines:
            return None
        
        # Find dependencies (simple heuristic)
        text = '\n'.join(chunk_lines)
        deps = self._find_dependencies(text)
        
        return CodeChunk(
            name=boundary['name'],
            kind=boundary['kind'],
            text=text,
            start_line=boundary['start_line'],
            end_line=boundary['end_line'],
            dependencies=deps,
        )
    
    def _find_dependencies(self, code: str) -> list[str]:
        """Find dependencies referenced in code."""
        deps = set()
        
        # Find function calls
        for match in re.finditer(r'\b(\w+)\s*\(', code):
            name = match.group(1)
            if name not in ('if', 'while', 'for', 'switch', 'sizeof', 'return'):
                deps.add(name)
        
        # Find type references
        for match in re.finditer(r'\b(struct|enum|union)\s+(\w+)', code):
            deps.add(match.group(2))
        
        return list(deps)
    
    def merge_small_chunks(
        self,
        chunks: list[CodeChunk],
        target_lines: int = 50
    ) -> list[CodeChunk]:
        """
        Merge small consecutive chunks into larger ones.
        
        Args:
            chunks: List of chunks.
            target_lines: Target minimum lines per merged chunk.
            
        Returns:
            Merged chunks.
        """
        if not chunks:
            return []
        
        merged = []
        current_chunks = [chunks[0]]
        current_lines = chunks[0].text.count('\n') + 1
        
        for chunk in chunks[1:]:
            chunk_lines = chunk.text.count('\n') + 1
            
            if current_lines < target_lines and current_lines + chunk_lines <= self.max_chunk_lines:
                current_chunks.append(chunk)
                current_lines += chunk_lines
            else:
                # Finalize current group
                if len(current_chunks) == 1:
                    merged.append(current_chunks[0])
                else:
                    merged.append(self._merge_chunks(current_chunks))
                
                current_chunks = [chunk]
                current_lines = chunk_lines
        
        # Don't forget the last group
        if current_chunks:
            if len(current_chunks) == 1:
                merged.append(current_chunks[0])
            else:
                merged.append(self._merge_chunks(current_chunks))
        
        return merged
    
    def _merge_chunks(self, chunks: list[CodeChunk]) -> CodeChunk:
        """Merge multiple chunks into one."""
        names = [c.name for c in chunks]
        combined_name = "_".join(names[:3])
        if len(names) > 3:
            combined_name += f"_and_{len(names) - 3}_more"
        
        combined_text = '\n\n'.join(c.text for c in chunks)
        all_deps = set()
        for c in chunks:
            all_deps.update(c.dependencies)
        
        return CodeChunk(
            name=combined_name,
            kind=ChunkKind.UNKNOWN,
            text=combined_text,
            start_line=chunks[0].start_line,
            end_line=chunks[-1].end_line,
            dependencies=list(all_deps),
        )

