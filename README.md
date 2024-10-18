# Atenea Iris CiberSeguridad

Este es el proyecto **Atenea Iris CiberSeguridad**, desarrollado por [Carlos Galindo](https://github.com/CarlosGal19), [Jorge Fraidez](https://github.com/Jorge-Fraidez), [Sofia Campos](https://github.com/SofiaCamposH), [Maricruz Torres](https://github.com/Maricruz1900) y [Fernanda Rodríguez](https://github.com/Mafer-Rodriguez) para la Hackathon de Ciberseguridad. Este repositorio contiene el código fuente de las aplicaciones desarrolladas en **React Native y React**, así
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

10. Obten el id del canister backend, internet_identity, internet_identity_middleware, qrcode, newtwork y nlp

    ```bash
    dfx canister id backend
    dfx canister id internet_identity
    dfx canister id internet_identity_middleware
    dfx canister id qrcode
    dfx canister id newtwork
    dfx canister id nlp
   
   Los IDs serán mostrados en el orden de ejecución

11. Navega a interner_identity_middleware, crea un archivo .env

   ```bash
   cd src/interner_identity
   touch .env
   ```
   Añade lo siguiente
  ```bash
   VITE_INTERNET_IDENTITY_URL="TU_URL_DE_TUNNEL/?canisterId=INTERNET_IDENTITY_CANISTER_ID"
   ```

12. Navega a react-dashboard y crea un archivo .env

   ```bash
   cd ../react-dashboard
   touch .env
   ```
   Añade lo siguiente
  ```bash
   VITE_CANISTER_ID_BACKEND='BACKEND_CANISTER_ID'
   VITE_CANISTER_ID_NETWORK='NETWORK_CANISTER_ID'
   VITE_CANISTER_ID_QRCODE='QRCODE_CANISTER_ID'
   VITE_CANISTER_ID_NLP='NLP_CANISTER_ID'
   VITE_REACT_APP_INTERNET_COMPUTER_PROVIDER='INTERNET_IDENTITY_CANISTER_ID.localhost:4943/'
  ```

13. Navega a react-native-app y crea un archivo .env

   ```bash
   cd ../react-native-app
   touch .env
   ```
   Añade lo siguiente
  ```bash
   EXPO_PUBLIC_IC_HOST_URL="TU_URL_DE_TUNNEL"
   EXPO_PUBLIC_INTERNET_IDENTITY_MIDDLEWARE_URL='TU_URL_DE_TUNNEL/?canisterId=INTERNET_IDENTITY_MIDDLEWARE_CANISTER_ID'
   EXPO_PUBLIC_APP_LINK="exp://173.16.16.49:8081" (o url generada por expo)
   DFX_NETWORK="local"
   BACKEND_CANISTER_ID="BACKEND_CANISTER_ID"
   ```

14. Vuelve a la raíz del proyecto y despliega el proyecto

    ```bash
    cd ../..
    dfx deploy

15. Navega a react-native-app y corre la app

    ```bash
    npx expo start
