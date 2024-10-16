import React from 'react';
import { View, Text, StyleSheet, Image, Pressable, useWindowDimensions } from 'react-native';
import { useNavigation } from 'expo-router';

export default function Report() {
  const { height, width } = useWindowDimensions();
  const navigation = useNavigation();

  return (
    <View style={[styles.container, { height: height * 0.25, width: width }]}>
      <View style={styles.contentContainer}>
        {/* Imagen al costado izquierdo */}
        <Image
          source={{ uri: 'https://via.placeholder.com/100' }}
          style={styles.image}
        />

        <View style={styles.textContainer}>
          {/* Título en la parte superior central */}
          <Text style={styles.title}>Título del Reporte</Text>

          {/* Resumen debajo del título */}
          <Text style={styles.summary}>Resumen corto sobre el contenido del reporte.</Text>

          {/* Descripción debajo del resumen */}
          <Text style={styles.description}>
            Aquí va una breve descripción más detallada sobre el reporte o información relevante.
          </Text>
        </View>
      </View>

      {/* Pressable que abarca todo el ancho */}
      <Pressable style={styles.button} onPress={() => navigation.navigate('CrearDenunciaScreen' as never)}>
        <Text style={styles.buttonText}>Hacer denuncia</Text>
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: 'white',
    borderRadius: 8,
    paddingTop: 32,
    paddingHorizontal: 16,
    marginVertical: 10,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 5,
  },
  contentContainer: {
    flexDirection: 'row',
    marginBottom: 16,
  },
  image: {
    width: 100,
    height: 100,
    borderRadius: 8,
    marginRight: 16,
  },
  textContainer: {
    flex: 1,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  summary: {
    fontSize: 14,
    color: 'orange',
    marginBottom: 4,
  },
  description: {
    fontSize: 12,
    color: 'darkgray',
  },
  button: {
    backgroundColor: 'blue',
    paddingVertical: 10,
    alignItems: 'center',
    borderRadius: 4,
    width: "80%",
    alignSelf: 'center',
  },
  buttonText: {
    color: 'white',
    fontWeight: 'bold',
    fontSize: 16,
  },
});
