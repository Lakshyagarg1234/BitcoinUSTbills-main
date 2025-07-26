import { actor } from "./agent";
import type { VerifiedBrokerPurchase } from "$lib/types";
import { get } from "svelte/store";

export async function getAllVerifiedBrokerPurchases(): Promise<VerifiedBrokerPurchase[]> {
  const backendActor = get(actor);
  if (!backendActor) {
    throw new Error("Actor not initialized");
  }
  return await backendActor.get_all_verified_broker_purchases();
}
