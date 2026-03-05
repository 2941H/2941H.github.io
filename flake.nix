{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        rust-toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-qqF33vNuAdU5vua96VKVIwuc43j4EFeEXbjQ6+l4mO4=";
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

        # make source out of html and cargo files
        src = let
          htmlFilter = path: _type: builtins.match ".*html$" path != null;
          htmlOrCargo = path: type:
            (htmlFilter path type) || (craneLib.filterCargoSources path type);
        in
          pkgs.lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source
            filter = htmlOrCargo;
            name = "source"; # Be reproducible, regardless of the directory name
          };
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-toolchain
            pkg-config
          ];
          buildInputs = with pkgs; [
            openssl
          ];
        };
        # expose deps only for possible caching
        packages.cargoArtifacts = craneLib.buildDepsOnly {
          src = craneLib.cleanCargoSource ./.;
        };
        # site generator
        packages.generator = craneLib.buildPackage {
          inherit src;

          cargoArtifacts = self.packages.${system}.cargoArtifacts;
        };
        checks.generator = craneLib.cargoClippy {
          inherit src;

          cargoArtifacts = self.packages.${system}.cargoArtifacts;

          cargoClippyExtraArgs = "-- --deny warnings";
        };
        # site files
        packages.default = pkgs.stdenv.mkDerivation {
          name = "page";
          version = self.packages.${system}.generator.version;

          # no src
          dontUnpack = true;

          buildPhase = ''
            mkdir -p $out

            # generate site
            ${self.packages.${system}.generator}/bin/page > $out/index.html

            # generate tailwind css
            ${pkgs.tailwindcss_4}/bin/tailwindcss --output $out/tailwind.css --cwd $out --minify
          '';
        };
        # open page in browser
        apps.default = let
          program = pkgs.writeShellApplication {
            name = "open-page";
            runtimeInputs = [pkgs.xdg-utils];
            text = ''
              xdg-open ${self.packages.${system}.default}/index.html
            '';
          };
        in {
          type = "app";
          program = "${program}/bin/open-page";
          meta = {
            description = "opens page in browser";
          };
        };
        formatter = pkgs.alejandra;
      }
    );
}
