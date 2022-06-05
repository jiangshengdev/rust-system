TARGET := riscv64gc-unknown-none-elf
MODE := debug
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
OBJCOPY := rust-objcopy

clean:
	@cargo clean

elf:
	@python3 build.py

binary: elf
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) -S -O binary \
		$(elf) $(elf).bin; \
	)

build: binary
