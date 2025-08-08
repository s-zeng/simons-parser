{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          nonRustDeps = [
            pkgs.libiconv
          ];

          # Build the library and run tests
          libraryPackage = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            buildInputs = nonRustDeps;

            # Build the library and run tests
            buildPhase = ''
              runHook preBuild
              echo "Building simons-parser library..."
              cargo build --release --lib
              echo "Running tests..."
              cargo test --release
              runHook postBuild
            '';

            # For a library crate, we install the built library files
            installPhase = ''
                            runHook preInstall
                            mkdir -p $out/lib

                            # Copy the built library artifacts
                            if [ -d target/release/deps ]; then
                              find target/release/deps -name "libsimons_parser*.rlib" -exec cp {} $out/lib/ \; 2>/dev/null || true
                              find target/release/deps -name "libsimons_parser*.so" -exec cp {} $out/lib/ \; 2>/dev/null || true
                            fi

                            # Copy any .rlib files from the main target directory
                            find target/release -maxdepth 1 -name "*.rlib" -exec cp {} $out/lib/ \; 2>/dev/null || true

                            # Create a marker file to indicate successful build and test
                            echo "simons-parser library built and tested successfully at $(date)" > $out/lib/build-success

                            # Install a simple test runner script
                            mkdir -p $out/bin
                            cat > $out/bin/simons-parser <<EOF
              #!/usr/bin/env bash
              echo "simons-parser is a library crate. Tests have been run during build."
              echo "Build completed successfully on: \$(cat $out/lib/build-success)"
              echo "Library artifacts available in: $out/lib/"
              ls -la $out/lib/
              EOF
                            chmod +x $out/bin/simons-parser

                            runHook postInstall
            '';

            # Skip the default install check since we're not installing standard binaries
            doInstallCheck = false;
          };
        in
        {
          # Default package - the library with test runner
          packages.default = libraryPackage;

          # Alias for the library
          packages.lib = libraryPackage;

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
            buildInputs = nonRustDeps;
            nativeBuildInputs = with pkgs; [
              just
              rustc
              cargo
              cargo-watch
              rust-analyzer
            ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}
