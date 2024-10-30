{
  rust-bin,
  naersk,
  callPackage,
  lib,
  udev,
  iconv,
  libiconv,
  stdenv,
  darwin,
  openssl,
  bzip2,
  pkg-config,
}:
let
  rust = rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
  rustPlatform = callPackage naersk {
    cargo = rust;
    rustc = rust;
  };
in
rustPlatform.buildPackage {
  src = ./.;
  buildInputs =
    [
      openssl
      openssl.dev
      bzip2
      pkg-config
    ]
    ++ (lib.optionals stdenv.isLinux [ udev ])
    ++ (lib.optionals stdenv.isDarwin [
      iconv
      libiconv
      darwin.apple_sdk.frameworks.SystemConfiguration
    ]);

}
