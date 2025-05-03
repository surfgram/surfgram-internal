<div align="center" style="max-width: 100%; overflow-x: auto; margin: 0 auto;">
  <div style="text-align: center; padding: 0 10px; width: 100%;">
    <h1 style="font-size: clamp(24px, 5vw, 36px); margin: 0 auto;">Surfgram Internal</h1>
  </div>

  <div style="display: flex; justify-content: center; margin: 20px auto; width: 100%;">
    <img src="./assets/surfgram_internal_logo.svg" alt="Surfgram Internal Logo" style="max-width: 100%; height: auto; display: block;">
  </div>

  <div style="text-align: center; margin: 20px auto; width: 100%;">
    <div style="display: inline-flex; flex-wrap: wrap; justify-content: center; gap: 8px;">
      <img src="https://img.shields.io/badge/Rust-1.70%2B-blue" alt="Rust 1.70+">
      <img src="https://img.shields.io/badge/Python-3.8%2B-blue" alt="Python 3.8+">
      <img src="https://img.shields.io/badge/License-MIT-green" alt="MIT License">
    </div>
  </div>
</div>

## Overview

Surfgram Internal is a high-performance Rust backend for Telegram bot development with Python bindings. Designed for maximum throughput and minimal latency.

## Key Features

- ⚡ **Blazing Fast** - Rust-powered core with async/await
- 🐍 **Python Friendly** - Clean PyO3 bindings
- 🔄 **Full Async** - Built on tokio runtime
- 🛡️ **Robust Error Handling** - Comprehensive error types
- 📦 **Lightweight** - Minimal dependencies

## Quick Start

1. Add to your Python project:

```bash
pip install surfgram-internal
```

2. Basic usage:

```python
from surfgram_internal import NativeClient
import asyncio

async def main():
    client = NativeClient("YOUR_BOT_TOKEN")
    response = await client.send_request(
        "getMe",
        "{}"
    )
    print(response)

asyncio.run(main())
```

## Contributing

We welcome contributions! Please see our [Contribution Guide](https://github.com/surfgram/surfgram-internal/CONTRIBUTING.md) for details.

## Support

🐞 [Issue Tracker](https://github.com/surfgram/surfgram-internal/issues)  
💬 [Discussion Forum](https://github.com/surfgram/surfgram-internal/discussions)  
📡 [Telegram Channel](https://t.me/surfgram_support)

## License

MIT License. See [LICENSE](https://github.com/surfgram/surfgram-internal/blob/main/LICENSE) for details.
