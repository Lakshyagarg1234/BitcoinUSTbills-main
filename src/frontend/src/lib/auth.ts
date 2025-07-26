
import { writable, get } from 'svelte/store';
import { AuthClient } from '@dfinity/auth-client';
import type { Identity } from '@dfinity/agent';

interface AuthState {
  isLoggedIn: boolean;
  identity: Identity | null;
  authClient: AuthClient | null;
}

const defaultState: AuthState = {
  isLoggedIn: false,
  identity: null,
  authClient: null,
};

export const authStore = writable<AuthState>(defaultState);

export async function initAuth() {
  const authClient = await AuthClient.create();
  const isLoggedIn = await authClient.isAuthenticated();
  const identity = isLoggedIn ? authClient.getIdentity() : null;

  authStore.set({ isLoggedIn, identity, authClient });
}

export async function login() {
  const authClient = get(authStore).authClient;
  if (!authClient) return;

  await authClient.login({
    identityProvider: 'https://identity.ic0.app',
    onSuccess: async () => {
      const identity = authClient.getIdentity();
      authStore.update(store => ({ ...store, isLoggedIn: true, identity }));
    },
  });
}

export async function logout() {
  const authClient = get(authStore).authClient;
  if (!authClient) return;

  await authClient.logout();
  authStore.update(store => ({ ...store, isLoggedIn: false, identity: null }));
}
