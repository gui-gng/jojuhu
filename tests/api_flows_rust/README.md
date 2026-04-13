# Jojuhu API Flow Tests (Rust)

Comprehensive API testing suite for the Jojuhu backend, written in Rust.

## Features

- **50 Pre-generated Test Users** - JSON file with diverse user data
- **Multiple Test Scenarios** - Authentication, Users, Posts, Forums, Messages, Groups
- **Parallel API Testing** - Async/await for efficient testing
- **Colored Terminal Output** - Easy-to-read test results
- **Detailed Reporting** - JSON output with all test results and created entities

## Installation

```bash
cd tests/api_flows_rust
cargo build --release
```

## Usage

### Generate Test Users

```bash
# Generate 50 test users
cargo run --bin generate-users

# With custom API URL
API_BASE_URL=http://localhost:8080 cargo run --bin generate-users
```

### Run All Tests

```bash
# Run all test scenarios
cargo run --bin api-flow-tests

# With custom API URL
cargo run --bin api-flow-tests -- --url http://localhost:8080
```

### Run Specific Scenarios

```bash
# Authentication only
cargo run --bin api-flow-tests -- --scenario auth

# User management
cargo run --bin api-flow-tests -- --scenario users

# Posts/Timeline
cargo run --bin api-flow-tests -- --scenario posts

# Forums
cargo run --bin api-flow-tests -- --scenario forums

# Messages
cargo run --bin api-flow-tests -- --scenario messages

# Groups
cargo run --bin api-flow-tests -- --scenario groups
```

## Test Scenarios

### 1. Authentication (`auth`)

- Health check
- User registration (up to 20 users)
- User login
- Profile retrieval
- Registration failure cases (duplicates, invalid email, short password)
- Login failure cases (wrong password, non-existent user, empty credentials)

### 2. User Management (`users`)

- Update user profiles (bio, display name)
- Follow/unfollow users
- Get user profiles

### 3. Posts/Timeline (`posts`)

- Create posts (2-5 per user)
- Like posts from other users
- Add comments to posts
- Get feeds (For You, Following)
- Get post details

### 4. Forums (`forums`)

- Create forums (1-2 per user)
- Join forums
- Create topics in forums
- List forums

### 5. Messages (`messages`)

- Send direct messages between users
- Get conversation list
- Reply to messages

### 6. Groups (`groups`)

- Create groups
- Join groups
- List groups

## Output Files

All test data is saved in the `data/` directory:

- `test_users.json` - Generated test users with credentials and tokens
- `test_results.json` - Complete test results with all created entities

## Environment Variables

- `API_BASE_URL` - Base URL for the API (default: `http://localhost:8080`)

## Example Output

```
══════════════════════════════════════════════════════════════════════
  JOJUHU API FLOW TESTS
  Comprehensive API Testing Suite
══════════════════════════════════════════════════════════════════════

══════════════════════════════════════════════════════════════════════
AUTHENTICATION SCENARIOS
══════════════════════════════════════════════════════════════════════

ℹ Starting Authentication Tests...
✓ Health check passed
✓ Registered: alice_testuser
✓ Registered: bob_testuser
✓ Login successful: alice_testuser
✓ Login successful: bob_testuser
...

══════════════════════════════════════════════════════════════════════
  TEST SUMMARY
══════════════════════════════════════════════════════════════════════

  Total Tests:    45
  ✅ Passed:       43
  ❌ Failed:       2

  Created Entities:
    👤 Users:      20
    📝 Posts:      65
    💬 Comments:   32
    🏛️  Forums:     12
    📌 Topics:     18
    💌 Messages:   45
    👥 Groups:     8

  📄 Results saved to: data/test_results.json
══════════════════════════════════════════════════════════════════════
```

## Architecture

```
api_flows_rust/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI and test runner
│   ├── bin/
│   │   └── generate_users.rs # User generation utility
│   ├── models/
│   │   └── mod.rs           # Data models (User, Post, Forum, etc.)
│   ├── scenarios/
│   │   ├── mod.rs
│   │   ├── auth.rs          # Authentication tests
│   │   ├── users.rs         # User management tests
│   │   ├── posts.rs         # Timeline/Posts tests
│   │   ├── forums.rs        # Forums tests
│   │   ├── messages.rs      # Messages tests
│   │   └── groups.rs        # Groups tests
│   └── utils/
│       ├── mod.rs
│       ├── api_client.rs    # HTTP client wrapper
│       └── logging.rs       # Terminal logging utilities
```

## Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` - Serialization
- `fake` - Fake data generation
- `colored` - Terminal colors
- `clap` - CLI arguments
- `uuid` - UUID generation
- `chrono` - Date/time handling
- `rand` - Random number generation
