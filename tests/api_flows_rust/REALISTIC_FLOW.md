# Realistic User Flow Simulation - Implementation Summary

## Overview

I've implemented a comprehensive **realistic user flow simulation** system that makes test users behave like real people using the app. Each user has their own individual flow with random actions and timing.

## 🎭 New Features

### 1. **User Actor System** (`src/scenarios/user_flow.rs`)

Each user is now an independent "actor" that:
- Runs in its own async task concurrently with other users
- Performs random actions with weighted probabilities
- Waits realistic amounts of time between actions
- Has varying activity levels (power users vs casual users)

### 2. **Available Actions**

Users can perform these actions with realistic timing:

| Action | Weight | Delay Range | Description |
|--------|--------|-------------|-------------|
| Create Post | 25% | 2-8s | Create a new post |
| Like Post | 20% | 0.5-2s | Like someone else's post |
| Comment | 15% | 3-10s | Comment on a post |
| Follow User | 10% | 1-3s | Follow another user |
| View Feed | 10% | 3-8s | Browse the timeline |
| Browse Profiles | 5% | 2-5s | Look at other profiles |
| Update Profile | 3% | 5-15s | Change bio/display name |
| Create Forum | 3% | 5-12s | Create a new forum |
| Join Forum | 3% | 1-4s | Join an existing forum |
| Create Topic | 2% | 4-12s | Create a topic in a forum |
| Create Group | 2% | 5-15s | Create a new group |
| Join Group | 1% | 1-4s | Join an existing group |
| Send Message | 1% | 2-6s | DM another user |

### 3. **User Activity Levels**

Different users have different activity levels:
- **Power Users** (20%): 15 actions per session
- **Regular Users** (40%): 10 actions per session  
- **Casual Users** (40%): 5 actions per session

### 4. **Dynamic Scaling** (`src/utils/scaling.rs`)

All test scenarios now scale dynamically based on the number of available users:
- Posts: 1-3 per user depending on total user count
- Forums: Scale based on available forum creators
- Interactions: Percentage-based with min/max bounds
- Delays: Random intervals between actions

## 🚀 Usage

### Generate Users (Dynamic Count)

```bash
# Generate 50 users (default)
cargo run --bin generate-users

# Generate custom number of users
cargo run --bin generate-users -c 100
cargo run --bin generate-users --count 200

# Custom output file
cargo run --bin generate-users -c 50 -o data/my_users.json
```

### Run Realistic User Flow

```bash
# Run realistic simulation (default)
cargo run --bin api-flow-tests --scenario realistic

# With custom number of users to register
cargo run --bin api-flow-tests --scenario auth -n 50

# Run with realistic flow as part of 'all'
cargo run --bin api-flow-tests --scenario all
```

### Available Scenarios

- `auth` - Register and login users (dynamic count based on available users)
- `users` - User management tests
- `posts` - Posts/timeline tests
- `forums` - Forum tests
- `messages` - Message tests
- `groups` - Group tests
- `realistic` - **NEW**: Realistic concurrent user simulation
- `all` - Run auth then realistic simulation (recommended)

## 📊 Example Output

```
══════════════════════════════════════════════════════════════════════
  REALISTIC USER FLOW SIMULATION
══════════════════════════════════════════════════════════════════════

🎭 This scenario simulates real users with individual behaviors:
  • Each user performs random actions with realistic timing
  • Users create posts, like content, follow others, join forums
  • Actions happen concurrently with random delays
  • Power users are more active than casual users

👥 10 authenticated users will participate
👤 User alice_123 started their session (15 actions planned)
👤 User bob_456 started their session (5 actions planned)
📝 alice_123 created a post
❤️ bob_456 liked alice_123's post
💬 charlie_789 commented on alice_123's post
👥 alice_123 followed bob_456
🏛️ bob_456 created forum: Technology Enthusiasts - a1b2c3
...
✅ User alice_123 completed 15 actions
✅ Simulation complete! Created: 12 posts, 3 forums, 2 groups, 5 topics, 8 comments, 3 messages
```

## 🔄 How It Works

1. **Registration Phase**: Users are registered and logged in (getting JWT tokens)
2. **Actor Creation**: Each authenticated user becomes an actor with random activity level
3. **Concurrent Execution**: All actors run simultaneously in separate tokio tasks
4. **Random Actions**: Each actor picks random actions based on weighted probabilities
5. **Realistic Timing**: Random delays between actions (e.g., 2-8s for posting, 0.5-2s for liking)
6. **Shared State**: All actors share state (posts, forums, groups) so they can interact
7. **Staggered Start**: Users don't all start at once - staggered by 500ms

## 📝 Key Files Changed

- `src/bin/generate_users.rs` - Added CLI args for dynamic user count
- `src/main.rs` - Added realistic scenario, dynamic user count support
- `src/scenarios/user_flow.rs` - **NEW**: User actor system
- `src/scenarios/mod.rs` - Added user_flow module
- `src/utils/scaling.rs` - **NEW**: Dynamic scaling utilities
- `src/utils/mod.rs` - Added scaling module

## 🎯 Benefits

1. **Realistic Load Testing**: Simulates actual user behavior patterns
2. **Concurrent Operations**: Tests race conditions and concurrency
3. **Variable Timing**: Realistic delays stress-test the system
4. **Dynamic Scaling**: Works with any number of users (10 to 10,000+)
5. **Individual Flows**: Each user has unique behavior like real people
6. **Shared Interactions**: Users like each other's posts, follow each other, etc.

## 🔧 Configuration

### Environment Variables
```bash
API_BASE_URL=http://localhost:8080  # API endpoint
```

### Command Line Options
```bash
# Number of users to generate/register
cargo run --bin generate-users -c 100
cargo run --bin api-flow-tests -n 50

# Max actions per user (0 = auto: 5-15)
cargo run --bin api-flow-tests -m 20

# Choose scenario
cargo run --bin api-flow-tests -s realistic
```

## 💡 Recommended Usage

For load testing with realistic behavior:

```bash
# 1. Generate many users
cargo run --bin generate-users -c 500

# 2. Run realistic simulation (registers users automatically)
cargo run --bin api-flow-tests --scenario all

# Or run just the realistic flow with already registered users:
cargo run --bin api-flow-tests --scenario realistic
```
