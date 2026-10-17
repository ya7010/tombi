#[derive(Debug, Clone)]
pub struct Options {
    pub no_cache: Option<bool>,
    pub cache_ttl: Option<std::time::Duration>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            no_cache: None,
            cache_ttl: std::env::var("TOMBI_CACHE_TTL")
                .map_or(None, |value| value.parse::<u64>().ok())
                .or(Some(DEFAULT_CACHE_TTL_SECONDS))
                .map(std::time::Duration::from_secs),
        }
    }
}

pub const DEFAULT_CACHE_TTL_SECONDS: u64 = 60 * 60 * 24;
