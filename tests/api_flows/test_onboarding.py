#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Onboarding
Test user registration and login with various scenarios
"""

import requests
import json
import sys
from datetime import datetime

# Configuration
BASE_URL = "http://localhost:8080"
API_PREFIX = "/api/v1"

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
    
    def test_register_success(self):
        """Test successful user registration"""
        log_info("Testing: Successful User Registration")
        
        test_users = [
            {
                "username": f"testuser_{datetime.now().strftime('%H%M%S')}_1",
                "email": f"test1_{datetime.now().strftime('%H%M%S')}@example.com",
                "password": "TestPassword123!"
            },
            {
                "username": f"testuser_{datetime.now().strftime('%H%M%S')}_2",
                "email": f"test2_{datetime.now().strftime('%H%M%S')}@example.com",
                "password": "SecurePass456!"
            },
            {
                "username": f"testuser_{datetime.now().strftime('%H%M%S')}_3",
                "email": f"test3_{datetime.now().strftime('%H%M%S')}@example.com",
                "password": "MyPassword789!"
            }
        ]
        
        for user in test_users:
            try:
                response = requests.post(
                    f"{BASE_URL}{API_PREFIX}/auth/register",
                    json=user,
                    headers={"Content-Type": "application/json"}
                )
                
                if response.status_code == 201:
                    data = response.json()
                    if data.get("success"):
                        self.created_users.append(user)
                        log_success(f"Registered user: {user['username']}")
                    else:
                        log_error(f"Registration failed for {user['username']}: {data.get('error')}")
                else:
                    log_error(f"Registration failed for {user['username']}: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception during registration: {e}")
        
        return len(self.created_users) > 0
    
    def test_register_failure_cases(self):
        """Test registration failure cases"""
        log_info("Testing: Registration Failure Cases")
        
        failure_cases = [
            {
                "name": "Duplicate username",
                "data": {
                    "username": self.created_users[0]["username"] if self.created_users else "existinguser",
                    "email": "new@example.com",
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
                "name": "Missing fields",
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
        
        for case in failure_cases:
            try:
                response = requests.post(
                    f"{BASE_URL}{API_PREFIX}/auth/register",
                    json=case["data"],
                    headers={"Content-Type": "application/json"}
                )
                
                if response.status_code == case["expected_status"]:
                    log_success(f"{case['name']}: Correctly rejected (HTTP {response.status_code})")
                else:
                    log_warning(f"{case['name']}: Expected HTTP {case['expected_status']}, got HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception during {case['name']}: {e}")
    
    def test_login_success(self):
        """Test successful login"""
        log_info("Testing: Successful Login")
        
        for user in self.created_users:
            try:
                response = requests.post(
                    f"{BASE_URL}{API_PREFIX}/auth/login",
                    json={
                        "username_or_email": user["username"],
                        "password": user["password"]
                    },
                    headers={"Content-Type": "application/json"}
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success") and data.get("data"):
                        token = data["data"].get("token")
                        self.tokens[user["username"]] = token
                        log_success(f"Login successful: {user['username']}")
                    else:
                        log_error(f"Login failed for {user['username']}: {data.get('error')}")
                else:
                    log_error(f"Login failed for {user['username']}: HTTP {response.status_code}")
                    
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
        
        for case in failure_cases:
            try:
                response = requests.post(
                    f"{BASE_URL}{API_PREFIX}/auth/login",
                    json=case["data"],
                    headers={"Content-Type": "application/json"}
                )
                
                if response.status_code == case["expected_status"]:
                    log_success(f"{case['name']}: Correctly rejected (HTTP {response.status_code})")
                else:
                    log_warning(f"{case['name']}: Expected HTTP {case['expected_status']}, got HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception during {case['name']}: {e}")
    
    def test_get_profile(self):
        """Test getting user profile with token"""
        log_info("Testing: Get User Profile")
        
        for username, token in self.tokens.items():
            try:
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/users/me",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        profile = data.get("data", {})
                        log_success(f"Got profile for {username}: {profile.get('username')}")
                    else:
                        log_error(f"Failed to get profile for {username}: {data.get('error')}")
                else:
                    log_error(f"Failed to get profile for {username}: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting profile: {e}")
    
    def run_all_tests(self):
        """Run all onboarding tests"""
        print("\n" + "="*60)
        print("JOJUHU API TEST FLOW: ONBOARDING")
        print("="*60 + "\n")
        
        # Test registrations
        if self.test_register_success():
            self.test_register_failure_cases()
        
        # Test logins
        if self.test_login_success():
            self.test_login_failure_cases()
            self.test_get_profile()
        
        print("\n" + "="*60)
        print(f"ONBOARDING TEST COMPLETE")
        print(f"Created Users: {len(self.created_users)}")
        print(f"Successful Logins: {len(self.tokens)}")
        print("="*60 + "\n")
        
        return {
            "users": self.created_users,
            "tokens": self.tokens
        }

if __name__ == "__main__":
    flow = OnboardingFlow()
    results = flow.run_all_tests()
    
    # Save results for other test flows
    with open("/tmp/jojuhu_test_users.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("Test results saved to /tmp/jojuhu_test_users.json")
