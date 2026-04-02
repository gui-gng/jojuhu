#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Messages
Test sending direct messages between users
"""

import requests
import json
import random
import sys
from datetime import datetime

# Configuration
BASE_URL = "http://localhost:50004"
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

class MessagesFlow:
    def __init__(self, users_data):
        self.users = users_data.get("users", [])
        self.tokens = users_data.get("tokens", {})
        self.sent_messages = []
        self.conversations = []
    
    def test_send_messages(self):
        """Test sending messages between users"""
        log_info("Testing: Send Messages Between Users")
        
        sample_messages = [
            "Hey! How are you doing? 👋",
            "Great to connect with you here! 🎉",
            "Thanks for the follow! Looking forward to your posts 💯",
            "Hi! I saw your profile and thought I'd say hello ☀️",
            "Hello! Love your content. Keep it up! 🌟",
            "Hey there! How's your day going? 😊",
            "Hi! Just wanted to introduce myself. Nice to meet you! 🤝",
            "Hello! Your recent post was really interesting 🧠",
            "Hey! Would love to connect and chat more 💬",
            "Hi there! Welcome to the community 🎈",
            "Hello! Thanks for being part of this platform 🙏",
            "Hey! Looking forward to seeing more from you 👀",
            "Hi! Hope you're having an amazing day! ✨",
            "Hello! Just dropping by to say hi 👋",
            "Hey there! Let's connect and share ideas 💡"
        ]
        
        # Each user sends messages to 2-3 other users
        for sender_username, sender_token in self.tokens.items():
            # Get list of other users
            other_users = [(u, t) for u, t in self.tokens.items() if u != sender_username]
            
            if len(other_users) >= 2:
                # Select 2-3 users to message
                num_recipients = min(random.randint(2, 3), len(other_users))
                recipients = random.sample(other_users, num_recipients)
                
                for recipient_username, recipient_token in recipients:
                    # Send 2-4 messages to this recipient
                    num_messages = random.randint(2, 4)
                    
                    for i in range(num_messages):
                        try:
                            # Get recipient user ID
                            # First, get the recipient's profile to get their ID
                            profile_response = requests.get(
                                f"{BASE_URL}{API_PREFIX}/users/me",
                                headers={
                                    "Authorization": f"Bearer {recipient_token}",
                                    "Content-Type": "application/json"
                                }
                            )
                            
                            if profile_response.status_code != 200:
                                continue
                            
                            recipient_data = profile_response.json()
                            if not recipient_data.get("success"):
                                continue
                            
                            recipient_id = recipient_data["data"]["id"]
                            
                            message_content = random.choice(sample_messages)
                            if i > 0:
                                # Add variation for follow-up messages
                                follow_ups = [
                                    "Just wanted to follow up on my last message 👆",
                                    "Also, I really enjoyed your recent post! 👏",
                                    "By the way, what do you think about the platform so far? 🤔",
                                    "Hope to hear from you soon! 📬"
                                ]
                                message_content = random.choice(follow_ups)
                            
                            response = requests.post(
                                f"{BASE_URL}{API_PREFIX}/messages",
                                json={
                                    "recipient_id": recipient_id,
                                    "content": message_content
                                },
                                headers={
                                    "Authorization": f"Bearer {sender_token}",
                                    "Content-Type": "application/json"
                                }
                            )
                            
                            if response.status_code == 201:
                                data = response.json()
                                if data.get("success") and data.get("data"):
                                    message = data["data"]
                                    message["sender"] = sender_username
                                    message["recipient"] = recipient_username
                                    self.sent_messages.append(message)
                                    log_success(f"{sender_username} → {recipient_username}: {message_content[:30]}...")
                                else:
                                    log_error(f"Failed to send: {data.get('error')}")
                            else:
                                log_error(f"Failed to send: HTTP {response.status_code}")
                                
                        except Exception as e:
                            log_error(f"Exception sending message: {e}")
        
        return len(self.sent_messages) > 0
    
    def test_get_conversations(self):
        """Test getting conversation list"""
        log_info("Testing: Get Conversations List")
        
        for username, token in self.tokens.items():
            try:
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/messages/conversations",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        conversations = data.get("data", [])
                        log_success(f"{username} has {len(conversations)} conversations")
                        
                        # Store conversation details
                        for conv in conversations:
                            conv["owner"] = username
                            self.conversations.append(conv)
                    else:
                        log_error(f"Failed to get conversations: {data.get('error')}")
                else:
                    log_error(f"Failed to get conversations: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting conversations: {e}")
    
    def test_get_conversation_messages(self):
        """Test getting messages in a conversation"""
        log_info("Testing: Get Conversation Messages")
        
        if not self.conversations:
            log_warning("No conversations to get messages from")
            return
        
        # Get messages for first 3 conversations
        for conv in self.conversations[:3]:
            try:
                username = conv["owner"]
                token = self.tokens.get(username)
                
                if not token:
                    continue
                
                # The conversation ID is the other user's ID
                other_user_id = conv["user_id"]
                
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/messages/{other_user_id}?page=1&per_page=20",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        messages = data.get("data", [])
                        log_success(f"{username} got {len(messages)} messages with {conv['username']}")
                    else:
                        log_error(f"Failed to get messages: {data.get('error')}")
                else:
                    log_error(f"Failed to get messages: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting messages: {e}")
    
    def test_reply_to_messages(self):
        """Test replying to received messages"""
        log_info("Testing: Reply to Messages")
        
        reply_messages = [
            "Hey! Thanks for reaching out! I'm doing great 😊",
            "Nice to meet you too! Love the community here 🎉",
            "Thanks! I appreciate the kind words 💯",
            "I'm doing well, thanks for asking! How about you? ☀️",
            "Thanks! I try to post interesting content 🌟",
            "My day is going great! Hope yours is too 🎈",
            "Nice to meet you as well! Welcome aboard 🤝",
            "Thanks! I'm glad you found it interesting 🧠",
            "Absolutely! Would love to chat more 💬",
            "Thanks for the warm welcome! 🙏",
            "Looking forward to your posts as well! 👀",
            "Having an amazing day, thank you! Hope you are too ✨",
            "Hi back! Thanks for saying hello 👋",
            "Thanks! Always happy to share ideas 💡",
            "Thanks so much! This is a great community ❤️"
        ]
        
        reply_count = 0
        
        # Get messages that need replies (roughly half of sent messages)
        messages_to_reply = [m for m in self.sent_messages if random.choice([True, False])]
        
        for message in messages_to_reply:
            try:
                # Reply from recipient back to sender
                recipient_username = message["recipient"]
                sender_username = message["sender"]
                
                recipient_token = self.tokens.get(recipient_username)
                sender_token = self.tokens.get(sender_username)
                
                if not recipient_token or not sender_token:
                    continue
                
                # Get sender's ID
                profile_response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/users/me",
                    headers={
                        "Authorization": f"Bearer {sender_token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if profile_response.status_code != 200:
                    continue
                
                sender_data = profile_response.json()
                if not sender_data.get("success"):
                    continue
                
                sender_id = sender_data["data"]["id"]
                
                reply_content = random.choice(reply_messages)
                
                response = requests.post(
                    f"{BASE_URL}{API_PREFIX}/messages",
                    json={
                        "recipient_id": sender_id,
                        "content": reply_content
                    },
                    headers={
                        "Authorization": f"Bearer {recipient_token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 201:
                    log_success(f"{recipient_username} replied to {sender_username}")
                    reply_count += 1
                else:
                    log_error(f"Failed to reply: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception replying: {e}")
        
        log_info(f"Total replies sent: {reply_count}")
    
    def run_all_tests(self):
        """Run all message tests"""
        print("\n" + "="*60)
        print("JOJUHU API TEST FLOW: MESSAGES")
        print("="*60 + "\n")
        
        if not self.users:
            log_error("No users available. Run onboarding test first.")
            return
        
        if self.test_send_messages():
            self.test_get_conversations()
            self.test_get_conversation_messages()
            self.test_reply_to_messages()
        
        print("\n" + "="*60)
        print(f"MESSAGES TEST COMPLETE")
        print(f"Total Messages Sent: {len(self.sent_messages)}")
        print(f"Total Conversations: {len(self.conversations)}")
        print("="*60 + "\n")
        
        return {
            "messages": self.sent_messages,
            "conversations": self.conversations
        }

if __name__ == "__main__":
    # Load users from previous test
    try:
        with open("./tmp/jojuhu_test_users.json", "r") as f:
            users_data = json.load(f)
    except FileNotFoundError:
        print("Error: No test users found. Run test_onboarding.py first.")
        sys.exit(1)
    
    flow = MessagesFlow(users_data)
    results = flow.run_all_tests()
    
    # Save results
    with open("./tmp/jojuhu_test_messages.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("Test results saved to ./tmp/jojuhu_test_messages.json")
