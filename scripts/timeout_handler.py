#!/usr/bin/env python3
"""
Timeout handler for processes

This script helps to monitor and kill processes that exceed a certain timeout.
It can be used as a standalone script or imported as a module.

Usage:
  1. As a command line tool:
     python timeout_handler.py --pid <process_id> --timeout <seconds>
  
  2. As a module:
     import timeout_handler
     timeout_handler.monitor_process(pid, timeout_seconds)
"""

import argparse
import os
import signal
import sys
import time
import subprocess
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger('timeout_handler')

def is_process_running(pid):
    """Check if a process with the given PID is running."""
    try:
        os.kill(pid, 0)
        return True
    except OSError:
        return False

def kill_process_tree(pid):
    """Kill a process and all its children."""
    logger.info(f"Attempting to kill process tree with root PID: {pid}")
    
    try:
        # First try to get child processes
        try:
            # Try using pgrep to find child processes
            child_pids = subprocess.check_output(['pgrep', '-P', str(pid)]).decode().strip().split('\n')
            child_pids = [int(p) for p in child_pids if p]
        except (subprocess.SubprocessError, FileNotFoundError):
            child_pids = []
        
        # Kill children first
        for child_pid in child_pids:
            try:
                kill_process_tree(child_pid)
            except Exception as e:
                logger.warning(f"Error killing child process {child_pid}: {e}")
        
        # Now kill the parent
        if is_process_running(pid):
            logger.info(f"Sending SIGTERM to process {pid}")
            os.kill(pid, signal.SIGTERM)
            
            # Wait a bit and check if it's still running
            time.sleep(1)
            if is_process_running(pid):
                logger.info(f"Process {pid} still running, sending SIGKILL")
                os.kill(pid, signal.SIGKILL)
            else:
                logger.info(f"Process {pid} terminated successfully")
        else:
            logger.info(f"Process {pid} is no longer running")
            
    except Exception as e:
        logger.error(f"Error killing process tree: {e}")

def monitor_process(pid, timeout_seconds):
    """Monitor a process and kill it if it exceeds the timeout."""
    logger.info(f"Monitoring process {pid} with timeout of {timeout_seconds} seconds")
    
    # Check if process exists before starting
    if not is_process_running(pid):
        logger.error(f"Process {pid} is not running")
        return False
    
    start_time = time.time()
    while is_process_running(pid):
        elapsed = time.time() - start_time
        if elapsed > timeout_seconds:
            logger.warning(f"Process {pid} exceeded timeout of {timeout_seconds} seconds")
            kill_process_tree(pid)
            return False
        
        # Sleep to avoid excessive CPU usage
        time.sleep(1)
    
    logger.info(f"Process {pid} completed within the timeout")
    return True

def main():
    """Main function when script is run directly."""
    parser = argparse.ArgumentParser(description='Monitor a process and kill it if it exceeds a timeout')
    parser.add_argument('--pid', type=int, required=True, help='Process ID to monitor')
    parser.add_argument('--timeout', type=int, required=True, help='Timeout in seconds')
    parser.add_argument('--verbose', action='store_true', help='Enable verbose logging')
    
    args = parser.parse_args()
    
    # Set log level based on verbose flag
    if args.verbose:
        logger.setLevel(logging.DEBUG)
    
    result = monitor_process(args.pid, args.timeout)
    sys.exit(0 if result else 1)

if __name__ == "__main__":
    main() 