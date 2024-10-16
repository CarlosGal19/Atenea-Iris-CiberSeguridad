import React, { useState } from 'react';
import { View, TextInput, TouchableOpacity, StyleSheet } from 'react-native';
import FontAwesome from '@expo/vector-icons/FontAwesome';
import AntDesign from '@expo/vector-icons/AntDesign';

export default function SearchInput() {
  const [text, setText] = useState('');

  return (
    <View style={styles.container}>
      {/* Ícono de lupa */}
      <FontAwesome name="search" size={20} color="gray" style={styles.iconLeft} />

      {/* Input */}
      <TextInput
        style={styles.input}
        placeholder="Buscar..."
        value={text}
        onChangeText={setText}
      />

      {/* Botón de tacha */}
      {text.length > 0 && (
        <TouchableOpacity onPress={() => setText('')}>
          <AntDesign name="closecircle" size={20} color="gray" style={styles.iconRight} />
        </TouchableOpacity>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: 'gray',
    borderRadius: 8,
    paddingHorizontal: 10,
    height: 40,
    width: '80%',
    backgroundColor: 'white',
  },
  iconLeft: {
    marginRight: 8,
  },
  input: {
    flex: 1,
    fontSize: 16,
  },
  iconRight: {
    marginLeft: 8,
  },
});
