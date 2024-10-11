import { View, Text, Button, StyleSheet } from "react-native";
import { useContext } from "react";
import { AuthContext } from "../../context/AuthContext";

export default function Index() {
  const { login, logout, isAuth } = useContext(AuthContext);
  return (
    <View style={styles.container}>
      <Text>React Native App</Text>
      <Text>{isAuth ? "Logged in" : "Logged out"}</Text>
      <Button
        title={`${isAuth ? "Logout" : "Login"}`}
        onPress={isAuth ? logout : login}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
});
