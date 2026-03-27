#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Master Runner
Run all test flows in sequence
"""

import subprocess
import sys
import os

# Change to the directory containing this script
os.chdir(os.path.dirname(os.path.abspath(__file__)))

def run_test(script_name, description):
    """Run a test script and report results"""
    print("\n" + "="*70)
    print(f"RUNNING: {description}")
    print("="*70 + "\n")
    
    result = subprocess.run(
        [sys.executable, script_name],
        capture_output=False,
        text=True
    )
    
    if result.returncode != 0:
        print(f"\n⚠️  {description} completed with warnings or errors")
    else:
        print(f"\n✅ {description} completed successfully")
    
    return result.returncode == 0

def main():
    print("\n" + "="*70)
    print("JOJUHU API TEST FLOWS - MASTER RUNNER")
    print("="*70)
    print("\nThis will run all API test flows in sequence:")
    print("1. Onboarding (Create test users)")
    print("2. Posts (Create posts and interactions)")
    print("3. Forums (Create forums and topics)")
    print("4. Messages (Send messages between users)")
    print("\nMake sure the backend is running on http://localhost:8080")
    print("="*70)
    
    # Check if backend is running
    import requests
    try:
        response = requests.get("http://localhost:8080/health", timeout=5)
        if response.status_code == 200:
            print("\n✅ Backend is running and healthy\n")
        else:
            print("\n⚠️  Backend returned non-200 status. Continuing anyway...\n")
    except requests.exceptions.ConnectionError:
        print("\n⚠️  WARNING: Backend doesn't appear to be running on localhost:8080")
        print("Please start the backend with: docker-compose up -d\n")
        response = input("Continue anyway? (y/N): ")
        if response.lower() != 'y':
            print("Exiting...")
            sys.exit(1)
    
    results = []
    
    # Run Onboarding tests
    results.append(("Onboarding", run_test("test_onboarding.py", "Onboarding Tests")))
    
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
    
    for name, success in results:
        status = "✅ PASSED" if success else "⚠️  COMPLETED WITH WARNINGS"
        print(f"{name:15} {status}")
    
    print("\n" + "="*70)
    print("Generated test data files:")
    print("  - /tmp/jojuhu_test_users.json")
    print("  - /tmp/jojuhu_test_posts.json")
    print("  - /tmp/jojuhu_test_forums.json")
    print("  - /tmp/jojuhu_test_messages.json")
    print("="*70 + "\n")
    
    # Return exit code
    return 0 if all(success for _, success in results) else 1

if __name__ == "__main__":
    sys.exit(main())
