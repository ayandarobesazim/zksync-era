use serde::Deserialize;
use std::time::Duration;

use super::envy_load;

/// Configuration for the fri proof compressor
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FriProofCompressorConfig {
    /// The compression mode to use
    pub compression_mode: u8,

    /// Configurations for prometheus
    pub prometheus_listener_port: u16,
    pub prometheus_pushgateway_url: String,
    pub prometheus_push_interval_ms: Option<u64>,

    /// Max time for proof compression to be performed
    pub generation_timeout_in_secs: u16,
    /// Max attempts for proof compression to be performed
    pub max_attempts: u32,
}

impl FriProofCompressorConfig {
    pub fn from_env() -> Self {
        envy_load("fri_proof_compressor", "FRI_PROOF_COMPRESSOR_")
    }

    pub fn generation_timeout(&self) -> Duration {
        Duration::from_secs(self.generation_timeout_in_secs as u64)
    }
}

#[cfg(test)]
mod tests {
    use crate::configs::test_utils::EnvMutex;

    use super::*;

    static MUTEX: EnvMutex = EnvMutex::new();

    fn expected_config() -> FriProofCompressorConfig {
        FriProofCompressorConfig {
            compression_mode: 1,
            prometheus_listener_port: 3316,
            prometheus_pushgateway_url: "http://127.0.0.1:9091".to_string(),
            prometheus_push_interval_ms: Some(100),
            generation_timeout_in_secs: 3000,
            max_attempts: 5,
        }
    }

    #[test]
    fn from_env() {
        let mut lock = MUTEX.lock();
        let config = r#"
            FRI_PROOF_COMPRESSOR_COMPRESSION_MODE=1
            FRI_PROOF_COMPRESSOR_PROMETHEUS_LISTENER_PORT=3316
            FRI_PROOF_COMPRESSOR_PROMETHEUS_PUSHGATEWAY_URL="http://127.0.0.1:9091"
            FRI_PROOF_COMPRESSOR_PROMETHEUS_PUSH_INTERVAL_MS=100
            FRI_PROOF_COMPRESSOR_GENERATION_TIMEOUT_IN_SECS=3000
            FRI_PROOF_COMPRESSOR_MAX_ATTEMPTS=5
        "#;
        lock.set_env(config);

        let actual = FriProofCompressorConfig::from_env();
        assert_eq!(actual, expected_config());
    }
}
