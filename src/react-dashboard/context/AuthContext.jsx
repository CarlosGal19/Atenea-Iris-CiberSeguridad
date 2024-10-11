import { createContext, useState, useEffect } from "react";
import PropTypes from 'prop-types';
import { AuthClient } from "@dfinity/auth-client";

export const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [isAuth, setIsAuth] = useState(false);

  useEffect(() => {
    init();
  }, []);

  async function init() {
    const authClient = await AuthClient.create();
    const identity = await authClient.getIdentity();
    const principal = identity.getPrincipal();

    if (!principal.isAnonymous()) {
      setIsAuth(true);
    }
  }

  const login = async () => {
    const authClient = await AuthClient.create();
    await authClient.login({
      identityProvider: "http://bd3sg-teaaa-aaaaa-qaaba-cai.localhost:4943/",
      onSuccess: () => {
        console.log("Logged in");
      },
      onError: (err) => {
        console.error(err);
      },
    });
    setIsAuth(true);
  };

  const logout = async () => {
    const authClient = await AuthClient.create();
    authClient.logout();
    setIsAuth(false);
  };

  return (
      <AuthContext.Provider value={{ isAuth, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

AuthProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
