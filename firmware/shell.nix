{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    probe-rs-tools
    rustup
    rustc
    cargo
  ];
}
