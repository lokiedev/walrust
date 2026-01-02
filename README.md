# Walrust
Open-source terminal-based wallpaper changer for Hyprpaper written in Rust

Note: Walrust is currently in active development, the software is may be unstable.

## Installation

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

- Use up/down arrow key to navigate list, or
- you can also use vim keybindings (j for down and k for up),
- click Enter to change wallpaper

## Image Preview

Terminal           | Protocol | Tested |
-------------------|----------|--------|
Kitty (>= v0.28.0) | `Kitty`  | ✅      |
Foot               | `Sixel`  | ✅      |

Walrust uses [`ratatui-image`](https://github.com/benjajaja/ratatui-image) to display image previews, so its terminal compatibility is very likely the same.
See https://github.com/benjajaja/ratatui-image/tree/master?tab=readme-ov-file#compatibility-matrix
