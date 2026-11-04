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
| P2 | 分词（jieba-rs） | PyO3 扩展，Core 改一行 import 切换 | ✅ 可用（CI 产 wheel + 对齐 diff） |
| P3 | LLM payload 热路径（图片压缩 / ContextItem 投影 / token 计数） | PyO3 库 | ✅ 可用（CI 产 wheel + 行为 diff） |
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

## P2 用法（jieba-pyo3）

```bash
# 从 CI 产物（Actions → jieba_pyo3-wheel）或本地构建获取 wheel：
pip install maturin && cd crates/jieba-pyo3 && maturin build --release
pip install target/wheels/*.whl
```

```python
# 词典：vendor 自 CPython jieba 官方 dict.txt（MIT，34.9 万词）+ HMM 新词发现（移植 jieba finalseg 概率表），与 Python 版行为对齐
import jieba_pyo3
jieba_pyo3.lcut("麦麦是最可爱的机器人")

j = jieba_pyo3.Jieba()
j.load_dict("user_dict.txt")   # 每行 "词 [词频]"
j.add_word("麦麦", 1000)
j.lcut("麦麦来了")
```

Core 切换（A_memorix 分词调用点）：`import jieba` → `import jieba_pyo3 as jieba`。
验收：CI 中与 CPython jieba 的语料对齐 diff（门槛 85%，当前实测见 Actions 日志）。

## P3 用法（llm-payload）

```python
import llm_payload, json

# 1) 对齐 utils.py::compress_messages：压缩 OpenAI 消息里的 data-uri 图片
msgs = json.dumps([{"role": "user", "content": [
    {"type": "text", "text": "看图"},
    {"type": "image_url", "image_url": {"url": "data:image/png;base64,...."}}]}])
compressed = json.loads(llm_payload.compress_message_images(msgs))  # target_size 默认 1MB

# 2) ContextItem（context_item.py 语义）→ OpenAI messages
items = [{"type": "message", "role": "user", "parts": [{"type": "text", "text": "你好"}]}]
messages = json.loads(llm_payload.items_to_messages(json.dumps(items, ensure_ascii=False)))

# 3) cl100k_base token 计数（与 tiktoken 逐 token 一致）
n = llm_payload.count_tokens("麦麦是最可爱的机器人")
```

⚠️ 范围说明：现代 MaiBot（≥v1.3）的 utils_model.py 已重构为 LLM 调度编排层（网络 IO 为主，
无 Rust 化价值）；"token 估算/历史裁剪/prompt 拼装"旧假设来自更早版本。P3 对齐的是当前
真实的 CPU 热路径：PIL 图片压缩管线、ContextItem 投影、token 计数。动图 GIF 压缩 v1 透传。
