"""
Rust Utilities - Functions for Rust compilation and testing.

"""

import json
import subprocess
import os
import re
from typing import Optional, List, Dict, Any
from collections import OrderedDict
import logging

logger = logging.getLogger("rustify.tools.rust_utils")


def cargo_new_project(
    project_path: str,
    project_type: str = "lib",
    cargo_bin: str = "cargo"
) -> Dict[str, Any]:
    """
    Create a new Rust project using cargo new.
    
    This is the recommended way to create a project (like the competition entry).
    
    Args:
        project_path: Path where to create the project.
        project_type: 'lib' or 'bin'.
        cargo_bin: Path to cargo binary.
        
    Returns:
        Dict with success status and output.
    """
    try:
        result = subprocess.run(
            [cargo_bin, "new", f"--{project_type}", project_path],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        if result.returncode == 0:
            # Clear the default content
            if project_type == "bin":
                main_path = os.path.join(project_path, "src", "main.rs")
                with open(main_path, "w") as f:
                    f.write("fn main() {\n\n}\n")
            else:
                lib_path = os.path.join(project_path, "src", "lib.rs")
                with open(lib_path, "w") as f:
                    f.write("//! Auto-generated library\n\n")
        
        return {
            "success": result.returncode == 0,
            "output": result.stderr + result.stdout
        }
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "output": "cargo new timed out"
        }
    except Exception as e:
        return {
            "success": False,
            "output": str(e)
        }


def cargo_check(
    project_path: str,
    *,
    cargo_bin: str = "cargo"
) -> Dict[str, Any]:
    """
    Run cargo check on a project.
    
    Args:
        project_path: Path to the Rust project.
        cargo_bin: Path to cargo binary.
        
    Returns:
        Dict with:
            - success: bool
            - errors: list of error dicts
            - output: raw output string
    """
    try:
        result = subprocess.run(
            [cargo_bin, "check", "--message-format=json"],
            cwd=project_path,
            capture_output=True,
            text=True,
            timeout=300
        )
        
        errors = parse_cargo_errors(result.stderr + result.stdout)
        
        return {
            "success": result.returncode == 0,
            "errors": errors,
            "output": result.stderr + result.stdout
        }
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "errors": [{"rendered": "Compilation timed out"}],
            "output": "Timeout"
        }
    except Exception as e:
        return {
            "success": False,
            "errors": [{"rendered": str(e)}],
            "output": str(e)
        }


def cargo_build(
    project_path: str,
    *,
    release: bool = False,
    cargo_bin: str = "cargo"
) -> Dict[str, Any]:
    """
    Run cargo build on a project.
    
    Args:
        project_path: Path to the Rust project.
        release: Whether to build in release mode.
        cargo_bin: Path to cargo binary.
        
    Returns:
        Dict with success, errors, output.
    """
    cmd = [cargo_bin, "build", "--message-format=json"]
    if release:
        cmd.append("--release")
    
    try:
        result = subprocess.run(
            cmd,
            cwd=project_path,
            capture_output=True,
            text=True,
            timeout=600
        )
        
        errors = parse_cargo_errors(result.stderr + result.stdout)
        
        return {
            "success": result.returncode == 0,
            "errors": errors,
            "output": result.stderr + result.stdout
        }
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "errors": [{"rendered": "Build timed out"}],
            "output": "Timeout"
        }
    except Exception as e:
        return {
            "success": False,
            "errors": [{"rendered": str(e)}],
            "output": str(e)
        }


def cargo_test(
    project_path: str,
    test_name: Optional[str] = None,
    *,
    cargo_bin: str = "cargo"
) -> Dict[str, Any]:
    """
    Run cargo test on a project.
    
    Args:
        project_path: Path to the Rust project.
        test_name: Specific test to run.
        cargo_bin: Path to cargo binary.
        
    Returns:
        Dict with:
            - success: bool (compilation succeeded)
            - errors: list of compilation errors
            - failed_tests: list of failed test info
            - output: raw output string
    """
    cmd = [cargo_bin, "test", "--message-format=json"]
    if test_name:
        cmd.extend(["--test", test_name])
    
    try:
        result = subprocess.run(
            cmd,
            cwd=project_path,
            capture_output=True,
            text=True,
            timeout=600
        )
        
        # Parse errors
        errors = parse_cargo_errors(result.stderr + result.stdout)
        
        # Check if compilation succeeded
        compile_success = "error[E" not in result.stderr
        
        # Parse failed tests from output
        failed_tests = []
        
        if compile_success:
            lines = result.stdout.split("\n")
            current_test = None
            
            for line in lines:
                if "test " in line and " ... FAILED" in line:
                    test_name_match = line.split("test ")[1].split(" ...")[0]
                    current_test = {"name": test_name_match, "error": ""}
                    failed_tests.append(current_test)
                elif current_test and "thread '" in line:
                    current_test["error"] = line
        
        return {
            "success": compile_success,
            "errors": errors,
            "failed_tests": failed_tests,
            "output": result.stdout + result.stderr
        }
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "errors": [{"rendered": "Test timed out"}],
            "failed_tests": [],
            "output": "Timeout"
        }
    except Exception as e:
        return {
            "success": False,
            "errors": [{"rendered": str(e)}],
            "failed_tests": [],
            "output": str(e)
        }


def cargo_bench(
    project_path: str,
    bench_name: Optional[str] = None,
    *,
    cargo_bin: str = "cargo"
) -> Dict[str, Any]:
    """
    Run cargo bench on a project.
    
    Args:
        project_path: Path to the Rust project.
        bench_name: Specific benchmark to run.
        cargo_bin: Path to cargo binary.
        
    Returns:
        Dict with success, errors, output.
    """
    cmd = [cargo_bin, "bench", "--message-format=json"]
    if bench_name:
        cmd.extend(["--bench", bench_name])
    
    try:
        result = subprocess.run(
            cmd,
            cwd=project_path,
            capture_output=True,
            text=True,
            timeout=900
        )
        
        errors = parse_cargo_errors(result.stderr + result.stdout)
        
        return {
            "success": result.returncode == 0,
            "errors": errors,
            "output": result.stdout + result.stderr
        }
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "errors": [{"rendered": "Benchmark timed out"}],
            "output": "Timeout"
        }
    except Exception as e:
        return {
            "success": False,
            "errors": [{"rendered": str(e)}],
            "output": str(e)
        }


def parse_cargo_errors(output: str) -> List[Dict[str, Any]]:
    """
    Parse cargo JSON output for errors.
    
    Args:
        output: Cargo output string.
        
    Returns:
        List of error dicts with 'rendered', 'code', etc.
    """
    errors = []
    
    for line in output.split("\n"):
        line = line.strip()
        if not line:
            continue
        
        try:
            msg = json.loads(line)
            
            if msg.get("reason") == "compiler-message":
                message = msg.get("message", {})
                level = message.get("level", "")
                
                if level in ("error", "warning"):
                    error_info = {
                        "level": level,
                        "rendered": message.get("rendered", ""),
                        "message": message.get("message", ""),
                        "code": message.get("code"),
                        "spans": message.get("spans", [])
                    }
                    errors.append(error_info)
                    
        except json.JSONDecodeError:
            # Not JSON, check for plain error text
            if "error[E" in line or "error:" in line:
                errors.append({"rendered": line})
    
    return errors


def update_cargo_toml(
    project_path: str,
    section: str,
    key: str,
    value: Any
) -> None:
    """
    Update a value in Cargo.toml.
    
    Uses OrderedDict to preserve key ordering (like the competition entry).
    
    Args:
        project_path: Path to the Rust project.
        section: Section name (e.g., 'dependencies').
        key: Key to update.
        value: New value.
    """
    import toml
    from collections import OrderedDict
    
    cargo_path = os.path.join(project_path, "Cargo.toml")
    
    with open(cargo_path, "r") as f:
        content = f.read()
    
    # Use OrderedDict to preserve key ordering
    cargo = toml.loads(content, _dict=OrderedDict)
    
    if section not in cargo:
        cargo[section] = OrderedDict()
    
    cargo[section][key] = value
    
    with open(cargo_path, "w") as f:
        f.write(toml.dumps(cargo))


# Common crates that may be needed for tests
COMMON_TEST_CRATES = {
    "tempfile": "3",
    "rand": "0.8",
    "quickcheck": "1",
    "proptest": "1",
    "criterion": "0.5",
    "test-case": "3",
    "serial_test": "3",
    "pretty_assertions": "1",
}

# Common external crates with their recommended versions
# This is used for auto-detecting dependencies from code
COMMON_CRATES = {
    # Core utilities
    "libc": "0.2",
    "nix": "0.29",
    "errno": "0.3",
    
    # Data structures
    "hashbrown": "0.15",
    "indexmap": "2",
    "smallvec": "1",
    "arrayvec": "0.7",
    "bitvec": "1",
    
    # Serialization
    "serde": "1",
    "serde_json": "1",
    "toml": "0.8",
    "csv": "1",
    "bincode": "1",
    
    # Async
    "tokio": "1",
    "async_std": "1",
    "futures": "0.3",
    
    # CLI
    "clap": "4",
    "structopt": "0.3",
    
    # Logging
    "log": "0.4",
    "env_logger": "0.11",
    "tracing": "0.1",
    
    # Error handling
    "anyhow": "1",
    "thiserror": "2",
    "eyre": "0.6",
    
    # Networking
    "reqwest": "0.12",
    "hyper": "1",
    "axum": "0.7",
    
    # Files & I/O
    "walkdir": "2",
    "glob": "0.3",
    "memmap2": "0.9",
    "tempfile": "3",
    
    # Regex & Parsing
    "regex": "1",
    "nom": "7",
    "pest": "2",
    "lalrpop": "0.22",
    
    # Crypto
    "sha2": "0.10",
    "md5": "0.7",
    "base64": "0.22",
    
    # Time
    "chrono": "0.4",
    "time": "0.3",
    
    # Random
    "rand": "0.8",
    "getrandom": "0.2",
    
    # Compression
    "flate2": "1",
    "bzip2": "0.4",
    "lz4": "1",
    "zstd": "0.13",
    
    # GUI & Graphics
    "x11rb": "0.13",
    "xcb": "1",
    "winit": "0.30",
    "pixels": "0.14",
    "wgpu": "23",
    "image": "0.25",
    
    # Testing
    "criterion": "0.5",
    "proptest": "1",
    "quickcheck": "1",
    
    # FFI
    "libc": "0.2",
    "libloading": "0.8",
    "bindgen": "0.70",
    
    # Collections & Iterators
    "itertools": "0.13",
    "once_cell": "1",
    "lazy_static": "1",
    
    # Unicode & Strings
    "unicode_segmentation": "1",
    "unicode_width": "0.2",
    
    # Math
    "num": "0.4",
    "num_traits": "0.2",
    "num_derive": "0.4",
    
    # Parallel
    "rayon": "1",
    "crossbeam": "0.8",
    "parking_lot": "0.12",
    
    # Memory
    "bytes": "1",
    "memchr": "2",
    
    # Other common crates
    "bitflags": "2",
    "derive_more": "1",
    "strum": "0.26",
    "cfg_if": "1",
    "either": "1",
}

# Cache for crate existence checks
_crate_exists_cache: Dict[str, Optional[str]] = {}


def check_crate_exists(crate_name: str, timeout: int = 10) -> Optional[str]:
    """
    Check if a crate exists on crates.io and get its latest version.
    
    Uses crates.io API to verify crate existence.
    Results are cached to avoid repeated network calls.
    
    Args:
        crate_name: Name of the crate to check.
        timeout: Request timeout in seconds.
        
    Returns:
        Latest version string if crate exists, None otherwise.
    """
    # Check cache first
    if crate_name in _crate_exists_cache:
        return _crate_exists_cache[crate_name]
    
    # Check our static mapping first (faster, no network)
    if crate_name in COMMON_CRATES:
        _crate_exists_cache[crate_name] = COMMON_CRATES[crate_name]
        return COMMON_CRATES[crate_name]
    
    # Query crates.io API
    try:
        import urllib.request
        import urllib.error
        
        url = f"https://crates.io/api/v1/crates/{crate_name}"
        request = urllib.request.Request(
            url,
            headers={"User-Agent": "rustify/1.0"}  # Required by crates.io
        )
        
        with urllib.request.urlopen(request, timeout=timeout) as response:
            if response.status == 200:
                import json
                data = json.loads(response.read().decode())
                version = data.get("crate", {}).get("newest_version")
                if version:
                    _crate_exists_cache[crate_name] = version
                    logger.info(f"Found crate {crate_name} on crates.io (version: {version})")
                    return version
    except urllib.error.HTTPError as e:
        if e.code == 404:
            logger.debug(f"Crate {crate_name} not found on crates.io")
        else:
            logger.warning(f"HTTP error checking crate {crate_name}: {e}")
    except Exception as e:
        logger.warning(f"Error checking crate {crate_name}: {e}")
    
    # Cache negative result
    _crate_exists_cache[crate_name] = None
    return None


def check_crate_exists_cargo(crate_name: str, timeout: int = 30) -> Optional[str]:
    """
    Check if a crate exists using cargo search.
    
    Fallback method if crates.io API is not available.
    
    Args:
        crate_name: Name of the crate to check.
        timeout: Command timeout in seconds.
        
    Returns:
        Version string if crate exists, None otherwise.
    """
    try:
        result = subprocess.run(
            ["cargo", "search", crate_name, "--limit", "1"],
            capture_output=True,
            text=True,
            timeout=timeout
        )
        
        if result.returncode == 0 and result.stdout.strip():
            # Parse output like: crate_name = "1.2.3"    # description
            import re
            match = re.match(rf'^{re.escape(crate_name)}\s*=\s*"([^"]+)"', result.stdout)
            if match:
                return match.group(1)
        
        return None
    except Exception as e:
        logger.warning(f"Error running cargo search for {crate_name}: {e}")
        return None


def detect_missing_crates(errors: List[Dict[str, Any]]) -> List[str]:
    """
    Detect missing crates from compilation errors.
    
    Args:
        errors: List of cargo error dicts.
        
    Returns:
        List of crate names that appear to be missing.
    """
    import re
    missing = []
    
    for error in errors:
        rendered = error.get("rendered", "")
        code = error.get("code", {}) or {}
        error_code = code.get("code", "") if isinstance(code, dict) else ""
        
        # E0432: unresolved import / E0433: failed to resolve
        if error_code in ("E0432", "E0433") or "unresolved import" in rendered or "undeclared crate" in rendered:
            # Extract crate name from error message
            match = re.search(r'use of undeclared crate or module `(\w+)`', rendered)
            if match:
                crate_name = match.group(1)
                if crate_name not in missing:
                    missing.append(crate_name)
    
    return missing


def detect_used_crates(rust_code: str) -> List[str]:
    """
    Detect external crates used in Rust code by analyzing use statements.
    
    Args:
        rust_code: Rust source code to analyze.
        
    Returns:
        List of external crate names found in the code.
    """
    import re
    
    # Rust primitive types - NOT crates!
    PRIMITIVE_TYPES = {
        # Integer types
        "i8", "i16", "i32", "i64", "i128", "isize",
        "u8", "u16", "u32", "u64", "u128", "usize",
        # Floating point
        "f32", "f64",
        # Other primitives
        "bool", "char", "str",
    }
    
    # Standard library modules that should be ignored
    STD_MODULES = {
        "std", "core", "alloc", "proc_macro",
        "self", "super", "crate",
        # Common std submodules
        "collections", "env", "ffi", "fmt", "fs", "io", "iter",
        "mem", "net", "ops", "os", "path", "process", "ptr",
        "slice", "str", "string", "sync", "thread", "time", "vec",
    }
    
    # Combined set of things to ignore
    IGNORE_SET = STD_MODULES | PRIMITIVE_TYPES
    
    crates = set()
    
    # Pattern 1: use crate_name::...
    # e.g., use x11rb::connection::Connection;
    pattern1 = re.compile(r'^\s*use\s+(\w+)::', re.MULTILINE)
    for match in pattern1.finditer(rust_code):
        crate_name = match.group(1)
        if crate_name not in IGNORE_SET:
            crates.add(crate_name)
    
    # Pattern 2: extern crate crate_name;
    pattern2 = re.compile(r'^\s*extern\s+crate\s+(\w+)', re.MULTILINE)
    for match in pattern2.finditer(rust_code):
        crate_name = match.group(1)
        if crate_name not in IGNORE_SET:
            crates.add(crate_name)
    
    # Pattern 3: crate_name::path in code (for inline usage)
    # e.g., let conn = x11rb::connect(None)?;
    pattern3 = re.compile(r'(?<![:\w])(\w+)::(?:\w+::)*\w+', re.MULTILINE)
    for match in pattern3.finditer(rust_code):
        crate_name = match.group(1)
        # Filter out common non-crate prefixes
        if (crate_name not in IGNORE_SET and 
            not crate_name[0].isupper() and  # Likely a type, not a crate
            crate_name not in ("Self",)):
            crates.add(crate_name)
    
    return list(crates)


def detect_used_crates_in_project(project_path: str) -> List[str]:
    """
    Scan all Rust files in a project and detect used external crates.
    
    Args:
        project_path: Path to the Rust project.
        
    Returns:
        List of external crate names used in the project.
    """
    all_crates = set()
    src_path = os.path.join(project_path, "src")
    
    if not os.path.exists(src_path):
        return []
    
    for root, dirs, files in os.walk(src_path):
        for filename in files:
            if filename.endswith(".rs"):
                filepath = os.path.join(root, filename)
                try:
                    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                        content = f.read()
                    crates = detect_used_crates(content)
                    all_crates.update(crates)
                except Exception as e:
                    logger.warning(f"Failed to read {filepath}: {e}")
    
    return list(all_crates)


def add_detected_dependencies(
    project_path: str,
    crates: Optional[List[str]] = None,
    *,
    scan_project: bool = True
) -> List[str]:
    """
    Automatically detect and add missing dependencies to Cargo.toml.
    
    Args:
        project_path: Path to the Rust project.
        crates: List of crate names to add. If None, will scan the project.
        scan_project: If True and crates is None, scan project for used crates.
        
    Returns:
        List of crate names that were added.
    """
    import toml
    
    # Detect crates if not provided
    if crates is None and scan_project:
        crates = detect_used_crates_in_project(project_path)
    
    if not crates:
        return []
    
    # Read existing Cargo.toml
    cargo_path = os.path.join(project_path, "Cargo.toml")
    if not os.path.exists(cargo_path):
        logger.warning(f"Cargo.toml not found at {cargo_path}")
        return []
    
    try:
        with open(cargo_path, "r") as f:
            cargo = toml.loads(f.read(), _dict=OrderedDict)
    except Exception as e:
        logger.error(f"Failed to parse Cargo.toml: {e}")
        return []
    
    # Get existing dependencies
    existing_deps = set(cargo.get("dependencies", {}).keys())
    existing_deps.update(cargo.get("dev-dependencies", {}).keys())
    
    # Add missing dependencies
    added = []
    if "dependencies" not in cargo:
        cargo["dependencies"] = OrderedDict()
    
    for crate_name in crates:
        # Skip if already present
        if crate_name in existing_deps:
            continue
        
        # Check if crate exists (first in static mapping, then via crates.io API)
        version = check_crate_exists(crate_name)
        if version:
            cargo["dependencies"][crate_name] = version
            added.append(crate_name)
            logger.info(f"Auto-added dependency: {crate_name} = \"{version}\"")
        else:
            # Crate doesn't exist on crates.io - skip it
            logger.debug(f"Skipping '{crate_name}' - not found on crates.io")
    
    # Write back
    if added:
        try:
            with open(cargo_path, "w") as f:
                f.write(toml.dumps(cargo))
            logger.info(f"Updated Cargo.toml with {len(added)} new dependencies")
        except Exception as e:
            logger.error(f"Failed to write Cargo.toml: {e}")
            return []
    
    return added


def auto_add_missing_dependencies(
    project_path: str,
    errors: List[Dict[str, Any]],
    *,
    dev_only: bool = True
) -> List[str]:
    """
    Automatically add missing dependencies to Cargo.toml based on compilation errors.
    
    Args:
        project_path: Path to the Rust project.
        errors: List of cargo error dicts.
        dev_only: If True, add to dev-dependencies instead of dependencies.
        
    Returns:
        List of crate names that were added.
    """
    missing = detect_missing_crates(errors)
    added = []
    
    section = "dev-dependencies" if dev_only else "dependencies"
    
    for crate_name in missing:
        # Check if crate exists (static mapping first, then crates.io API)
        version = COMMON_TEST_CRATES.get(crate_name) or check_crate_exists(crate_name)
        if version:
            try:
                update_cargo_toml(project_path, section, crate_name, version)
                added.append(crate_name)
                logger.info(f"Added {crate_name} = \"{version}\" to {section}")
            except Exception as e:
                logger.warning(f"Failed to add {crate_name}: {e}")
        else:
            # Crate doesn't exist on crates.io - skip it
            logger.debug(f"Skipping '{crate_name}' - not found on crates.io")
    
    return added

