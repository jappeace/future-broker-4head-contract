
with import ./pin-unstable.nix { };

stdenv.mkDerivation {
  name = "rust-env";
  # RUST_SRC_PATH = import ./rust-src.nix {
  # inherit stdenv;
  # inherit rustc;
  # };
  nativeBuildInputs = [
    rustc cargo
    inotify-tools
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
