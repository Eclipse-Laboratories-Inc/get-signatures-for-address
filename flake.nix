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

        get-transaction-for-address = pkgs.callPackage ./get-transaction-for-address.nix { inherit naersk; };
        shell = pkgs.mkShell {
          inputsFrom = [ get-transaction-for-address ];

          buildInputs = [
            pkgs.sqlx-cli
          ];
        };
      in
      {
        packages.get-transaction-for-address = get-transaction-for-address;
        packages.default = get-transaction-for-address;

        devShells.default = shell;
      }
    );
}
