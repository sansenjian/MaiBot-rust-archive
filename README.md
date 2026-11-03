# MaiBot-rust

MaiBot Core 的 Rust 重写（绞杀者模式）。与适配器仓库配合，逐步替换原版 Python 麦麦的各个模块。

## 生态位

```
SnowLuma / NapCat ⇄ MaiBot-SnowLuma-Adapter-rust(已完成) ⇄ MaiBot-rust(本仓库,进行中) ⇄ LLM API
```

- 适配器仓库：https://github.com/sansenjian/MaiBot-SnowLuma-Adapter-rust （对 Core 说 plugin RPC / socketio）
- 原版 Core：https://github.com/Mai-with-u/MaiBot （Python，重写完成前继续服役）
- 协议文档（Core⇄适配器的全部契约）：适配器仓库 `docs/event-schema.md`

## 路线图（绞杀顺序：边缘 → 大脑）

| 阶段 | 内容 | 形态 | 状态 |
|---|---|---|---|
| P1 | 消息适配器 | 独立进程 | ✅ 完成（Adapter-rust 仓库，CI 全绿） |
| P2 | 分词（jieba-rs） | PyO3 扩展，Core 改一行 import 切换 | 🚧 骨架就位 |
| P3 | LLM payload 构建（token 估算、历史裁剪、prompt 拼装） | PyO3 库 | 未开始 |
| P4 | 记忆检索下沉 | —— | ❌ 已决定不做（保持 Python 兼容） |
| P5+ | chat 管道 / plugin Host / 人格引擎 | 待定 | 远期 |

原则：每一步都可独立验收、行为对齐原版、原版 Core 全程可运行可回退。

## 仓库结构

```
crates/
├── maibot-core/    # Core 本体（远期主 crate，现为占位）
└── jieba-pyo3/     # P2：jieba-rs 的 PyO3 封装，API 对齐 Python jieba.cut/lcut
```

## License

GPL-3.0（与 MaiBot 生态一致）
