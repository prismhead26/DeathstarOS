<script lang="ts">
  import { onMount } from 'svelte';
  import { toggleWifi, toggleBluetooth, getNetworkState, type NetworkState } from '../api/commands';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  let state: NetworkState = $state({ wifi_enabled: false, bluetooth_enabled: false });
  let loading = $state(false);
  let error: string | null = $state(null);

  async function load(showLoading = false) {
    try {
      if (showLoading) loading = true;
      error = null;
      state = await getNetworkState();
    } catch (e) {
      error = String(e);
    } finally {
      if (showLoading) loading = false;
    }
  }

  async function toggle(fn: () => Promise<boolean>, key: keyof NetworkState) {
    try {
      loading = true;
      error = null;
      const val = await fn();
      state = { ...state, [key]: val };
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    load(true);
    const interval = setInterval(() => load(), 3000);
    return () => clearInterval(interval);
  });
</script>

<ControlCard title="Network" {loading} {error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">WiFi</span>
    <ToggleBtn active={state.wifi_enabled} disabled={loading} color="green"
      onclick={() => toggle(toggleWifi, 'wifi_enabled')} />
  </div>

  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Bluetooth</span>
    <ToggleBtn active={state.bluetooth_enabled} disabled={loading} color="green"
      onclick={() => toggle(toggleBluetooth, 'bluetooth_enabled')} />
  </div>
</ControlCard>
