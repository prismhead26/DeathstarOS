<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleMicrophone, getMicrophoneMuteState, getMicrophoneVolume } from '../api/commands';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  let muted = $state(false);
  let volume = $state(50);
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      [muted, volume] = await Promise.all([getMicrophoneMuteState(), getMicrophoneVolume()]);
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
      muted = await toggleMicrophone();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    load(true);
    const interval = setInterval(() => load(), 2000);
    return () => clearInterval(interval);
  });
</script>

<ControlCard title="Microphone" {loading} {error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Microphone Input</span>
    <ToggleBtn active={!muted} disabled={loading} labels={['Active', 'Muted']} color="green" onclick={toggle} />
  </div>

  <span class="text-zinc-400 text-sm">Input Level: {Math.round(volume)}%</span>
</ControlCard>
