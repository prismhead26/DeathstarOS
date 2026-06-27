<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleAudioMute, getAudioState, setAudioVolume, type AudioState } from '../api/commands';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  let state: AudioState = $state({ muted: false, volume: 50 });
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      state = await getAudioState();
    } catch (e) {
      error = String(e);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function toggleMute() {
    try {
      loading = true;
      error = null;
      const muted = await toggleAudioMute();
      state = { ...state, muted };
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function setVolume(e: Event) {
    const volume = parseFloat((e.target as HTMLInputElement).value);
    try {
      error = null;
      await setAudioVolume(volume);
      state = { ...state, volume };
    } catch (e) {
      error = String(e);
    }
  }

  onMount(() => {
    load(true);
    const interval = setInterval(() => load(), 2000);
    return () => clearInterval(interval);
  });
</script>

<ControlCard title="Audio" {loading} {error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">System Audio</span>
    <ToggleBtn active={!state.muted} disabled={loading} labels={['On', 'Muted']} color="green" onclick={toggleMute} />
  </div>

  <div class="flex flex-col gap-2">
    <span class="text-zinc-300 text-sm">Volume: {Math.round(state.volume)}%</span>
    <input
      type="range" min="0" max="100" value={state.volume}
      oninput={setVolume}
      disabled={loading || state.muted}
      class="thumb-blue bg-zinc-700"
    />
  </div>
</ControlCard>
