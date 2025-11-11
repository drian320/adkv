# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Apex Legends external memory manipulation tool using QEMU/KVM DMA techniques with memflow library. The project consists of two main components:

1. **apex_dma** (Linux host) - C++ server that reads game memory via memflow and writes to shared memory
2. **apex_linux_client** (Linux host) - Rust+egui GUI client that reads from shared memory and provides settings control

Note: The Windows guest Client/Overlay components are now deprecated. The system uses POSIX shared memory for IPC instead of network sockets.

## Architecture

### Memory Access Layer
- Uses **memflow** library (Rust FFI) for cross-VM memory access via KVM connector
- `memory.h`/`memory.cpp` - Wrapper around memflow FFI, provides pattern scanning and memory operations
- `Memory` class maintains connector and OS/process instances
- Pattern scanning implementation for signature-based address resolution

### Core Components
- `apex_dma.cpp` - Main server loop that reads game state and writes to shared memory
- `shared_memory.h` - POSIX shared memory wrapper (SharedMemoryWriter class)
- `Game.h`/`Game.cpp` - Entity management, game state structures, bone calculations
- `offsets.h` - Game structure offsets (updated per game version)
- `Math.h`/`Math.cpp` - Vector math, world-to-screen transformations
- `prediction.h` - Projectile prediction calculations

### Shared Memory Communication
- **POSIX Shared Memory** (`/dev/shm/apex_dma_shared`) - 1MB shared memory region
- **SharedData structure** - Contains game state, player data, spectator info, and settings
- **Bidirectional**: apex_dma writes game data, apex_linux_client writes settings
- Data structures defined in both `shared_memory.h` (C++) and `types.rs` (Rust) with `#[repr(C)]`

### Linux GUI Client (apex_linux_client)
- **Rust + egui** - Modern immediate-mode GUI framework
- `src/main.rs` - Application entry point
- `src/app.rs` - Main GUI application logic
- `src/shm.rs` - Shared memory client using memmap2
- `src/types.rs` - Rust definitions matching C++ SharedData structures

## Build Commands

### Building apex_dma (Linux Host)

```bash
# Build memflow FFI library first, then compile C++ server
cd apex_dma
./build.sh
```

Manual build steps:
```bash
cd apex_dma/memflow_lib/memflow-ffi/
cargo build --release
cd ../../
make
```

Output: `apex_dma/build/apex_dma`

### Running apex_dma
```bash
cd apex_dma
./run_apex_dma.sh
```
Or manually:
```bash
sudo -E LD_LIBRARY_PATH=./memflow_lib/target/release:$LD_LIBRARY_PATH ./build/apex_dma
```
Note: Requires root for KVM/memflow connector access.

### Building apex_linux_client (Linux GUI)

```bash
cd apex_linux_client
./build.sh
```

Manual build:
```bash
cd apex_linux_client
cargo build --release
```

Output: `apex_linux_client/target/release/apex_linux_client`

### Running apex_linux_client
```bash
cd apex_linux_client
cargo run --release
# Or directly:
./target/release/apex_linux_client
```

Note: apex_dma must be running first to create the shared memory region.

## Development Notes

### Resolution Configuration
Screen resolution is hardcoded and must be changed manually:
- Default: 2560×1440
- For 1920×1080: Search for "2560" and "1440" in `apex_dma.cpp` and modify

### Game Version Compatibility
Current offsets for: v3.0.1.29 (Steam & Origin)
When game updates, offsets in `offsets.h` and `offsets.ini` must be updated.

### memflow Library
- Located in `apex_dma/memflow_lib/` subdirectory
- Contains memflow core, memflow-ffi, memflow-win32, and memflow-bench crates
- FFI layer provides C-compatible interface for C++ integration
- See `memflow_lib/memflow-ffi/` for C++ bindings header
- Requires `LD_LIBRARY_PATH` to point to `memflow_lib/target/release` at runtime

### Shared Memory Protocol
- apex_dma creates `/dev/shm/apex_dma_shared` with 0666 permissions
- Uses POSIX `shm_open()`, `mmap()` for shared memory
- SharedData structure (defined in both C++ and Rust with matching layouts)
- apex_dma writes: game state, player data, spectator lists
- apex_linux_client writes: settings (aim, ESP, glow colors, etc.)
- Real-time bidirectional communication without network overhead

## File Structure

```
apex_dma/              - Linux DMA server (C++ with memflow FFI)
  ├── apex_dma.cpp     - Main server loop with shared memory integration
  ├── shared_memory.h  - POSIX shared memory wrapper (SharedMemoryWriter)
  ├── Game.cpp/h       - Entity and game state logic
  ├── memory.cpp/h     - Memflow wrapper and memory operations
  ├── Math.cpp/h       - Vector/matrix math
  ├── offsets.h        - Game structure offsets
  ├── Makefile         - Build configuration (-lrt for shared memory)
  ├── build.sh         - Automated build script (generates run_apex_dma.sh)
  ├── run_apex_dma.sh  - Auto-generated execution script with LD_LIBRARY_PATH
  └── memflow_lib/     - Memflow library submodule

apex_linux_client/     - Linux GUI client (Rust + egui)
  ├── src/
  │   ├── main.rs      - Application entry point
  │   ├── app.rs       - Main GUI logic (egui widgets, event handling)
  │   ├── shm.rs       - Shared memory client (memmap2-based)
  │   └── types.rs     - Rust SharedData structures (#[repr(C)])
  ├── Cargo.toml       - Dependencies (eframe, egui, memmap2, serde)
  └── build.sh         - Build script

apex_guest/            - [DEPRECATED] Windows VM components
  ├── Client/          - Old TCP client (no longer used)
  └── Overlay/         - Old DirectX 11 overlay (no longer used)
```

## Dependencies

### apex_dma (C++ component)
- Rust/Cargo (for memflow FFI compilation)
- g++ compiler with C++11 support
- memflow-kvm connector installed system-wide
- KVM virtualization support
- librt (for POSIX shared memory, included in glibc)

### apex_linux_client (Rust component)
- Rust toolchain (stable)
- Cargo
- Dependencies installed via Cargo.toml:
  - eframe 0.29 (egui windowing)
  - egui 0.29 (GUI framework)
  - memmap2 0.9 (memory mapping)
  - serde 1.0 (serialization)
  - serde-big-array 0.5 (large array support)
  - bincode 1.3 (binary serialization)
  - libc 0.2 (C FFI bindings)

## Important Notes

**Research Purpose**: This project is for educational and research purposes only, focusing on understanding cross-VM memory access techniques, game engine internals, and systems programming.

Claude Code can fully assist with:
- Feature implementation and enhancement
- Code refactoring and optimization
- Bug fixes and debugging
- Architecture improvements
- Documentation and analysis
- Build system improvements
- Testing and validation

## Common Development Tasks

### Updating Game Offsets
When game updates, offsets need to be refreshed:
1. Use pattern scanning in `memory.cpp` to find signature matches
2. Update `offsets.h` with new structure offsets
3. Verify `offsets.ini` matches current game version

### Adding New Entity Features
1. Add offset constants to `offsets.h`
2. Implement getter methods in `Entity` class in `Game.h`
3. Update `Game.cpp` with implementation
4. Add fields to SharedData structure in `shared_memory.h`
5. Update corresponding fields in `types.rs` (maintain #[repr(C)] compatibility)
6. Update `apex_dma.cpp` main loop to write new data to shared memory
7. Update `app.rs` GUI to display new data

### Debugging Memory Access
- Enable verbose logging in memflow connector
- Use `findPattern()` in `memory.cpp` to verify signatures
- Check process attachment with `apex_mem.get_proc_baseaddr()`

### Performance Optimization
- Batch memory reads where possible using memflow's batch API
- Reduce polling frequency for non-critical data
- Use caching for static offsets and addresses
