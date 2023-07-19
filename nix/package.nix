{ rustPlatform, openssl, pkg-config, lib }:
  rustPlatform.buildRustPackage {
    pname = "iws-rs";
    version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
    meta = with lib; {
      description = "";
      homepage = "https://github.com/StckOverflw/iws-rs";
      license = licenses.mit;
    };
    buildInputs = [
      openssl
    ];
    nativeBuildInputs = [
      pkg-config
    ];
  }