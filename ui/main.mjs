const status = document.querySelector('#engine-status');
const streamStatus = document.querySelector('#stream-status');
const activeVisualizer = document.querySelector('#dome-active-vis');
const flashSpeed = document.querySelector('#flash-speed');
const flashSpeedValue = document.querySelector('#flash-speed-value');
const simVolume = document.querySelector('#sim-volume');
const simVolumeValue = document.querySelector('#sim-volume-value');
const simBeatProgress = document.querySelector('#sim-beat-progress');
const simBeatProgressValue = document.querySelector('#sim-beat-progress-value');
const simFlashActive = document.querySelector('#sim-flash-active');
const paletteIndex = document.querySelector('#palette-index');
const palettePrimary = document.querySelector('#palette-primary');
const paletteSecondary = document.querySelector('#palette-secondary');
const paletteAccent = document.querySelector('#palette-accent');
const metricFrames = document.querySelector('#metric-frames');
const metricSimulatorFrames = document.querySelector('#metric-simulator-frames');
const canvas = document.querySelector('#dome-simulator');
const context = canvas?.getContext('2d');

async function request(path, options = {}) {
  const response = await fetch(path, {
    headers: { 'content-type': 'application/json' },
    ...options,
  });
  if (!response.ok) {
    throw new Error(`${path} failed with ${response.status}`);
  }
  return response.json();
}

function updateSnapshot(snapshot) {
  status.textContent = snapshot.running ? 'running' : 'stopped';
  metricFrames.textContent = String(snapshot.metrics.frames);
  metricSimulatorFrames.textContent = String(snapshot.metrics.simulator_frames);
  activeVisualizer.value = String(snapshot.config.dome_active_vis);
  simVolume.value = String(snapshot.simulator.volume);
  simVolumeValue.textContent = String(snapshot.simulator.volume);
  simBeatProgress.value = String(snapshot.simulator.beat_progress);
  simBeatProgressValue.textContent = String(snapshot.simulator.beat_progress);
  simFlashActive.checked = snapshot.simulator.flash_active;
  paletteIndex.value = String(snapshot.simulator.palette_index);
  palettePrimary.value = toColorInput(snapshot.simulator.primary);
  paletteSecondary.value = toColorInput(snapshot.simulator.secondary);
  paletteAccent.value = toColorInput(snapshot.simulator.accent);
}

function toColorInput(color) {
  return `#${color.toString(16).padStart(6, '0')}`;
}

function fromColorInput(color) {
  return Number.parseInt(color.replace('#', ''), 16);
}

function clearCanvas() {
  if (!context) {
    return;
  }
  context.fillStyle = '#000000';
  context.fillRect(0, 0, canvas.width, canvas.height);
}

function drawFrame(colors) {
  if (!context || !colors.length) {
    return;
  }

  const cellCount = Math.ceil(Math.sqrt(colors.length));
  const cellSize = canvas.width / cellCount;
  context.clearRect(0, 0, canvas.width, canvas.height);

  colors.forEach((color, index) => {
    const x = (index % cellCount) * cellSize;
    const y = Math.floor(index / cellCount) * cellSize;
    context.fillStyle = `#${color.toString(16).padStart(6, '0')}`;
    context.fillRect(x, y, Math.ceil(cellSize), Math.ceil(cellSize));
  });
}

function drawPixel(command) {
  if (!context) {
    return;
  }

  const columns = 20;
  const cellSize = canvas.width / columns;
  const index = command.strut_index * 3 + command.led_index;
  const x = (index % columns) * cellSize + cellSize / 2;
  const y = Math.floor(index / columns) * cellSize + cellSize / 2;
  context.fillStyle = toColorInput(command.color);
  context.beginPath();
  context.arc(x, y, Math.max(5, cellSize * 0.3), 0, Math.PI * 2);
  context.fill();
}

function handleSimulatorFrame(frame) {
  metricFrames.textContent = String(frame.metrics.frames);
  metricSimulatorFrames.textContent = String(frame.metrics.simulator_frames);
  clearCanvas();

  for (const command of frame.commands) {
    if (command.kind === 'frame') {
      drawFrame(command.colors);
    } else if (command.kind === 'pixel') {
      drawPixel(command);
    }
  }
}

async function refreshState() {
  const snapshot = await request('/api/state');
  updateSnapshot(snapshot);
  handleSimulatorFrame(await request('/api/simulator/frame'));
}

async function patchSimulatorControls() {
  const snapshot = await request('/api/simulator', {
    method: 'PATCH',
    body: JSON.stringify({
      volume: Number(simVolume.value),
      beat_progress: Number(simBeatProgress.value),
      flash_active: simFlashActive.checked,
      palette_index: Number(paletteIndex.value),
      primary: fromColorInput(palettePrimary.value),
      secondary: fromColorInput(paletteSecondary.value),
      accent: fromColorInput(paletteAccent.value),
    }),
  });
  updateSnapshot(snapshot);
  handleSimulatorFrame(await request('/api/simulator/frame'));
}

function connectSimulatorStream() {
  const scheme = window.location.protocol === 'https:' ? 'wss' : 'ws';
  const socket = new WebSocket(`${scheme}://${window.location.host}/ws/simulator`);
  streamStatus.textContent = 'stream connecting';

  socket.addEventListener('open', () => {
    streamStatus.textContent = 'stream connected';
  });

  socket.addEventListener('message', event => {
    handleSimulatorFrame(JSON.parse(event.data));
  });

  socket.addEventListener('close', () => {
    streamStatus.textContent = 'stream disconnected';
    window.setTimeout(connectSimulatorStream, 1000);
  });
}

document.querySelector('#start-engine')?.addEventListener('click', async () => {
  updateSnapshot(await request('/api/start', { method: 'POST' }));
});

document.querySelector('#stop-engine')?.addEventListener('click', async () => {
  updateSnapshot(await request('/api/stop', { method: 'POST' }));
});

activeVisualizer?.addEventListener('change', async () => {
  updateSnapshot(
    await request('/api/config/dome', {
      method: 'PATCH',
      body: JSON.stringify({ active_visualizer: Number(activeVisualizer.value) }),
    }),
  );
  handleSimulatorFrame(await request('/api/simulator/frame'));
});

flashSpeed?.addEventListener('input', () => {
  flashSpeedValue.textContent = flashSpeed.value;
});

for (const input of [
  simVolume,
  simBeatProgress,
  simFlashActive,
  paletteIndex,
  palettePrimary,
  paletteSecondary,
  paletteAccent,
]) {
  input?.addEventListener('input', async () => {
    simVolumeValue.textContent = simVolume.value;
    simBeatProgressValue.textContent = simBeatProgress.value;
    await patchSimulatorControls();
  });
}

await refreshState();
connectSimulatorStream();
