{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  name = "workers-blog";
  
  buildInputs = [
    cargo
    rustup
    nodejs
    nodePackages.wrangler
  ];
}
