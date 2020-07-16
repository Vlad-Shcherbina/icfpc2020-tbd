with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "manpages-scratch-2020";

  buildInputs = [
    mitscheme
  ];

  shellHook = ''
  '';
}
