# VSNA Project
**Virtualized Storage and Network Access** is education open source project, realized on Rust{tokio, tungstenite} and CPP{boost} to exchange data between devices on a VLAN.

At the moment, the Rust implementation is ready for sending files between *client* and *server* & C++ impl is in dev...

# Run Rust (MVP)
To run one of Rust realization (*server*|*client*) you need to push `cargo run` cmd in one of the (*server*|*client*) dir.

# Run CPP (in DEV)
To run project from scratch, you should goto `cpp_server/` and run `cmake -G Ninja -B build`. After that, move to `build/` and run `ninja`.