#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPECTRUM_REPO="${SPECTRUM_REPO:-"$(cd "$ROOT/.." && pwd)/spectrum"}"
MADMOM_ROOT="$SPECTRUM_REPO/Madmom"

APT_PACKAGES=(
  build-essential
  pkg-config
  python3-dev
  python3-pip
  libasound2-dev
  portaudio19-dev
)

usage() {
  cat <<'EOF'
usage: tools/install_dev_deps.sh [--check] [--python-only] [--system-only]

Installs local development dependencies for dome-rs.

Options:
  --check        only validate installed dependencies
  --python-only skip apt/system packages
  --system-only install apt/system packages and skip Python packages

Environment:
  SPECTRUM_REPO  path to the sibling Spectrum checkout; defaults to ../spectrum
EOF
}

CHECK_ONLY=0
PYTHON_ONLY=0
SYSTEM_ONLY=0
for arg in "$@"; do
  case "$arg" in
    --check) CHECK_ONLY=1 ;;
    --python-only) PYTHON_ONLY=1 ;;
    --system-only) SYSTEM_ONLY=1 ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $arg" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$CHECK_ONLY" == "1" ]]; then
  exec python3 "$ROOT/tools/check_dev_deps.py"
fi

install_system_deps() {
  if [[ "$PYTHON_ONLY" == "1" ]]; then
    return
  fi
  if ! command -v apt-get >/dev/null 2>&1; then
    echo "No apt-get found; install these packages with your OS package manager:"
    printf '  %s\n' "${APT_PACKAGES[@]}"
    return
  fi
  echo "Installing system dependencies with apt..."
  sudo apt-get update
  sudo apt-get install -y "${APT_PACKAGES[@]}"
}

install_python_deps() {
  if [[ "$SYSTEM_ONLY" == "1" ]]; then
    return
  fi
  if [[ ! -d "$MADMOM_ROOT" ]]; then
    echo "Spectrum Madmom checkout not found at $MADMOM_ROOT" >&2
    echo "Set SPECTRUM_REPO=/path/to/spectrum and rerun." >&2
    exit 1
  fi

  echo "Installing Python Madmom dependencies into the current user site..."
  python3 -m pip install --user \
    'numpy<2' \
    scipy \
    cython \
    mido \
    pyaudio

  echo "Installing CPU Torch packages for Madmom..."
  python3 -m pip install --user \
    torch \
    torchvision \
    torchaudio \
    --index-url https://download.pytorch.org/whl/cpu

  echo "Installing Spectrum Madmom checkout in editable mode..."
  python3 -m pip install --user --no-build-isolation -e "$MADMOM_ROOT"
}

install_system_deps
install_python_deps

echo
python3 "$ROOT/tools/check_dev_deps.py"
