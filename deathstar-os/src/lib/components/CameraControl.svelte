<script lang="ts">
  import { getCameraState, toggleCamera, openCameraSettings, type CameraState } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  const ctrl = createControl<CameraState>(
    { enabled: false, in_use: false, used_by: null },
    getCameraState,
    { pollMs: 5000 },
  );

  const toggle = () =>
    ctrl.run(async () => {
      const enabled = await toggleCamera();
      ctrl.data = { ...ctrl.data, enabled };
      ctrl.schedule(() => ctrl.load(), 500); // re-read once the daemon settles
    });
</script>

<ControlCard title="Camera" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Camera System</span>
    <ToggleBtn active={ctrl.data.enabled} disabled={ctrl.loading} labels={['Enabled', 'Disabled']} onclick={toggle} />
  </div>

  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Usage Status</span>
    <span class="px-3 py-1 rounded text-sm font-medium
      {ctrl.data.in_use ? 'bg-orange-500/20 text-orange-400' : 'bg-zinc-700 text-zinc-400'}">
      {ctrl.data.in_use ? '● In Use' : '○ Idle'}
    </span>
  </div>

  {#if ctrl.data.used_by}
    <div class="flex items-center gap-2 px-3 py-2 bg-orange-500/10 rounded-lg border-l-2 border-orange-500">
      <span class="text-zinc-400 text-sm">Used by:</span>
      <span class="text-orange-400 text-sm font-semibold font-mono">{ctrl.data.used_by}</span>
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
