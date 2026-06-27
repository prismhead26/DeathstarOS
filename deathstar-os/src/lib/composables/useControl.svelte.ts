import { onMount } from 'svelte';

/** Normalize an unknown thrown value into a readable message. */
function normalizeError(e: unknown): string {
  if (e instanceof Error) return e.message;
  if (typeof e === 'string') return e;
  try {
    return JSON.stringify(e);
  } catch {
    return String(e);
  }
}

export interface ControlOptions {
  /** If set, re-run the loader on this interval (ms) to track external changes. */
  pollMs?: number;
}

/**
 * Shared state machine for a system-control panel: holds reactive `data`,
 * `loading` and `error`, loads once on mount (optionally polling), and wraps
 * mutating actions with consistent loading/error handling.
 *
 * Returned object exposes reactive getters; read `ctrl.data` / `ctrl.loading`
 * / `ctrl.error` directly in markup.
 */
export function createControl<T>(
  initial: T,
  loader: () => Promise<T>,
  options: ControlOptions = {},
) {
  let data = $state(initial);
  let loading = $state(false);
  let error: string | null = $state(null);

  let inFlight = false;
  let paused = false;
  const timers = new Set<ReturnType<typeof setTimeout>>();

  async function load(showLoading = false) {
    if (inFlight) return; // never let polls overlap
    inFlight = true;
    try {
      if (showLoading) loading = true;
      error = null;
      data = await loader();
    } catch (e) {
      error = normalizeError(e);
    } finally {
      inFlight = false;
      if (showLoading) loading = false;
    }
  }

  /** Run a mutating action, toggling `loading` and capturing any error. */
  async function run(action: () => Promise<void>) {
    try {
      loading = true;
      error = null;
      await action();
    } catch (e) {
      error = normalizeError(e);
    } finally {
      loading = false;
    }
  }

  /** Like `run` but without the loading flag — for high-frequency actions
   *  (e.g. slider drags) where toggling `loading` would disable the control. */
  async function runQuiet(action: () => Promise<void>) {
    try {
      error = null;
      await action();
    } catch (e) {
      error = normalizeError(e);
    }
  }

  /** Schedule a one-shot callback that is auto-cancelled on unmount. */
  function schedule(fn: () => void, ms: number) {
    const id = setTimeout(() => {
      timers.delete(id);
      fn();
    }, ms);
    timers.add(id);
  }

  onMount(() => {
    load(true);
    let interval: ReturnType<typeof setInterval> | undefined;
    if (options.pollMs) {
      interval = setInterval(() => {
        if (!paused) load();
      }, options.pollMs);
    }
    return () => {
      if (interval) clearInterval(interval);
      timers.forEach(clearTimeout);
      timers.clear();
    };
  });

  return {
    get data() {
      return data;
    },
    set data(v: T) {
      data = v;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    load,
    run,
    runQuiet,
    schedule,
    pausePolling: () => {
      paused = true;
    },
    resumePolling: () => {
      paused = false;
    },
  };
}
