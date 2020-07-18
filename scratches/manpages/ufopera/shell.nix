with import <nixpkgs> {};

stdenvNoCC.mkDerivation {
  name = "ufopera";
  nativeBuildInputs = [

		python3

		python38Packages.flask

  ];
}
