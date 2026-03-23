# Celer 👁️‍🗨️ - Zero-Copy IPC Bridge (Receiver PoC)

This repository contains the **Proof of Concept (PoC)** for the Phase 1 architecture of **Celer**, an ultra-low latency Complex Event Processing (CEP) engine. 

This specific component acts as the **Sentinel (Receiver)** in a Zero-Copy Inter-Process Communication (IPC) bridge, designed to bypass the Linux TCP/IP networking stack entirely when communicating with other local nodes (like the Aegis L4 Proxy).

## ⚡ Architecture Highlights

Instead of serializing data and sending it through loopback network sockets, this architecture uses:
* **Anonymous RAM Allocation:** The emitter allocates memory that bypasses the virtual file system entirely using `memfd_create`.
* **Kernel Control Messages:** The File Descriptor (FD) of that memory is injected into a Unix Domain Socket (`AF_UNIX`) using the `SCM_RIGHTS` control message payload.
* **Instantaneous Memory Mapping:** Celer receives the raw FD from the kernel and uses `mmap` to map that exact physical memory block into its own userspace. 

**Result:** Both processes can read and write to the same memory segment with $O(1)$ complexity, zero network overhead, and zero copy (`sendfile` / `mmap` semantics).

## 🛠️ Tech Stack
* **Language:** Rust 🦀
* **Syscalls & OS Interfaces:** `nix` crate (version 0.28.0 for stable `mman` and `socket` APIs).
* **Memory Management:** `memmap2`.

## 🚀 How to Run the Bridge

Because this is a decoupled architecture, the Receiver (Celer) must be active and listening before the Emitter (Aegis) attempts to pass the File Descriptor.

**1. Start the Sentinel (Terminal 1):**
```bash
git clone [https://github.com/TuUsuario/celer_mock.git](https://github.com/TuUsuario/celer_mock.git)
cd celer_mock
cargo run
```
Celer will open `/tmp/celer_bridge.sock` and wait for the `SCM_RIGHTS` message.

## Trigger the Payload (Terminal 2):

In a separate terminal, run the emitter (aegis_mock) to allocate the RAM, write the data, and shoot the FD through the Unix Socket.

**Expected Output:**

```bash
👁️‍🗨️ [CELER] Centinela activo. Escuchando en el socket temporal...
👁️‍🗨️ [CELER] Conexión entrante detectada. Extrayendo paquetes...
👁️‍🗨️ [CELER] EXTRACCIÓN EXITOSA. Leyendo memoria de Aegis: 'HOLA CELER: EL PUENTE ESTA ABIERTO Y ASEGURADO'
```

### 📤 PROTOCOLO DE SUBIDA FINAL

```bash
Una vez que guardes ese texto en el archivo `README.md` (y cambies el "TuUsuario" en el link del emisor por tu usuario real), ejecuta esta ráfaga en la terminal para empujarlo a la nube:
```
