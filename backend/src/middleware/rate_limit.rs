use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Rate limit entry for a user
#[derive(Debug, Clone)]
struct RateLimitEntry {
    requests: u32,
    reset_time: Instant,
}

/// Per-user rate limiter
#[derive(Debug, Clone)]
pub struct UserRateLimiter {
    limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window: Duration,
}

impl UserRateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub async fn check_rate_limit(&self, user_id: &str) -> Result<(), String> {
        let now = Instant::now();
        let mut limits = self.limits.write().await;

        // Clean up expired entries periodically (simple cleanup)
        if limits.len() > 10000 {
            limits.retain(|_, entry| entry.reset_time > now);
        }

        let entry = limits.entry(user_id.to_string()).or_insert(RateLimitEntry {
            requests: 0,
            reset_time: now + self.window,
        });

        // Reset if window has passed
        if now > entry.reset_time {
            entry.requests = 0;
            entry.reset_time = now + self.window;
        }

        if entry.requests >= self.max_requests {
            let retry_after = entry.reset_time.duration_since(now).as_secs();
            return Err(format!(
                "Rate limit exceeded. Try again in {} seconds",
                retry_after
            ));
        }

        entry.requests += 1;
        Ok(())
    }
}

/// Middleware for per-user rate limiting
pub struct UserRateLimit {
    limiter: Arc<UserRateLimiter>,
}

impl UserRateLimit {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            limiter: Arc::new(UserRateLimiter::new(max_requests, window_seconds)),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for UserRateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserRateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self,
        service: S,
    ) -> Self::Future {
        ready(Ok(UserRateLimitMiddleware {
            service,
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct UserRateLimitMiddleware<S> {
    service: S,
    limiter: Arc<UserRateLimiter>,
}

impl<S, B> Service<ServiceRequest> for UserRateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(
        &self,
        req: ServiceRequest,
    ) -> Self::Future {
        let limiter = self.limiter.clone();
        
        // Extract user ID before moving the request
        let user_id = req.extensions().get::<crate::utils::auth::Claims>().map(|c| c.sub.to_string());
        
        let fut = self.service.call(req);

        Box::pin(async move {
            // Check rate limit if user is authenticated
            if let Some(uid) = user_id {
                if let Err(msg) = limiter.check_rate_limit(&uid).await {
                    return Err(actix_web::error::ErrorTooManyRequests(msg));
                }
            }
            
            let res = fut.await?;
            Ok(res)
        })
    }
}