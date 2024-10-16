import React from "react";
import {
  View,
  TouchableOpacity,
  StyleSheet,
  useWindowDimensions,
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import AntDesign from "@expo/vector-icons/AntDesign";
import FontAwesome from "@expo/vector-icons/FontAwesome";
import SearchInput from "./SearchInput";

export default function Header() {
  const { height } = useWindowDimensions();

  return (
    <LinearGradient
      colors={["#800080", "#FF00FF"]}
      style={{ height: height * 0.15 }}
    >
      <View style={styles.container}>
        <TouchableOpacity>
          <AntDesign name="menuunfold" size={24} color="white" />
        </TouchableOpacity>
        <View style={styles.buttonContainer}>
          <TouchableOpacity>
            <FontAwesome name="bell" size={24} color="white" />
          </TouchableOpacity>
          <TouchableOpacity>
            <FontAwesome name="gear" size={24} color="white" />
          </TouchableOpacity>
        </View>
      </View>
      <View style={styles.searchInput}>
        <SearchInput />
      </View>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "row",
    justifyContent: "space-between",
    paddingHorizontal: 16,
    paddingTop: 16,
  },
  buttonContainer: {
    gap: 16,
    flexDirection: "row",
  },
  searchInput: {
    alignItems: "center",
    justifyContent: "center",
    marginTop: 40,
    marginBottom: 8,
  },
});
