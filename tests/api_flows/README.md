# Jojuhu API Test Flows

Automated API testing flows for the Jojuhu social network backend.

## Overview

This directory contains comprehensive test flows that simulate real user interactions with the Jojuhu API:

1. **Onboarding** - User registration, login, and profile management
2. **Posts** - Creating posts, liking, and commenting
3. **Forums** - Creating forums, joining, and creating topics
4. **Messages** - Direct messaging between users

## Prerequisites

```bash
pip install requests
```

## Usage

### Run All Tests

```bash
./run_all_tests.py
```

### Run Individual Flows

```bash
# Test user onboarding
python3 test_onboarding.py

# Test posts (requires onboarding first)
python3 test_posts.py

# Test forums (requires onboarding first)
python3 test_forums.py

# Test messages (requires onboarding first)
python3 test_messages.py
```

## Test Flows

### 1. Onboarding (`test_onboarding.py`)

**Tests:**
- ✅ Successful user registration (creates 3 test users)
- ✅ Registration failure cases:
  - Duplicate username
  - Invalid email format
  - Password too short
  - Missing fields
  - Empty username
- ✅ Successful login
- ✅ Login failure cases:
  - Wrong password
  - Non-existent user
  - Empty credentials
  - Missing fields
- ✅ Get user profile with token

**Output:** Saves user credentials and tokens to `/tmp/jojuhu_test_users.json`

### 2. Posts (`test_posts.py`)

**Prerequisites:** Run onboarding first

**Tests:**
- ✅ Create 3-5 posts per user with various content
- ✅ Like posts from other users
- ✅ Comment on posts from other users
- ✅ Get For You feed
- ✅ Get Following feed
- ✅ Get post details with comments

**Output:** Saves posts and comments to `/tmp/jojuhu_test_posts.json`

### 3. Forums (`test_forums.py`)

**Prerequisites:** Run onboarding first

**Tests:**
- ✅ Create forums (1-2 per user)
  - Technology Enthusiasts
  - Photography Lovers
  - Book Club
  - Fitness & Health
  - Travel Adventures
  - Food & Cooking
  - Music Discovery
  - Gaming Community
- ✅ Join forums created by other users
- ✅ Create topics in forums
- ✅ Reply to topics
- ✅ Get forums list

**Output:** Saves forums and topics to `/tmp/jojuhu_test_forums.json`

### 4. Messages (`test_messages.py`)

**Prerequisites:** Run onboarding first

**Tests:**
- ✅ Send direct messages between users (2-4 messages per conversation)
- ✅ Get conversations list
- ✅ Get messages in a conversation
- ✅ Reply to received messages

**Output:** Saves messages and conversations to `/tmp/jojuhu_test_messages.json`

## Configuration

Edit the configuration at the top of each test file:

```python
BASE_URL = "http://localhost:8080"  # Change if your backend runs elsewhere
API_PREFIX = "/api/v1"
```

## Test Output

All tests use colored output for easy reading:

- ✅ **Green** - Success
- ✗ **Red** - Error
- ℹ **Blue** - Information
- ⚠ **Yellow** - Warning

## Sample Data

Each test flow generates realistic sample data:

**Users:**
- `testuser_12345_1`, `testuser_12345_2`, `testuser_12345_3`
- Passwords: `TestPassword123!`, `SecurePass456!`, `MyPassword789!`

**Posts:**
- Various sample posts about daily life, technology, nature, etc.

**Forums:**
- Technology, Photography, Books, Fitness, Travel, Food, Music, Gaming

**Messages:**
- Friendly introductions and conversations

## Running Tests Step-by-Step

```bash
# 1. Start the backend
docker-compose up -d

# 2. Run onboarding (creates test users)
python3 test_onboarding.py

# 3. Run posts test (creates posts and interactions)
python3 test_posts.py

# 4. Run forums test (creates forums and topics)
python3 test_forums.py

# 5. Run messages test (sends messages between users)
python3 test_messages.py

# 6. Check results
cat /tmp/jojuhu_test_users.json
cat /tmp/jojuhu_test_posts.json
cat /tmp/jojuhu_test_forums.json
cat /tmp/jojuhu_test_messages.json
```

## Expected Results

After running all tests, you should have:

- **3 test users** registered and logged in
- **9-15 posts** created across all users
- **Multiple likes and comments** on posts
- **4-6 forums** created by different users
- **Multiple topics and replies** in forums
- **10-20 messages** exchanged between users

## Troubleshooting

**Backend not running:**
```bash
Error: Connection refused
```
Solution: Start the backend with `docker-compose up -d`

**No test users:**
```bash
Error: No test users found. Run test_onboarding.py first.
```
Solution: Run onboarding test before other tests

**Port conflicts:**
If port 8080 is in use, change the `BASE_URL` in each test file.

## Integration with CI/CD

These tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions
- name: Run API Tests
  run: |
    docker-compose up -d
    sleep 10  # Wait for services to start
    pip install requests
    python3 tests/api_flows/test_onboarding.py
    python3 tests/api_flows/test_posts.py
    python3 tests/api_flows/test_forums.py
    python3 tests/api_flows/test_messages.py
```

## Contributing

To add new test cases:

1. Copy an existing test flow
2. Add your test methods
3. Call them from `run_all_tests()`
4. Update this README

## License

Same as Jojuhu project
