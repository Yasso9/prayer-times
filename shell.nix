let pkgs = import <nixpkgs> { };
in pkgs.mkShell { buildInputs = [ pkgs.pkg-config pkgs.openssl ]; }
