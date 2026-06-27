<script lang="ts">
  import { toggleAudioMute, getAudioState, setAudioVolume, type AudioState } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  const ctrl = createControl<AudioState>({ muted: false, volume: 50 }, getAudioState, { pollMs: 2000 });

  const toggleMute = () =>
    ctrl.run(async () => {
      const muted = await toggleAudioMute();
      ctrl.data = { ...ctrl.data, muted };
    });

  const setVolume = (e: Event) => {
    const volume = parseFloat((e.target as HTMLInputElement).value);
    ctrl.data = { ...ctrl.data, volume }; // optimistic; poll is paused mid-drag
    ctrl.runQuiet(() => setAudioVolume(volume));
  };
</script>

<ControlCard title="Audio" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">System Audio</span>
    <ToggleBtn active={!ctrl.data.muted} disabled={ctrl.loading} labels={['On', 'Muted']} onclick={toggleMute} />
  </div>

  <div class="flex flex-col gap-2">
    <span class="text-zinc-300 text-sm">Volume: {Math.round(ctrl.data.volume)}%</span>
    <input
      type="range" min="0" max="100" value={ctrl.data.volume}
      oninput={setVolume}
      onpointerdown={ctrl.pausePolling}
      onpointerup={ctrl.resumePolling}
      disabled={ctrl.loading || ctrl.data.muted}
      aria-label="Volume"
      class="thumb-blue bg-zinc-700"
    />
  </div>
</ControlCard>
