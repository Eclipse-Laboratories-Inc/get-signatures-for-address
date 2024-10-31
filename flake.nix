{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        get-signatures-for-address = pkgs.callPackage ./get-signatures-for-address.nix { inherit naersk; };
        shell = pkgs.mkShell {
          inputsFrom = [ get-signatures-for-address ];
        };
      in
      {
        packages.get-signatures-for-address = get-signatures-for-address;
        packages.default = get-signatures-for-address;

        devShells.default = shell;
      }
    );
}
