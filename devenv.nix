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

  scripts = { run-dev.exec = "cargo run -- $@"; };

  enterTest = ''
    cargo clippy -- --allow clippy::all --allow clippy::pedantic --allow clippy::restriction
  '';

  # Fix build on nixos
  env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
