{
  description = "Rise up, ziggers";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs ,... }: let
    system = "x86_64-linux";
  in {
    devShells."x86_64-linux".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      # create an environment with nodejs_18, pnpm, and yarn
      packages = with pkgs; [
        cargo
        clippy
        rust-analyzer
        rustc
        rustfmt
      ];

      shellHook = ''
        cargo --version
        cargo clippy --version
        cargo fmt --version
        rustc --version
        rust-analyzer --version

        fish
      '';
    };
  };
}
