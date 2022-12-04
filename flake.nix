{
  description = "Linux I2C device interface";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    arc = {
      url = "github:arcnmx/nixexprs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flakelib, nixpkgs, rust, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    devShells = {
      plain = {
        mkShell, writeShellScriptBin, hostPlatform
      , udev
      , pkg-config, python3
      , libiconv
      , enableRust ? true, cargo
      , rustTools ? [ ]
      }: mkShell {
        inherit rustTools;
        buildInputs =
          nixlib.optional hostPlatform.isLinux udev;
        nativeBuildInputs = [ pkg-config python3 ]
          ++ nixlib.optional enableRust cargo
          ++ [
            (writeShellScriptBin "generate" ''nix run .#generate "$@"'')
          ];
        RUST_LOG = "i2c=debug";
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { arc'rustPlatforms'nightly, rust'distChannel, outputs'devShells'plain }: let
        channel = rust'distChannel {
          inherit (arc'rustPlatforms'nightly) channel date manifestPath;
        };
      in outputs'devShells'plain.override {
        inherit (channel) mkShell;
        enableRust = false;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    legacyPackages = { callPackageSet }: callPackageSet {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;
      version = { rust'builders, source }: rust'builders.check-contents {
        src = source;
        patterns = [
          { path = "src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.package) name version;
          }; }
        ];
      };
      test = { rustPlatform, source }: rustPlatform.buildRustPackage {
        pname = self.lib.crate.package.name;
        inherit (self.lib.crate) cargoLock version;
        src = source;
        buildType = "debug";
        meta.name = "cargo test";
      };

      generate = { rust'builders, outputHashes }: rust'builders.generateFiles {
        paths = {
          "lock.nix" = outputHashes;
        };
      };
      outputHashes = { rust'builders }: rust'builders.cargoOutputHashes {
        inherit (self.lib) crate;
      };
    } { };
    checks = {
      version = { rust'builders, source }: rust'builders.check-contents {
        src = source;
        patterns = [
          { path = "src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.package) name version;
          }; }
        ];
      };
      test = { outputs'devShells'plain, rustPlatform, source }: rustPlatform.buildRustPackage {
        pname = self.lib.crate.package.name;
        inherit (self.lib.crate) version cargoLock;
        inherit (outputs'devShells'plain.override { enableRust = false; }) buildInputs nativeBuildInputs;
        src = source;
        cargoBuildFlags = [ "--all" ];
        cargoTestFlags = [ "--all" "--all-features" ];
        buildType = "debug";
        meta.name = "cargo test";
      };
    };
    lib = with nixlib; {
      crate = rust.lib.importCargo {
        path = ./Cargo.toml;
        inherit (import ./lock.nix) outputHashes;
      };
      inherit (self.lib.crate) version;
      releaseTag = "v${self.lib.version}";
    };
    config = rec {
      name = "i2c-linux-rs";
    };
  };
}
