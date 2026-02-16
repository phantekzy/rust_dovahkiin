# RUST_TUI SYSTEM MONITOR: TECHNICAL SPECIFICATION

A high-performance, stateful Terminal User Interface (TUI) engineered for real-time system telemetry. This application provides deep visibility into hardware utilization, network throughput, and process-level resource allocation with minimal system overhead.



## 1. Core Technical Foundations

The architecture is built upon the Rust safety model, ensuring zero-cost abstractions and memory-safe hardware interfacing. Key dependencies include:

- **Ratatui (v0.26+)**: An immediate-mode rendering engine. Unlike retained-mode UIs, it reconstructs the interface state on every 500ms "tick," ensuring the visual representation is an exact reflection of kernel data.
- **Sysinfo (v0.31+)**: A sophisticated abstraction layer that interfaces with system-specific APIs (such as `/proc` on Linux) to retrieve hardware metrics.
- **Crossterm (v0.27+)**: Manages the terminal "Raw Mode" lifecycle, allowing the application to bypass standard line-buffering and handle keyboard interrupts (like 'q' to exit) with sub-millisecond latency.

## 2. Functional Architecture

### A. Stateful Data Acquisition (src/system.rs)
The application utilizes a persistent `SystemTracker` struct. This is a critical design choice for calculating time-series data:
- **Network Delta Tracking**: Real-time throughput (KB/s) is calculated by measuring the variance in total bytes received between two discrete points in time.
- **Dynamic Sorting**: Implements an unstable partial sort on the global process list, ensuring the top 50 resource consumers are prioritized for visibility.
- **Metric Normalization**: Raw kernel values are normalized into human-readable formats (e.g., converting 64-bit byte counts into Megabytes).

### B. Immediate-Mode UI Pipeline (src/ui.rs)
The UI is built using a nested constraint-based layout engine that scales dynamically to any terminal dimensions:
- **Geometric Layout**: The screen is divided into four primary horizontal sectors using `Layout::default()`. 
- **Resource Gauges**: Provides high-contrast visual bars for CPU, RAM, and Swap utilization.
- **Telemetry Tables**: A structured data grid for process management, providing fixed-width column alignment for PID, Process Name, CPU%, and Memory usage.



### C. Execution Lifecycle (src/main.rs)
The main controller manages the bridge between user input and the refresh loop:
- **Event Polling**: Uses a non-blocking poll mechanism to check for user input every 500ms.
- **Terminal Restoration**: Implements a strict cleanup sequence that restores terminal raw mode, alternate screen buffers, and cursor visibility upon application termination.

## 3. Implementation Details

- **Language**: Rust (Edition 2021)
- **Update Frequency**: 500ms (Adjustable)
- **Sorting Logic**: CPU-heavy prioritization using `partial_cmp`.
- **String Handling**: Lossy UTF-8 conversion for non-standard process names to prevent application panics.

## 4. Operational Keybindings

| Key | Action |
| :--- | :--- |
| **Q** | Graceful Shutdown (Restores Terminal State) |
| **Refresh** | Automatic (Asynchronous 500ms Interval) |

## 5. Build and Deployment

To compile the application with full release optimizations:

```bash
cargo build --release
./target/release/rust_tui
