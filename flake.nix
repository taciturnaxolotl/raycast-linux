{
  description = "A cross-platform translation software";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "raycast-linux";
          version = "2.7.9";

          src = ./.;

          nativeBuildInputs = with pkgs; [
            rustPlatform.cargoSetupHook
            cargo
            rustc
            cargo-tauri
            wrapGAppsHook
            nodePackages.pnpm
            pkg-config
            jq
            moreutils
            swift
            swiftpm
            clang
            llvmPackages.bintools
          ];

          buildInputs = with pkgs; [
            gtk3
            libsoup_2_4
            libayatana-appindicator
            openssl
            webkitgtk_4_1
            xdotool
            cacert
            xorg.libXtst  # Added XTest library
          ];

          # Import Cargo.lock from the src-tauri directory
          cargoDeps = pkgs.rustPlatform.importCargoLock {
            lockFile = ./src-tauri/Cargo.lock;
          };

          # Set up pnpm store directory in a writable location
          PNPM_HOME = "/tmp/pnpm";

          # Add LD_LIBRARY_PATH for SoulverWrapper
          LD_LIBRARY_PATH = "./src-tauri/SoulverWrapper/Vendor/SoulverCore-linux";

          # Override esbuild to use a specific version
          ESBUILD_BINARY_PATH = "${pkgs.lib.getExe (pkgs.esbuild.override {
            buildGoModule = args: pkgs.buildGoModule (args // rec {
              version = "0.18.20";
              src = pkgs.fetchFromGitHub {
                owner = "evanw";
                repo = "esbuild";
                rev = "v${version}";
                hash = "sha256-mED3h+mY+4H465m02ewFK/BgA1i/PQ+ksUNxBlgpUoI=";
              };
              vendorHash = "sha256-+BfxCyg0KkDQpHt/wycy/8CTG6YBA/VJvJFhhzUnSiQ=";
            });
          })}";

          configurePhase = ''
            runHook preConfigure

            # Set up home directory in a writable location
            export HOME=$(mktemp -d)

            # Create a writable directory for pnpm
            mkdir -p $PNPM_HOME
            pnpm config set store-dir $PNPM_HOME

            # Install dependencies with pnpm
            pnpm install --frozen-lockfile --no-optional

            # Make node_modules writable for the build
            chmod -R +w node_modules

            # Navigate to the Tauri directory
            cd src-tauri

            # Set LD_LIBRARY_PATH for SoulverWrapper
            export LD_LIBRARY_PATH="$(pwd)/SoulverWrapper/Vendor/SoulverCore-linux:$LD_LIBRARY_PATH"

            # Fix the libappindicator path
            substituteInPlace $cargoDepsCopy/libappindicator-sys-*/src/lib.rs \
              --replace "libayatana-appindicator3.so.1" "${pkgs.libayatana-appindicator}/lib/libayatana-appindicator3.so.1"

            runHook postConfigure
          '';

          buildPhase = ''
            runHook preBuild

            # Build the Tauri application
            cargo tauri build -b deb

            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall

            # Install the built application
            mkdir -p $out
            mv target/release/bundle/deb/*/data/usr/* $out/

            runHook postInstall
          '';

          meta = with pkgs.lib; {
            description = "A cross-platform translation software";
            mainProgram = "raycast-linux";
            platforms = platforms.linux;
            license = licenses.gpl3Only;
            maintainers = with maintainers; [ linsui ];
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          buildInputs = with pkgs; [
            nodejs
            nodePackages.pnpm
            esbuild
          ];
          shellHook = ''
            export LD_LIBRARY_PATH="$(pwd)/src-tauri/SoulverWrapper/Vendor/SoulverCore-linux:$LD_LIBRARY_PATH"
            # Ensure Swift tools are available in PATH
            export PATH="${pkgs.swift}/bin:$PATH"
            # Set up Swift compiler environment
            export CC="${pkgs.clang}/bin/clang"
            export CXX="${pkgs.clang}/bin/clang++"
          '';
        };
      }
    );
}
