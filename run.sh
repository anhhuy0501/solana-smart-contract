#!/bin/bash

function build_bpf() {
    cargo build-bpf --manifest-path=swap_program/Cargo.toml --bpf-out-dir=dist/swap_program
}

case $1 in
    "build-bpf")
	build_bpf
	;;
    "deploy")
	build_bpf
	solana program deploy dist/swap_program/helloworld.so
	;;
    "client")
	(cd client/; timeout 5 cargo run ../dist/swap_program/helloworld-keypair.json)
	;;
    "clean")
	(cd swap_program/; cargo clean)
	(cd client/; cargo clean)
	rm -rf dist/
	;;
    *)
	echo "usage: $0 build-bpf"
	;;
esac
