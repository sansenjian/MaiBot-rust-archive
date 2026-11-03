//! MaiBot Core 的 Rust 重写主 crate（占位）。
//!
//! 远期模块规划（按绞杀顺序）：
//! - `llm_payload`：P3，token 估算 / 历史裁剪 / prompt 拼装（对齐 utils_model.py）
//! - `chat_pipeline`：消息 → 心情/意愿 → 回复生成的完整管道（远期，最后动）
//! - `plugin_host`：plugin_runtime Host 侧（msgpack-RPC server，拉起 Runner）
//! - `memory`：仅检索之外的增量优化（P4 已决定不做下沉）

pub const PLAN: &str = "P2 jieba-pyo3 → P3 llm-payload → chat pipeline (远期)";
