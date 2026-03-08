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
        # clippy check
        checks.generator = craneLib.cargoClippy {
          inherit src;

          cargoArtifacts = self.packages.${system}.cargoArtifacts;

          cargoClippyExtraArgs = "-- --deny warnings";
        };
        # fonts
        packages.fonts = pkgs.stdenv.mkDerivation {
          name = "fonts";

          # no src
          dontUnpack = true;

          lexend = builtins.fetchurl {
            url = "https://fonts.gstatic.com/s/lexend/v26/wlpwgwvFAVdoq2_v-6QU.woff2";
            sha256 = "0x68059x1h18l2bv7xzr71jfh3z79677yqnp84l83wdzb67m7rxp";
          };
          hk_grotesk = builtins.fetchurl {
            url = "https://fonts.gstatic.com/s/hankengrotesk/v12/ieVq2YZDLWuGJpnzaiwFXS9tYvBRzyFLlZg_f_Ncs2Zq5vBM.woff2";
            sha256 = "0zfqsmr1k5fs4ysikjdlhijn2i0s3njffk71gjp5jvby1gfkyyvq";
          };

          installPhase = ''
            mkdir -p $out
            cp $lexend $out/lexend.woff2
            cp $hk_grotesk $out/hk-grotesk.woff2
          '';
        };
        # site files
        packages.page = pkgs.stdenv.mkDerivation {
          name = "page";
          version = self.packages.${system}.generator.version;

          # no src
          dontUnpack = true;

          assets = ./assets;

          buildPhase = ''
            mkdir -p $out

            # copy assets
            mkdir -p $out/assets
            cp $assets/* $out/assets

            # copy fonts
            mkdir -p $out/assets/fonts
            cp ${self.packages.${system}.fonts}/* $out/assets/fonts

            # generate site
            ${self.packages.${system}.generator}/bin/page > $out/index.html

            # generate tailwind css
            ${pkgs.tailwindcss_4}/bin/tailwindcss --input $out/assets/input.css --output $out/tailwind.css --cwd $out --minify
            rm $out/assets/input.css # not needed anymore
          '';
        };
        packages.default = self.packages.${system}.page;
        # validator nu check
        checks.vnu = pkgs.stdenv.mkDerivation {
          name = "validator-nu-check";
          src = self.packages.${system}.page;

          nativeBuildInputs = with pkgs; [
            validator-nu
          ];

          buildPhase = ''
            vnu $src/index.html

            mkdir -p $out
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
