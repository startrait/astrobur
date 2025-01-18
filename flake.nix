{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ rust-overlay.overlays.default ];
      pkgs = import nixpkgs { inherit system overlays; };

      version = "1.82.0";
      rust = pkgs.rust-bin.stable.${version}.default.override {
        targets = [];
        extensions = [
          "clippy"
          "rust-analyzer"
        ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell { 
      	buildInputs = with pkgs; [ pkg-config openssl ] ++ [ rust ];
	shellHook = ''
		alias -- 'sqlx'='~/.cargo/bin/sqlx'
	'';

      };
    };
}
