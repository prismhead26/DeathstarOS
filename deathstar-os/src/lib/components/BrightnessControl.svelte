<script lang="ts">
  import { onMount } from 'svelte';
  import { getBrightness, setBrightness } from '../api/commands';
  import ControlCard from './ControlCard.svelte';

  let brightness = $state(50);
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      brightness = await getBrightness();
    } catch (e) {
      error = String(e);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function setLevel(e: Event) {
    const level = parseFloat((e.target as HTMLInputElement).value);
    try {
      error = null;
      await setBrightness(level);
      brightness = level;
    } catch (e) {
      error = String(e);
    }
  }

  onMount(() => load(true));
</script>

<ControlCard title="Display Brightness" {loading} {error}>
  <div class="flex flex-col gap-2">
    <span class="text-zinc-300 text-sm">Brightness: {Math.round(brightness)}%</span>
    <input
      type="range" min="0" max="100" value={brightness}
      oninput={setLevel}
      disabled={loading}
      class="thumb-yellow bg-zinc-700"
    />
  </div>
</ControlCard>
