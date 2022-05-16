clean:
	@cargo clean

build: clean
	@cargo build

objcopy: build
	@rust-objcopy -O binary -S \
		target/riscv64gc-unknown-none-elf/debug/os \
		target/riscv64gc-unknown-none-elf/debug/os.bin

run: objcopy
	@qemu-system-riscv64 -M virt \
		-display none -serial stdio \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000

server: objcopy
	@qemu-system-riscv64 -s -S -M virt \
		-display none -serial stdio \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000

client:
	@riscv64-unknown-elf-gdb-py \
		-ex "file target/riscv64gc-unknown-none-elf/debug/os" \
		-ex "target remote localhost:1234"
