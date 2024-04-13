bootloader = "./bootloader/rustsbi-qemu.bin"

target:
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/sonari -O binary target/riscv64gc-unknown-none-elf/release/sonari.bin
run:
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios $(bootloader) \
    -device loader,file=./target/riscv64gc-unknown-none-elf/release/sonari.bin,addr=0x80200000
clean:
	cargo clean