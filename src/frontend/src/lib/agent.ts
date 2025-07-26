import { Actor, HttpAgent, type Identity, type ActorSubclass } from "@dfinity/agent";
import { idlFactory } from "../../../declarations/backend";
import { writable } from "svelte/store";
import { authStore } from "./auth";
import type { _SERVICE } from "../../../declarations/backend/backend.did";

const canisterId = import.meta.env.VITE_CANISTER_ID_BACKEND;

async function createActor(agent: HttpAgent) {
  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId,
  });
}

function createAgent(identity: Identity) {
  return new HttpAgent({
    identity,
    host: import.meta.env.VITE_DFX_NETWORK === "ic" ? "https://ic0.app" : "http://localhost:4943",
  });
}

export const actor = writable<ActorSubclass<_SERVICE> | null>(null);

authStore.subscribe(async ({ identity }) => {
  if (identity) {
    const agent = createAgent(identity);
    const newActor = await createActor(agent);
    actor.set(newActor);
  } else {
    actor.set(null);
  }
});