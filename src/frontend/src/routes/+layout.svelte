<script>
  import Toasts from "$lib/components/ui/Toasts.svelte";
  import { onMount } from 'svelte';
  import { authStore, initAuth, login, logout } from '$lib/auth';

  onMount(async () => {
    await initAuth();
  });
</script>

<header class="bg-white shadow-md">
  <nav class="container mx-auto px-6 py-3">
    <div class="flex justify-between items-center">
      <a href="/" class="text-xl font-semibold text-gray-700">USTBills on ICP</a>
      <div>
        <a href="/" class="text-gray-700 hover:text-blue-500 mx-2">Home</a>
        <a href="/public-ledger" class="text-gray-700 hover:text-blue-500 mx-2">Public Ledger</a>
        {#if $authStore.isLoggedIn}
          <button on:click={logout} class="text-gray-700 hover:text-blue-500 mx-2">Logout</button>
        {:else}
          <button on:click={login} class="text-gray-700 hover:text-blue-500 mx-2">Login</button>
        {/if}
      </div>
    </div>
  </nav>
</header>

<Toasts />

<main>
  <slot />
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, sans-serif;
    background: #f8fafc;
    color: #1a202c;
  }

  :global(*) {
    box-sizing: border-box;
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100vh;
    padding: 2rem;
  }
</style>
