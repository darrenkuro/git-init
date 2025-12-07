{
  description = "CLI for initializing repos to GitHub";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "git-init";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        # Optional: if your program has tests
        doCheck = false;

        meta = with pkgs.lib; {
          description = "CLI for initializing repos to GitHub";
          homepage = "https://github.com/darrenkuro/git-init";
          license = licenses.mit;
          maintainers = [maintainers.darrenkuro];
        };
      };
    });
}
