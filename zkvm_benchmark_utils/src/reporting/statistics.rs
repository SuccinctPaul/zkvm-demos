// Statistical Analysis for Benchmark Results
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub stddev: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub range: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub ci_95_lower: f64,
    pub ci_95_upper: f64,
    pub outliers: Vec<usize>,
}

/// Calculate statistics from a series of measurements
pub fn calculate_statistics(mut values: Vec<f64>, outlier_threshold: f64) -> Option<Statistics> {
    if values.is_empty() {
        return None;
    }

    let count = values.len();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Basic stats
    let min = values[0];
    let max = values[count - 1];
    let range = max - min;
    let mean = values.iter().sum::<f64>() / count as f64;

    // Median
    let median = if count % 2 == 0 {
        (values[count / 2 - 1] + values[count / 2]) / 2.0
    } else {
        values[count / 2]
    };

    // Variance and standard deviation
    let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / count as f64;
    let stddev = variance.sqrt();

    // Percentiles
    let p50 = percentile(&values, 50.0);
    let p75 = percentile(&values, 75.0);
    let p90 = percentile(&values, 90.0);
    let p95 = percentile(&values, 95.0);
    let p99 = percentile(&values, 99.0);

    // 95% Confidence Interval (assuming normal distribution)
    let se = stddev / (count as f64).sqrt();
    let z_95 = 1.96; // Z-score for 95% CI
    let ci_95_lower = mean - z_95 * se;
    let ci_95_upper = mean + z_95 * se;

    // Outlier detection using Z-score
    let outliers = detect_outliers(&values, mean, stddev, outlier_threshold);

    Some(Statistics {
        count,
        mean,
        median,
        stddev,
        variance,
        min,
        max,
        range,
        p50,
        p75,
        p90,
        p95,
        p99,
        ci_95_lower,
        ci_95_upper,
        outliers,
    })
}

/// Calculate percentile
fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0) * (sorted_values.len() - 1) as f64;
    let lower = idx.floor() as usize;
    let upper = idx.ceil() as usize;
    let weight = idx - lower as f64;

    if lower == upper {
        sorted_values[lower]
    } else {
        sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
    }
}

/// Detect outliers using Z-score method
fn detect_outliers(values: &[f64], mean: f64, stddev: f64, threshold: f64) -> Vec<usize> {
    if stddev == 0.0 {
        return Vec::new();
    }

    values
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| {
            let z_score = (value - mean).abs() / stddev;
            if z_score > threshold {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

/// Remove outliers from dataset
pub fn remove_outliers(values: Vec<f64>, threshold: f64) -> Vec<f64> {
    if values.is_empty() {
        return values;
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let stddev = variance.sqrt();

    if stddev == 0.0 {
        return values;
    }

    values
        .into_iter()
        .filter(|&value| {
            let z_score = (value - mean).abs() / stddev;
            z_score <= threshold
        })
        .collect()
}

/// Calculate coefficient of variation (relative standard deviation)
pub fn coefficient_of_variation(mean: f64, stddev: f64) -> f64 {
    if mean == 0.0 {
        0.0
    } else {
        (stddev / mean) * 100.0
    }
}

/// Detect performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    pub metric_name: String,
    pub baseline_mean: f64,
    pub current_mean: f64,
    pub change_percent: f64,
    pub is_regression: bool,
    pub threshold_percent: f64,
    pub confidence: f64,
}

pub fn detect_regression(
    metric_name: String,
    baseline: &Statistics,
    current: &Statistics,
    threshold_percent: f64,
) -> RegressionAnalysis {
    let change_percent = ((current.mean - baseline.mean) / baseline.mean) * 100.0;
    let is_regression = change_percent > threshold_percent;

    // Calculate confidence based on overlap of confidence intervals
    let overlap = if current.ci_95_lower > baseline.ci_95_upper
        || current.ci_95_upper < baseline.ci_95_lower
    {
        0.0 // No overlap = high confidence
    } else {
        1.0 // Overlap = lower confidence
    };

    let confidence = 1.0 - overlap;

    RegressionAnalysis {
        metric_name,
        baseline_mean: baseline.mean,
        current_mean: current.mean,
        change_percent,
        is_regression,
        threshold_percent,
        confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_statistics() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = calculate_statistics(values, 2.0).unwrap();

        assert_eq!(stats.count, 5);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_percentiles() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let stats = calculate_statistics(values, 2.0).unwrap();

        assert_eq!(stats.p50, 5.5);
        assert_eq!(stats.p90, 9.1);
    }

    #[test]
    fn test_outlier_detection() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is outlier
        let stats = calculate_statistics(values, 2.0).unwrap();

        assert!(!stats.outliers.is_empty());
    }

    #[test]
    fn test_remove_outliers() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0];
        let cleaned = remove_outliers(values, 2.0);

        assert_eq!(cleaned.len(), 5);
        assert!(!cleaned.contains(&100.0));
    }

    #[test]
    fn test_regression_detection() {
        let baseline = Statistics {
            count: 5,
            mean: 100.0,
            median: 100.0,
            stddev: 5.0,
            variance: 25.0,
            min: 95.0,
            max: 105.0,
            range: 10.0,
            p50: 100.0,
            p75: 102.5,
            p90: 104.0,
            p95: 104.5,
            p99: 104.9,
            ci_95_lower: 95.0,
            ci_95_upper: 105.0,
            outliers: vec![],
        };

        let current = Statistics {
            mean: 115.0, // 15% slower
            ci_95_lower: 110.0,
            ci_95_upper: 120.0,
            ..baseline.clone()
        };

        let regression = detect_regression("prove_time".to_string(), &baseline, &current, 10.0);

        assert!(regression.is_regression);
        assert_eq!(regression.change_percent, 15.0);
    }

    #[test]
    fn test_coefficient_of_variation() {
        let cv = coefficient_of_variation(100.0, 5.0);
        assert_eq!(cv, 5.0); // 5% CV
    }
}
