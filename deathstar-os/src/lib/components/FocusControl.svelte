<script lang="ts">
  import { toggleDoNotDisturb, getDoNotDisturbState, openFocusSettings } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  const ctrl = createControl<boolean>(false, getDoNotDisturbState, { pollMs: 5000 });

  const toggle = () =>
    ctrl.run(async () => {
      ctrl.data = await toggleDoNotDisturb();
    });
</script>

<ControlCard title="Do Not Disturb" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Focus Mode</span>
    <ToggleBtn active={ctrl.data} disabled={ctrl.loading} color="purple" onclick={toggle} />
  </div>

  <button
    class="w-full py-2 px-4 rounded-lg border border-zinc-600 bg-zinc-700 text-white text-sm
           hover:bg-zinc-600 hover:border-zinc-500 transition-colors cursor-pointer"
    onclick={openFocusSettings}
  >
    Open Focus Settings
  </button>

  <p class="text-zinc-600 text-xs leading-relaxed">
    DND control may require macOS accessibility permissions. Use Focus Settings for full control.
  </p>
</ControlCard>
