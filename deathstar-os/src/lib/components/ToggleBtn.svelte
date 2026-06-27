<script lang="ts">
  let { active, disabled = false, labels = ['On', 'Off'], color = 'green', onclick }: {
    active: boolean;
    disabled?: boolean;
    labels?: [string, string];
    color?: 'green' | 'purple';
    onclick: () => void;
  } = $props();

  const activeColors = {
    green:  'bg-green-500 hover:bg-green-600',
    purple: 'bg-purple-600 hover:bg-purple-700',
  };
</script>

<button
  class="min-w-20 px-5 py-2 rounded-lg text-sm font-medium text-white transition-colors cursor-pointer
         disabled:opacity-40 disabled:cursor-not-allowed
         {active ? activeColors[color] : 'bg-zinc-600 hover:bg-zinc-500'}"
  aria-pressed={active}
  {disabled}
  {onclick}
>
  {active ? labels[0] : labels[1]}
</button>
