import { AuthClient } from "@dfinity/auth-client";
import { DelegationIdentity, Ed25519PublicKey } from "@dfinity/identity";
import { useContext } from "react";

import { IncompleteEd25519KeyIdentity } from "@bundly/ares-core";

import { AuthContext } from "../context/AuthContext";

export default function LoginButton() {
  const { setAuthError, authenticate, publicKey, setDelegation } = useContext(AuthContext);

  if (!publicKey) {
    throw new Error("No public key found");
  }

  async function handleClick(publicKey: Ed25519PublicKey) {
    try {
      const incompleteIdentity = new IncompleteEd25519KeyIdentity(publicKey);

      const client = await AuthClient.create({
        identity: incompleteIdentity,
        idleOptions: {
          disableIdle: true,
        }
      });

      console.log("client", client);

      client.login({
        identityProvider: process.env.VITE_INTERNET_IDENTITY_URL,
        onSuccess: async () => {
          const identity = client.getIdentity();
          if (identity instanceof DelegationIdentity) {
            const delegation = identity.getDelegation();

            setAuthError(null);
            setDelegation(delegation);
            authenticate();
          }
        },
        onError: () => {
          setAuthError(new Error("An error occurred"));
        },
      });
    } catch (e) {
      setAuthError(new Error("An error occurred"));
    }
  }

  return (
    <button
      onClick={() => handleClick(publicKey)}
      className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full">
      Login
    </button>
  );
}
