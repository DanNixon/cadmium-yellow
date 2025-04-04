{pkgs, ...}: {
  packages = with pkgs; [
    # Rust toolchain
    rustup

    # Code formatting tools
    treefmt
    alejandra
    mdl
  ];
}
