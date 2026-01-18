{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable" ;
        flake-utils.url = "github:numtide/flake-utils" ;
        rust-overlay = {
            url = "github:oxalica/rust-overlay" ;
            inputs = { nixpkgs.follows = "nixpkgs" ; };
        };
    };

	outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
		flake-utils.lib.eachDefaultSystem ( system:
			let
				overlays = [( import rust-overlay )];
				pkgs = import nixpkgs { inherit system overlays ; };
			in {
				devShells.default = pkgs.mkShell {
					buildInputs = with pkgs; [

						# rust
						rust-bin.stable.latest.default
						lld
						pkg-config
						clang

						# capnproto
						capnproto

						# libui
						llvmPackages.libclang
						cmake
						gtk3
						glib
						cairo
						pango
						gdk-pixbuf
						atk

					];
					shellHook = ''
						export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
					'';
				};
			}
		);
}
