# Atenea Iris CiberSeguridad

Este es el proyecto **Atenea Iris CiberSeguridad**, desarrollado por [Carlos Galindo](https://github.com/CarlosGal19), []() para la Hackathon de Ciberseguridad. Este repositorio contiene el código fuente de las aplicaciones desarrolladas en **React Native y React**, así
como de los canister de **Rust y Motoko**.

## Requisitos

Antes de comenzar, asegúrate de tener instaladas las siguientes herramientas:

- [Node.js](https://nodejs.org/) (versión 14 o superior)
- [Git](https://git-scm.com/)
- [Expo CLI](https://docs.expo.dev/get-started/installation/)
- [DFX](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) (version 24)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Clonar el Repositorio

Para clonar este repositorio, sigue estos pasos:

1. Abre tu terminal o línea de comandos.
2. Ejecuta el siguiente comando:

   ```bash
   git clone https://github.com/CarlosGal19/Atenea-Iris-CiberSeguridad.git

3. Navega a la carpeta generada

   ```bash
   cd Atenea-Iris-CiberSeguridad

3. Instala las dependencias globales

   ```bash
   npm install

4. Crea el servidor local con tunnel para acceder desde el móvil

   ```bash
   npm run tunnel:start
   ```
   Vas a obtener una salida como: 
   ```bash
   > tunnel:start
   > npx --yes tsx ./src/tunnel/src/index.ts

   https://rotten-onions-yawn.loca.lt
   
6. Abre otra terminal, navega a internet_identity_middleware e instala las dependencias locales

   ```bash
   cd src/internet_identity_middleware
   npm install

7. Navega a la carpeta react-dashboard e instala las dependencias y declaraciones locales

   ```bash
   cd ../react-dashboard
   npm install
   dfx generate backend network nlp qrcode vectordb

8. Navega a la carpeta react-native-app e instala las dependencias locales

   ```bash
   cd ../react-native-app
   npm install
   dfx generate backend

9. Navega a la raíz del proyecto e inicia la réplica de dfx

    ```bash
    cd ../../
    dfx start --background --clean

10. Despliega el canister backend

    ```bash
    dfx deploy backend
   
   Obtendrás una salida como
   ```
   
