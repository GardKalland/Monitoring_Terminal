# System Monitor TUI

Terminal-based system monitoring tool for Arch Linux written in Rust

## Prerequisites

**Required:**

- **Linux** (Arch Linux) Dont know if it works on other ones, probs should. idk
- **Rust** (1.70 or newer) - Install from [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Terminal** with unicode support. yours default should probs work. i am using alacritty.

**Optional:**

- **ProtonVPN** - I use this. Havent tested any other vpns.... soooo dont know if they work

## Features

My genius

## Installation

1. **Clone or download** this repository
2. **Ensure you have Rust installed** (see Prerequisites above)
3. **Build the project:**
   ```bash
   cd my_workspace
   cargo build --release
   ```

The binary will be available at `target/release/myWorkspace`

**Optional: Install system-wide**

```bash
sudo cp target/release/myWorkspace /usr/local/bin/sm
# Now you can run it from anywhere with: sm
```

## Usage

Simply run the compiled binary:

```bash
./target/release/myWorkspace
```

Or run directly with cargo:

```bash
cargo run --release
```

## Keyboard Shortcuts

- `q` or `Ctrl+C` - Quit application
- `?` or `h` - Toggle help screen
- `l` or `Right Arrow` - Next tab
- `h` or `Left Arrow` - Previous tab
- `1`, `2`, `3`, `4` - Jump to specific tab
- `s` - Cycle process sort (CPU/Memory/Name/PID)
- `o` - Toggle sort order (ascending/descending)
- `Up`/`k`, `Down`/`j` - Scroll through processes

## Why This Exists?

Got bored of typing commands to see system stats. Why not use existing tools? Because where's the fun in that?

## Technical Details

Built with:

- **ratatui** (0.29) - Modern TUI framework
- **crossterm** (0.28) - Cross-platform terminal manipulation
- **sysinfo** (0.32) - System information library
- **chrono** (0.4) - Date and time functionality
- **anyhow** (1.0) - Error handling

Oh yea, if your computer explodes when using this, or anything else goes wrong. i am NOT responsible.... Good luck
