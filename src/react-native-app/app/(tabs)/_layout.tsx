import { AuthProvider } from "../../context/AuthContext";
import { Tabs } from "expo-router";

export default function Layout({ children }) {
  return (
    <AuthProvider>
      <Tabs
        screenOptions={{
          headerShown: false,
          tabBarActiveTintColor: "black",
        }}
      >
        <Tabs.Screen
          name="index"
          options={{
            title: "Home",
          }}
        />
        <Tabs.Screen
          name="about"
          options={{
            title: "About",
          }}
        />
      </Tabs>
    </AuthProvider>
  );
}
