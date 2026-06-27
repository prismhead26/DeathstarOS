<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleMicrophone, getMicrophoneMuteState, getMicrophoneVolume } from '../api/commands';

  let muted = false;
  let volume = 50;
  let loading = false;
  let error: string | null = null;

  async function loadState(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      muted = await getMicrophoneMuteState();
      volume = await getMicrophoneVolume();
    } catch (e) {
      error = `Failed to load microphone state: ${e}`;
      console.error(error);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleToggle() {
    try {
      loading = true;
      error = null;
      muted = await toggleMicrophone();
    } catch (e) {
      error = `Failed to toggle microphone: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadState(true);
    // Refresh state every 2 seconds to catch external changes (without loading indicator)
    const interval = setInterval(() => loadState(false), 2000);
    return () => clearInterval(interval);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Microphone</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    <div class="control-row">
      <span class="control-label">Microphone Input</span>
      <button
        class="toggle-button {muted ? 'off' : 'on'}"
        onclick={handleToggle}
        disabled={loading}
      >
        {muted ? 'Muted' : 'Active'}
      </button>
    </div>

    <div class="control-row">
      <span class="control-label">Input Level: {Math.round(volume)}%</span>
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

  .error-message {
    color: #ff6b6b;
    font-size: 12px;
    padding: 8px;
    background: rgba(255, 107, 107, 0.1);
    border-radius: 4px;
  }
</style>
