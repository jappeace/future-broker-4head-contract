
with import ./pin-unstable.nix { };

stdenv.mkDerivation {
  name = "rust-env";
  # RUST_SRC_PATH = import ./rust-src.nix {
  # inherit stdenv;
  # inherit rustc;
  # };
  nativeBuildInputs = [
    (rustc.override {
      stdenv = stdenv.override{
        targetPlatform= {
          parsed = {
            cpu = { name = "wasm32"; };
            vendor = {name = "unknown";};
            kernel = {name = "unknown";};
            abi = {name = "unknown";};
          };
        };
      };
    }) cargo wasm-pack
    inotify-tools
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
