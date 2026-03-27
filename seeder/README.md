# Jojuhu Database Seeder

A simple Rust tool to populate the Jojuhu database with test data using the REST API.

## Features

- ✅ Creates users with profiles and bios
- ✅ Generates random follow relationships
- ✅ Creates posts with realistic content
- ✅ Creates forums with descriptions
- ✅ Users join forums automatically
- ✅ Sends direct messages between users

## Data Files

- `data/users.json` - User accounts to create
- `data/posts.json` - Sample posts content
- `data/forums.json` - Forum definitions
- `data/messages.json` - Sample messages

## Usage

### Prerequisites

Make sure the backend is running:
```bash
cd backend
docker-compose up -d
cargo run
```

### Run the Seeder

```bash
cd seeder
cargo run
```

### What it does

1. **Registers users** - Creates 8 test users
2. **Updates profiles** - Sets display names and bios
3. **Creates follows** - Each user follows 2-5 random users
4. **Creates posts** - Generates posts from random users
5. **Creates forums** - Creates 6 different forums
6. **Joins forums** - Users join forums
7. **Sends messages** - Creates conversations between users

### Output

```
🌱 Jojuhu Database Seeder

📊 Seed data loaded:
   - 8 users
   - 15 posts
   - 6 forums
   - 10 messages

👤 Step 1: Registering users...
   ✅ Registered: alice
   ✅ Registered: bob
   ...

🎉 Seeding complete!
   • Users: 8
   • Follow relationships: 32
   • Posts: 15
   • Forums: 6
   • Forum memberships: 25
   • Messages: 10

✨ Database is now populated with test data!
```

## Customization

Edit the JSON files in `data/` to change the seed data:

- Add more users to `users.json`
- Create different posts in `posts.json`
- Define new forums in `forums.json`
- Add message templates in `messages.json`

## API Endpoint

The seeder connects to: `http://localhost:8080/api/v1`

To change this, edit the `API_BASE` constant in `src/main.rs`.

## Running Multiple Times

The seeder is idempotent - it will:
- Log in existing users instead of failing
- Skip if forums already exist
- Continue creating new data on subsequent runs

## Test Accounts

After running the seeder, you can log in with:

| Username | Email | Password |
|----------|-------|----------|
| alice | alice@example.com | password123 |
| bob | bob@example.com | password123 |
| charlie | charlie@example.com | password123 |
| diana | diana@example.com | password123 |
| eve | eve@example.com | password123 |
| frank | frank@example.com | password123 |
| grace | grace@example.com | password123 |
| henry | henry@example.com | password123 |