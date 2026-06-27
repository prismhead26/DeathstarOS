<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleAudioMute, getAudioState, setAudioVolume, type AudioState } from '../api/commands';

  let audioState: AudioState = { muted: false, volume: 50 };
  let loading = false;
  let error: string | null = null;

  async function loadAudioState(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      audioState = await getAudioState();
    } catch (e) {
      error = `Failed to load audio state: ${e}`;
      console.error(error);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleToggleMute() {
    try {
      loading = true;
      error = null;
      const newMuteState = await toggleAudioMute();
      audioState = { ...audioState, muted: newMuteState };
    } catch (e) {
      error = `Failed to toggle mute: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleVolumeChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const newVolume = parseFloat(target.value);

    try {
      error = null;
      await setAudioVolume(newVolume);
      audioState = { ...audioState, volume: newVolume };
    } catch (e) {
      error = `Failed to set volume: ${e}`;
      console.error(error);
    }
  }

  onMount(() => {
    loadAudioState(true); // Show loading on initial load
    // Refresh state every 2 seconds to catch external changes (without loading indicator)
    const interval = setInterval(() => loadAudioState(false), 2000);
    return () => clearInterval(interval);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Audio</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    <div class="control-row">
      <span class="control-label">System Audio</span>
      <button
        class="toggle-button {audioState.muted ? 'off' : 'on'}"
        onclick={handleToggleMute}
        disabled={loading}
      >
        {audioState.muted ? 'Muted' : 'On'}
      </button>
    </div>

    <div class="control-row">
      <span class="control-label">Volume: {Math.round(audioState.volume)}%</span>
      <input
        type="range"
        min="0"
        max="100"
        value={audioState.volume}
        oninput={handleVolumeChange}
        disabled={loading || audioState.muted}
        class="volume-slider"
      />
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

  .volume-slider {
    flex: 1;
    height: 6px;
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
    background: #444;
  }

  .volume-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #2196F3;
    cursor: pointer;
  }

  .volume-slider::-moz-range-thumb {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #2196F3;
    cursor: pointer;
    border: none;
  }

  .volume-slider:disabled {
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
