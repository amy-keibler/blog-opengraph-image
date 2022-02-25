{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.blog-opengraph-image = naersk-lib.buildPackage {
        pname = "blog-opengraph-image";
        root = ./.;
        doCheck = true;
      };
      defaultPackage = packages.blog-opengraph-image;

      # `nix run`
      apps.blog-opengraph-image = utils.lib.mkApp {
        drv = packages.blog-opengraph-image;
        exePath = "/bin/blog-image";
      };
      defaultApp = apps.blog-opengraph-image;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
