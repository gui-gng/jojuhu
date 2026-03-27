#!/usr/bin/env python3
"""
Jojuhu API Test Flows - Posts
Test creating posts, comments, and interactions between users
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

class PostFlow:
    def __init__(self, users_data):
        self.users = users_data.get("users", [])
        self.tokens = users_data.get("tokens", {})
        self.created_posts = []
        self.comments = []
    
    def test_create_posts(self):
        """Test creating multiple posts for each user"""
        log_info("Testing: Create Posts for Each User")
        
        sample_posts = [
            "Just joined Jojuhu! Excited to connect with everyone 🎉",
            "Beautiful day today! Hope you're all doing well ☀️",
            "Working on something exciting. Can't wait to share! 💡",
            "What's everyone up to this weekend? 🎈",
            "Just had the best coffee ever! ☕️",
            "Learning new things every day. Growth mindset! 🌱",
            "Anyone else love coding as much as I do? 💻",
            "Nature is amazing. Went for a hike today! 🏔️",
            "Good music + good company = perfect evening 🎵",
            "Remember to take breaks and stay hydrated! 💧",
            "Just finished reading an amazing book 📚",
            "The sunset today was absolutely beautiful 🌅",
            "Trying out new recipes in the kitchen today 👨‍🍳",
            "Technology is incredible when used for good 🚀",
            "Grateful for all the amazing people in my life ❤️"
        ]
        
        for username, token in self.tokens.items():
            # Create 3-5 posts per user
            num_posts = random.randint(3, 5)
            log_info(f"Creating {num_posts} posts for {username}")
            
            for i in range(num_posts):
                try:
                    content = random.choice(sample_posts)
                    is_public = random.choice([True, True, True, False])  # 75% public
                    
                    response = requests.post(
                        f"{BASE_URL}{API_PREFIX}/timeline/posts",
                        json={
                            "content": f"{content} #{i+1}",
                            "is_public": is_public
                        },
                        headers={
                            "Authorization": f"Bearer {token}",
                            "Content-Type": "application/json"
                        }
                    )
                    
                    if response.status_code == 201:
                        data = response.json()
                        if data.get("success") and data.get("data"):
                            post = data["data"]
                            post["author_username"] = username
                            self.created_posts.append(post)
                            log_success(f"Created post by {username}: {content[:30]}...")
                        else:
                            log_error(f"Failed to create post: {data.get('error')}")
                    else:
                        log_error(f"Failed to create post: HTTP {response.status_code}")
                        
                except Exception as e:
                    log_error(f"Exception creating post: {e}")
        
        return len(self.created_posts) > 0
    
    def test_like_posts(self):
        """Test liking posts from other users"""
        log_info("Testing: Like Posts from Other Users")
        
        if len(self.created_posts) < 2:
            log_warning("Not enough posts to test liking")
            return
        
        like_count = 0
        
        for username, token in self.tokens.items():
            # Like 2-4 posts from other users
            other_posts = [p for p in self.created_posts if p["author_username"] != username]
            
            if len(other_posts) >= 2:
                posts_to_like = random.sample(other_posts, min(random.randint(2, 4), len(other_posts)))
                
                for post in posts_to_like:
                    try:
                        response = requests.post(
                            f"{BASE_URL}{API_PREFIX}/timeline/posts/{post['id']}/like",
                            headers={
                                "Authorization": f"Bearer {token}",
                                "Content-Type": "application/json"
                            }
                        )
                        
                        if response.status_code == 200:
                            log_success(f"{username} liked post by {post['author_username']}")
                            like_count += 1
                        elif response.status_code == 409:
                            log_warning(f"{username} already liked post by {post['author_username']}")
                        else:
                            log_error(f"Failed to like: HTTP {response.status_code}")
                            
                    except Exception as e:
                        log_error(f"Exception liking post: {e}")
        
        log_info(f"Total likes created: {like_count}")
    
    def test_comment_on_posts(self):
        """Test commenting on posts from other users"""
        log_info("Testing: Comment on Posts")
        
        sample_comments = [
            "Great post! Thanks for sharing 👍",
            "I totally agree with this! 💯",
            "This is so true! Thanks for posting 🙏",
            "Love this! Keep them coming ❤️",
            "Interesting perspective! 🤔",
            "Couldn't have said it better myself! 👏",
            "This made my day! ☀️",
            "So relatable! Thanks for sharing 💭",
            "Absolutely love this! 🎉",
            "Great insight! Thanks for posting 🌟"
        ]
        
        comment_count = 0
        
        for username, token in self.tokens.items():
            # Comment on 2-3 posts from other users
            other_posts = [p for p in self.created_posts if p["author_username"] != username]
            
            if len(other_posts) >= 2:
                posts_to_comment = random.sample(other_posts, min(random.randint(2, 3), len(other_posts)))
                
                for post in posts_to_comment:
                    try:
                        comment = random.choice(sample_comments)
                        
                        response = requests.post(
                            f"{BASE_URL}{API_PREFIX}/timeline/posts/{post['id']}/comments",
                            json={"content": comment},
                            headers={
                                "Authorization": f"Bearer {token}",
                                "Content-Type": "application/json"
                            }
                        )
                        
                        if response.status_code == 201:
                            data = response.json()
                            if data.get("success"):
                                self.comments.append({
                                    "id": data["data"]["id"],
                                    "post_id": post["id"],
                                    "author": username,
                                    "content": comment
                                })
                                log_success(f"{username} commented on {post['author_username']}'s post")
                                comment_count += 1
                            else:
                                log_error(f"Failed to comment: {data.get('error')}")
                        else:
                            log_error(f"Failed to comment: HTTP {response.status_code}")
                            
                    except Exception as e:
                        log_error(f"Exception commenting: {e}")
        
        log_info(f"Total comments created: {comment_count}")
    
    def test_get_feed(self):
        """Test getting the feed"""
        log_info("Testing: Get Feed")
        
        for username, token in self.tokens.items():
            try:
                # Get For You feed
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/timeline/feed?page=1&per_page=10",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        posts = data.get("data", [])
                        log_success(f"{username} got {len(posts)} posts in For You feed")
                    else:
                        log_error(f"Failed to get feed: {data.get('error')}")
                else:
                    log_error(f"Failed to get feed: HTTP {response.status_code}")
                
                # Get Following feed
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/timeline/following?page=1&per_page=10",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        posts = data.get("data", [])
                        log_success(f"{username} got {len(posts)} posts in Following feed")
                        
            except Exception as e:
                log_error(f"Exception getting feed: {e}")
    
    def test_get_post_details(self):
        """Test getting post details with comments"""
        log_info("Testing: Get Post Details")
        
        if not self.created_posts:
            log_warning("No posts to get details for")
            return
        
        # Get details for first 3 posts
        for post in self.created_posts[:3]:
            try:
                token = self.tokens.get(post["author_username"])
                if not token:
                    continue
                
                response = requests.get(
                    f"{BASE_URL}{API_PREFIX}/timeline/posts/{post['id']}",
                    headers={
                        "Authorization": f"Bearer {token}",
                        "Content-Type": "application/json"
                    }
                )
                
                if response.status_code == 200:
                    data = response.json()
                    if data.get("success"):
                        log_success(f"Got details for post {post['id'][:8]}...")
                    else:
                        log_error(f"Failed to get post: {data.get('error')}")
                else:
                    log_error(f"Failed to get post: HTTP {response.status_code}")
                    
            except Exception as e:
                log_error(f"Exception getting post: {e}")
    
    def run_all_tests(self):
        """Run all post tests"""
        print("\n" + "="*60)
        print("JOJUHU API TEST FLOW: POSTS")
        print("="*60 + "\n")
        
        if not self.users:
            log_error("No users available. Run onboarding test first.")
            return
        
        if self.test_create_posts():
            self.test_like_posts()
            self.test_comment_on_posts()
            self.test_get_feed()
            self.test_get_post_details()
        
        print("\n" + "="*60)
        print(f"POST TEST COMPLETE")
        print(f"Total Posts Created: {len(self.created_posts)}")
        print(f"Total Comments Created: {len(self.comments)}")
        print("="*60 + "\n")
        
        return {
            "posts": self.created_posts,
            "comments": self.comments
        }

if __name__ == "__main__":
    # Load users from previous test
    try:
        with open("/tmp/jojuhu_test_users.json", "r") as f:
            users_data = json.load(f)
    except FileNotFoundError:
        print("Error: No test users found. Run test_onboarding.py first.")
        sys.exit(1)
    
    flow = PostFlow(users_data)
    results = flow.run_all_tests()
    
    # Save results
    with open("/tmp/jojuhu_test_posts.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("Test results saved to /tmp/jojuhu_test_posts.json")
