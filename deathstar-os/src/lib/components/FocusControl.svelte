<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleDoNotDisturb, getDoNotDisturbState, openFocusSettings } from '../api/commands';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  let enabled = $state(false);
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      enabled = await getDoNotDisturbState();
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
      enabled = await toggleDoNotDisturb();
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

<ControlCard title="Do Not Disturb" {loading} {error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Focus Mode</span>
    <ToggleBtn active={enabled} disabled={loading} color="purple" onclick={toggle} />
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
