build: 
	date
	RUSTFLAGS="-C linker=lld" cargo build --release --target wasm32-unknown-unknown

# TODO make work with watch, maybe builtfarm
clean:
	cargo clean 

watch:
	nix-shell --run "scripts/watch.sh"
file-watch: watch

