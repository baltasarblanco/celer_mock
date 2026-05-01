# Celer — Zero-Copy IPC Bridge (Receiver PoC)

![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)
![PoC](https://img.shields.io/badge/status-proof%20of%20concept-yellow?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-lightgrey?style=flat-square)

A proof‑of‑concept for the **Celer** Complex Event Processing (CEP) engine.  
This component works as a **receiver** in a zero‑copy Inter‑Process Communication (IPC) bridge, designed to move data between local Vanguard services (e.g., Aegis proxy) without touching the TCP/IP stack.

---

## ⚙️ How it works

Instead of sending data through loopback sockets, the bridge uses:

- **`memfd_create`** – allocates anonymous memory that remains in RAM, bypassing the filesystem.
- **Unix Domain Socket + `SCM_RIGHTS`** – the emitter passes the file descriptor of that memory via a control message.
- **`mmap`** – the receiver maps the same physical memory directly into its own address space.

Both processes read/write the same memory region with no data copying and no kernel‑network overhead.

---

## 📦 Tech stack

- **Rust** (stable)
- `nix` 0.28 – safe bindings for `memfd_create` and `SCM_RIGHTS`
- `memmap2` – memory mapping

---

## 🚀 Quick Start

```bash
# Terminal 1: Receiver
cargo run --release

# Terminal 2: Emitter (ejemplo mock)
./emitter_mock  # debe enviar FD via /tmp/celer_bridge.sock
```

### 🔗 Architecture
- **Socket**: `/tmp/celer_bridge.sock` (Unix domain)
- **FD Passing**: `SCM_RIGHTS` + `sendmsg()`/`recvmsg()`
- **Shared Memory**: `mmap()` zero-copy entre procesos
- **Throughput**: 28.97 Mpps | ~34.5 ns/packet (Ryzen, Pop!_OS 22.04)

### 📄 License
MIT