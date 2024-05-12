{
  description = "Quatrix";

  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  # Based on https://github.com/oxalica/rust-overlay
  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Input pkgs
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Setup crane with toolchain
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # crane define src
        src = craneLib.cleanCargoSource ./.;

        runtimeInputs = (import ./nix/inputs.nix pkgs).runtimeInputs;
        buildInputs = (import ./nix/inputs.nix pkgs).buildInputs ++ [ rustToolchain ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        # build artifacts
        commonArgs = {
          inherit src buildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
    in
    with pkgs;
    {
      devShells.default = mkShell {
        inherit LD_LIBRARY_PATH;

        inputsFrom = [ bin ];
      };
      packages = {
        inherit LD_LIBRARY_PATH;
        inherit bin;
        inherit runtimeInputs;
        default = bin;
      };
    }
  );
}
