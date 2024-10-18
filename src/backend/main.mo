import Principal "mo:base/Principal";
import Result "mo:base/Result";
// import Debug "mo:base/Debug";
import Array "mo:base/Array";

actor {

  type ResultDescription = Result.Result<(Text), Text>;
  type ResultDenuncia = Result.Result<(Denuncia), Text>;

  type Denuncia = {
    id : Nat;
    denunciante : Denunciante;
    denunciado : Denunciado;
    entidad : Text;
    municipio : Text;
    bienJuridicoAfectado : Text;
    tipoDelito : Text;
    subtipo : Text;
    hora : Text;
    descripcion : Text;
    status : Text;
  };

  type ResumenDenuncia = {
    id : Nat;
    entidad : Text;
    municipio : Text;
    bienJuridicoAfectado : Text;
    tipoDelito : Text;
    subtipo : Text;
    hora : Text;
    status : Text;
  };

  type Denunciante = {
    id : Nat;
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
    id : Nat;
    nombre : Text;
    apellidoPaterno : Text;
    apellidoMaterno : Text;
    calle : Text;
    numero : Text;
    colonia : Text;
    municipio : Text;
    estado : Text;
    telefono : Text;
  };

  stable var denuncias : [Denuncia] = [];

  public shared ({caller}) func getAllDenuncias() : async [ResumenDenuncia] {
    if (Principal.isAnonymous(caller)) return [];
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

  public shared ({caller}) func filterDenuncias(filterType: Text, filter: Text): async [ResumenDenuncia] {
    if (Principal.isAnonymous(caller)) return [];
    var resumenes : [ResumenDenuncia] = Array.map<Denuncia, ResumenDenuncia>(
      Array.filter<Denuncia>(denuncias, func(d : Denuncia) : Bool {
        switch (filterType) {
          case ("entidad") {
            return d.entidad == filter;
          };
          case ("municipio") {
            return d.municipio == filter;
          };
          case ("bienJuridicoAfectado") {
            return d.bienJuridicoAfectado == filter;
          };
          case ("tipoDelito") {
            return d.tipoDelito == filter;
          };
          case ("subtipo") {
            return d.subtipo == filter;
          };
          case ("status") {
            switch (d.status) {
              case ("Denuncia") {
                return filter == "Denuncia";
              };
              case ("Pendiente") {
                return filter == "Pendiente";
              };
              case ("Reporte") {
                return filter == "Reporte";
              };
            };
          };
          case (_) {
            return false;
          };
        };
      }),
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

  public shared ({caller}) func updateStatusDenuncia(id: Nat, newStatus: Text) : async ResultDescription {
      if (Principal.isAnonymous(caller)) return #err("You must be authenticated to update a denuncia");
      if (id <= 0) {
          return #err("El id de la denuncia debe ser válido");
      };

      switch (Array.find<Denuncia>(denuncias, func(d : Denuncia) : Bool { d.id == id })) {
          case (null) {
              return #err("Denuncia no encontrada");
          };
          case (_denuncia) {
              denuncias := Array.map<Denuncia, Denuncia>(
                  denuncias,
                  func(d : Denuncia) : Denuncia {
                      if (d.id == id) {
                          return {
                              id = d.id;
                              denunciante = d.denunciante;
                              denunciado = d.denunciado;
                              entidad = d.entidad;
                              municipio = d.municipio;
                              bienJuridicoAfectado = d.bienJuridicoAfectado;
                              tipoDelito = d.tipoDelito;
                              subtipo = d.subtipo;
                              hora = d.hora;
                              descripcion = d.descripcion;
                              status = newStatus;
                          };
                      } else {
                          return d;
                      };
                  }
              );
              return #ok("Status de la denuncia actualizado correctamente");
          };
      };
  };

  public func addDenuncia(denuncia: Denuncia): async ResultDenuncia {
    // Validaciones para los campos del Denunciante
      if (denuncia.denunciante.id == 0) {
          return #err("El id del denunciante no puede ser 0");
      };
      if (denuncia.denunciante.nombre == "") {
          return #err("El nombre del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.apellidoPaterno == "") {
          return #err("El apellido paterno del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.apellidoMaterno == "") {
          return #err("El apellido materno del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.calle == "") {
          return #err("La calle del denunciante no puede estar vacía");
      };
      if (denuncia.denunciante.numero == "") {
          return #err("El número del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.colonia == "") {
          return #err("La colonia del denunciante no puede estar vacía");
      };
      if (denuncia.denunciante.municipio == "") {
          return #err("El municipio del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.estado == "") {
          return #err("El estado del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.telefono == "") {
          return #err("El teléfono del denunciante no puede estar vacío");
      };
      if (denuncia.denunciante.email == "") {
          return #err("El email del denunciante no puede estar vacío");
      };

      // Validaciones para los campos del Denunciado (obligatorios)
      if (denuncia.denunciado.id == 0) {
          return #err("El id del denunciado no puede ser 0");
      };
      if (denuncia.denunciado.nombre == "") {
          return #err("El nombre del denunciado no puede estar vacío");
      };
      if (denuncia.denunciado.apellidoPaterno == "") {
          return #err("El apellido paterno del denunciado no puede estar vacío");
      };

      // Validaciones para otros campos en la Denuncia
      if (denuncia.entidad == "") {
          return #err("La entidad no puede estar vacía");
      };
      if (denuncia.municipio == "") {
          return #err("El municipio no puede estar vacío");
      };
      if (denuncia.bienJuridicoAfectado == "") {
          return #err("El bien jurídico afectado no puede estar vacío");
      };
      if (denuncia.tipoDelito == "") {
          return #err("El tipo de delito no puede estar vacío");
      };
      if (denuncia.hora == "") {
          return #err("La hora no puede estar vacía");
      };
      if (denuncia.descripcion == "") {
          return #err("La descripción no puede estar vacía");
      };
      if (denuncia.id == 0) {
          return #err("El id de la denuncia no puede ser 0");
      };

      // Si todas las validaciones pasan, se agrega la denuncia
      denuncias := Array.append<Denuncia>(denuncias, [denuncia]);
      return #ok(denuncia);
  };

  public query func getDenuncia(id : Nat) : async ResultDenuncia {
    if(id <= 0) {
      return #err("El id de la denuncia debe ser válido");
    };
    switch (Array.find<Denuncia>(denuncias, func(d : Denuncia) : Bool { d.id == id })) {
      case (null) {
        return #err("Denuncia no encontrada");
      };
      case (?denuncia) {
        return #ok(denuncia);
      };
    };
  };

  public query func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };

  public query func getStatus() : async [Text] {
      return [
          "Pendiente",
          "Reporte",
          "Denunciado"
      ];
  };

};
