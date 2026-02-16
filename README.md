# RUST_TUI SYSTEM MONITOR: TECHNICAL SPECIFICATION

A sophisticated, stateful Terminal User Interface (TUI) designed for real-time system telemetry. This project implements advanced resource monitoring, process management, and network throughput analysis using the Rust programming language.

## 1. Core Technologies and Dependencies

The application leverages the following "best-in-class" Rust libraries to achieve high performance with a low memory footprint:

- **Ratatui (v0.26+)**: An immediate-mode rendering library. Unlike traditional retained-mode UIs, Ratatui redraws the entire interface on every "tick" (500ms), ensuring the UI is never out of sync with the underlying system data.
- **Sysinfo (v0.31+)**: A cross-platform library that provides the data acquisition layer. It interfaces directly with the operating system kernels (via `/proc` on Linux or Mach APIs on macOS) to fetch hardware metrics.
- **Crossterm (v0.27+)**: Handles the "Raw Mode" terminal state and cross-platform input. It allows the application to capture keyboard events (like 'q') without the overhead of standard input buffering.

## 2. Functional Module Breakdown

### A. Stateful Data Engine (src/system.rs)
The data layer is the most complex part of the application. It utilizes a custom `SystemTracker` struct to solve the "Time-Delta" problem.



- **Network Delta Tracking**: Network speed is not a single value stored by the OS; it is the difference in bytes received between two points in time. Our tracker persists the `Networks` object to calculate:
  `Speed = (Current_Bytes - Previous_Bytes) / Time_Elapsed`.
- **Process Resource Allocation**: The engine iterates through the global process list, maps them to a custom `ProcessData` struct, and performs an unstable partial sort. This ensures the 50 most CPU-intensive processes are always visible.
- **Memory & Swap Logic**: Uses 64-bit unsigned integers to track byte-level accuracy for both physical RAM and virtual swap space, calculating percentages safely to avoid division-by-zero errors.

### B. Adaptive UI Pipeline (src/ui.rs)
The UI logic implements a "Constraint-Based" layout system, allowing the dashboard to scale dynamically to any terminal size.



- **Hierarchical Layouts**: The screen is split into four primary vertical sections (Header, Gauges, Network, Table). The Gauges section is further split horizontally into three equal segments using `Constraint::Percentage(33)`.
- **Immediate-Mode Drawing**: Every 500ms, the `render` function receives a `Frame`. It clears the area and reconstructs the following widgets:
    - **Gauges**: Used for CPU, RAM, and Swap. They provide a high-contrast visual bar.
    - **Table Widget**: Implements a structured grid with a persistent header. It handles column alignment (PID, Name, CPU, Memory) to ensure data remains readable even as process names change length.
    - **Paragraph Widgets**: Used for the header and network blocks to display formatted text strings.

### C. Main Controller & Event Loop (src/main.rs)
The controller acts as the orchestrator for the application's lifecycle.

- **Terminal Raw Mode**: Upon startup, the app executes `enable_raw_mode()`. This instructs the terminal to stop processing control characters (like `Ctrl+C`) and pass them directly to our Rust logic.
- **The Tick Rate**: A `Duration` of 500ms is used to balance system load with real-time accuracy. The loop uses `event::poll` to remain non-blocking, allowing the UI to refresh even if no keys are pressed.
- **Graceful De-initialization**: The application uses a custom cleanup sequence to ensure the terminal is restored to its original state (showing the cursor and leaving the alternate screen) even if the program logic terminates.

## 3. Implementation Logic: Detailed View

### Data Transformation
Raw data from the kernel is often provided in bytes or raw percentages. Our application implements the following transformations:
- **Bytes to Megabytes**: `p.memory() / 1024 / 1024` to ensure human readability in the process table.
- **Load Average Stringification**: Transforms the `LoadAvg` struct into a formatted "1m, 5m, 15m" string.
- **OS String Conversion**: Uses `to_string_lossy()` to handle non-UTF8 process names, preventing application panics on "weird" system filenames.

### Sorting Algorithm
We implement an unstable partial sort on the process list:
```rust
procs.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
