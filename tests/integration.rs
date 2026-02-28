//! Integration test: spawn the MCP server binary and exchange JSON-RPC over stdio.
//! Run with: cargo test --test integration (requires built binary; run from WSL Ubuntu for full compatibility).

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn server_exe() -> std::path::PathBuf {
    if let Ok(path) = env::var("CARGO_BIN_EXE_yfinance_rmcp") {
        return std::path::PathBuf::from(path);
    }
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(manifest_dir).join("target/debug/yfinance-rmcp")
}

#[test]
fn server_starts_and_responds_to_initialize() {
    let exe = server_exe();
    if !exe.exists() {
        eprintln!("Binary not found at {:?}; run cargo build first", exe);
        return;
    }

    let mut child = Command::new(&exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "integration-test", "version": "0.1.0" }
        }
    });
    writeln!(stdin, "{}", init).expect("write init");
    stdin.flush().expect("flush");

    let mut line = String::new();
    reader.read_line(&mut line).expect("read init response");
    let line = line.trim_end_matches('\n').trim_end_matches('\r');
    let resp: serde_json::Value = serde_json::from_str(line).expect("parse init response");

    assert!(resp.get("result").is_some(), "initialize should return result: {}", resp);
    let result = &resp["result"];
    let name = result["serverInfo"]["name"]
        .as_str()
        .or_else(|| result["server_info"]["name"].as_str())
        .expect("server name in result");
    assert_eq!(name, "yfinance_rmcp");
}

#[test]
fn tools_list_returns_three_tools() {
    let exe = server_exe();
    if !exe.exists() {
        eprintln!("Binary not found at {:?}; run cargo build first", exe);
        return;
    }

    let mut child = Command::new(&exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "integration-test", "version": "0.1.0" }
        }
    });
    writeln!(stdin, "{}", init).expect("write init");
    stdin.flush().expect("flush");

    let mut line = String::new();
    reader.read_line(&mut line).expect("read init response");

    let notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });
    writeln!(stdin, "{}", notif).expect("write initialized");
    stdin.flush().expect("flush");

    let list = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    });
    writeln!(stdin, "{}", list).expect("write tools/list");
    stdin.flush().expect("flush");

    line.clear();
    reader.read_line(&mut line).expect("read tools/list response");
    let line = line.trim_end_matches('\n').trim_end_matches('\r');
    let resp: serde_json::Value = serde_json::from_str(line).expect("parse tools/list response");

    let tools = resp["result"]["tools"].as_array().expect("tools array");
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(
        names.contains(&"get_historical_stock_prices") && names.contains(&"get_stock_quote") && names.contains(&"search_ticker"),
        "expected 3 tools, got: {:?}",
        names
    );
}
