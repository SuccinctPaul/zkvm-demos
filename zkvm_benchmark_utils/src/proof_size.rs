//! Proof size measurement utilities
//!
//! Provides traits and helpers for measuring proof sizes across different zkVMs

use serde::Serialize;

/// Trait for measuring proof sizes
pub trait ProofSize {
    fn measure_size(&self) -> usize;
    fn measure_size_detailed(&self) -> ProofSizeDetail;
}

#[derive(Debug, Clone)]
pub struct ProofSizeDetail {
    pub bincode_size: Option<usize>,
    pub json_size: Option<usize>,
    pub final_size: usize,
    pub method: String,
}

impl<T: Serialize> ProofSize for T {
    fn measure_size(&self) -> usize {
        // Priority: bincode (more compact, closer to real size)
        if let Ok(bytes) = bincode::serialize(self) {
            return bytes.len();
        }

        // Fallback: serde_json
        if let Ok(bytes) = serde_json::to_vec(self) {
            return bytes.len();
        }

        0
    }

    fn measure_size_detailed(&self) -> ProofSizeDetail {
        let bincode_size = bincode::serialize(self).ok().map(|b| b.len());
        let json_size = serde_json::to_vec(self).ok().map(|b| b.len());

        let (final_size, method) = if let Some(size) = bincode_size {
            (size, "bincode".to_string())
        } else if let Some(size) = json_size {
            (size, "serde_json".to_string())
        } else {
            (0, "unavailable".to_string())
        };

        ProofSizeDetail {
            bincode_size,
            json_size,
            final_size,
            method,
        }
    }
}

/// Helper macro for measuring proof size in instrumented code
#[macro_export]
macro_rules! measure_stage {
    ($stage_name:expr, $proof:expr, $time:expr) => {
        #[cfg(feature = "benchmark")]
        {
            use $crate::proof_size::ProofSize;
            let size = $proof.measure_size();
            println!(
                "BENCHMARK: {}_time_s={:.3}",
                $stage_name,
                $time.as_secs_f64()
            );
            println!(
                "BENCHMARK: {}_size_kb={:.2}",
                $stage_name,
                size as f64 / 1024.0
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_size() {
        let data = vec![1u8, 2, 3, 4, 5];
        let size = data.measure_size();
        assert!(size > 0);

        let detail = data.measure_size_detailed();
        println!("Size detail: {:?}", detail);
        assert!(detail.bincode_size.is_some() || detail.json_size.is_some());
    }
}
