<script lang="ts">
  import AudioControl from './lib/components/AudioControl.svelte';
  import MicrophoneControl from './lib/components/MicrophoneControl.svelte';
  import CameraControl from './lib/components/CameraControl.svelte';
  import NetworkControl from './lib/components/NetworkControl.svelte';
  import BrightnessControl from './lib/components/BrightnessControl.svelte';
  import FocusControl from './lib/components/FocusControl.svelte';
</script>

<main class="min-h-screen bg-zinc-900 flex justify-center px-5 py-8 box-border">
  <div class="w-full max-w-lg">
    <header class="text-center mb-8 pb-6 border-b border-zinc-700">
      <h1 class="text-3xl font-bold text-white m-0 mb-2">DeathstarOS</h1>
      <p class="text-zinc-500 text-sm m-0">System Control Center</p>
    </header>

    <div class="flex flex-col gap-4">
      <AudioControl />
      <MicrophoneControl />
      <CameraControl />
      <NetworkControl />
      <BrightnessControl />
      <FocusControl />
    </div>

    <footer class="mt-8 pt-6 border-t border-zinc-700 text-center">
      <p class="text-zinc-600 text-xs m-0">Built with Tauri + Svelte · macOS</p>
    </footer>
  </div>
</main>
