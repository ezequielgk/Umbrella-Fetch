#!/usr/bin/env bash
set -euo pipefail

REPO="ezequielgk/Umbrella-Fetch"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
COMPLETIONS_BASH="${COMPLETIONS_BASH:-$HOME/.bash_completion.d}"
COMPLETIONS_ZSH="${COMPLETIONS_ZSH:-$HOME/.zsh/completions}"
COMPLETIONS_FISH="${COMPLETIONS_FISH:-$HOME/.config/fish/completions}"

# Verificar dependencias
if ! command -v curl &> /dev/null; then
  echo "ERROR: 'curl' es requerido para la instalación."
  exit 1
fi
if ! command -v tar &> /dev/null; then
  echo "ERROR: 'tar' es requerido para la instalación."
  exit 1
fi

# Detectar OS y arquitectura
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS-$ARCH" in
  Linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
  Linux-aarch64)  TARGET="aarch64-unknown-linux-gnu" ;;
  *)
    echo "ERROR: plataforma $OS-$ARCH no soportada."
    exit 1
    ;;
esac

# Obtener versión latest
VERSION=$(curl -fsSL \
  "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "ERROR: no se pudo obtener la versión latest."
  exit 1
fi

# Manejar opciones adicionales
if [ "${1:-}" = "--uninstall" ]; then
  rm -f "$INSTALL_DIR/umbrella-fetch"
  rm -f "$COMPLETIONS_BASH/umbrella-fetch"
  rm -f "$COMPLETIONS_ZSH/_umbrella-fetch"
  rm -f "$COMPLETIONS_FISH/umbrella-fetch.fish"
  echo "umbrella-fetch desinstalado."
  exit 0
fi

if [ "${1:-}" = "--version" ]; then
  echo "$VERSION"
  exit 0
fi

# Descargar y verificar
ASSET="umbrella-fetch-${VERSION}-${TARGET}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT   # limpiar siempre al salir

echo "→ Descargando $ASSET..."
curl -fsSL "$URL" -o "$TMP/$ASSET"

echo "→ Extrayendo..."
tar -xzf "$TMP/$ASSET" -C "$TMP"

# Instalar binario
mkdir -p "$INSTALL_DIR"
cp "$TMP/umbrella-fetch" "$INSTALL_DIR/umbrella-fetch"
chmod +x "$INSTALL_DIR/umbrella-fetch"
echo "→ Binario instalado en $INSTALL_DIR/umbrella-fetch"

# Advertir si INSTALL_DIR no está en PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "AVISO: $INSTALL_DIR no está en tu PATH."
  echo "  Agregá esto a tu ~/.bashrc o ~/.zshrc:"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

# Instalar completions

# bash
if command -v bash &>/dev/null; then
  mkdir -p "$COMPLETIONS_BASH"
  cp "$TMP/completions/umbrella-fetch.bash" \
     "$COMPLETIONS_BASH/umbrella-fetch"
  echo "→ Completions bash instaladas."
  echo "  Asegurate de tener en ~/.bashrc:"
  echo "  for f in ~/.bash_completion.d/*; do . \"\$f\"; done"
fi

# zsh
if command -v zsh &>/dev/null; then
  mkdir -p "$COMPLETIONS_ZSH"
  cp "$TMP/completions/_umbrella-fetch" \
     "$COMPLETIONS_ZSH/_umbrella-fetch"
  echo "→ Completions zsh instaladas en $COMPLETIONS_ZSH"
  echo "  Asegurate de tener en ~/.zshrc:"
  echo "  fpath=($COMPLETIONS_ZSH \$fpath)"
  echo "  autoload -Uz compinit && compinit"
fi

# fish
if command -v fish &>/dev/null; then
  mkdir -p "$COMPLETIONS_FISH"
  cp "$TMP/completions/umbrella-fetch.fish" \
     "$COMPLETIONS_FISH/umbrella-fetch.fish"
  echo "→ Completions fish instaladas."
fi

# Resumen final
echo ""
echo "✓ umbrella-fetch $VERSION instalado correctamente."
echo ""
echo "  umbrella-fetch          → fetch normal"
echo "  umbrella-fetch full     → fetch completo"
echo "  umbrella-fetch mini     → fetch minimalista"
echo "  umbrella-fetch ubcs     → roster UBCS"
echo "  umbrella-fetch uss      → roster USS"
echo "  umbrella-fetch virus --strain t-virus  → simulación viral"
echo ""
