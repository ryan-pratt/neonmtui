{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        runtimeLibs = with pkgs; [
          stdenv.cc.cc.lib
          zlib
          glibc
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
            rust-analyzer
          ];

          env = {
            RUST_BACKTRACE = "1";
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

          NIX_LD = pkgs.lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";
          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

          buildInputs = [ pkgs.zsh ] ++ runtimeLibs;

          shellHook = ''
            if [ -z "$DIRENV_IN_ENVRC" ]; then
              exec zsh
            fi
          '';
        };
      }
    );
}

