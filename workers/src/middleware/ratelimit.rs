use std::collections::HashMap;
use std::sync::Mutex;

/// Simple rate limiter for Workers.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, RateLimitBucket>>,
    max_requests: u32,
    window_secs: u64,
}

#[derive(Debug, Clone)]
struct RateLimitBucket {
    count: u32,
    reset_at: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self { buckets: Mutex::new(HashMap::new()), max_requests, window_secs }
    }

    pub fn check(&self, client_id: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let bucket = buckets.entry(client_id.to_string()).or_insert(RateLimitBucket {
            count: 0,
            reset_at: now + self.window_secs,
        });

        if now > bucket.reset_at {
            bucket.count = 0;
            bucket.reset_at = now + self.window_secs;
        }

        if bucket.count >= self.max_requests {
            return false;
        }

        bucket.count += 1;
        true
    }
}
