#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Onboarding
Test user registration and login with various scenarios

Usage:
    python test_onboarding.py
    
Environment Variables:
    API_BASE_URL - Backend API URL (default: http://localhost:8080)
"""

import requests
import json
import sys
import os
import time
from datetime import datetime

# Configuration - Use environment variable or default
# For Kubernetes: export API_BASE_URL=http://localhost:8080 (with port-forward)
# For direct access: export API_BASE_URL=http://jojuhu-backend:8080
BASE_URL = os.environ.get("API_BASE_URL", "http://localhost:8080")
API_PREFIX = "/api/v1"
USERS_FILE = os.path.join(os.path.dirname(__file__), "test_users.json")

class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    END = '\033[0m'

def log_success(message):
    print(f"{Colors.GREEN}✓{Colors.END} {message}")

def log_error(message):
    print(f"{Colors.RED}✗{Colors.END} {message}")

def log_info(message):
    print(f"{Colors.BLUE}ℹ{Colors.END} {message}")

def log_warning(message):
    print(f"{Colors.YELLOW}⚠{Colors.END} {message}")

class OnboardingFlow:
    def __init__(self):
        self.created_users = []
        self.tokens = {}
        self.session = requests.Session()
        self.session.headers.update({"Content-Type": "application/json"})
        
        log_info(f"Using API endpoint: {BASE_URL}")
    
    def _make_request(self, method, url, **kwargs):
        """Make HTTP request with delay to avoid rate limiting"""
        time.sleep(0.5)  # 500ms delay between requests
        return self.session.request(method, url, **kwargs)
    
    def health_check(self):
        """Check if backend is running"""
        try:
            response = self.session.get(f"{BASE_URL}/health", timeout=5)
            if response.status_code == 200:
                log_success(f"Backend is healthy at {BASE_URL}")
                return True
            else:
                log_error(f"Backend returned status {response.status_code}")
                return False
        except requests.exceptions.ConnectionError:
            log_error(f"Cannot connect to backend at {BASE_URL}")
            log_info("Make sure the backend is running. For Kubernetes:")
            log_info("  kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080")
            return False
        except Exception as e:
            log_error(f"Health check failed: {e}")
            return False
    
    def test_register_success(self):
        """Test successful user registration"""
        log_info("Testing: Successful User Registration")
        
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        test_users = [
            {
                "username": f"alice_{timestamp}",
                "email": f"alice_{timestamp}@test.com",
                "password": "SecurePass123!",
                "display_name": "Alice Tester"
            },
            {
                "username": f"bob_{timestamp}",
                "email": f"bob_{timestamp}@test.com",
                "password": "SecurePass456!",
                "display_name": "Bob Tester"
            },
            {
                "username": f"charlie_{timestamp}",
                "email": f"charlie_{timestamp}@test.com",
                "password": "SecurePass789!",
                "display_name": "Charlie Tester"
            }
        ]
        
        for user in test_users:
            try:
                response = self.session.post(
                    f"{BASE_URL}{API_PREFIX}/auth/register",
                    json=user
                )
                
                if response.status_code == 201:
                    data = response.json()
                    if data.get("success"):
                        # Store user with additional metadata
                        user_data = {
                            **user,
                            "created_at": datetime.now().isoformat(),
                            "user_id": data.get("data", {}).get("user_id")
                        }
                        self.created_users.append(user_data)
                        log_success(f"Registered user: {user['username']} (ID: {user_data.get('user_id', 'N/A')})")
                    else:
                        log_error(f"Registration failed for {user['username']}: {data.get('error')}")
                else:
                    log_error(f"Registration failed for {user['username']}: HTTP {response.status_code} - {response.text}")
                    
            except Exception as e:
                log_error(f"Exception during registration: {e}")
        
        return len(self.created_users) > 0
    
    def test_register_failure_cases(self):
        """Test registration failure cases"""
        log_info("Testing: Registration Failure Cases")
        
        if not self.created_users:
            log_warning("No users created, skipping some failure tests")
            return
        
        failure_cases = [
            {
                "name": "Duplicate username",
                "data": {
                    "username": self.created_users[0]["username"],
                    "email": "new@example.com",
                    "password": "Password123!"
                },
                "expected_status": 409
            },
            {
                "name": "Duplicate email",
                "data": {
                    "username": f"newuser_{datetime.now().strftime('%H%M%S')}",
                    "email": self.created_users[0]["email"],
                    "password": "Password123!"
                },
                "expected_status": 409
            },
            {
                "name": "Invalid email format",
                "data": {
                    "username": f"testuser_{datetime.now().strftime('%H%M%S')}_invalid",
                    "email": "not-an-email",
                    "password": "Password123!"
                },
                "expected_status": 400
            },
            {
                "name": "Password too short",
                "data": {
                    "username": f"testuser_{datetime.now().strftime('%H%M%S')}_short",
                    "email": f"short_{datetime.now().strftime('%H%M%S')}@example.com",
                    "password": "123"
                },
                "expected_status": 400
            },
            {
                "name": "Missing email",
                "data": {
                    "username": f"testuser_{datetime.now().strftime('%H%M%S')}_missing"
                },
                "expected_status": 400
            },
            {
                "name": "Empty username",
                "data": {
                    "username": "",
                    "email": f"empty_{datetime.now().strftime('%H%M%S')}@example.com",
                    "password": "Password123!"
                },
                "expected_status": 400
            }
        ]
        
        passed = 0
        for case in failure_cases:
            try:
                response = self.session.post(
                    f"{BASE_URL}{API_PREFIX}/auth/register",
                    json=case["data"]
                )
                
                if response.status_code == case["expected_status"]:
                    log_success(f"{case['name']}: Correctly rejected (HTTP {response.status_code})")
                    passed += 1
                else:
                    log_warning(f"{case['name']}: Expected HTTP {case['expected_status']}, got HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception during {case['name']}: {e}")
        
        log_info(f"Failure cases: {passed}/{len(failure_cases)} passed")
    
    def test_login_success(self):
        """Test successful login"""
        log_info("Testing: Successful Login")
        
        for user in self.created_users:
            try:
                response = self.session.post(
                    f"{BASE_URL}{API_PREFIX}/auth/login",
                    json={
                        "username_or_email": user["username"],
                        "password": user["password"]
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success") and data.get("data"):
                        token = data["data"].get("token")
                        user_id = data["data"].get("user_id")
                        self.tokens[user["username"]] = token
                        # Update user with ID if not set
                        if not user.get("user_id"):
                            user["user_id"] = user_id
                        log_success(f"Login successful: {user['username']} (ID: {user_id})")
                    else:
                        log_error(f"Login failed for {user['username']}: {data.get('error')}")
                else:
                    log_error(f"Login failed for {user['username']}: HTTP {response.status_code} - {response.text}")
                    
            except Exception as e:
                log_error(f"Exception during login: {e}")
        
        return len(self.tokens) > 0
    
    def test_login_failure_cases(self):
        """Test login failure cases"""
        log_info("Testing: Login Failure Cases")
        
        if not self.created_users:
            log_warning("No users to test login failures with")
            return
        
        failure_cases = [
            {
                "name": "Wrong password",
                "data": {
                    "username_or_email": self.created_users[0]["username"],
                    "password": "WrongPassword123!"
                },
                "expected_status": 401
            },
            {
                "name": "Non-existent user",
                "data": {
                    "username_or_email": "nonexistentuser12345",
                    "password": "Password123!"
                },
                "expected_status": 401
            },
            {
                "name": "Empty credentials",
                "data": {
                    "username_or_email": "",
                    "password": ""
                },
                "expected_status": 400
            },
            {
                "name": "Missing fields",
                "data": {},
                "expected_status": 400
            }
        ]
        
        passed = 0
        for case in failure_cases:
            try:
                response = self.session.post(
                    f"{BASE_URL}{API_PREFIX}/auth/login",
                    json=case["data"]
                )
                
                if response.status_code == case["expected_status"]:
                    log_success(f"{case['name']}: Correctly rejected (HTTP {response.status_code})")
                    passed += 1
                else:
                    log_warning(f"{case['name']}: Expected HTTP {case['expected_status']}, got HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception during {case['name']}: {e}")
        
        log_info(f"Login failure cases: {passed}/{len(failure_cases)} passed")
    
    def test_get_profile(self):
        """Test getting user profile with token"""
        log_info("Testing: Get User Profile")
        
        success_count = 0
        for username, token in self.tokens.items():
            try:
                response = self.session.get(
                    f"{BASE_URL}{API_PREFIX}/users/me",
                    headers={"Authorization": f"Bearer {token}"}
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        profile = data.get("data", {})
                        log_success(f"Got profile for {username}: {profile.get('username')} (ID: {profile.get('id')})")
                        success_count += 1
                    else:
                        log_error(f"Failed to get profile for {username}: {data.get('error')}")
                else:
                    log_error(f"Failed to get profile for {username}: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting profile: {e}")
        
        log_info(f"Profile retrieval: {success_count}/{len(self.tokens)} successful")
    
    def test_token_validation(self):
        """Test token validation on protected endpoints"""
        log_info("Testing: Token Validation")
        
        # Test without token
        try:
            response = self.session.get(f"{BASE_URL}{API_PREFIX}/users/me")
            if response.status_code == 401:
                log_success("Protected endpoint correctly rejects unauthenticated requests")
            else:
                log_warning(f"Expected 401 for unauthenticated request, got {response.status_code}")
        except Exception as e:
            log_error(f"Exception testing token validation: {e}")
        
        # Test with invalid token
        try:
            response = self.session.get(
                f"{BASE_URL}{API_PREFIX}/users/me",
                headers={"Authorization": "Bearer invalid_token"}
            )
            if response.status_code == 401:
                log_success("Protected endpoint correctly rejects invalid tokens")
            else:
                log_warning(f"Expected 401 for invalid token, got {response.status_code}")
        except Exception as e:
            log_error(f"Exception testing invalid token: {e}")
    
    def save_test_users(self):
        """Save created users to JSON file in api_flow folder"""
        log_info(f"Saving test users to: {USERS_FILE}")
        
        # Prepare data for saving
        users_data = {
            "created_at": datetime.now().isoformat(),
            "api_base_url": BASE_URL,
            "user_count": len(self.created_users),
            "users": []
        }
        
        for user in self.created_users:
            user_entry = {
                "user_id": user.get("user_id"),
                "username": user["username"],
                "email": user["email"],
                "password": user["password"],  # Stored for testing purposes
                "display_name": user.get("display_name"),
                "token": self.tokens.get(user["username"]),
                "created_at": user.get("created_at")
            }
            users_data["users"].append(user_entry)
        
        try:
            with open(USERS_FILE, "w") as f:
                json.dump(users_data, f, indent=2)
            log_success(f"Saved {len(self.created_users)} users to {USERS_FILE}")
        except Exception as e:
            log_error(f"Failed to save users file: {e}")
    
    def run_all_tests(self):
        """Run all onboarding tests"""
        print("\n" + "="*70)
        print("JOJUHU API TEST FLOW: ONBOARDING")
        print("="*70)
        print(f"API Endpoint: {BASE_URL}")
        print("="*70 + "\n")
        
        # Health check first
        if not self.health_check():
            log_error("Backend health check failed. Aborting tests.")
            return {"users": [], "tokens": {}, "success": False}
        
        # Test registrations
        if self.test_register_success():
            self.test_register_failure_cases()
        else:
            log_error("No users registered successfully. Aborting further tests.")
            return {"users": [], "tokens": {}, "success": False}
        
        # Test logins
        if self.test_login_success():
            self.test_login_failure_cases()
            self.test_get_profile()
            self.test_token_validation()
        else:
            log_error("No successful logins. Skipping authenticated tests.")
        
        # Save results
        self.save_test_users()
        
        # Summary
        print("\n" + "="*70)
        print("ONBOARDING TEST COMPLETE")
        print("="*70)
        print(f"✓ Created Users: {len(self.created_users)}")
        print(f"✓ Successful Logins: {len(self.tokens)}")
        print(f"✓ Data saved to: {USERS_FILE}")
        print("="*70 + "\n")
        
        return {
            "users": self.created_users,
            "tokens": self.tokens,
            "success": len(self.created_users) > 0 and len(self.tokens) > 0
        }

if __name__ == "__main__":
    flow = OnboardingFlow()
    results = flow.run_all_tests()
    
    # Exit with appropriate code
    sys.exit(0 if results["success"] else 1)
