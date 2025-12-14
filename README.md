# Walrust
Open-source terminal-based wallpaper changer for Hyprpaper written in Rust

## Demo
| Changing wallpaper using Walrust |
| ------------------------- |
| ![](./assets/demo.gif)    |

Note: Walrust is currently in active development, the software is not yet ready for use.

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

## Compability

- Image preview is tested on Kitty terminal
