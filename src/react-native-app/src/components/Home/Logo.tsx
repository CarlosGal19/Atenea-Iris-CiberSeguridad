import React from 'react';
import { View, Text, Image, StyleSheet, useWindowDimensions } from 'react-native';

export default function Banner() {
  const { width } = useWindowDimensions(); // Obtener el ancho del dispositivo

  return (
    <View style={styles.container}>
      {/* Imagen con el texto superpuesto */}
      <Image
        source={{ uri: 'https://via.placeholder.com/200x150' }} // URL de ejemplo, reemplázala con la imagen deseada
        style={[styles.image, { width: width }]} // Imagen que abarca el ancho completo del dispositivo
      />
      {/* Texto superpuesto en la esquina inferior izquierda */}
      <Text style={styles.text}>
        Denuncia fácil,{"\n"}estamos para ayudarte
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'relative',
    padding: 32
  },
  image: {
    height: 250,
    // resizeMode: 'cover',
  },
  text: {
    position: 'absolute',
    bottom: 10,
    left: 10,
    color: 'black',
    fontSize: 16,
    fontWeight: 'bold',
    padding: 8,
    borderRadius: 5,
  },
});
