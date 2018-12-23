with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "tornado";
    buildInputs = with pkgs; [
    	automake
	autoconf
	libtool
	cmake

        openssl
	curl

        rustup
	gcc
    ];
}

