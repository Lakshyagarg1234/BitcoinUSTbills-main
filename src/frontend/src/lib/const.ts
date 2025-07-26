export const HOST = import.meta.env.VITE_DFX_NETWORK != "ic" ? "http://localhost:8080" : "https://ic0.app"

export const FETCH_ROOT_KEY = import.meta.env.VITE_DFX_NETWORK != "ic"

export const IDENTITY_PROVIDER =
    import.meta.env.VITE_DFX_NETWORK === "ic"
        ? "https://identity.ic0.app"
        : `http://${import.meta.env.VITE_CANISTER_ID_II}.localhost:8080`;
