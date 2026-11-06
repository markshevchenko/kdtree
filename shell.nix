{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
        rustup
        wasm-pack
        sfz
    ];
}
