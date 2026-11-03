//! P2：jieba-rs 的 PyO3 封装，API 对齐 Python `jieba.cut / lcut`。
//!
//! 用法（对齐 Python）：
//!   import jieba_pyo3
//!   jieba_pyo3.lcut("麦麦是最可爱的机器人")  # -> ["麦麦", "是", "最", "可爱", "的", "机器人"]
//!
//! 切换点：Core 的 A_memorix 分词调用处从 `jieba.lcut` 改为 `jieba_pyo3.lcut`。
//! 验收：同一语料库上 cut 结果与 Python jieba 一致（抽样 diff）。

use pyo3::prelude::*;

/// 对齐 jieba.lcut(text, cut_all=False)
#[pyfunction]
fn lcut(text: &str) -> Vec<String> {
    jieba_rs::Jieba::new()
        .cut(text, false)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// 对齐 jieba.cut(text, cut_all=False)，返回生成器
#[pyfunction]
fn cut(text: String) -> Vec<String> {
    lcut(&text)
}

#[pymodule]
fn jieba_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lcut, m)?)?;
    m.add_function(wrap_pyfunction!(cut, m)?)?;
    Ok(())
}
