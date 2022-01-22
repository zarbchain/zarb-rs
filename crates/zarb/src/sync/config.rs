use crate::sync::firewall;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub firewall: firewall::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            firewall: firewall::Config::default(),
        }
    }
}