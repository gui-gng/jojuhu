//! API documentation module
//!
//! This module provides API documentation endpoints.
//! For a full OpenAPI spec, consider using utoipa with derive macros
//! on all your models and handlers.

use actix_web::HttpResponse;

/// Serve API documentation HTML
pub async fn swagger_ui() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(API_DOCS_HTML)
}

/// Serve OpenAPI JSON specification
pub async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(OPENAPI_SPEC)
}

const API_DOCS_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Social Network API Documentation</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            line-height: 1.6;
            background: #f5f5f5;
        }
        .container {
            background: white;
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }
        h2 { color: #555; margin-top: 30px; }
        h3 { color: #666; }
        .endpoint {
            background: #f8f9fa;
            padding: 15px;
            margin: 10px 0;
            border-left: 4px solid #4CAF50;
            border-radius: 4px;
        }
        .method {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 4px;
            font-weight: bold;
            font-size: 0.85em;
            margin-right: 10px;
        }
        .get { background: #61affe; color: white; }
        .post { background: #49cc90; color: white; }
        .put { background: #fca130; color: white; }
        .delete { background: #f93e3e; color: white; }
        .protected { 
            background: #fff3cd; 
            padding: 10px; 
            border-radius: 4px; 
            margin: 10px 0;
            border-left: 4px solid #ffc107;
        }
        code {
            background: #f4f4f4;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
        }
        pre {
            background: #f4f4f4;
            padding: 15px;
            border-radius: 4px;
            overflow-x: auto;
        }
        table {
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }
        th, td {
            text-align: left;
            padding: 12px;
            border-bottom: 1px solid #ddd;
        }
        th {
            background: #f8f9fa;
            font-weight: 600;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Social Network API Documentation</h1>
        
        <div class="protected">
            <strong>Authentication:</strong> Protected endpoints require a Bearer token in the 
            <code>Authorization</code> header: <code>Authorization: Bearer YOUR_JWT_TOKEN</code>
        </div>

        <h2>Base URL</h2>
        <p><code>http://localhost:8080/api/v1</code></p>

        <h2>Public Endpoints</h2>
        
        <div class="endpoint">
            <h3><span class="method post">POST</span> /auth/register</h3>
            <p>Register a new user account.</p>
            <strong>Request Body:</strong>
            <pre>{
  "username": "string (3-32 chars, alphanumeric + underscore)",
  "email": "string (valid email)",
  "password": "string (8-128 chars)",
  "display_name": "string (optional)"
}</pre>
            <strong>Response:</strong> 201 Created with user data and JWT token
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /auth/login</h3>
            <p>Authenticate and receive JWT token.</p>
            <strong>Request Body:</strong>
            <pre>{
  "username_or_email": "string",
  "password": "string"
}</pre>
            <strong>Response:</strong> 200 OK with user data and JWT token
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /health</h3>
            <p>Health check endpoint.</p>
            <strong>Response:</strong> 200 OK with service status
        </div>

        <h2>Protected Endpoints</h2>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /me</h3>
            <p>Get current authenticated user information.</p>
            <strong>Response:</strong> 200 OK with user profile
        </div>

        <h3>Messages</h3>
        
        <div class="endpoint">
            <h3><span class="method get">GET</span> /messages/conversations</h3>
            <p>List all conversations for the authenticated user.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /messages/conversations/{user_id}</h3>
            <p>Get messages between current user and specified user.</p>
            <strong>Query Parameters:</strong>
            <ul>
                <li><code>page</code> - Page number (default: 1)</li>
                <li><code>per_page</code> - Items per page (default: 20, max: 100)</li>
            </ul>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /messages</h3>
            <p>Send a message to another user.</p>
            <strong>Request Body:</strong>
            <pre>{
  "recipient_id": "uuid",
  "content": "string (max 2000 chars)"
}</pre>
        </div>

        <div class="endpoint">
            <h3><span class="method put">PUT</span> /messages/{message_id}/read</h3>
            <p>Mark a message as read.</p>
        </div>

        <h3>Timeline</h3>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /timeline/feed</h3>
            <p>Get social feed with posts from followed users.</p>
            <strong>Query Parameters:</strong>
            <ul>
                <li><code>page</code> - Page number (default: 1)</li>
                <li><code>per_page</code> - Items per page (default: 20, max: 100)</li>
            </ul>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /timeline/posts</h3>
            <p>Create a new post.</p>
            <strong>Request Body:</strong>
            <pre>{
  "content": "string (max 5000 chars)",
  "is_public": "boolean (default: true)",
  "media_urls": "array of strings (optional)"
}</pre>
        </div>

        <h3>Forums</h3>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /forums</h3>
            <p>List all public forums.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /forums</h3>
            <p>Create a new forum.</p>
            <strong>Request Body:</strong>
            <pre>{
  "name": "string (max 200 chars)",
  "description": "string (optional, max 10000 chars)",
  "is_public": "boolean (default: true)"
}</pre>
        </div>

        <h3>Groups</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /groups</h3>
            <p>Create a new group.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /groups</h3>
            <p>List groups (public or user's groups).</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /groups/{id}/join</h3>
            <p>Join a public group.</p>
        </div>

        <h3>Stories</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /stories</h3>
            <p>Create a new story (expires in 24 hours).</p>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /stories</h3>
            <p>Get stories from followed users.</p>
        </div>

        <h3>Polls</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /polls</h3>
            <p>Create a new poll.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /polls/{id}/vote</h3>
            <p>Vote on a poll.</p>
        </div>

        <h3>Privacy & GDPR</h3>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /privacy/settings</h3>
            <p>Get user privacy settings.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method put">PUT</span> /privacy/settings</h3>
            <p>Update privacy settings.</p>
            <strong>Request Body:</strong>
            <pre>{
  "profile_visibility": "string (public/followers_only/private)",
  "show_email": "boolean",
  "show_phone": "boolean",
  "allow_mentions": "boolean",
  "allow_tags": "boolean",
  "show_online_status": "boolean",
  "show_activity": "boolean",
  "allow_search_engines": "boolean",
  "data_processing_consent": "boolean",
  "marketing_emails_consent": "boolean"
}</pre>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /privacy/data-export</h3>
            <p>Request GDPR data export.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /privacy/data-export</h3>
            <p>Check data export status.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /privacy/account-deletion</h3>
            <p>Request account deletion (30-day grace period).</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /privacy/account-deletion/cancel</h3>
            <p>Cancel pending account deletion.</p>
        </div>

        <h3>Scheduled Posts</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /scheduled-posts</h3>
            <p>Create a scheduled post.</p>
            <strong>Request Body:</strong>
            <pre>{
  "content": "string",
  "media_urls": "array of strings (optional)",
  "scheduled_for": "datetime (ISO 8601)"
}</pre>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /scheduled-posts</h3>
            <p>List scheduled posts for current user.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method put">PUT</span> /scheduled-posts/{id}</h3>
            <p>Update a scheduled post.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /scheduled-posts/{id}/cancel</h3>
            <p>Cancel a scheduled post.</p>
        </div>

        <h3>Post Drafts</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /drafts</h3>
            <p>Save/update post draft (auto-save).</p>
            <strong>Request Body:</strong>
            <pre>{
  "content": "string (optional)",
  "media_urls": "array of strings (optional)",
  "visibility": "string (optional)"
}</pre>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /drafts</h3>
            <p>Get current user's draft.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method delete">DELETE</span> /drafts</h3>
            <p>Delete current user's draft.</p>
        </div>

        <h3>Push Notifications</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /push-tokens</h3>
            <p>Register device push notification token.</p>
            <strong>Request Body:</strong>
            <pre>{
  "device_token": "string",
  "device_type": "string (ios/android/web)",
  "device_name": "string (optional)"
}</pre>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /push-tokens/deactivate</h3>
            <p>Deactivate a device token.</p>
        </div>

        <h3>Moderation (Admin)</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /moderation/reports</h3>
            <p>Report content for moderation.</p>
        </div>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /moderation/reports</h3>
            <p>List moderation reports (admin only).</p>
        </div>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /moderation/reports/{id}/resolve</h3>
            <p>Resolve a moderation report (admin only).</p>
        </div>

        <h3>Link Preview</h3>

        <div class="endpoint">
            <h3><span class="method post">POST</span> /link-preview</h3>
            <p>Generate link preview metadata.</p>
            <strong>Request Body:</strong>
            <pre>{
  "url": "string (valid URL)"
}</pre>
            <strong>Response:</strong> Link preview with title, description, and image
        </div>

        <h3>Search</h3>

        <div class="endpoint">
            <h3><span class="method get">GET</span> /search</h3>
            <p>Search across posts, forums, topics, and users.</p>
            <strong>Query Parameters:</strong>
            <ul>
                <li><code>q</code> - Search query (required, min 2 chars)</li>
                <li><code>search_type</code> - Type: posts, forums, topics, users, all (default: all)</li>
                <li><code>page</code> - Page number (default: 1)</li>
                <li><code>per_page</code> - Items per page (default: 20, max: 100)</li>
            </ul>
            <strong>Response:</strong> Paginated search results with relevance scores
        </div>

        <h2>Error Responses</h2>
        <table>
            <tr>
                <th>Status Code</th>
                <th>Meaning</th>
                <th>Description</th>
            </tr>
            <tr>
                <td>400</td>
                <td>Bad Request</td>
                <td>Invalid input data</td>
            </tr>
            <tr>
                <td>401</td>
                <td>Unauthorized</td>
                <td>Missing or invalid authentication</td>
            </tr>
            <tr>
                <td>403</td>
                <td>Forbidden</td>
                <td>Insufficient permissions</td>
            </tr>
            <tr>
                <td>404</td>
                <td>Not Found</td>
                <td>Resource does not exist</td>
            </tr>
            <tr>
                <td>409</td>
                <td>Conflict</td>
                <td>Resource already exists (e.g., duplicate username)</td>
            </tr>
            <tr>
                <td>429</td>
                <td>Too Many Requests</td>
                <td>Rate limit exceeded (1 req/sec)</td>
            </tr>
            <tr>
                <td>500</td>
                <td>Internal Server Error</td>
                <td>Unexpected server error</td>
            </tr>
        </table>

        <h2>Rate Limiting</h2>
        <p>API requests are limited to <strong>1 request per second</strong> per IP address with a burst capacity of <strong>10 requests</strong>.</p>

        <h2>Security Headers</h2>
        <p>All responses include security headers:</p>
        <ul>
            <li><code>X-Content-Type-Options: nosniff</code></li>
            <li><code>X-Frame-Options: DENY</code></li>
            <li><code>X-XSS-Protection: 1; mode=block</code></li>
            <li><code>Strict-Transport-Security</code></li>
            <li><code>Content-Security-Policy</code></li>
        </ul>

        <h2>Input Validation</h2>
        <p>All user-generated content is validated and sanitized:</p>
        <ul>
            <li>XSS protection via HTML sanitization</li>
            <li>SQL injection detection</li>
            <li>Content length limits</li>
            <li>Special character filtering</li>
        </ul>
    </div>
</body>
</html>"#;

const OPENAPI_SPEC: &str = r#"{
  "openapi": "3.0.3",
  "info": {
    "title": "Social Network API",
    "description": "A RESTful API for a social network platform with messaging, timeline, and forums.",
    "version": "1.0.0",
    "contact": {
      "name": "API Support",
      "email": "support@example.com"
    },
    "license": {
      "name": "MIT",
      "url": "https://opensource.org/licenses/MIT"
    }
  },
  "servers": [
    {
      "url": "http://localhost:8080/api/v1",
      "description": "Development server"
    }
  ],
  "paths": {
    "/health": {
      "get": {
        "summary": "Health check",
        "description": "Check if the API is running",
        "responses": {
          "200": {
            "description": "Service is healthy",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "status": { "type": "string", "example": "healthy" },
                    "service": { "type": "string", "example": "jojuhu_backend" }
                  }
                }
              }
            }
          }
        }
      }
    },
    "/auth/register": {
      "post": {
        "summary": "Register new user",
        "description": "Create a new user account",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": ["username", "email", "password"],
                "properties": {
                  "username": { "type": "string", "minLength": 3, "maxLength": 32 },
                  "email": { "type": "string", "format": "email" },
                  "password": { "type": "string", "minLength": 8, "maxLength": 128 },
                  "display_name": { "type": "string" }
                }
              }
            }
          }
        },
        "responses": {
          "201": { "description": "User created successfully" },
          "400": { "description": "Invalid input" },
          "409": { "description": "Username or email already exists" }
        }
      }
    },
    "/auth/login": {
      "post": {
        "summary": "User login",
        "description": "Authenticate and receive JWT token",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": ["username_or_email", "password"],
                "properties": {
                  "username_or_email": { "type": "string" },
                  "password": { "type": "string" }
                }
              }
            }
          }
        },
        "responses": {
          "200": { "description": "Login successful" },
          "401": { "description": "Invalid credentials" }
        }
      }
    },
    "/me": {
      "get": {
        "summary": "Get current user",
        "description": "Get profile of authenticated user",
        "security": [{ "bearerAuth": [] }],
        "responses": {
          "200": { "description": "User profile" },
          "401": { "description": "Not authenticated" }
        }
      }
    },
    "/search": {
      "get": {
        "summary": "Search",
        "description": "Search across posts, forums, topics, and users",
        "security": [{ "bearerAuth": [] }],
        "parameters": [
          {
            "name": "q",
            "in": "query",
            "required": true,
            "schema": { "type": "string", "minLength": 2 },
            "description": "Search query"
          },
          {
            "name": "search_type",
            "in": "query",
            "schema": { 
              "type": "string", 
              "enum": ["posts", "forums", "topics", "users", "all"],
              "default": "all"
            },
            "description": "Type of content to search"
          },
          {
            "name": "page",
            "in": "query",
            "schema": { "type": "integer", "default": 1 },
            "description": "Page number"
          },
          {
            "name": "per_page",
            "in": "query",
            "schema": { "type": "integer", "default": 20, "maximum": 100 },
            "description": "Items per page"
          }
        ],
        "responses": {
          "200": { "description": "Search results" },
          "400": { "description": "Invalid query" },
          "401": { "description": "Not authenticated" }
        }
      }
    }
  },
  "components": {
    "securitySchemes": {
      "bearerAuth": {
        "type": "http",
        "scheme": "bearer",
        "bearerFormat": "JWT"
      }
    }
  }
}"#;
