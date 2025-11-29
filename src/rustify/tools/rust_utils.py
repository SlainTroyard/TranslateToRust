"""
Rust Utilities - Functions for Rust compilation and testing.

"""

import json
import subprocess
import os
from typing import Optional, List, Dict, Any
import logging

logger = logging.getLogger("rustify.tools.rust_utils")


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
    
    Args:
        project_path: Path to the Rust project.
        section: Section name (e.g., 'dependencies').
        key: Key to update.
        value: New value.
    """
    import toml
    
    cargo_path = os.path.join(project_path, "Cargo.toml")
    
    with open(cargo_path, "r") as f:
        cargo = toml.load(f)
    
    if section not in cargo:
        cargo[section] = {}
    
    cargo[section][key] = value
    
    with open(cargo_path, "w") as f:
        toml.dump(cargo, f)


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
        
        # E0432: unresolved import
        if error_code == "E0432" or "unresolved import" in rendered:
            # Extract crate name from error message
            match = re.search(r'use of undeclared crate or module `(\w+)`', rendered)
            if match:
                crate_name = match.group(1)
                if crate_name not in missing:
                    missing.append(crate_name)
    
    return missing


def auto_add_missing_dependencies(
    project_path: str,
    errors: List[Dict[str, Any]],
    *,
    dev_only: bool = True
) -> List[str]:
    """
    Automatically add missing dependencies to Cargo.toml.
    
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
        if crate_name in COMMON_TEST_CRATES:
            version = COMMON_TEST_CRATES[crate_name]
            try:
                update_cargo_toml(project_path, section, crate_name, version)
                added.append(crate_name)
                logger.info(f"Added {crate_name} = \"{version}\" to {section}")
            except Exception as e:
                logger.warning(f"Failed to add {crate_name}: {e}")
    
    return added

