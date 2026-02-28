# yfinance-rmcp

A [Model Context Protocol](https://modelcontextprotocol.io/) (MCP) server written in Rust that provides stock market data via stdio. It is designed for AI agents and MCP clients that need real-time quotes, historical OHLCV series, and ticker search.

---

## Overview

**What it is** — A standalone MCP server that exposes three tools: stock quotes, historical price data (OHLCV), and symbol search. It runs as a single process and communicates over stdio using JSON-RPC.

**Purpose** — To supply agents (e.g. ChatGPT, Claude, Gemini) and other MCP clients with stock data so they can answer questions about prices, history, and symbols, or to act as a reusable data source from Python scripts, dashboards, or other tooling.

**When to use it** — Use this server when you need programmatic access to stock data through MCP; when you want a single, lightweight Rust binary that multiple clients can connect to; or when you prefer stdio-based transport (no HTTP) for local integration.

---

## Tools

| Tool | Description |
|------|-------------|
| `get_historical_stock_prices` | Returns historical OHLCV (Date, Open, High, Low, Close, Volume). Parameters: `ticker`, `period` (`1d`, `5d`, `1mo`, `3mo`, `6mo`, `1y`, `2y`, `5y`, `10y`, `ytd`, `max`), `interval` (e.g. `1d`, `1wk`, `1mo`). |
| `get_stock_quote` | Returns the latest quote for the given ticker (open, high, low, close, volume). Parameter: `ticker`. |
| `search_ticker` | Searches for symbols by name or keyword. Parameter: `query`. |

---

## Requirements

- [Rust](https://www.rust-lang.org/) (stable) for building.
- Building and running under WSL (Ubuntu) is recommended on Windows.

---

## Build and run

Install Rust if needed (one-time):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

From the project root:

```bash
cargo build --release
./target/release/yfinance-rmcp
```

The server uses stdio; any MCP client (e.g. MCP Inspector) should start this binary and communicate via stdin/stdout.

---

## Configuration

### Claude Desktop

Add the server to `claude_desktop_config.json`. Adjust the path to the built binary as needed.

**Linux / WSL:**

```json
{
  "mcpServers": {
    "yfinance_rmcp": {
      "command": "/path/to/yfinance_rmcp/target/release/yfinance-rmcp",
      "args": []
    }
  }
}
```

**Windows (native):**

```json
{
  "mcpServers": {
    "yfinance_rmcp": {
      "command": "C:\\path\\to\\yfinance_rmcp\\target\\release\\yfinance-rmcp.exe",
      "args": []
    }
  }
}
```

### MCP Inspector

To test the server:

```bash
npx @modelcontextprotocol/inspector
```

Configure the inspector to start the `yfinance-rmcp` binary (path to the built executable), then invoke the tools from the UI.

### Python / other MCP clients

Start the `yfinance-rmcp` process and use your MCP client to call `list_tools` and `call_tool` with the tool names and parameters listed in the Tools table above.
