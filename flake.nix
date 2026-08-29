{
  description = "nixos-gen-diff: plain-English diff between two NixOS generations";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.toolchain;
        devToolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.toolchain
          fenix.packages.${system}.stable.rust-src
          fenix.packages.${system}.stable.rust-analyzer
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        commonArgs = {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;
          nativeBuildInputs = with pkgs; [
            pkg-config
            autoPatchelfHook
            mold
            makeWrapper
          ];
          buildInputs = [
            pkgs.stdenv.cc.cc.lib
          ];
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold -C link-arg=-B${pkgs.mold}/bin";
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        nixos-gen-diff = craneLib.buildPackage (
          commonArgs
          // {
            postInstall = ''
              wrapProgram $out/bin/nixos-gen-diff \
                --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.nix ]}
            '';
            inherit cargoArtifacts;
          }
        );
      in
      {
        packages.nixos-gen-diff = nixos-gen-diff;
        packages.default = nixos-gen-diff;
        apps.default = {
          type = "app";
          program = "${nixos-gen-diff}/bin/nixos-gen-diff";
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ nixos-gen-diff ];
          nativeBuildInputs = [
            devToolchain
            pkgs.nix
          ];
          RUST_LOG = "debug";
        };
      }
    );
}
