import { StatusBar } from "expo-status-bar";
import { ScrollView, View, StyleSheet, Button, Alert } from "react-native";
import Header from "../../components/Home/Header";
import Categories from "../../components/Home/Categories";
import Report from "../../components/Home/Report";
import Logo from "../../components/Home/Logo";

import { useAuth, useCandidActor } from "@bundly/ares-react";
import { InternetIdentityMidlewareButton, LogoutButton } from "@bundly/ares-react-native";
import { CandidActors } from "../../canisters";

import React from "react";

export default function IndexScreen() {
  const { isAuthenticated, currentIdentity } = useAuth();
  const backend = useCandidActor<CandidActors>("backend", currentIdentity) as CandidActors["backend"];

  console.log("currentIdentity", currentIdentity);
  console.log("isAuthenticated", isAuthenticated);
  console.log(process.env.EXPO_PUBLIC_IC_HOST_URL, process.env.EXPO_PUBLIC_INTERNET_IDENTITY_MIDDLEWARE_URL, process.env.EXPO_PUBLIC_APP_LINK);

  const handlePress = async () => {
    // Alert.alert("Principal", currentIdentity.getPrincipal().toString());
    // console.log("Principal", currentIdentity.getPrincipal().toString());
    const response = await backend.greet("Carlos");
    console.log(response);
  };


  return (
    <ScrollView
      style={{
        paddingTop: 20,
      }}
    >
      {isAuthenticated ? <LogoutButton identity={currentIdentity} /> : <InternetIdentityMidlewareButton />}
      <Button title="Get principal" onPress={handlePress} />
      <StatusBar style="auto" />
      <Header />
      <View style={styles.container}>
        <Categories />
        <Logo />
        <Report />
        <Report />
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '85%',
    alignItems: 'center',
    justifyContent: 'center',
    alignSelf: 'center',
    paddingTop: 10,
    paddingBottom: 20
  },
});
