# yfinance-rmcp

MCP server in Rust that exposes Yahoo Finance data over stdio for AI agents and MCP clients.

## Tools

| Tool | Description |
|------|-------------|
| `get_historical_stock_prices` | OHLCV history. Params: `ticker`, `period` (1d, 1mo, 1y, …), `interval` (1d, 1wk, …). |
| `get_stock_quote` | Latest quote for a ticker. Param: `ticker`. |
| `search_ticker` | Search symbols by name. Param: `query`. |

## Build & run

Requires [Rust](https://www.rust-lang.org/). WSL (Ubuntu) recommended on Windows.

```bash
cargo build --release
./target/release/yfinance-rmcp
```

The server talks over stdio (JSON-RPC). Point your MCP client at this binary.

## Tests

From project root (in WSL if on Windows):

```bash
cargo build && cargo test
```

Or `bash scripts/test-wsl.sh`.

## Cursor IDE

The repo includes `.cursor/mcp.json` so Cursor can use the server as a tool.

**Recommended on Windows: use Cursor in WSL** so the MCP server runs in the same environment and the handshake works.

1. In Cursor, connect to WSL: **Ctrl+Shift+P** → “WSL: Connect to WSL” (or “Remote-WSL: Connect to WSL”), or from a WSL terminal run `cursor .` in the project folder.
2. Open the project from the WSL filesystem (e.g. `/mnt/c/Users/.../yfinance_rmcp` or the path shown in WSL).
3. In WSL, build once: `cargo build --release`.
4. Ensure `.cursor/mcp.json` uses the **Linux** path to the binary (no `wsl` wrapper), e.g. `"command": "/mnt/c/Users/SDS/Documents/projects/public/yfinance_rmcp/target/release/yfinance-rmcp"`, `"args": []`. Adjust the path if your project lives elsewhere.
5. Restart Cursor (or reload the window) and enable `yfinance_rmcp` under Settings → Tools & MCP.

Then you can ask e.g. “What’s the current price of AAPL?” or “Search for Microsoft ticker.”

**Alternative (Cursor on Windows, no WSL):** Install Rust on Windows, run `cargo build --release` in the project in PowerShell, and in `.cursor/mcp.json` set `"command"` to the full Windows path to `yfinance-rmcp.exe` (e.g. `C:\\Users\\...\\target\\release\\yfinance-rmcp.exe`), `"args": []`.

## Other clients

**Claude Desktop** — Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "yfinance_rmcp": {
      "command": "/path/to/target/release/yfinance-rmcp",
      "args": []
    }
  }
}
```

**MCP Inspector:** `npx @modelcontextprotocol/inspector` and point it at the binary.
