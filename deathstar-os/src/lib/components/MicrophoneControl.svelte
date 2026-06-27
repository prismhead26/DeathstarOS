<script lang="ts">
  import { toggleMicrophone, getMicrophoneMuteState, getMicrophoneVolume } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  const ctrl = createControl(
    { muted: false, volume: 50 },
    async () => {
      const [muted, volume] = await Promise.all([getMicrophoneMuteState(), getMicrophoneVolume()]);
      return { muted, volume };
    },
    { pollMs: 2000 },
  );

  const toggle = () =>
    ctrl.run(async () => {
      const muted = await toggleMicrophone();
      ctrl.data = { ...ctrl.data, muted };
    });
</script>

<ControlCard title="Microphone" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Microphone Input</span>
    <ToggleBtn active={!ctrl.data.muted} disabled={ctrl.loading} labels={['Active', 'Muted']} onclick={toggle} />
  </div>

  <span class="text-zinc-300 text-sm">Input Level: {Math.round(ctrl.data.volume)}%</span>
</ControlCard>
