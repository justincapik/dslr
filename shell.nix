{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # openssl
    # pkg-config
    rust-bin.stable.latest.default
  ];

  shellHook = ''
    # for kaleido
    export PATH="$HOME/.cargo/bin:$PATH"
  '';
}
