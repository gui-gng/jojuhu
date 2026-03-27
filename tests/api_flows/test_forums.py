#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Forums
Test creating forums, joining forums, and creating topics
"""

import requests
import json
import random
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

class ForumsFlow:
    def __init__(self, users_data):
        self.users = users_data.get("users", [])
        self.tokens = users_data.get("tokens", {})
        self.created_forums = []
        self.created_topics = []
        self.joined_forums = {}
    
    def test_create_forums(self):
        """Test creating forums by different users"""
        log_info("Testing: Create Forums")
        
        sample_forums = [
            {
                "name": "Technology Enthusiasts",
                "description": "Discuss the latest in tech, programming, and innovation"
            },
            {
                "name": "Photography Lovers",
                "description": "Share your best shots and get feedback from the community"
            },
            {
                "name": "Book Club",
                "description": "Discover new books and discuss your favorite reads"
            },
            {
                "name": "Fitness & Health",
                "description": "Tips, motivation, and support for a healthy lifestyle"
            },
            {
                "name": "Travel Adventures",
                "description": "Share travel stories, tips, and recommendations"
            },
            {
                "name": "Food & Cooking",
                "description": "Recipes, restaurant reviews, and culinary discussions"
            },
            {
                "name": "Music Discovery",
                "description": "Find new music and discuss your favorite artists"
            },
            {
                "name": "Gaming Community",
                "description": "Discuss games, strategies, and find teammates"
            }
        ]
        
        # Each user creates 1-2 forums
        for username, token in self.tokens.items():
            num_forums = random.randint(1, 2)
            available_forums = [f for f in sample_forums if f not in [cf.get("template") for cf in self.created_forums]]
            
            if not available_forums:
                break
            
            forums_to_create = random.sample(available_forums, min(num_forums, len(available_forums)))
            
            for forum_template in forums_to_create:
                try:
                    forum_data = {
                        "name": f"{forum_template['name']} {datetime.now().strftime('%H%M%S')}",
                        "description": forum_template["description"],
                        "is_public": random.choice([True, True, True, False])
                    }
                    
                    response = requests.post(
                        f"{BASE_URL}{API_PREFIX}/forums",
                        json=forum_data,
                        headers={
                            "Authorization": f"Bearer {token}",
                            "Content-Type": "application/json"
                        }
                    )
                    
                    if response.status_code == 201:
                        data = response.json()
                        if data.get("success") and data.get("data"):
                            forum = data["data"]
                            forum["creator"] = username
                            forum["template"] = forum_template
                            self.created_forums.append(forum)
                            log_success(f"Created forum '{forum['name']}' by {username}")
                        else:
                            log_error(f"Failed to create forum: {data.get('error')}")
                    else:
                        log_error(f"Failed to create forum: HTTP {response.status_code}")
                        
                except Exception as e:
                    log_error(f"Exception creating forum: {e}")
        
        return len(self.created_forums) > 0
    
    def test_join_forums(self):
        """Test users joining forums created by others"""
        log_info("Testing: Join Forums")
        
        join_count = 0
        
        for username, token in self.tokens.items():
            # Join 2-4 forums not created by this user
            other_forums = [f for f in self.created_forums if f["creator"] != username]
            
            if len(other_forums) >= 2:
                forums_to_join = random.sample(other_forums, min(random.randint(2, 4), len(other_forums)))
                
                for forum in forums_to_join:
                    try:
                        response = requests.post(
                            f"{BASE_URL}{API_PREFIX}/forums/{forum['id']}/join",
                            headers={
                                "Authorization": f"Bearer {token}",
                                "Content-Type": "application/json"
                            }
                        )
                        
                        if response.status_code == 200:
                            if username not in self.joined_forums:
                                self.joined_forums[username] = []
                            self.joined_forums[username].append(forum)
                            log_success(f"{username} joined forum '{forum['name']}'")
                            join_count += 1
                        elif response.status_code == 409:
                            log_warning(f"{username} already member of '{forum['name']}'")
                        else:
                            log_error(f"Failed to join: HTTP {response.status_code}")
                            
                    except Exception as e:
                        log_error(f"Exception joining forum: {e}")
        
        log_info(f"Total forum joins: {join_count}")
    
    def test_create_topics(self):
        """Test creating topics in forums"""
        log_info("Testing: Create Topics")
        
        sample_topics = [
            {
                "title": "Welcome to our community!",
                "content": "Hello everyone! This is a place to share and learn. Feel free to introduce yourself!"
            },
            {
                "title": "Best practices for beginners",
                "content": "What are your best tips for someone just starting out? Share your wisdom!"
            },
            {
                "title": "Weekly challenge - Week 1",
                "content": "Let's start our first weekly challenge! Post your progress here."
            },
            {
                "title": "Resources and recommendations",
                "content": "Share your favorite resources, tools, or recommendations with the community."
            },
            {
                "title": "Q&A Session",
                "content": "Ask any questions you have! Our community is here to help."
            },
            {
                "title": "Success stories",
                "content": "Share your achievements and success stories to inspire others!"
            },
            {
                "title": "Tips and tricks",
                "content": "What are some lesser-known tips that have helped you?"
            },
            {
                "title": "Introduction thread",
                "content": "New here? Introduce yourself and tell us a bit about you!"
            }
        ]
        
        for username, token in self.tokens.items():
            # Get forums this user is a member of (created or joined)
            user_forums = [f for f in self.created_forums if f["creator"] == username]
            if username in self.joined_forums:
                user_forums.extend(self.joined_forums[username])
            
            if user_forums:
                # Create 1-2 topics per user
                num_topics = random.randint(1, 2)
                forums_for_topics = random.sample(user_forums, min(num_topics, len(user_forums)))
                
                for forum in forums_for_topics:
                    try:
                        topic_template = random.choice(sample_topics)
                        topic_data = {
                            "title": f"{topic_template['title']} - {datetime.now().strftime('%H:%M')}",
                            "content": topic_template["content"]
                        }
                        
                        response = requests.post(
                            f"{BASE_URL}{API_PREFIX}/forums/{forum['id']}/topics",
                            json=topic_data,
                            headers={
                                "Authorization": f"Bearer {token}",
                                "Content-Type": "application/json"
                            }
                        )
                        
                        if response.status_code == 201:
                            data = response.json()
                            if data.get("success") and data.get("data"):
                                topic = data["data"]
                                topic["forum_name"] = forum["name"]
                                topic["author"] = username
                                self.created_topics.append(topic)
                                log_success(f"{username} created topic in '{forum['name']}'")
                            else:
                                log_error(f"Failed to create topic: {data.get('error')}")
                        else:
                            log_error(f"Failed to create topic: HTTP {response.status_code}")
                            
                    except Exception as e:
                        log_error(f"Exception creating topic: {e}")
        
        return len(self.created_topics) > 0
    
    def test_reply_to_topics(self):
        """Test replying to topics"""
        log_info("Testing: Reply to Topics")
        
        sample_replies = [
            "Great topic! Thanks for starting this discussion 👍",
            "I completely agree with your points! 💯",
            "This is really helpful, thanks for sharing! 🙏",
            "Love this! Keep the content coming ❤️",
            "Interesting perspective, I hadn't thought of it that way 🤔",
            "Thanks for sharing this with us! 👏",
            "This is exactly what I needed to hear today ☀️",
            "So true! Thanks for posting this 💭",
            "Absolutely love this discussion! 🎉",
            "Great insights everyone! Thanks for participating 🌟"
        ]
        
        reply_count = 0
        
        for username, token in self.tokens.items():
            # Reply to 2-3 topics not created by this user
            other_topics = [t for t in self.created_topics if t["author"] != username]
            
            if len(other_topics) >= 2:
                topics_to_reply = random.sample(other_topics, min(random.randint(2, 3), len(other_topics)))
                
                for topic in topics_to_reply:
                    try:
                        reply_content = random.choice(sample_replies)
                        
                        # Find forum ID for this topic
                        forum_id = None
                        for forum in self.created_forums:
                            if forum["name"] == topic["forum_name"]:
                                forum_id = forum["id"]
                                break
                        
                        if not forum_id:
                            continue
                        
                        response = requests.post(
                            f"{BASE_URL}{API_PREFIX}/forums/{forum_id}/topics/{topic['id']}/replies",
                            json={"content": reply_content},
                            headers={
                                "Authorization": f"Bearer {token}",
                                "Content-Type": "application/json"
                            }
                        )
                        
                        if response.status_code == 201:
                            log_success(f"{username} replied to topic '{topic['title'][:30]}...'")
                            reply_count += 1
                        else:
                            log_error(f"Failed to reply: HTTP {response.status_code}")
                            
                    except Exception as e:
                        log_error(f"Exception replying: {e}")
        
        log_info(f"Total replies created: {reply_count}")
    
    def test_get_forums_list(self):
        """Test getting list of forums"""
        log_info("Testing: Get Forums List")
        
        for username, token in self.tokens.items():
            try:
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/forums?page=1&per_page=10",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        forums = data.get("data", [])
                        log_success(f"{username} retrieved {len(forums)} forums")
                    else:
                        log_error(f"Failed to get forums: {data.get('error')}")
                else:
                    log_error(f"Failed to get forums: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting forums: {e}")
    
    def run_all_tests(self):
        """Run all forum tests"""
        print("\n" + "="*60)
        print("JOJUHU API TEST FLOW: FORUMS")
        print("="*60 + "\n")
        
        if not self.users:
            log_error("No users available. Run onboarding test first.")
            return
        
        if self.test_create_forums():
            self.test_join_forums()
            self.test_get_forums_list()
            
            if self.test_create_topics():
                self.test_reply_to_topics()
        
        print("\n" + "="*60)
        print(f"FORUM TEST COMPLETE")
        print(f"Total Forums Created: {len(self.created_forums)}")
        print(f"Total Topics Created: {len(self.created_topics)}")
        print("="*60 + "\n")
        
        return {
            "forums": self.created_forums,
            "topics": self.created_topics
        }

if __name__ == "__main__":
    # Load users from previous test
    try:
        with open("./tmp/jojuhu_test_users.json", "r") as f:
            users_data = json.load(f)
    except FileNotFoundError:
        print("Error: No test users found. Run test_onboarding.py first.")
        sys.exit(1)
    
    flow = ForumsFlow(users_data)
    results = flow.run_all_tests()
    
    # Save results
    with open("./tmp/jojuhu_test_forums.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("Test results saved to ./tmp/jojuhu_test_forums.json")
