<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllVerifiedBrokerPurchases } from '$lib/api';
  import type { VerifiedBrokerPurchase } from '$lib/types';

  let purchases: VerifiedBrokerPurchase[] = [];
  let isLoading = true;
  let error: string | null = null;

  onMount(async () => {
    try {
      purchases = await getAllVerifiedBrokerPurchases();
    } catch (e) {
      error = 'Error fetching purchase records.';
      console.error(e);
    } finally {
      isLoading = false;
    }
  });
</script>

<div class="container mx-auto p-8">
  <h1 class="text-3xl font-bold mb-6">Public Purchase Ledger</h1>
  <p class="mb-8 text-gray-600">
    This ledger provides a transparent, on-chain record of all T-Bill purchases executed through our partner brokers. Each entry is permanently stored on the blockchain for public verification.
  </p>

  {#if isLoading}
    <p>Loading records...</p>
  {:else if error}
    <p class="text-red-500">{error}</p>
  {:else if purchases.length === 0}
    <p>No purchase records found.</p>
  {:else}
    <div class="overflow-x-auto shadow-md sm:rounded-lg">
      <table class="min-w-full bg-white">
        <thead class="bg-gray-800 text-white">
          <tr>
            <th class="py-3 px-6 text-left">Timestamp</th>
            <th class="py-3 px-6 text-left">T-Bill Type</th>
            <th class="py-3 px-6 text-left">Amount</th>
            <th class="py-3 px-6 text-left">Price</th>
            <th class="py-3 px-6 text-left">Broker Txn ID</th>
          </tr>
        </thead>
        <tbody class="text-gray-700">
          {#each purchases as purchase, i}
            <tr class="border-b hover:bg-gray-100 {i % 2 === 0 ? 'bg-gray-50' : ''}">
              <td class="py-3 px-6">{new Date(Number(purchase.timestamp) * 1000).toLocaleString()}</td>
              <td class="py-3 px-6">{purchase.ustbill_type}</td>
              <td class="py-3 px-6">{purchase.amount}</td>
              <td class="py-3 px-6">${(Number(purchase.price) / 100).toFixed(2)}</td>
              <td class="py-3 px-6 font-mono">{purchase.broker_txn_id}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>