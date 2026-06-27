<script lang="ts">
  import { toggleWifi, toggleBluetooth, getNetworkState, type NetworkState } from '../api/commands';
  import { createControl } from '../composables/useControl.svelte';
  import ControlCard from './ControlCard.svelte';
  import ToggleBtn from './ToggleBtn.svelte';

  const ctrl = createControl<NetworkState>(
    { wifi_enabled: false, bluetooth_enabled: false },
    getNetworkState,
    { pollMs: 3000 },
  );

  const toggle = (fn: () => Promise<boolean>, key: keyof NetworkState) =>
    ctrl.run(async () => {
      const val = await fn();
      ctrl.data = { ...ctrl.data, [key]: val };
    });
</script>

<ControlCard title="Network" loading={ctrl.loading} error={ctrl.error}>
  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">WiFi</span>
    <ToggleBtn active={ctrl.data.wifi_enabled} disabled={ctrl.loading}
      onclick={() => toggle(toggleWifi, 'wifi_enabled')} />
  </div>

  <div class="flex justify-between items-center">
    <span class="text-zinc-300 text-sm">Bluetooth</span>
    <ToggleBtn active={ctrl.data.bluetooth_enabled} disabled={ctrl.loading}
      onclick={() => toggle(toggleBluetooth, 'bluetooth_enabled')} />
  </div>
</ControlCard>
