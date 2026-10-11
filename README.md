# Taskee — Desktop System Resource Monitor

<p align="center">
  <img src="assets/logo.jpg" alt="Taskee Logo" width="180" style="border-radius: 12px;"/>
</p>

<p align="center">
  <strong>100% Pure Rust • High-Density Workstation Telemetry • ArtCraft Ultra-Dark HUD Design</strong>
</p>

---

## ⚡ Overview

**Taskee** is a lightweight, blazing-fast, cross-platform Desktop System Resource Monitor and Mini Task Manager written entirely in pure Rust using `eframe`, `egui`, `egui_plot`, and `sysinfo`.

Designed with a brutalist, high-density technical HUD aesthetic inspired by [ArtCraft](https://getartcraft.com/), Taskee dispenses with generic bubbly cards in favor of razor-sharp borders, monospace typography, and comprehensive hardware instrumentation.

---

## 🎨 ArtCraft Ultra-Dark Color Palette

| Token | Hex | Role |
| :--- | :--- | :--- |
| `bg_main` | `#0D0E10` | Main window background |
| `bg_card` | `#121316` | Card & container background |
| `bg_input` | `#181A1F` | Button, input field, & badge background |
| `border` | `#22252C` | Crisp borders & grid dividers |
| `text_main` | `#EEEEEE` | High-contrast primary text |
| `text_muted` | `#888C96` | Technical secondary & metric text |
| `status_ok` | `#10B981` | Emerald live indicator & memory telemetry |
| `accent_blue` | `#2563EB` | Primary HUD indigo/blue accent |
| `accent_red` | `#EF4444` | High load alerts & process kill buttons |
| `accent_purple` | `#8B5CF6` | Swap memory instrumentation |
| `accent_amber` | `#F59E0B` | Storage & medium load warnings |
| `accent_teal` | `#14B8A6` | Network transmission throughput |

---

## 🚀 Key Features

### 1. Unified Cross-Platform Telemetry
- **Hardware & OS Identity**: Detects host name, kernel version, distribution/OS (Windows or Linux/WSL), physical/logical core counts, and formatted uptime.
- **Calibrated CPU Engine**: Avoids 0% cold-start deltas by priming measurement windows, guaranteeing accurate, identical telemetry across both Linux and Windows.

### 2. Multi-Core Performance & Live Charts
- **60-Second Time-Series Plots**: Dual live charts powered by `egui_plot` displaying real-time CPU load and RAM saturation.
- **Per-Core Utilization Grid**: Dynamic thread matrix tracking usage percentage and load bars for every logical core.
- **Swap Space Tracking**: Real-time swap memory usage and capacity instrumentation.

### 3. Workstation Process Manager
- **Dense Telemetry**: PID, Status (`RUN`/`SLEEP`), Process Name, CPU %, RAM (MB), RAM %, and Runtime (`MM:SS`).
- **Instant Sorting**: One-click sorting by CPU %, Memory, PID, Name, or Runtime.
- **Real-Time Filtering**: Instant keyword search for process names or PID numbers.
- **Safe Process Termination**: Send kill signals directly with immediate feedback notifications.

### 4. Storage & Network Instrumentation
- **Partitions & Mount Points**: Live inspection of every drive (`/`, `/mnt/d`, `C:\`, `D:\`), filesystem type, available space, and percentage utilization bars.
- **Network Interface Statistics**: Live throughput (KB/s RX/TX rates) and total byte counters per network interface.

---

## 🛠️ Project Structure

```text
taskee/
├── Cargo.toml
├── README.md
├── assets/
│   └── logo.jpg             # High-resolution cyber HUD logo
└── src/
    ├── main.rs              # Application entry point & ArtCraft theme init
    ├── app.rs               # Main GUI state, HUD navigation, & 1-second update loop
    ├── monitor.rs           # Multi-threaded telemetry engine (CPU, Cores, RAM, Swap, Disks, Net, Processes)
    └── ui/
        ├── mod.rs           # UI module declarations
        ├── theme.rs         # ArtCraft ultra-dark color tokens & frame styles
        ├── overview.rs      # System specs banner, 4-card HUD, top process consumers
        ├── performance.rs   # Live egui_plot time-series charts & per-core thread grid
        ├── processes.rs     # Workstation striped process table with search & sorting
        └── storage_net.rs   # Mounted filesystem partitions & network adapters
```

---

## 💻 How to Run

### Natively on Windows
Run directly from PowerShell or Command Prompt:

### 1. Pre-built Executable (Recommended for Non-Developers)
Download the standalone executable directly from the [Releases](https://github.com/USERNAME/taskee/releases) tab:
- **Windows**: Download `taskee-windows-x86_64.exe` and double-click to launch.
- **Linux**: Download `taskee-linux-x86_64`, mark executable (`chmod +x taskee-linux-x86_64`), and run `./taskee-linux-x86_64`.

No Rust, Node.js, or external runtimes are required.

---

### 2. Install via Cargo (One-Liner for Rust Users)
Anyone with Cargo installed can install it globally to their system with a single command:

```bash
cargo install --git https://github.com/USERNAME/taskee
```

Once installed, simply launch `taskee` from anywhere in your terminal or prompt:
```bash
taskee
```

---

### 3. Build & Run from Source

#### Windows:
```powershell
git clone https://github.com/USERNAME/taskee.git
cd taskee
cargo run --release
```

#### Linux & WSL:
```bash
git clone https://github.com/USERNAME/taskee.git
cd taskee
LIBGL_ALWAYS_SOFTWARE=1 cargo run --release
```

---

## 🔒 Requirements

- Zero Webview, zero Node.js/JavaScript, zero Tauri. 100% Pure Native Rust.

