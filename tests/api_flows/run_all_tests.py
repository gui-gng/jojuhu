#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Master Runner
Run all test flows in sequence

Usage:
    ./run_all_tests.py                    # Run with default endpoint (http://localhost:8080)
    API_BASE_URL=http://backend:8080 ./run_all_tests.py  # Use custom endpoint
    
For Kubernetes:
    kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080
    ./run_all_tests.py
"""

import subprocess
import sys
import os

# Change to the directory containing this script
script_dir = os.path.dirname(os.path.abspath(__file__))
os.chdir(script_dir)

# Configuration
BASE_URL = os.environ.get("API_BASE_URL", "http://localhost:8080")
USERS_FILE = os.path.join(script_dir, "test_users.json")

def log_info(message):
    print(f"ℹ {message}")

def log_success(message):
    print(f"✅ {message}")

def log_error(message):
    print(f"❌ {message}")

def log_warning(message):
    print(f"⚠️  {message}")

def run_test(script_name, description):
    """Run a test script and report results"""
    print("\n" + "="*70)
    print(f"RUNNING: {description}")
    print("="*70 + "\n")
    
    # Pass environment variables to subprocess
    env = os.environ.copy()
    env["API_BASE_URL"] = BASE_URL
    
    result = subprocess.run(
        [sys.executable, script_name],
        capture_output=False,
        text=True,
        env=env
    )
    
    if result.returncode != 0:
        log_warning(f"{description} completed with warnings or errors")
    else:
        log_success(f"{description} completed successfully")
    
    return result.returncode == 0

def check_backend():
    """Check if backend is running"""
    import requests
    try:
        response = requests.get(f"{BASE_URL}/health", timeout=5)
        if response.status_code == 200:
            log_success(f"Backend is running and healthy at {BASE_URL}")
            return True
        else:
            log_warning(f"Backend returned status {response.status_code}")
            return False
    except requests.exceptions.ConnectionError:
        log_error(f"Cannot connect to backend at {BASE_URL}")
        return False
    except Exception as e:
        log_error(f"Health check failed: {e}")
        return False

def main():
    print("\n" + "="*70)
    print("JOJUHU API TEST FLOWS - MASTER RUNNER")
    print("="*70)
    print(f"\nAPI Endpoint: {BASE_URL}")
    print("\nThis will run all API test flows in sequence:")
    print("1. Onboarding (Create test users)")
    print("2. Posts (Create posts and interactions)")
    print("3. Forums (Create forums and topics)")
    print("4. Messages (Send messages between users)")
    print("\n" + "="*70)
    
    # Check if backend is running
    if not check_backend():
        log_info("Backend doesn't appear to be running")
        log_info("For Kubernetes, run: kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080")
        response = input("\nContinue anyway? (y/N): ")
        if response.lower() != 'y':
            log_info("Exiting...")
            sys.exit(1)
    
    results = []
    
    # Run Onboarding tests (must run first to create users)
    results.append(("Onboarding", run_test("test_onboarding.py", "Onboarding Tests")))
    
    # Check if users were created
    if not os.path.exists(USERS_FILE):
        log_error(f"Users file not found: {USERS_FILE}")
        log_error("Cannot continue with dependent tests")
        sys.exit(1)
    
    # Run Posts tests
    results.append(("Posts", run_test("test_posts.py", "Posts Tests")))
    
    # Run Forums tests
    results.append(("Forums", run_test("test_forums.py", "Forums Tests")))
    
    # Run Messages tests
    results.append(("Messages", run_test("test_messages.py", "Messages Tests")))
    
    # Summary
    print("\n" + "="*70)
    print("TEST SUMMARY")
    print("="*70)
    
    all_passed = True
    for name, success in results:
        status = "✅ PASSED" if success else "❌ FAILED"
        print(f"{name:15} {status}")
        if not success:
            all_passed = False
    
    print("\n" + "="*70)
    print("Generated test data files (in api_flow folder):")
    print(f"  - {os.path.join(script_dir, 'test_users.json')}")
    print(f"  - {os.path.join(script_dir, 'test_posts.json')}")
    print(f"  - {os.path.join(script_dir, 'test_forums.json')}")
    print(f"  - {os.path.join(script_dir, 'test_messages.json')}")
    print("="*70 + "\n")
    
    # Return exit code
    return 0 if all_passed else 1

if __name__ == "__main__":
    sys.exit(main())
