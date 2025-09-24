# dir_nav

`dir_nav` is a terminal-based navigation tool written in Rust.
It lets you explore your filesystem using **vim-like keybindings** and quickly open the [Zed editor](https://zed.dev) from the current folder.

**Project status:** This project is still **experimental** and not yet stable.
Contributions, feedback, and bug reports are highly encouraged!

---

## Features

- **Fast terminal navigation** with simple, intuitive keybindings:
  - `j` → move **down**
  - `k` → move **up**
  - `f` → enter directory
  - `b` → go back (up one directory)
  - `z` → open current folder in **Zed editor**
  - `.` → toggle dot files and dirs show
  - `p` → open images/pdf with native app
  - `g` → toggle files show
  - `escape` → exit program
- Minimal and distraction-free interface
- Powered by **Rust** for speed and safety

---

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)

### Install with Cargo
```bash
cargo install dirnav

dirnav
