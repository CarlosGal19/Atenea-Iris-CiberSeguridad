actor {
  public query func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };
  public query func React(): async Text {
    return "React!";
  };
  public query func ReactNative(): async Text {
    return "ReactNative!";
  };
};
