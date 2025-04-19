{
  description = "Tauri development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:fractal-tess/nix-systems";
  };

  outputs =
    { nixpkgs, systems, ... }@inputs:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
      packages =
        pkgs: with pkgs; [
          pkg-config
          gobject-introspection
          gtk3
          pango
          cairo
          gdk-pixbuf
          glib
          libsoup_3
          webkitgtk_4_1
          atkmm
          mold

          cargo
          rustc
          rustfmt

          mold
          clang

          nodejs_22
          pnpm
          nodePackages."npm-check-updates"
          npkill
        ];
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          buildInputs = packages pkgs;

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (packages pkgs)}:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS


            export WEBKIT_DISABLE_DMABUF_RENDERER=1
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
            export GDK_BACKEND=x11

            echo "
              ______                 _ 
             /_  __/___ ___  _______(_)
              / / / __ \`/ / / / ___/ / 
             / / / /_/ / /_/ / /  / /  
            /_/  \__,_/\__,_/_/  /_/   

            Tauri Development Environment
            NodeJS - $(${pkgs.nodejs_22}/bin/node --version)
            Rustc - $(${pkgs.rustc}/bin/rustc --version)
            " | ${pkgs.lolcat}/bin/lolcat
          '';
        };
      });
    };
}
