import { AnonymousIdentity } from "@dfinity/agent";

export const anonymousIdentity = () => {
  const anon = new AnonymousIdentity();
  return anon.getPrincipal().toText();
};
