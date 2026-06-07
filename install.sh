#!/usr/bin/env bash
set -euo pipefail

REPO="ezequielgk/Umbrella-Fetch"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
COMPLETIONS_BASH="${COMPLETIONS_BASH:-$HOME/.bash_completion.d}"
COMPLETIONS_ZSH="${COMPLETIONS_ZSH:-$HOME/.zsh/completions}"
COMPLETIONS_FISH="${COMPLETIONS_FISH:-$HOME/.config/fish/completions}"

# Check dependencies
if ! command -v curl &> /dev/null; then
  echo "ERROR: 'curl' is required for installation."
  exit 1
fi
if ! command -v tar &> /dev/null; then
  echo "ERROR: 'tar' is required for installation."
  exit 1
fi

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS-$ARCH" in
  Linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
  Linux-aarch64)  TARGET="aarch64-unknown-linux-gnu" ;;
  *)
    echo "ERROR: platform $OS-$ARCH is not supported."
    exit 1
    ;;
esac

# Get latest release version
VERSION=$(curl -fsSL \
  "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "ERROR: could not fetch latest version."
  exit 1
fi

# Handle additional options
if [ "${1:-}" = "--uninstall" ]; then
  rm -f "$INSTALL_DIR/umbrella-fetch"
  rm -f "$COMPLETIONS_BASH/umbrella-fetch"
  rm -f "$COMPLETIONS_ZSH/_umbrella-fetch"
  rm -f "$COMPLETIONS_FISH/umbrella-fetch.fish"
  echo "umbrella-fetch uninstalled."
  exit 0
fi

if [ "${1:-}" = "--version" ]; then
  echo "$VERSION"
  exit 0
fi

# Download and verify
ASSET="umbrella-fetch-${VERSION}-${TARGET}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT   # always clean up on exit

echo "→ Downloading $ASSET..."
curl -fsSL "$URL" -o "$TMP/$ASSET"

echo "→ Extracting..."
tar -xzf "$TMP/$ASSET" -C "$TMP"

# Install binary
mkdir -p "$INSTALL_DIR"
cp "$TMP/umbrella-fetch" "$INSTALL_DIR/umbrella-fetch"
chmod +x "$INSTALL_DIR/umbrella-fetch"
echo "→ Binary installed in $INSTALL_DIR/umbrella-fetch"

# Warn if INSTALL_DIR is not in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "WARNING: $INSTALL_DIR is not in your PATH."
  echo "  Add this to your ~/.bashrc or ~/.zshrc:"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

# Install completions

# bash
if command -v bash &>/dev/null; then
  mkdir -p "$COMPLETIONS_BASH"
  cp "$TMP/completions/umbrella-fetch.bash" \
     "$COMPLETIONS_BASH/umbrella-fetch"
  echo "→ Bash completions installed."
  echo "  Make sure you have this in your ~/.bashrc:"
  echo "  for f in ~/.bash_completion.d/*; do . \"\$f\"; done"
fi

# zsh
if command -v zsh &>/dev/null; then
  mkdir -p "$COMPLETIONS_ZSH"
  cp "$TMP/completions/_umbrella-fetch" \
     "$COMPLETIONS_ZSH/_umbrella-fetch"
  echo "→ Zsh completions installed in $COMPLETIONS_ZSH"
  echo "  Make sure you have this in your ~/.zshrc:"
  echo "  fpath=($COMPLETIONS_ZSH \$fpath)"
  echo "  autoload -Uz compinit && compinit"
fi

# fish
if command -v fish &>/dev/null; then
  mkdir -p "$COMPLETIONS_FISH"
  cp "$TMP/completions/umbrella-fetch.fish" \
     "$COMPLETIONS_FISH/umbrella-fetch.fish"
  echo "→ Fish completions installed."
fi

# Final summary
echo ""
echo "✓ umbrella-fetch $VERSION installed successfully."
echo ""
echo "  umbrella-fetch          → full dashboard (default)"
echo "  umbrella-fetch full     → full dashboard"
echo "  umbrella-fetch minimal  → minimalist fetch"
echo "  umbrella-fetch ubcs     → U.B.C.S. roster"
echo "  umbrella-fetch uss      → U.S.S. roster"
echo "  umbrella-fetch virus --strain t-virus  → viral simulation"
echo ""
