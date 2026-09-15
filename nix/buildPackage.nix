{ pkgs ? import <nixpkgs> { }, nemc ? null }:
let
  compiler = if nemc == null then null else nemc.packages.${pkgs.system}.default;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "nem";
  version = pkgs.lib.strings.trim (builtins.readFile ../VERSION);
  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [ pkgs.makeWrapper ];

  postInstall = if compiler == null then "" else ''
    wrapProgram $out/bin/nem \
      --prefix PATH : ${pkgs.lib.makeBinPath [ compiler ]}
  '';

  meta = {
    mainProgram = "nem";
  };
}
