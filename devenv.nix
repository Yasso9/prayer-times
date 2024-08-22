{ pkgs, ... }:

{
  name = "Prayer Times";

  languages.rust.enable = true;
  languages.nix.enable = true;
  packages = with pkgs; [
    # for devenv.nix
    nil
    nixd
    nixfmt-classic
    statix
    deadnix
  ];

  scripts = {
    # run.exec = ''
    #   nix-instantiate --eval --strict test.nix
    # '';
  };
}
