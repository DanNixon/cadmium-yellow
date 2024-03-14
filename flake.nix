{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = fenix.packages.${system}.toolchainOf {
          channel = "1.74";
          date = "2023-12-07";
          sha256 = "PjvuouwTsYfNKW5Vi5Ye7y+lL7SsWGBxCtBOOm2z14c=";
        };
      in {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            toolchain.toolchain

            # Formatting tools
            treefmt
            alejandra
            mdl
          ];
        };
      }
    );
}
