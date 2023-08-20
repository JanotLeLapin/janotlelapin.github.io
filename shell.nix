{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      nodejs typescript
      nodePackages.typescript-language-server nodePackages."@astrojs/language-server" nodePackages."@tailwindcss/language-server"
    ];
}
