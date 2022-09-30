let
  pkgs = import <nixpkgs> {};
in with pkgs; stdenv.mkDerivation rec {
  name = "shuttle";

  # Allow cargo to download crates.
  SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";

  buildInputs = [
    git
    jetbrains.idea-community
    openssl
    pkg-config
    rustup
  ];
}
