# RS-VSNA Project
**RUST Virtualized Storage and Network Access** is education open-source project to exchange data between devices on a VLAN.

# Dependencies
- `tungstenite` - websocket implementation
- `tokio` & `futures-util` - async implementation
- `walkdir` - directory traversal
- `serde` & `serde_json` - serialization
- `tracing` - logging

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
