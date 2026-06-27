<script lang="ts">
  import { getBrightness, setBrightness } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';

  // No poll: brightness only changes via this slider or hardware keys.
  const ctrl = createControl<number>(50, getBrightness);

  const setLevel = (e: Event) => {
    const level = parseFloat((e.target as HTMLInputElement).value);
    ctrl.data = level; // optimistic
    ctrl.runQuiet(() => setBrightness(level));
  };
</script>

<ControlCard title="Display Brightness" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex flex-col gap-2">
    <span class="text-zinc-300 text-sm">Brightness: {Math.round(ctrl.data)}%</span>
    <input
      type="range" min="0" max="100" value={ctrl.data}
      oninput={setLevel}
      disabled={ctrl.loading}
      aria-label="Display brightness"
      class="thumb-yellow bg-zinc-700"
    />
  </div>
</ControlCard>
