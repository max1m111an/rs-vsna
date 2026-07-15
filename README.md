# RS-VSNA Project
**RUST Virtualized Storage and Network Access** is education open-source CLI project to exchange data between devices on a VLAN.

# Dependencies
- `tungstenite` - websocket implementation
- `tokio` & `futures-util` - async implementation
- `walkdir` - directory traversal
- `serde` & `serde_json` - serialization
- `tracing` - logging
- `clap` - command line argument parsing

# To run
To run one of Rust realization (*server*|*client*) you need to push `cargo run` cmd in one of the (*server*|*client*) dir.

If you have not `cargo` yet -> [install it](https://doc.rust-lang.org/cargo/getting-started/installation.html)

**Or** download [Rust toolchain](https://www.rust-lang.org/tools/install) and `cargo` will be installed automatically.

**Or** install last release from repository.

# Instructions
1. **Server**
   - just running and accept all connections from VLAN (logging parallel)

2. **Client**
   - connect to the server using websocket protocol
   - send and receive data (with path tree visualization)

3. **CLI (client)**

|Short, Long name|Description|Default value|
|---|---|---|
| `-h`, `--help` | show help message |-|
| `-p`, `--port <port>` | set port | 8080 |
| `-i`, `--ip <ip>` | set server address | 0.0.0.0 |
| `-d`, `--dir <path>` | set client path | <current directory> |
| `-a`, `--auto-sync` | enable auto sync between client and server | false |
| `-c`, `--config <path>` | set config file path | none |

# Run Examples
1. With CLI flags
```bash
.\client.exe -p 8080 -d /path/to/client -i 192.168.0.1
```

2. With config
```bash
.\client.exe -c .\__config__.example.json
```

3. As cargo
```bash
cargo run --bin client -- -p 8080 -d D:\ -i 192.168.0.1
```