<script lang="ts">
  import AudioControl from './lib/components/AudioControl.svelte';
  import MicrophoneControl from './lib/components/MicrophoneControl.svelte';
  import CameraControl from './lib/components/CameraControl.svelte';
  import NetworkControl from './lib/components/NetworkControl.svelte';
  import BrightnessControl from './lib/components/BrightnessControl.svelte';
  import FocusControl from './lib/components/FocusControl.svelte';
</script>

<main>
  <div class="app-container">
    <header>
      <h1>DeathstarOS</h1>
      <p class="subtitle">System Control Center</p>
    </header>

    <div class="controls-container">
      <AudioControl />
      <MicrophoneControl />
      <CameraControl />
      <NetworkControl />
      <BrightnessControl />
      <FocusControl />
    </div>

    <footer>
      <p class="footer-text">
        Built with Tauri + Svelte • macOS Sequoia
      </p>
    </footer>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
      'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
      sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    background: #1e1e1e;
    color: #ffffff;
  }

  main {
    width: 100%;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    padding: 20px;
    box-sizing: border-box;
  }

  .app-container {
    width: 100%;
    max-width: 600px;
  }

  header {
    text-align: center;
    margin-bottom: 32px;
    padding-bottom: 24px;
    border-bottom: 1px solid #333;
  }

  h1 {
    margin: 0 0 8px 0;
    font-size: 32px;
    font-weight: 700;
    color: #ffffff;
  }

  .subtitle {
    margin: 0;
    font-size: 14px;
    color: #888;
  }

  .controls-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  footer {
    margin-top: 32px;
    padding-top: 24px;
    border-top: 1px solid #333;
    text-align: center;
  }

  .footer-text {
    margin: 0;
    font-size: 12px;
    color: #666;
  }
</style>
