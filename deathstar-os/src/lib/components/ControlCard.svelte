<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, loading = false, error = null, children }: {
    title: string;
    loading?: boolean;
    error?: string | null;
    children: Snippet;
  } = $props();
</script>

<div class="bg-zinc-800 rounded-xl p-5">
  <div class="flex justify-between items-center mb-4">
    <h3 class="text-white text-lg font-semibold m-0">{title}</h3>
    {#if loading}
      <span class="text-zinc-500 text-sm tracking-widest">···</span>
    {/if}
  </div>

  <div class="flex flex-col gap-4">
    {@render children()}
  </div>

  {#if error}
    <div class="mt-3 text-red-400 text-xs p-2 bg-red-400/10 rounded">{error}</div>
  {/if}
</div>
