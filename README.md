# Umbrella Fetch

![Umbrella Corporation](https://img.shields.io/badge/Umbrella_Corporation-Secure_Network-red?style=for-the-badge)

**Umbrella Fetch** is an interactive telemetry and diagnostic dashboard for terminals, inspired by the extended Resident Evil universe. It offers everything from a minimalist "neofetch" style screen to complete dashboards with CPU and RAM monitoring, biological simulations (cellular automata), and classified personnel databases (U.B.C.S. / U.S.S.).

## Installation

You can quickly install the latest version (which includes auto-completion scripts for Bash, Zsh, and Fish) using the automatic installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/ezequielgk/Umbrella-Fetch/main/install.sh | bash
```

By default, it is installed in `~/.local/bin`. If you want to install it in a specific directory, you can specify it like this:

```bash
curl -fsSL https://raw.githubusercontent.com/ezequielgk/Umbrella-Fetch/main/install.sh > install.sh
INSTALL_DIR=/usr/local/bin bash install.sh
```

## General Usage

Once installed, you have several commands at your disposal. Some open a Terminal User Interface (TUI) and others statically list information in the console.

```bash
# Start the interactive minimalist fetch
umbrella-fetch minimal
# or simply:
umbrella-fetch

# Start the advanced dashboard (multiple windows, telemetry, and global threat)
umbrella-fetch full

# View the U.B.C.S. mercenaries database (TUI)
umbrella-fetch ubcs

# View the classified U.S.S. forces database (TUI)
umbrella-fetch uss

# View a cellular automaton simulation of a viral outbreak
umbrella-fetch virus --strain t-virus
```

### Static Options

If you only want to print information in the terminal instead of opening the interactive mode (ideal for scripts):

```bash
umbrella-fetch ubcs --list
umbrella-fetch uss --list
umbrella-fetch virus --list
```

## Shell Completions

The installer (`install.sh`) will automatically try to configure the completion scripts on your system. If you want to generate them manually, you can run:

```bash
umbrella-fetch completions bash > ~/.bash_completion.d/umbrella-fetch
umbrella-fetch completions zsh > ~/.zsh/completions/_umbrella-fetch
umbrella-fetch completions fish > ~/.config/fish/completions/umbrella-fetch.fish
```

## Local Build

If you prefer to compile it from source (requires Rust and Cargo installed):

```bash
git clone https://github.com/ezequielgk/Umbrella-Fetch.git
cd Umbrella-Fetch
cargo install --path .
```

---

*"Obedience Breeds Discipline, Discipline Breeds Unity, Unity Breeds Power."*
