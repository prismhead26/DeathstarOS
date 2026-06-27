<script lang="ts">
  import { onMount } from 'svelte';
  import { getCameraState, toggleCamera, openCameraSettings, type CameraState } from '../api/commands';

  let cameraState: CameraState = { enabled: false, in_use: false, used_by: null };
  let loading = false;
  let error: string | null = null;

  async function loadState(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      cameraState = await getCameraState();
    } catch (e) {
      error = `Failed to load camera state: ${e}`;
      console.error(error);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleToggle() {
    try {
      loading = true;
      error = null;
      const newState = await toggleCamera();
      cameraState = { ...cameraState, enabled: newState };
      // Refresh state after a short delay (without loading indicator)
      setTimeout(() => loadState(false), 500);
    } catch (e) {
      error = `Failed to toggle camera: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleOpenSettings() {
    try {
      await openCameraSettings();
    } catch (e) {
      error = `Failed to open camera settings: ${e}`;
      console.error(error);
    }
  }

  onMount(() => {
    loadState(true);
    // Refresh status every 5 seconds (without loading indicator)
    const interval = setInterval(() => loadState(false), 5000);
    return () => clearInterval(interval);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Camera</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    <div class="control-row">
      <span class="control-label">Camera System</span>
      <button
        class="toggle-button {cameraState.enabled ? 'on' : 'off'}"
        onclick={handleToggle}
        disabled={loading}
      >
        {cameraState.enabled ? 'Enabled' : 'Disabled'}
      </button>
    </div>

    <div class="control-row">
      <span class="control-label">Usage Status</span>
      <span class="status-badge {cameraState.in_use ? 'active' : 'inactive'}">
        {cameraState.in_use ? '● In Use' : '○ Not in Use'}
      </span>
    </div>

    {#if cameraState.used_by}
      <div class="app-info">
        <span class="app-label">Used by:</span>
        <span class="app-name">{cameraState.used_by}</span>
      </div>
    {/if}

    <button class="settings-button" onclick={handleOpenSettings}>
      Open Camera Settings
    </button>

    {#if error}
      <div class="error-message">{error}</div>
    {/if}

    <div class="info-text">
      <strong>Note:</strong> macOS automatically restarts the camera daemon for security. Clicking "Disable" kills the process temporarily, but macOS will restart it within seconds. For reliable camera control, use "Open Camera Settings" to manage per-app permissions.
    </div>
  </div>
</div>

<style>
  .control-card {
    background: #2a2a2a;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 16px;
  }

  .control-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  h3 {
    margin: 0;
    color: #ffffff;
    font-size: 18px;
    font-weight: 600;
  }

  .loading-indicator {
    color: #888;
    font-size: 20px;
  }

  .control-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .control-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .control-label {
    color: #cccccc;
    font-size: 14px;
  }

  .toggle-button {
    padding: 8px 20px;
    border-radius: 8px;
    border: none;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 80px;
  }

  .toggle-button.on {
    background: #4CAF50;
    color: white;
  }

  .toggle-button.on:hover:not(:disabled) {
    background: #45a049;
  }

  .toggle-button.off {
    background: #f44336;
    color: white;
  }

  .toggle-button.off:hover:not(:disabled) {
    background: #da190b;
  }

  .toggle-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .status-badge {
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
  }

  .status-badge.active {
    background: rgba(255, 152, 0, 0.2);
    color: #FF9800;
  }

  .status-badge.inactive {
    background: rgba(136, 136, 136, 0.2);
    color: #888;
  }

  .settings-button {
    padding: 10px 16px;
    border-radius: 8px;
    border: 1px solid #444;
    background: #333;
    color: #fff;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .settings-button:hover {
    background: #404040;
    border-color: #555;
  }

  .info-text {
    font-size: 11px;
    color: #666;
    line-height: 1.4;
  }

  .error-message {
    color: #ff6b6b;
    font-size: 12px;
    padding: 8px;
    background: rgba(255, 107, 107, 0.1);
    border-radius: 4px;
  }

  .app-info {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px;
    background: rgba(255, 152, 0, 0.1);
    border-radius: 8px;
    border-left: 3px solid #FF9800;
  }

  .app-label {
    color: #aaa;
    font-size: 13px;
  }

  .app-name {
    color: #FF9800;
    font-size: 14px;
    font-weight: 600;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
  }
</style>
