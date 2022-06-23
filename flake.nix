{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    rust-overlay.url = github:oxalica/rust-overlay/master;
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" ];

      # Function to generate a set based on supported systems
      forAllSystems = f:
        nixpkgs.lib.genAttrs supportedSystems (system: f system);

      nixpkgsFor = forAllSystems (system: import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlay
        ];
      });

    in {
      devShell = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rls-preview"
            ];
            targets = [
              "x86_64-unknown-linux-gnu"
            ];
          };
        in import ./shell.nix {
          inherit pkgs rust;
        });
    };
}
