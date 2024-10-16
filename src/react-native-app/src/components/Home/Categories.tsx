import React from 'react';
import { View, Text, StyleSheet, useWindowDimensions } from 'react-native';
import FontAwesome from '@expo/vector-icons/FontAwesome';

export default function Categories() {
  const { width, height } = useWindowDimensions(); // Obtener las dimensiones de la pantalla

  return (
    <View style={[styles.container, { height: height * 0.1, width }]}>
      {/* Leyenda "Categorías" */}
      <Text style={styles.title}>Categorías</Text>

      {/* Barra de separación */}
      <View style={styles.separator} />

      {/* Contenedor de íconos */}
      <View style={styles.iconContainer}>
        <FontAwesome name="warning" size={24} color="black" />
        <FontAwesome name="phone" size={24} color="black" />
        <FontAwesome name="stop" size={24} color="black" />
        <FontAwesome name="edit" size={24} color="black" />
        <FontAwesome name="bicycle" size={24} color="black" />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: 'white',
    justifyContent: 'center',
    paddingHorizontal: 16,
    paddingTop: 8,
    paddingBottom: 10,
    borderRadius: 16, // Bordes redondeados
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    textAlign: 'left',
    color: '#333', // Color del texto
  },
  separator: {
    height: 2,
    backgroundColor: '#40E0D0', // Cambiado a fucsia
    marginVertical: 8,
    borderRadius: 1, // Bordes redondeados para la barra
  },
  iconContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
});
