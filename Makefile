
all:
	cargo build --target riscv64imac-unknown-none-elf --release
	ckb-vm-b-cli --bin ./target/riscv64imac-unknown-none-elf/release/try-raw-asm
