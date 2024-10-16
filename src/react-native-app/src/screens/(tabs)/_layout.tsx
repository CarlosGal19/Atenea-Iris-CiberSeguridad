import { Tabs } from "expo-router";
import React from "react";
import AntDesign from "@expo/vector-icons/AntDesign";
import FontAwesome from "@expo/vector-icons/FontAwesome";
import MaterialCommunityIcons from "@expo/vector-icons/MaterialCommunityIcons";
import FontAwesome5 from "@expo/vector-icons/FontAwesome5";
import MaterialIcons from "@expo/vector-icons/MaterialIcons";
import { View, StyleSheet } from "react-native";

export default function TabsLayout({ children }) {
  return (
    <>
      <Tabs
        screenOptions={{
          headerShown: false,
          tabBarStyle: {
            backgroundColor: "#f8f9fa",
          },
        }}
      >
        <Tabs.Screen
          name="index"
          options={{
            title: "",
            tabBarIcon: ({ color, focused }) => (
              <FontAwesome
                name="home"
                size={24}
                color={focused ? "blue" : "black"}
              />
            ),
          }}
        />
        <Tabs.Screen
          name="Historial"
          options={{
            title: "",
            tabBarIcon: ({ color, focused }) => (
              <MaterialCommunityIcons
                name="history"
                size={24}
                color={focused ? "blue" : "black"}
              />
            ),
          }}
        />
        <Tabs.Screen
          name="CrearDenunciaScreen"
          options={{
            title: "",
            tabBarIcon: ({ focused }) => (
              <View style={[styles.iconContainer, focused && styles.focused]}>
                <MaterialIcons name="edit-square" size={24} color="white" />
              </View>
            ),
          }}
        />
        <Tabs.Screen
          name="Emergencia"
          options={{
            title: "",
            tabBarIcon: ({ color, focused }) => (
              <FontAwesome5
                name="ambulance"
                size={24}
                color={focused ? "blue" : "black"}
              />
            ),
          }}
        />
        <Tabs.Screen
          name="Soporte"
          options={{
            title: "",
            tabBarIcon: ({ color, focused }) => (
              <AntDesign
                name="questioncircleo"
                size={24}
                color={focused ? "blue" : "black"}
              />
            ),
          }}
        />
      </Tabs>
    </>
  );
}

const styles = StyleSheet.create({
  iconContainer: {
    width: 60,
    height: 60,
    borderRadius: 30,
    backgroundColor: "purple",
    justifyContent: "center",
    alignItems: "center",
    marginBottom: 20,
    borderWidth: 6,
    borderColor: 'white',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 1,
    },
    shadowOpacity: 0.2,
    shadowRadius: 1.41,
    elevation: 2, // Para Android
  },
  focused: {
    backgroundColor: "darkviolet",
  },
});
