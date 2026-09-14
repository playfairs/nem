{ pkgs, inputs }:
let
  treefmt = inputs.treefmt-nix.lib.mkWrapper pkgs {
    programs.nixfmt.enable = true;
    programs.rustfmt.enable = true;
  };
in
  treefmt
