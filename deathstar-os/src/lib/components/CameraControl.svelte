<script lang="ts">
  import { onMount } from 'svelte';
  import { getCameraState, toggleCamera, openCameraSettings, type CameraState } from '../api/commands';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  let state: CameraState = $state({ enabled: false, in_use: false, used_by: null });
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      state = await getCameraState();
    } catch (e) {
      error = String(e);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function toggle() {
    try {
      loading = true;
      error = null;
      const enabled = await toggleCamera();
      state = { ...state, enabled };
      setTimeout(() => load(), 500);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    load(true);
    const interval = setInterval(() => load(), 5000);
    return () => clearInterval(interval);
  });
</script>

<ControlCard title="Camera" {loading} {error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Camera System</span>
    <ToggleBtn active={state.enabled} disabled={loading} labels={['Enabled', 'Disabled']} color="green" onclick={toggle} />
  </div>

  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Usage Status</span>
    <span class="px-3 py-1 rounded text-sm font-medium
      {state.in_use ? 'bg-orange-500/20 text-orange-400' : 'bg-zinc-700 text-zinc-400'}">
      {state.in_use ? '● In Use' : '○ Idle'}
    </span>
  </div>

  {#if state.used_by}
    <div class="flex items-center gap-2 px-3 py-2 bg-orange-500/10 rounded-lg border-l-2 border-orange-500">
      <span class="text-zinc-400 text-sm">Used by:</span>
      <span class="text-orange-400 text-sm font-semibold font-mono">{state.used_by}</span>
    </div>
  {/if}

  <button
    class="w-full py-2 px-4 rounded-lg border border-zinc-600 bg-zinc-700 text-white text-sm
           hover:bg-zinc-600 hover:border-zinc-500 transition-colors cursor-pointer"
    onclick={openCameraSettings}
  >
    Open Camera Settings
  </button>

  <p class="text-zinc-600 text-xs leading-relaxed">
    macOS restarts the camera daemon automatically. Use Camera Settings to manage per-app permissions.
  </p>
</ControlCard>
