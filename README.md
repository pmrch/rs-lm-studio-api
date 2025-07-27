[![github]](https://github.com/pmrch/rs-lm-studio-api-extended)&ensp;
[![crates-io]](https://crates.io/crates/lm-studio-api-extended)&ensp;
[![docs-rs]](https://docs.rs/lm-studio-api)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# 🧠 LM Studio API (Extended)

This crate extends [`rs-lm-studio-api`](https://github.com/fuderis/rs-lm-studio-api) 
by [Bulat Sh.], adding **embedding support**.
Use this to interact with locally hosted LM Studio models — send prompts, receive 
completions (streaming or not), and now... generate embeddings as well!


---

## Features

- ✅ Chat completions (fully async, with or without streaming)
- ✅ Embedding support (POST /v1/embeddings, sync-style usage)
- ✅ Minimal setup — just point to your LM Studio server
- ✅ Customizable prompts, model selection, and context memory

---

## Quick Start

Add it to your `Cargo.toml`:

```toml
lm-studio-api-extended = "0.1.0"
```

## Examples
Check out the **examples/** directory for working demos. You can run them like this:

```bash
cargo run --example embedding_test
cargo run --example chat_completion
cargo run --example chat_completion_streaming
```

## Licensing:

This project is licensed under the MIT License, inheriting from the original work by Bulat Sh.
See LICENSE for details.


## Feedback:

You can contact the original creator via GitHub or send a message to their Telegram [@fuderis](https://t.me/fuderis).  
This library is constantly evolving, and they welcome your suggestions and feedback.
