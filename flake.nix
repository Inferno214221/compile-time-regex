{
  description = "Compile Time Regex";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk-pkg = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, naersk-pkg, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-P39FCgpfDT04989+ZTNEdM/k/AE869JKSB4qjatYTSs=";
        };
        naersk = pkgs.callPackage naersk-pkg {
          cargo = toolchain;
          rustc = toolchain;
        };
        buildInputs = with pkgs; [];
        nativeBuildInputs = with pkgs; [
          toolchain
          pkg-config
          gcc
          cargo-expand
          cargo-public-api
          cargo-llvm-cov
          man-pages
        ] ++ buildInputs;
      in with pkgs; rec
      {
        devShells.default = mkShell {
          inherit nativeBuildInputs;
        };

        packages.docs-raw = (naersk.buildPackage rec {
          src = ./.;

          inherit nativeBuildInputs;

          mode = "check";
          doDoc = true;
          doDocFail = true;
          cargoDocCommands = let
            advancedOptions = ''
              -- \
              --theme ${src}/doc/kali-dark.css \
              --html-in-header ${./doc/robots.html} \
              --enable-index-page \
              --generate-macro-expansion \
              -Z unstable-options
            # '';
            # ^ This is really bad but we just use a bash open comment here so that eval doesn't
            # complain about "unexpected syntax token near '||'". We need doDocFail anyway, so the
            # generated code is `|| false`. (No effect)
          in old: [
            "cargo $cargo_options rustdoc -p standard-lib ${advancedOptions}"
            "cargo $cargo_options rustdoc -p ct-regex --all-features ${advancedOptions}"
            "cargo $cargo_options rustdoc -p ct-regex-internal --all-features ${advancedOptions}"
          ];

          postInstall = ''
            cp $src/doc/robots.txt $out/
            cp $src/doc/CNAME $out/
          '';
        }).doc;

        packages.docs = let
          findAndSed = "find ./standard_lib ./ct_regex ./ct_regex_internal -type f -name \"*html\" -exec sed -E";
        in stdenv.mkDerivation {
          name = "compile-time-regex-doc";
          version = "0.1.0";
          src = "${packages.docs-raw}";

          buildPhase = ''
            # Highlight keywords
            ${findAndSed} "s/(>|>([^\">]*[; \[\(])?)(((pub|const|fn|self|Self|struct|enum|type|impl|for|unsafe|as|mut) ?)+)([<& \n:,\)])/\1<span class=\"extra-kw\">\3<\/span>\6/g" -i {} \;
            # Second pass for references and pointers
            ${findAndSed} "s/(>|>([^\">]*[; \[\(]*)?)(mut|const) /\1<span class=\"extra-kw\">\3<\/span> /g" -i {} \;
            # Highlight operators
            ${findAndSed} "s/(>|>([^\">]*[; \[\(\w])?)(&amp;|-&gt;|::|\*)([^/])/\1<span class=\"extra-op\">\3<\/span>\4/g" -i {} \;
            # Where
            ${findAndSed} "s/(<div class=\"where\">)(where)/\1<span class=\"extra-kw\">\2<\/span>/g" -i {} \;
            # TODO: '\w+, mut, <>, (), []
          '';

          installPhase = ''
            mkdir -p $out
            cp -R ./* $out/
          '';
        };

        apps.docs = {
          type = "app";
          program = "${(
            writeShellScript
              "open-docs"
              "${xdg-utils}/bin/xdg-open ${packages.docs}/index.html"
          )}";
        };
      }
    );
}
