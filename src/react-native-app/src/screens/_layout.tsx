import { Stack } from "expo-router/stack";
import React from "react";

export default function Layout() {
  return (
    <>
      <Stack
        // screenOptions={{
        //   headerStyle: {
        //     backgroundColor: "#8fc5e4",
        //   },
        //   headerTintColor: "black",
        //   headerTitleStyle: {
        //     fontWeight: "bold",
        //   },
        //   headerTitle: "Mafercilla",
        // }}
        screenOptions={{
          headerShown: false,
        }}
      />
    </>
  );
}
