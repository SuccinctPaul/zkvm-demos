Your requirement is essentially building a **Heterogeneous System Standardization Benchmarking Framework (Heterogeneous Benchmarking Harness)**.

In the industry, the most mature architectural reference for solving "parsing specific data -> unified analysis report" is the **OpenTelemetry (OTel)** **Collector** architecture, and the **ETL (Extract-Transform-Load)** data flow design pattern.

Here are the architectural references and suggestions for your scenario (zkVM Benchmark):

### 1. Reference Architecture Pattern: OpenTelemetry Collector

OpenTelemetry is the observability standard in the cloud-native era, and the problem it solves is exactly the same as yours: **collecting metrics in different formats from different vendors (AWS, GCP, Azure), transforming them into a unified standard, and then outputting them to different analysis backends.**

Its core architecture is divided into three layers, which we can directly copy:

1.  **Receivers (Extraction Layer)**: Corresponds to your `LogParser` + `sp1.toml [parsed_metrics]`.
    *   **Responsibility**: Only responsible for "receiving". Does not care about business meaning, only responsible for turning unstructured Logs into semi-structured Maps.
2.  **Processors (Standardization Layer)**: Corresponds to your `metrics.rs`.
    *   **Responsibility**: Responsible for "cleaning". This is the core layer. It performs **Renaming** (Mapping), **Unit Unification** (Normalization), **Missing Value Filling** (Filling), and **Derived Calculation** (Derivation).
3.  **Exporters (Reporting Layer)**: Corresponds to your `Reporter`.
    *   **Responsibility**: Responsible for "sending". Output standardized data as CSV/JSON/SQL as needed.

---

### 2. Specific Code Architecture Design (The "Better Way")

To coordinate the architecture, it is recommended to introduce a middle layer configuration: **`Schema Mapping`**.

Don't let `metrics.rs` guess what "sp1_cycles" is, but explicitly define in the configuration that it corresponds to the standard field `total_cycles`.

#### A. Configuration File Upgrade (Mapping Configuration)

Explicitly define the mapping relationship from "Raw Key" to "Standard Key" in `sp1.toml`:

```toml
[parsed_metrics]
# Raw Parsing
raw_cpu_cycles = 'BENCHMARK: total_cycles=(\d+)'
raw_duration = 'BENCHMARK: total_prove_time_s=([\d.]+)'

[metric_mapping]
# Mapping Rules: Standard Field = Raw Field (Transform)
# In this way, Rust code only recognizes the standard Key on the left
total_cycles = "raw_cpu_cycles"
prove_time_s = "raw_duration"
```

#### B. Rust Core Architecture (Standardization Engine)

Implement a standardized processing flow in `metrics.rs`:

```rust
pub struct UnifiedMetrics {
    // 1. Standard Fields (First Class Citizens) - For strong horizontal comparison
    pub total_cycles: Option<u64>,
    pub total_time_s: Option<f64>,
    pub throughput_hz: Option<f64>,
    
    // 2. Extended Fields (Second Class Citizens) - For vertical deep diving
    // Store metrics unique to a certain zkVM but cannot be standardized (e.g., "air_constraints")
    pub extensions: HashMap<String, String>,
}

impl UnifiedMetrics {
    pub fn from_raw(raw: HashMap<String, String>, mapping: &HashMap<String, String>) -> Self {
        let mut unified = Self::default();
        
        // 1. Mapping (Renaming)
        for (std_key, raw_key) in mapping {
            if let Some(val) = raw.get(raw_key) {
                unified.set_std_field(std_key, val);
            }
        }
        
        // 2. Derivation (Derived Calculation - your calculate_derived_metrics logic)
        unified.derive_missing_metrics();
        
        unified
    }
    
    fn derive_missing_metrics(&mut self) {
        // Fault-tolerant calculation: If there are Cycles and Time, automatically calculate Throughput
        if self.throughput_hz.is_none() {
            if let (Some(c), Some(t)) = (self.total_cycles, self.total_time_s) {
                if t > 0.0 {
                    self.throughput_hz = Some(c as f64 / t);
                }
            }
        }
    }
}
```

---

### 3. Open Source Project References

Although it is difficult to find a Bench framework exactly the same as zkVM, the following projects have extremely high reference value:

#### **1. Criterion.rs (Rust)**
*   **Reference Point**: Its **`Measurement` trait** design.
*   It allows different underlying measurement tools (CpuTime, WallTime, HardwareCounters) to access, but finally unifies them into `Duration` and `Throughput` two standard metric outputs.
*   **Borrow**: Define a `ZkVmMeasurement` trait, stipulating which core metrics each zkVM must output.

#### **2. Google Benchmark**
*   **Reference Point**: Its **Output Format (JSON/CSV)** design.
*   It defines a very strict JSON Schema (`cpu_time`, `real_time`, `iterations`).
*   **Borrow**: Your CSV column names should align with Google Benchmark's standards, because there are many ready-made tools (such as Python pandas scripts) written for this format.

#### **3. Pantheon (by Datadog)**
*   **Reference Point**: This is a platform for comparing the performance of different blockchains/databases.
*   **Architecture**: Its core idea is what you want - write Adapters for different backends, transform into a unified Schema, and then display uniformly on the frontend.

#### **4. Vector (by Datadog, written in Rust)**
*   **Reference Point**: High-performance observability data pipeline.
*   **Borrow**: Its `transforms` module design (how to use Rust enums and DSL to process data cleaning) is very worth referencing.

### 4. Summary Suggestions

To achieve "horizontal and vertical comparison", **do not try to unify in `LogParser`** (that is a bottomless pit).

1.  **Keep `LogParser` stupid**: Only responsible for regex extraction, save what you get.
2.  **Establish `UnifiedMetrics` layer**: This is your "Esperanto". All zkVM data must be translated into this structure to enter the Reporter.
3.  **Configuration as Mapping**: Use TOML configuration to define the translation rules from "Dialect" (Log Key) to "Esperanto" (Standard Key).

In this way, when you add a new zkVM, you only need to write
configuration files (translation dictionaries), and do not need to modify Rust code (syntax rules).
