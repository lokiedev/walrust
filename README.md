# Walrust
Open-source terminal-based wallpaper changer for Hyprpaper written in Rust

![walrust_demo](https://github.com/user-attachments/assets/48a7bf51-f3ee-4efb-9078-c60b41a48cd6)


Note: Walrust is currently in beta.

## Installation

### Cargo Install

```bash
cargo install --git https://github.com/lokiedev/walrust.git
```

and then you can run:

```bash
walrust ~/path/to/file-or-directory
```

### Build from Source

```bash
git clone https://github.com/lokiedev/walrust.git
cd walrust
cargo build --release
```

and then you can run:

```bash
./target/release/walrust ~/path/to/file-or-directory
```

## Usage

### Running
Pass a file path to change wallpaper without opening the TUI:

```bash
walrust ~/pictures/image.jpg
```

Pass a folder path to open a folder and choose wallpaper:

```bash
walrust ~/pictures/
```

Note: Currently Walrust does not support recursive image listing.

### Navigation
- Use j/Down key to move the cursor down
- Use k/Up key to move the cursor up
- Use Enter key to change wallpaper

## Image Preview

Terminal           | Protocol | Tested |
-------------------|----------|--------|
Kitty (>= v0.28.0) | `Kitty`  | ✅      |
Foot               | `Sixel`  | ✅      |

Walrust uses [`ratatui-image`](https://github.com/benjajaja/ratatui-image) to display image previews, so its terminal compatibility is very likely the same.
See https://github.com/benjajaja/ratatui-image/tree/master?tab=readme-ov-file#compatibility-matrix
