import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Debug "mo:base/Debug";
import Array "mo:base/Array";

actor {

  type ResultDescription = Result.Result<(Text), Text>;
  type ResultDenuncia = Result.Result<(Denuncia), Text>;

  type Denuncia = {
    id : Text;
    denunciante : Denunciante;
    denunciado : Denunciado;
    entidad : Text;
    municipio : Text;
    bienJuridicoAfectado : Text;
    tipoDelito : Text;
    subtipo : Text;
    hora : Text;
    descripcion : Text;
    status : Status;
  };

  type Status = {
    #Pendiente;
    #EnProceso;
    #Finalizado;
  };

  type ResumenDenuncia = {
  id : Text;
  entidad : Text;
  municipio : Text;
  bienJuridicoAfectado : Text;
  tipoDelito : Text;
  subtipo : Text;
  hora : Text;
  status : Status;
};

  type Denunciante = {
    id : Text;
    nombre : Text;
    apellidoPaterno : Text;
    apellidoMaterno : Text;
    calle : Text;
    numero : Text;
    colonia : Text;
    municipio : Text;
    estado : Text;
    telefono : Text;
    email : Text;
  };

  type Denunciado = {
    id : Text;
    nombre : Text;
    apellidoPaterno : Text;
    apellidoMaterno : ?Text;
    calle : ?Text;
    numero : ?Text;
    colonia : ?Text;
    municipio : ?Text;
    estado : ?Text;
    telefono : ?Text;
  };

  stable var denuncias : [Denuncia] = [];

  public query func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };

  // public func addDenuncia(denuncia : Denuncia) : async ResultDescription {
  //   denuncias := Array.append<Denuncia>(denuncias, [denuncia]);
  //   return #ok("Denuncia agregada");
  // };

  public func getDenuncia(id : Text) : async ResultDenuncia {
    switch (Array.find<Denuncia>(denuncias, func(d : Denuncia) : Bool { d.id == id })) {
      case (null) {
        return #err("Denuncia no encontrada");
      };
      case (?denuncia) {
        return #ok(denuncia);
      };
    };
  };

  public func getAllDenuncias() : async [ResumenDenuncia] {
  var resumenes : [ResumenDenuncia] = Array.map<Denuncia, ResumenDenuncia>(
    denuncias,
    func(d : Denuncia) : ResumenDenuncia {
      var resumen: ResumenDenuncia = {
        id = d.id;
        entidad = d.entidad;
        municipio = d.municipio;
        bienJuridicoAfectado = d.bienJuridicoAfectado;
        tipoDelito = d.tipoDelito;
        subtipo = d.subtipo;
        hora = d.hora;
        status = d.status;
      };
      return resumen;
    },
  );
  return resumenes;
  };

};