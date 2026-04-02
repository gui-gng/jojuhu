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

## Configuration

The tests use environment variables for configuration:

```bash
# Default: http://localhost:8080
export API_BASE_URL="http://localhost:8080"

# For Kubernetes with port-forward
kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080
export API_BASE_URL="http://localhost:8080"
```

## Usage

### Run All Tests

```bash
./run_all_tests.py
```

Or with custom endpoint:

```bash
API_BASE_URL=http://backend:8080 ./run_all_tests.py
```

### Run Individual Flows

```bash
# Test user onboarding (must run first!)
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
- ✅ Health check before starting
- ✅ Successful user registration (creates 3 test users)
- ✅ Registration failure cases:
  - Duplicate username
  - Duplicate email
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
- ✅ Token validation on protected endpoints

**Output:** Saves user credentials and tokens to `./test_users.json`

### 2. Posts (`test_posts.py`)

**Prerequisites:** Run onboarding first

**Tests:**
- ✅ Create 3-5 posts per user with various content
- ✅ Like posts from other users
- ✅ Comment on posts from other users
- ✅ Get For You feed
- ✅ Get Following feed
- ✅ Get post details with comments

**Output:** Saves posts and comments to `./test_posts.json`

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

**Output:** Saves forums and topics to `./test_forums.json`

### 4. Messages (`test_messages.py`)

**Prerequisites:** Run onboarding first

**Tests:**
- ✅ Send direct messages between users (2-4 messages per conversation)
- ✅ Get conversations list
- ✅ Get messages in a conversation
- ✅ Reply to received messages

**Output:** Saves messages and conversations to `./test_messages.json`

## Test Output

All tests use colored output for easy reading:

- ✅ **Green** - Success
- ❌ **Red** - Error
- ℹ **Blue** - Information
- ⚠️ **Yellow** - Warning

## Generated Test Data Files

All test data is saved in the `api_flow` folder:

- `test_users.json` - Created users with credentials and tokens
- `test_posts.json` - Created posts and comments
- `test_forums.json` - Created forums and topics
- `test_messages.json` - Sent messages

## Sample Data

Each test flow generates realistic sample data:

**Users:**
- `alice_20240115_143022`, `bob_20240115_143022`, `charlie_20240115_143022`
- Passwords: `SecurePass123!`, `SecurePass456!`, `SecurePass789!`

**Posts:**
- Various sample posts about daily life, technology, nature, etc.

**Forums:**
- Technology, Photography, Books, Fitness, Travel, Food, Music, Gaming

**Messages:**
- Friendly introductions and conversations

## Running Tests Step-by-Step

### Local Development

```bash
# 1. Start the backend
docker-compose up -d

# 2. Run all tests
./run_all_tests.py

# 3. Check results
cat test_users.json
cat test_posts.json
cat test_forums.json
cat test_messages.json
```

### Kubernetes

```bash
# 1. Port-forward the backend
kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080

# 2. In another terminal, run tests
cd tests/api_flows
./run_all_tests.py

# 3. Or with explicit endpoint
API_BASE_URL=http://localhost:8080 ./run_all_tests.py
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
❌ Cannot connect to backend at http://localhost:8080
```
Solution: 
- Local: Start with `docker-compose up -d`
- Kubernetes: Run `kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080`

**No test users:**
```bash
❌ Users file not found: test_users.json
```
Solution: Run onboarding test first (`python3 test_onboarding.py`)

**Port conflicts:**
If port 8080 is in use, use a different port:
```bash
kubectl port-forward -n jojuhu svc/jojuhu-backend 9090:8080
API_BASE_URL=http://localhost:9090 ./run_all_tests.py
```

**Permission denied:**
```bash
chmod +x run_all_tests.py
```

## Integration with CI/CD

These tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions
- name: Run API Tests
  run: |
    docker-compose up -d
    sleep 10  # Wait for services to start
    pip install requests
    cd tests/api_flows
    ./run_all_tests.py
  env:
    API_BASE_URL: http://localhost:8080
```

## API Endpoints Tested

- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `GET /api/v1/users/me` - Get current user profile
- `POST /api/v1/timeline/posts` - Create post
- `GET /api/v1/timeline/feed` - Get feed
- `POST /api/v1/timeline/posts/{id}/like` - Like post
- `POST /api/v1/timeline/posts/{id}/comments` - Comment on post
- `GET /api/v1/forums` - List forums
- `POST /api/v1/forums` - Create forum
- `POST /api/v1/forums/{id}/join` - Join forum
- `POST /api/v1/forums/{id}/topics` - Create topic
- `POST /api/v1/messages` - Send message
- `GET /api/v1/messages/conversations` - Get conversations

## Contributing

To add new test cases:

1. Copy an existing test flow
2. Add your test methods
3. Call them from the flow's `run_all_tests()` method
4. Update this README

## License

Same as Jojuhu project
