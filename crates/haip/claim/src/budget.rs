#[derive(Debug, Clone)]
pub struct CognitiveBudget {
    pub daily_limit: u32,
    pub remaining: u32,
}

impl CognitiveBudget {
    pub fn new(daily_limit: u32) -> Self { Self { daily_limit, remaining: daily_limit } }
    pub fn consume(&mut self, credits: u32) { self.remaining = self.remaining.saturating_sub(credits); }
    pub fn reset(&mut self, limit: u32) { self.daily_limit = limit; self.remaining = limit; }
}
