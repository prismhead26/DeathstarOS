<script lang="ts">
  import { onMount } from 'svelte';
  import { getBrightness, setBrightness, openAccessibilitySettings } from '../api/commands';

  let brightness = 50;
  let loading = false;
  let error: string | null = null;
  let permissionRequired = false;

  async function loadBrightness(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      permissionRequired = false;
      brightness = await getBrightness();
    } catch (e) {
      const errorStr = String(e);
      if (errorStr.includes('PERMISSION_REQUIRED')) {
        permissionRequired = true;
        error = null;
      } else {
        error = `Failed to load brightness: ${e}`;
      }
      console.error(e);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function handleBrightnessChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const newBrightness = parseFloat(target.value);

    try {
      error = null;
      await setBrightness(newBrightness);
      brightness = newBrightness;
    } catch (e) {
      const errorStr = String(e);
      if (errorStr.includes('PERMISSION_REQUIRED')) {
        permissionRequired = true;
        error = null;
      } else {
        error = `Failed to set brightness: ${e}`;
      }
      console.error(e);
    }
  }

  async function handleOpenSettings() {
    try {
      await openAccessibilitySettings();
    } catch (e) {
      error = `Failed to open settings: ${e}`;
      console.error(e);
    }
  }

  onMount(() => {
    loadBrightness(true);
  });
</script>

<div class="control-card">
  <div class="control-header">
    <h3>Display Brightness</h3>
    {#if loading}
      <span class="loading-indicator">...</span>
    {/if}
  </div>

  <div class="control-content">
    {#if permissionRequired}
      <div class="permission-notice">
        <div class="notice-icon">⚠️</div>
        <div class="notice-content">
          <p class="notice-title">Brightness Control Unavailable</p>
          <p class="notice-text">
            Software brightness control doesn't work with your current display setup. This is a macOS hardware limitation for:
          </p>
          <ul class="notice-list">
            <li>External displays (DDC/CI not supported)</li>
            <li>Some built-in displays with hardware restrictions</li>
          </ul>
          <p class="notice-text">
            <strong>Alternative:</strong> Use keyboard brightness keys (F1/F2) or your display's physical buttons.
          </p>
        </div>
      </div>
    {:else}
      <div class="control-row">
        <span class="control-label">Brightness: {Math.round(brightness)}%</span>
        <input
          type="range"
          min="0"
          max="100"
          value={brightness}
          oninput={handleBrightnessChange}
          disabled={loading}
          class="brightness-slider"
        />
      </div>
    {/if}

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
    flex-direction: column;
    gap: 8px;
  }

  .control-label {
    color: #cccccc;
    font-size: 14px;
  }

  .brightness-slider {
    width: 100%;
    height: 6px;
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
    background: #444;
  }

  .brightness-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #FFC107;
    cursor: pointer;
  }

  .brightness-slider::-moz-range-thumb {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #FFC107;
    cursor: pointer;
    border: none;
  }

  .brightness-slider:disabled {
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

  .permission-notice {
    display: flex;
    gap: 16px;
    padding: 16px;
    background: rgba(255, 193, 7, 0.1);
    border-radius: 8px;
    border: 1px solid rgba(255, 193, 7, 0.3);
  }

  .notice-icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .notice-content {
    flex: 1;
  }

  .notice-title {
    margin: 0 0 8px 0;
    color: #FFC107;
    font-size: 14px;
    font-weight: 600;
  }

  .notice-text {
    margin: 0 0 8px 0;
    color: #ccc;
    font-size: 13px;
    line-height: 1.4;
  }

  .notice-list {
    margin: 0 0 12px 0;
    padding-left: 20px;
    color: #aaa;
    font-size: 12px;
  }

  .notice-list li {
    margin: 4px 0;
  }

  .permission-button {
    padding: 8px 16px;
    border-radius: 6px;
    border: none;
    background: #FFC107;
    color: #1e1e1e;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .permission-button:hover {
    background: #FFD54F;
  }
</style>
