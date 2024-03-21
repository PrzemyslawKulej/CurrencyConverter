use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct RatesResponse {
    pub base: String,
    pub rates: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub rates: HashMap<String, f64>,
    pub last_updated: SystemTime,
}

lazy_static! {
    pub static ref CURRENCY_CACHE: Mutex<HashMap<String, CacheEntry>> = Mutex::new(HashMap::new());
}
