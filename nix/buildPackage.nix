{ pkgs ? import <nixpkgs> { }, }:
pkgs.rustPlatform.buildRustPackage {
  pname = "nem";
  version = pkgs.lib.strings.trim (builtins.readFile ../VERSION);
  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  meta = {
    mainProgram = "nem";
  };
}
