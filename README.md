# crimson-river-8122
MCP / Rust / Haikunator

# poem-mcpserver

https://github.com/poem-web/poem/blob/c4600f723f6130e7f946283286c13f0ed4b6562c/examples/mcpserver/counter/src/main.rs

# Usage

```jsonc
// .vscode/mcp.json
{
    "servers": {
        "my-mcp-server-ad12adc5": {
            "type": "stdio",
            "command": "/workspaces/crimson-river-8122/target/release/crimson-river-8122",
            "args": []
        }
    }
}
```

```console
# Copilot Chat Agent
haikunateを１０回実行してマークダウンのリストにして
```

# Development

## Enviroment

```console
$ rustc --version
rustc 1.86.0 (05f9846f8 2025-03-31)
$ cargo --version
cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

## Release

```console
$ cargo build --release
```
