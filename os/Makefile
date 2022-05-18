TARGET := riscv64gc-unknown-none-elf
MODE := debug
TARGET_DIR := target/$(TARGET)/$(MODE)
ELF := $(TARGET_DIR)/os
BIN := $(ELF).bin
BOOTLOADER := ../bootloader/rustsbi-qemu.bin
OBJCOPY := rust-objcopy

clean:
	@cargo clean

elf:
	@cargo build

binary: elf
	@$(OBJCOPY) -S -O binary \
		$(ELF) $(BIN)

build: binary

run: build
	@qemu-system-riscv64 -M virt \
		-display none -serial stdio \
		-bios $(BOOTLOADER) \
		-device loader,file=$(BIN),addr=0x80200000

server: build
	@qemu-system-riscv64 -s -S -M virt \
		-display none -serial stdio \
		-bios $(BOOTLOADER) \
		-device loader,file=$(BIN),addr=0x80200000

client:
	@riscv64-unknown-elf-gdb-py \
		-ex "file $(ELF)" \
		-ex "target remote localhost:1234"
