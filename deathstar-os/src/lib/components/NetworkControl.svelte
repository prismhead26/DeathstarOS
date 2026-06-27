<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleWifi, toggleBluetooth, getNetworkState, type NetworkState } from '../api/commands';

  let networkState: NetworkState = { wifi_enabled: false, bluetooth_enabled: false };
  let loading = false;
  let error: string | null = null;

  async function loadState(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      networkState = await getNetworkState();
    } catch (e) {
      error = `Failed to load network state: ${e}`;
      console.error(error);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleToggleWifi() {
    try {
      loading = true;
      error = null;
      const newState = await toggleWifi();
      networkState = { ...networkState, wifi_enabled: newState };
    } catch (e) {
      error = `Failed to toggle WiFi: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleToggleBluetooth() {
    try {
      loading = true;
      error = null;
      const newState = await toggleBluetooth();
      networkState = { ...networkState, bluetooth_enabled: newState };
    } catch (e) {
      error = `Failed to toggle Bluetooth: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadState(true);
    // Refresh state every 3 seconds to catch external changes (without loading indicator)
    const interval = setInterval(() => loadState(false), 3000);
    return () => clearInterval(interval);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Network</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    <div class="control-row">
      <span class="control-label">WiFi</span>
      <button
        class="toggle-button {networkState.wifi_enabled ? 'on' : 'off'}"
        onclick={handleToggleWifi}
        disabled={loading}
      >
        {networkState.wifi_enabled ? 'On' : 'Off'}
      </button>
    </div>

    <div class="control-row">
      <span class="control-label">Bluetooth</span>
      <button
        class="toggle-button {networkState.bluetooth_enabled ? 'on' : 'off'}"
        onclick={handleToggleBluetooth}
        disabled={loading}
      >
        {networkState.bluetooth_enabled ? 'On' : 'Off'}
      </button>
    </div>

    {#if error}
      <div class="error-message">{error}</div>
    {/if}
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
    background: #666;
    color: white;
  }

  .toggle-button.off:hover:not(:disabled) {
    background: #555;
  }

  .toggle-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error-message {
    color: #ff6b6b;
    font-size: 12px;
    padding: 8px;
    background: rgba(255, 107, 107, 0.1);
    border-radius: 4px;
  }
</style>
