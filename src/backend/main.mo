import Principal "mo:base/Principal";
import Result "mo:base/Result";

actor {

type ResultDescription = Result.Result<(Text), Text>;

  public query func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };
  public shared ({caller}) func React(): async ResultDescription {
    if (Principal.isAnonymous(caller)) return #err("You must be authenticated to get an OK");
    return #ok("OK");
  };
  public shared ({caller}) func ReactNative(): async ResultDescription {
    if (Principal.isAnonymous(caller)) return #err("You must be authenticated to get an OK");
    return #ok("Todo nice");
  };
};
