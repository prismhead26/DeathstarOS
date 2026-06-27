<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleDoNotDisturb, getDoNotDisturbState, openFocusSettings } from '../api/commands';

  let dndEnabled = false;
  let loading = false;
  let error: string | null = null;

  async function loadState(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      dndEnabled = await getDoNotDisturbState();
    } catch (e) {
      error = `Failed to load Do Not Disturb state: ${e}`;
      console.error(error);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleToggle() {
    try {
      loading = true;
      error = null;
      dndEnabled = await toggleDoNotDisturb();
    } catch (e) {
      error = `Failed to toggle Do Not Disturb: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleOpenSettings() {
    try {
      await openFocusSettings();
    } catch (e) {
      error = `Failed to open focus settings: ${e}`;
      console.error(error);
    }
  }

  onMount(() => {
    loadState(true);
    // Refresh state every 5 seconds to catch external changes (without loading indicator)
    const interval = setInterval(() => loadState(false), 5000);
    return () => clearInterval(interval);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Do Not Disturb</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    <div class="control-row">
      <span class="control-label">Focus Mode</span>
      <button
        class="toggle-button {dndEnabled ? 'on' : 'off'}"
        onclick={handleToggle}
        disabled={loading}
      >
        {dndEnabled ? 'On' : 'Off'}
      </button>
    </div>

    <button class="settings-button" onclick={handleOpenSettings}>
      Open Focus Settings
    </button>

    {#if error}
      <div class="error-message">{error}</div>
    {/if}

    <div class="info-text">
      Note: DND control may require macOS accessibility permissions or manual configuration. Using the settings button is recommended for full control.
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
    background: #9C27B0;
    color: white;
  }

  .toggle-button.on:hover:not(:disabled) {
    background: #7B1FA2;
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
</style>
