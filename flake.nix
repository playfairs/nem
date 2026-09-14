{
  description = "NEM language project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    nemc.url = "github:playfairs/nemc";
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
      nemc,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: {
        nem = pkgs.callPackage ./nix/buildPackage.nix { };
        default = pkgs.callPackage ./nix/buildPackage.nix { };
      });

      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.default}/bin/nem";
        };
      });

      formatter = forAllSystems (
        pkgs:
        import ./nix/formatter.nix {
          inherit pkgs;
          inputs = { inherit treefmt-nix; };
        }
      );

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            self.packages.${pkgs.system}.default
            nemc.packages.${pkgs.system}.default
            pkgs.clang
            pkgs.gcc
            pkgs.nixfmt
            pkgs.rustc
            pkgs.cargo
          ];

          shellHook = ''
            export PATH="${nemc.packages.${pkgs.system}.default}/bin:$PATH"
          '';
        };
      });
    };
}
