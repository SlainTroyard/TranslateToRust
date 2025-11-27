"""
Rustify Tools - Utility functions for Rust compilation and testing.
"""

from rustify.tools.rust_utils import (
    cargo_check,
    cargo_build,
    cargo_test,
    cargo_bench,
    parse_cargo_errors,
    update_cargo_toml,
)
from rustify.tools.file_tools import (
    read_file,
    write_file,
    add_line_numbers,
)
from rustify.tools.patch import (
    apply_changes,
    extract_code_block_changes,
)

__all__ = [
    "cargo_check",
    "cargo_build",
    "cargo_test",
    "cargo_bench",
    "parse_cargo_errors",
    "update_cargo_toml",
    "read_file",
    "write_file",
    "add_line_numbers",
    "apply_changes",
    "extract_code_block_changes",
]

