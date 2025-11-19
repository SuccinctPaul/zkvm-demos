[5 tools called]

你的需求本质上是在构建一个 **异构系统的标准化测评框架 (Heterogeneous Benchmarking Harness)**。

在工业界，解决“解析特异数据 -> 统一分析报告”最成熟的架构参考是 **OpenTelemetry (OTel)** 的 **Collector** 架构，以及 **ETL (Extract-Transform-Load)** 数据流设计模式。

以下是针对你的场景（zkVM Benchmark）的架构参考和建议：

### 1. 参考架构模式：OpenTelemetry Collector

OpenTelemetry 是云原生时代的可观测性标准，它解决的问题与你一模一样：**从不同厂商（AWS, GCP, Azure）收集格式各异的指标，转化为统一标准，然后输出给不同的分析后端。**

它的核心架构分为三层，我们可以直接照搬：

1.  **Receivers (接收器/提取层)**: 对应你的 `LogParser` + `sp1.toml [parsed_metrics]`。
    *   **职责**: 只负责“收”。不关心业务含义，只负责把非结构化的 Log 变成半结构化的 Map。
2.  **Processors (处理器/标准化层)**: 对应你的 `metrics.rs`。
    *   **职责**: 负责“洗”。这是最核心的层。它执行**重命名** (Mapping)、**单位统一** (Normalization)、**缺失值填充** (Filling) 和 **衍生计算** (Derivation)。
3.  **Exporters (导出器/报告层)**: 对应你的 `Reporter`。
    *   **职责**: 负责“发”。将标准化后的数据按需输出为 CSV/JSON/SQL。

---

### 2. 具体的代码架构设计 (The "Better Way")

为了统筹架构，建议引入一个中间层配置：**`Schema Mapping` (模式映射)**。

不要让 `metrics.rs` 去猜 "sp1_cycles" 是什么，而是在配置中明确它对应标准字段 `total_cycles`。

#### A. 配置文件升级 (Mapping Configuration)

在 `sp1.toml` 中明确“原始 Key”到“标准 Key”的映射关系：

```toml
[parsed_metrics]
# 原始解析 (Raw)
raw_cpu_cycles = 'BENCHMARK: total_cycles=(\d+)'
raw_duration = 'BENCHMARK: total_prove_time_s=([\d.]+)'

[metric_mapping]
# 映射规则: 标准字段 = 原始字段 (Transform)
# 这样 Rust 代码只认左边的标准 Key
total_cycles = "raw_cpu_cycles"
prove_time_s = "raw_duration"
```

#### B. Rust 核心架构 (Standardization Engine)

在 `metrics.rs` 中实现一个标准化的处理流程：

```rust
pub struct UnifiedMetrics {
    // 1. 标准字段 (First Class Citizens) - 用于横向强对比
    pub total_cycles: Option<u64>,
    pub total_time_s: Option<f64>,
    pub throughput_hz: Option<f64>,
    
    // 2. 扩展字段 (Second Class Citizens) - 用于纵向深挖
    // 存放那些某个 zkVM 独有但无法标准化的指标 (如 "air_constraints")
    pub extensions: HashMap<String, String>,
}

impl UnifiedMetrics {
    pub fn from_raw(raw: HashMap<String, String>, mapping: &HashMap<String, String>) -> Self {
        let mut unified = Self::default();
        
        // 1. Mapping (重命名)
        for (std_key, raw_key) in mapping {
            if let Some(val) = raw.get(raw_key) {
                unified.set_std_field(std_key, val);
            }
        }
        
        // 2. Derivation (衍生计算 - 你的 calculate_derived_metrics 逻辑)
        unified.derive_missing_metrics();
        
        unified
    }
    
    fn derive_missing_metrics(&mut self) {
        // 容错计算：如果有 Cycles 和 Time，自动算出 Throughput
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

### 3. 开源项目参考

虽然很难找到跟 zkVM 一模一样的 Bench 框架，但以下项目有极高的参考价值：

#### **1. Criterion.rs (Rust)**
*   **参考点**: 它的 **`Measurement` trait** 设计。
*   它允许不同的底层测量工具（CpuTime, WallTime, HardwareCounters）接入，但最后都统一转化为 `Duration` 和 `Throughput` 两个标准指标输出。
*   **借鉴**: 定义一个 `ZkVmMeasurement` trait，规定每个 zkVM 必须吐出哪些核心指标。

#### **2. Google Benchmark**
*   **参考点**: 它的 **Output Format (JSON/CSV)** 设计。
*   它定义了一套非常严格的 JSON Schema（`cpu_time`, `real_time`, `iterations`）。
*   **借鉴**: 你的 CSV 列名应该向 Google Benchmark 的标准靠拢，因为有很多现成的工具（如 Python pandas 脚本）是针对这种格式写的。

#### **3. Pantheon (by Datadog)**
*   **参考点**: 这是一个对比不同区块链/数据库性能的平台。
*   **架构**: 它的核心思想就是你想要的——针对不同后端写 Adapter，转化为统一 Schema，然后前端统一展示。

#### **4. Vector (by Datadog, Rust编写)**
*   **参考点**: 高性能可观测性数据管道。
*   **借鉴**: 它的 `transforms` 模块设计（如何用 Rust 枚举和 DSL 处理数据清洗）非常值得参考。

### 4. 总结建议

要实现“横向纵向对比”，**不要试图在 `LogParser` 里做统一**（那是无底洞）。

1.  **保持 `LogParser` 愚蠢**：只管正则提取，拿到什么存什么。
2.  **建立 `UnifiedMetrics` 层**：这是你的“世界语”。所有 zkVM 的数据必须翻译成这个结构体才能进入 Reporter。
3.  **配置即映射**：利用 TOML 配置来定义“方言”（Log Key）到“世界语”（Standard Key）的翻译规则。

这样，当你加入一个新的 zkVM 时，你只需要写

配置文件（翻译字典），而不需要修改 Rust 代码（语法规则）。