import { createContext, useState, useEffect } from "react";
import PropTypes from 'prop-types';
import { AuthClient } from "@dfinity/auth-client";
import { AnonymousIdentity } from "@dfinity/agent";

export const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [isAuth, setIsAuth] = useState(false);
  const [identity, setIdentity] = useState(new AnonymousIdentity());

  useEffect(() => {
    init();
  }, []);

  async function init() {
    const authClient = await AuthClient.create();
    const identity = await authClient.getIdentity();
    setIdentity(identity);
    const principal = identity.getPrincipal();

    if (!principal.isAnonymous()) {
      setIsAuth(true);
    }
  }

  const login = async () => {
    const authClient = await AuthClient.create();
    await authClient.login({
      identityProvider: import.meta.env.VITE_REACT_APP_INTERNET_COMPUTER_PROVIDER,
      onSuccess: () => {
        console.log("Logged in");
        setIdentity(identity);
        setIsAuth(true);
      },
      onError: (err) => {
        console.error(err);
      },
    });
  };

  const logout = async () => {
    const authClient = await AuthClient.create();
    authClient.logout();
    setIdentity(new AnonymousIdentity());
    setIsAuth(false);
  };

  return (
      <AuthContext.Provider value={{ isAuth, login, logout, identity }}>
      {children}
    </AuthContext.Provider>
  );
}

AuthProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
