#!/usr/bin/env python3
"""Capture headless fixtures by executing legacy Spectrum C# code."""

from __future__ import annotations

import json
import os
import platform
import subprocess
import sys
from pathlib import Path

import build_spectrum_csharp


ROOT = Path(__file__).resolve().parents[1]
SPECTRUM = ROOT.parent / "spectrum"
OUT = ROOT / "fixtures" / "spectrum-csharp" / "executable_capture.json"
RUNNER_DIR = ROOT / "target" / "spectrum-fixture-capture"
RUNNER_CSPROJ = RUNNER_DIR / "SpectrumFixtureCapture.csproj"
RUNNER_PROGRAM = RUNNER_DIR / "Program.cs"


RUNNER_PROJECT = """<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0-windows</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>

  <ItemGroup>
    <ProjectReference Include="../../../spectrum/Spectrum/Spectrum.csproj" />
  </ItemGroup>
</Project>
"""


RUNNER_SOURCE = r"""using System.Text.Json;
using Spectrum;
using Spectrum.Base;
using Spectrum.LEDs;

static object BarCommandRecord(BarLEDCommand command) => new {
  is_flush = command.isFlush,
  is_runner = command.isRunner,
  led_index = command.ledIndex,
  color = command.color,
};

static object StageCommandRecord(StageLEDCommand command) => new {
  is_flush = command.isFlush,
  side_index = command.sideIndex,
  led_index = command.ledIndex,
  layer_index = command.layerIndex,
  color = command.color,
};

var config = new SpectrumConfiguration {
  barSimulationEnabled = true,
  barInfinityLength = 3,
  barInfinityWidth = 2,
  barRunnerLength = 4,
  stageSimulationEnabled = true,
  stageSideLengths = Enumerable.Range(0, 48).Select(i => 4 + (i % 3)).ToArray(),
};

var bar = new LEDBarOutput(config);
bar.SetPixel(false, 0, 0x112233);
bar.SetPixel(false, 4, 0x445566);
bar.SetPixel(true, 2, 0x778899);
bar.Flush();

var stage = new LEDStageOutput(config);
stage.SetPixel(4, 3, 2, 0xaabbcc);
stage.Flush();

var barCommands = new List<object>();
while (config.barCommandQueue.TryDequeue(out var barCommand)) {
  barCommands.Add(BarCommandRecord(barCommand));
}

var stageCommands = new List<object>();
while (config.stageCommandQueue.TryDequeue(out var stageCommand)) {
  stageCommands.Add(StageCommandRecord(stageCommand));
}

var fixture = new {
  bar_simulator_commands = barCommands,
  stage_simulator_commands = stageCommands,
};

Console.WriteLine(JsonSerializer.Serialize(fixture, new JsonSerializerOptions {
  WriteIndented = true,
}));
"""


def is_wsl() -> bool:
    return "microsoft" in platform.release().lower()


def write_runner() -> None:
    RUNNER_DIR.mkdir(parents=True, exist_ok=True)
    RUNNER_CSPROJ.write_text(RUNNER_PROJECT, encoding="utf-8")
    RUNNER_PROGRAM.write_text(RUNNER_SOURCE, encoding="utf-8")


def run_capture() -> str:
    env = os.environ.copy()
    env["DOTNET_CLI_TELEMETRY_OPTOUT"] = "1"
    if is_wsl():
        runner_win = build_spectrum_csharp.wslpath(RUNNER_CSPROJ)
        command = (
            "$env:DOTNET_CLI_TELEMETRY_OPTOUT = '1'; "
            "$dotnet = Join-Path $env:USERPROFILE '.dotnet\\dotnet.exe'; "
            f"& $dotnet run --project {build_spectrum_csharp.quote_ps(runner_win)} "
            "--configuration Release --nologo --verbosity quiet"
        )
        args = [
            "powershell.exe",
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            command,
        ]
    else:
        args = [
            build_spectrum_csharp.local_dotnet(None),
            "run",
            "--project",
            str(RUNNER_CSPROJ),
            "--configuration",
            "Release",
            "--nologo",
            "--verbosity",
            "quiet",
        ]
    result = subprocess.run(
        args,
        env=env,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout


def main() -> int:
    write_runner()
    raw_fixture = json.loads(run_capture())
    fixture = {
        "metadata": {
            "source": str(SPECTRUM.relative_to(ROOT.parent)),
            "runner": str(RUNNER_CSPROJ.relative_to(ROOT)),
            "command": "python3 tools/capture_spectrum_executable_fixtures.py",
            "hardware_required": False,
            "description": (
                "Headless execution of Spectrum C# simulator command semantics."
            ),
        },
        **raw_fixture,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(fixture, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
