import { useAuth, useCandidActor } from "@bundly/ares-react";
import { View, Text, Pressable, TextInput, StyleSheet, ScrollView, SafeAreaView } from "react-native";
import { CandidActors } from "../../canisters";
import { useState } from "react";


  type Denuncia = {
    id : any;
    denunciante : Denunciante;
    denunciado : Denunciado;
    entidad : string;
    municipio : string;
    bienJuridicoAfectado : string;
    tipoDelito : string;
    subtipo : string;
    hora : string;
    descripcion : string;
    status : string;
  };

  type Denunciante = {
    id : any;
    nombre : string;
    apellidoPaterno : string;
    apellidoMaterno : string;
    calle : string;
    numero : string;
    colonia : string;
    municipio : string;
    estado : string;
    telefono : string;
    email : string;
  };

  type Denunciado = {
    id : any;
    nombre : string;
    apellidoPaterno : string;
    apellidoMaterno : string;
    calle : string;
    numero : string;
    colonia : string;
    municipio : string;
    estado : string;
    telefono : string;
  };

export default function CrearDenunciaScreen() {

    const [denunciaData, setDenunciaData] = useState({} as Denuncia);
    const [denuncianteData, setDenuncianteData] = useState({} as Denunciante);
    const [denunciadoData, setDenunciadoData] = useState({} as Denunciado);

    const { currentIdentity } = useAuth();

    const backend = useCandidActor<CandidActors>("backend", currentIdentity) as CandidActors["backend"];

    const handleChangeText = (field, value , setState) => {
        setState(prevState => ({...prevState, [field]: value}));
    };

    async function handlePress() {
        try {
            // Crea los nuevos objetos localmente antes de setear el estado
            const newDenunciante = { ...denuncianteData, id: Date.now() + 1 };
            const newDenunciado = { ...denunciadoData, id: Date.now() + 2 };
            const newDenuncia: Denuncia = {
                ...denunciaData,
                id: Date.now(),
                denunciante: newDenunciante,
                denunciado: newDenunciado,
                status: 'Pendiente',
                hora: "09:15"
            };

            setDenuncianteData(newDenunciante);
            setDenunciadoData(newDenunciado);
            setDenunciaData(newDenuncia);

            // console.log(typeof newDenuncia.denunciado.municipio);
            // console.log(newDenuncia);
            // console.log(typeof newDenuncia.denunciado.municipio);
            // const result = await backend.addDenuncia(newDenuncia);
            const result = await backend.addDenuncia(newDenuncia);
            console.log(result);

        } catch (error) {
            console.log(error.message);
        }
    }

    return(
        <SafeAreaView style={styles.view}>
        <ScrollView style={styles.content}>
            <Text>
                Crear Denuncia
            </Text>
            <View>
                <View>
                    <Text>Entidad</Text>
                    <TextInput
                        placeholder="Entidad"
                        value={denunciaData.entidad || ''}
                        onChangeText={(text) => handleChangeText('entidad', text, setDenunciaData)}
                    />
                </View>
                <View>
                    <Text>Municipio</Text>
                    <TextInput
                        placeholder="Municipio"
                        value={denunciaData.municipio || ''}
                        onChangeText={(text) => handleChangeText('municipio', text, setDenunciaData)}
                    />
                </View>
                <View>
                    <Text>Bien jurídico afectado</Text>
                    <TextInput
                        placeholder="Bien jurídico afectado"
                        value={denunciaData.bienJuridicoAfectado || ''}
                        onChangeText={(text) => handleChangeText('bienJuridicoAfectado', text, setDenunciaData)}
                    />
                </View>
                <View>
                    <Text>Tipo de delito</Text>
                    <TextInput
                        placeholder="Tipo de delito"
                        value={denunciaData.tipoDelito || ''}
                        onChangeText={(text) => handleChangeText('tipoDelito', text, setDenunciaData)}
                    />
                </View>
                <View>
                    <Text>Subtipo de delito</Text>
                    <TextInput
                        placeholder="Subtipo de delito"
                        value={denunciaData.subtipo || ''}
                        onChangeText={(text) => handleChangeText('subtipo', text, setDenunciaData)}
                    />
                </View>
                <View>
                    <Text>Datos del denunciante</Text>
                    <View>
                        <Text>Nombre</Text>
                        <TextInput
                            placeholder="Nombre del denunciante"
                            value={denuncianteData.nombre || ''}
                            onChangeText={(text) => handleChangeText('nombre', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Apellido Paterno</Text>
                        <TextInput
                            placeholder="Apellido Paterno del denunciante"
                            value={denuncianteData.apellidoPaterno || ''}
                            onChangeText={(text) => handleChangeText('apellidoPaterno', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Apellido Materno</Text>
                        <TextInput
                            placeholder="Apellido Materno del denunciante"
                            value={denuncianteData.apellidoMaterno || ''}
                            onChangeText={(text) => handleChangeText('apellidoMaterno', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Calle</Text>
                        <TextInput
                            placeholder="Calle del denunciante"
                            value={denuncianteData.calle || ''}
                            onChangeText={(text) => handleChangeText('calle', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Número</Text>
                        <TextInput
                            placeholder="Número de casa del denunciante"
                            value={denuncianteData.numero || ''}
                            onChangeText={(text) => handleChangeText('numero', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Colonia</Text>
                        <TextInput
                            placeholder="Colonia del denunciante"
                            value={denuncianteData.colonia || ''}
                            onChangeText={(text) => handleChangeText('colonia', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Municipio</Text>
                        <TextInput
                            placeholder="Municipio del denunciante"
                            value={denuncianteData.municipio || ''}
                            onChangeText={(text) => handleChangeText('municipio', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Estado</Text>
                        <TextInput
                            placeholder="Estado del denunciante"
                            value={denuncianteData.estado || ''}
                            onChangeText={(text) => handleChangeText('estado', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Teléfono</Text>
                        <TextInput
                            placeholder="Número telefónico del denunciante"
                            value={denuncianteData.telefono || ''}
                            onChangeText={(text) => handleChangeText('telefono', text, setDenuncianteData)}
                        />
                    </View>
                    <View>
                        <Text>Email</Text>
                        <TextInput
                            placeholder="Email del denunciante"
                            value={denuncianteData.email || ''}
                            onChangeText={(text) => handleChangeText('email', text, setDenuncianteData)}
                        />
                    </View>
                </View>
                <View>
                    <Text>Datos del denunciado</Text>
                    <View>
                        <Text>Nombre</Text>
                        <TextInput
                            placeholder="Nombre del denunciado"
                            value={denunciadoData.nombre || ''}
                            onChangeText={(text) => handleChangeText('nombre', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Apellido Paterno</Text>
                        <TextInput
                            placeholder="Apellido Paterno del denunciado"
                            value={denunciadoData.apellidoPaterno || ''}
                            onChangeText={(text) => handleChangeText('apellidoPaterno', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Apellido Materno</Text>
                        <TextInput
                            placeholder="Apellido Materno del denunciado"
                            value={denunciadoData.apellidoMaterno || ''}
                            onChangeText={(text) => handleChangeText('apellidoMaterno', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Calle</Text>
                        <TextInput
                            placeholder="Calle del denunciado"
                            value={denunciadoData.calle || '' }
                            onChangeText={(text) => handleChangeText('calle', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Numero</Text>
                        <TextInput
                            placeholder="Numero del denunciado"
                            value={denunciadoData.numero || ''}
                            onChangeText={(text) => handleChangeText('numero', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Colonia</Text>
                        <TextInput
                            placeholder="Colonia del denunciado"
                            value={denunciadoData.colonia || ''}
                            onChangeText={(text) => handleChangeText('colonia', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Municipio</Text>
                        <TextInput
                            placeholder="Municipio del denunciado"
                            value={denunciadoData.municipio || ''}
                            onChangeText={(text) => handleChangeText('municipio', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Estado</Text>
                        <TextInput
                            placeholder="Estado del denunciado"
                            value={denunciadoData.estado || ''}
                            onChangeText={(text) => handleChangeText('estado', text, setDenunciadoData)}
                        />
                    </View>
                    <View>
                        <Text>Teléfono</Text>
                        <TextInput
                            placeholder="Teléfono del denunciado"
                            value={denunciadoData.telefono || ''}
                            onChangeText={(text) => handleChangeText('telefono', text, setDenunciadoData)}
                        />
                    </View>
                </View>
                <View>
                    <Text>Descripción del suceso</Text>
                    <TextInput
                        placeholder="Descripción del suceso"
                        value={denunciaData.descripcion || ''}
                        onChangeText={(text) => handleChangeText('descripcion', text, setDenunciaData)}
                    />
                </View>
                <Pressable onPress={() => handlePress()}>
                    <Text>Hola</Text>
                </Pressable>
            </View>
        </ScrollView>
        </SafeAreaView>
    );
};

const styles = StyleSheet.create({
    pressable: {
        backgroundColor: "white",
        width: "90%"
    },
    content: {
        paddingBottom: 70
    },
    formItem: {
        padding: 8
    },
    view: {
        flex: 1
    }
});
