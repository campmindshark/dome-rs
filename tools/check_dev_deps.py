#!/usr/bin/env python3
"""Check local development dependencies for dome-rs."""

from __future__ import annotations

import importlib
import os
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPECTRUM_ROOT = Path(os.environ.get("SPECTRUM_REPO", ROOT.parent / "spectrum"))
MADMOM_ROOT = SPECTRUM_ROOT / "Madmom"
DBN_TRACKER = MADMOM_ROOT / "bin" / "DBNBeatTracker"


def ok(message: str) -> None:
    print(f"ok: {message}")


def fail(message: str) -> None:
    print(f"missing: {message}")


def command_exists(command: str) -> bool:
    return shutil.which(command) is not None


def pkg_config_exists(package: str) -> bool:
    if not command_exists("pkg-config"):
        return False
    return (
        subprocess.run(
            ["pkg-config", "--exists", package],
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        ).returncode
        == 0
    )


def import_exists(module: str) -> bool:
    try:
        importlib.import_module(module)
    except Exception as error:
        fail(f"python module {module}: {error}")
        return False
    ok(f"python module {module}")
    return True


def dbn_help_runs() -> bool:
    if not DBN_TRACKER.exists():
        fail(f"{DBN_TRACKER} (set SPECTRUM_REPO=/path/to/spectrum if needed)")
        return False
    env = os.environ.copy()
    env["PYTHONPATH"] = str(MADMOM_ROOT)
    result = subprocess.run(
        [sys.executable, str(DBN_TRACKER), "--help"],
        check=False,
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
        timeout=30,
    )
    if result.returncode == 0:
        ok("DBNBeatTracker --help")
        return True
    fail(f"DBNBeatTracker --help exited {result.returncode}: {result.stderr.strip()}")
    return False


def main() -> int:
    checks = []

    checks.append(("cargo", command_exists("cargo")))
    checks.append(("python3", command_exists("python3")))
    checks.append(("pip", command_exists("pip3") or command_exists("pip")))
    checks.append(("pkg-config", command_exists("pkg-config")))
    checks.append(("ALSA headers", pkg_config_exists("alsa")))
    checks.append(("PortAudio headers", pkg_config_exists("portaudio-2.0")))

    for name, passed in checks:
        if passed:
            ok(name)
        else:
            fail(name)

    python_modules = ["numpy", "scipy", "Cython", "mido", "pyaudio", "torch", "madmom"]
    passed = all(passed for _, passed in checks)
    passed = all(import_exists(module) for module in python_modules) and passed
    passed = dbn_help_runs() and passed

    if not passed:
        print()
        print("Run: tools/install_dev_deps.sh")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
